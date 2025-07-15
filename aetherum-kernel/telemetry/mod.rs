use spin::Mutex;

const RING_LEN: usize = 1024;

#[derive(Copy, Clone)]
pub struct Sample {
    tsc: u64,
    event: u32,
    arg0: u32,
}

static RING: Mutex<[Sample; RING_LEN]> = Mutex::new(
    [Sample {
        tsc: 0,
        event: 0,
        arg0: 0,
    }; RING_LEN],
);
static mut HEAD: usize = 0;

pub fn init() {
    // Reset ring buffer at boot
    let mut ring = RING.lock();
    for slot in ring.iter_mut() {
        *slot = Sample {
            tsc: 0,
            event: 0,
            arg0: 0,
        };
    }
    unsafe {
        HEAD = 0;
    }
}

pub fn record_task_switch(task_id: u32) {
    push(Sample {
        tsc: rdtsc(),
        event: 1,
        arg0: task_id,
    });
}

fn push(s: Sample) {
    let mut ring = RING.lock();
    unsafe {
        ring[HEAD] = s;
        HEAD = (HEAD + 1) % RING_LEN;
    }
}

/// cheap rdtsc
#[inline(always)]
fn rdtsc() -> u64 {
    let low: u32;
    let high: u32;
    unsafe {
        core::arch::asm!("rdtsc", out("eax") low, out("edx") high);
    }
    ((high as u64) << 32) | (low as u64)
}
