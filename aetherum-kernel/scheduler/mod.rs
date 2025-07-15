use crate::telemetry;
use spin::Mutex;

type TaskId = usize;
type TaskFn = fn();

#[derive(Copy, Clone)]
pub struct Task {
    id: TaskId,
    func: TaskFn,
    // AI-enhanced fields
    cpu_burst_pred: u32, // AIKLE: placeholder
}

static RUNQ: Mutex<[Option<Task>; 64]> = Mutex::new([None; 64]);
static mut NEXT_ID: TaskId = 0;

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
    // In this toy scheduler we simply record a yield event
    telemetry::record_task_switch(u32::MAX);
    core::hint::spin_loop();
}

pub fn run() -> ! {
    loop {
        // simple round-robin prototype
        let mut q = RUNQ.lock();
        for slot in q.iter_mut() {
            if let Some(task) = slot {
                (task.func)();
                telemetry::record_task_switch(task.id as u32); // AI hook
            }
        }
    }
}
