use crate::telemetry;
use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use spin::Mutex;

type TaskId = usize;
type TaskFn = fn();

#[derive(Copy, Clone)]
pub struct Task {
    id: TaskId,
    func: TaskFn,
    // AI-enhanced fields
    cpu_burst_pred: u32,
}

static RUNQ: Mutex<[Option<Task>; 64]> = Mutex::new([None; 64]);
static mut NEXT_ID: TaskId = 0;
static CURRENT_TASK: AtomicUsize = AtomicUsize::new(usize::MAX);
static LAST_START: AtomicU64 = AtomicU64::new(0);

#[inline(always)]
fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        core::arch::asm!("rdtsc", out("eax") low, out("edx") high);
    }
    ((high as u64) << 32) | (low as u64)
}

fn update_burst_pred(task: &mut Task, actual: u32) {
    if task.cpu_burst_pred == 0 {
        task.cpu_burst_pred = actual;
    } else {
        task.cpu_burst_pred = (task.cpu_burst_pred + actual) / 2;
    }
}

pub fn init() {
    // nothing yet
}

pub fn spawn(f: TaskFn) {
    let mut q = RUNQ.lock();
    unsafe {
        let id = NEXT_ID;
        NEXT_ID += 1;
        q[id] = Some(Task {
            id,
            func: f,
            cpu_burst_pred: 0,
        });
    }
}

pub fn yield_now() {
    telemetry::record_task_switch(u32::MAX);
    let end = rdtsc();
    let current = CURRENT_TASK.load(Ordering::SeqCst);
    if current != usize::MAX {
        let start = LAST_START.load(Ordering::SeqCst);
        let burst = (end - start) as u32;
        let mut q = RUNQ.lock();
        if let Some(task) = q[current].as_mut() {
            update_burst_pred(task, burst);
        }
    }
    CURRENT_TASK.store(usize::MAX, Ordering::SeqCst);
}

pub fn run() -> ! {
    loop {
        let (idx, task) = {
            let q = RUNQ.lock();
            let mut best: Option<(usize, Task)> = None;
            for (i, t) in q.iter().enumerate() {
                if let Some(task) = t {
                    if best.map_or(true, |b| task.cpu_burst_pred < b.1.cpu_burst_pred) {
                        best = Some((i, *task));
                    }
                }
            }
            match best {
                Some(v) => v,
                None => continue,
            }
        };

        CURRENT_TASK.store(idx, Ordering::SeqCst);
        LAST_START.store(rdtsc(), Ordering::SeqCst);
        (task.func)();
        let end = rdtsc();
        let burst = (end - LAST_START.load(Ordering::SeqCst)) as u32;
        {
            let mut q = RUNQ.lock();
            if let Some(t) = q[idx].as_mut() {
                update_burst_pred(t, burst);
            }
        }
        telemetry::record_task_switch(task.id as u32);
        CURRENT_TASK.store(usize::MAX, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;

    #[test]
    fn test_burst_prediction_update() {
        let mut task = Task {
            id: 0,
            func: test_fn,
            cpu_burst_pred: 0,
        };
        update_burst_pred(&mut task, 10);
        assert_eq!(task.cpu_burst_pred, 10);
        update_burst_pred(&mut task, 6);
        assert_eq!(task.cpu_burst_pred, 8);
    }

    fn test_fn() {}
}
