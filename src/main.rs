#![feature(naked_functions)]

use std::arch::{global_asm, naked_asm};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Default stack size for green thread
const DEFAULT_STACK_SIZE: usize = 1024 * 1024 * 2;
/// Max threads for user tasks running
const MAX_THREADS: usize = 4;

/// Pointer to our runtime, we're only setting this variable on initialization
static mut RUNTIME: usize = 0;

/// Runtime schedule and switch threads
pub struct Runtime {
    threads: Vec<Thread>,
    // the id of the currently running thread
    current: usize,
}

/// Green thread
struct Thread {
    id: usize,
    stack: Vec<u8>,
    ctx: ThreadContext,
    state: State,
}

/// Thread state
#[derive(PartialEq, Eq, Debug)]
enum State {
    // ready to be assigned a task if needed
    Available,
    // running
    Running,
    // ready to move forward and resume execution
    Ready,
}

#[derive(Debug, Default)]
#[repr(C)]
pub struct ThreadContext {
    // return address
    ra: u64,
    // stack pointer
    sp: u64,
    // s0 - s11 (callee saved registers)
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
    // task entry
    entry: u64,
}

impl Thread {
    fn new(id: usize) -> Self {
        Thread {
            id,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state: State::Available,
        }
    }

    fn new_with_state(id: usize, state: State) -> Self {
        Thread {
            id,
            stack: vec![0_u8; DEFAULT_STACK_SIZE],
            ctx: ThreadContext::default(),
            state,
        }
    }
}

impl Runtime {
    pub fn new() -> Self {
        // Base thread is for runtime running
        let base_thread_id = 0;
        let base_thread = Thread::new_with_state(base_thread_id, State::Running);

        // These threads is for user tasks running
        let mut threads = vec![base_thread];
        let mut available_threads = (1..MAX_THREADS + 1).map(|i| Thread::new(i)).collect();
        threads.append(&mut available_threads);

        Runtime {
            threads,
            current: base_thread_id,
        }
    }

    /// This is cheating a bit, but we need a pointer to our Runtime
    /// stored so we can call yield on it even if we don't have a
    /// reference to it.
    pub fn init(&self) {
        unsafe {
            let r_ptr: *const Runtime = self;
            RUNTIME = r_ptr as usize;
        }
    }

    /// Start running our `Runtime`. It will continually call `t_yield()` until
    /// it returns false which means that there is no more work to do.
    pub fn run(&mut self) {
        while self.t_yield() {}
        println!("All tasks finished!");
    }

    /// User tasks call this function to return and schedule a new thread to be run
    fn t_return(&mut self) {
        // Mark current thread available, so it can be assigned a new task
        self.threads[self.current].state = State::Available;

        self.t_schedule();
    }

    /// Suspend current thread and schedule a new thread to be run
    fn t_yield(&mut self) -> bool {
        // Mark current thread ready, so it can be scheduled again
        self.threads[self.current].state = State::Ready;

        self.t_schedule()
    }

    /// Schedule a new thread to be run
    fn t_schedule(&mut self) -> bool {
        let thread_count = self.threads.len();

        // Find next ready thread
        let mut pos = (self.current + 1) % thread_count;
        while self.threads[pos].state != State::Ready {
            pos = (pos + 1) % thread_count;

            // If no other ready thread, means all user tasks finished
            // so current thread must be base thread
            if pos == self.current {
                return false;
            }
        }
        println!("RUNTIME: schedule next thread {} to be run", pos);

        // Switch to a new thread
        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;
        unsafe {
            switch(&mut self.threads[old_pos].ctx, &self.threads[pos].ctx);
        }

        true
    }

    /// Spawn a new task to be executed by a green thread in runtime
    pub fn spawn(&mut self, f: fn()) {
        let available = self
            .threads
            .iter_mut()
            .find(|t| t.state == State::Available)
            .expect("no available green thread.");

        println!("RUNTIME: spawning task on green thread {}", available.id);
        let size = available.stack.len();
        unsafe {
            let s_ptr = available.stack.as_mut_ptr().offset(size as isize);

            // make sure our stack itself is 8 byte aligned
            let s_ptr = (s_ptr as usize & !7) as *mut u8;

            available.ctx.ra = task_return as u64; // task return address
            available.ctx.sp = s_ptr as u64; // stack pointer
            available.ctx.entry = f as u64; // task entry
        }
        available.state = State::Ready;
    }
}

/// When user task completed, then will jump to this function to return
fn task_return() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    }
}

/// Call yield from an arbitrary place in user task code
pub fn r#yield() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    };
}

/// We could use naked function to implement switch function
#[naked]
#[no_mangle]
unsafe extern "C" fn switch(old: *mut ThreadContext, new: *const ThreadContext) {
    // a0: old, a1: new
    naked_asm!(
        "
        sd ra, 0x00(a0)
        sd sp, 0x08(a0)
        sd s0, 0x10(a0)
        sd s1, 0x18(a0)
        sd s2, 0x20(a0)
        sd s3, 0x28(a0)
        sd s4, 0x30(a0)
        sd s5, 0x38(a0)
        sd s6, 0x40(a0)
        sd s7, 0x48(a0)
        sd s8, 0x50(a0)
        sd s9, 0x58(a0)
        sd s10, 0x60(a0)
        sd s11, 0x68(a0)
        sd ra, 0x70(a0)

        ld ra, 0x00(a1)
        ld sp, 0x08(a1)
        ld s0, 0x10(a1)
        ld s1, 0x18(a1)
        ld s2, 0x20(a1)
        ld s3, 0x28(a1)
        ld s4, 0x30(a1)
        ld s5, 0x38(a1)
        ld s6, 0x40(a1)
        ld s7, 0x48(a1)
        ld s8, 0x50(a1)
        ld s9, 0x58(a1)
        ld s10, 0x60(a1)
        ld s11, 0x68(a1)
        ld t0, 0x70(a1)

        jr t0
    "
    );
}

extern "C" {
    /// We could also use global_asm macro to implement switch function
    fn __switch(old: *mut ThreadContext, new: *const ThreadContext);
}
global_asm!(include_str!("switch.S"));

static FINISHED_TASK_COUNT: AtomicUsize = AtomicUsize::new(0);

fn main() {
    let mut runtime = Runtime::new();
    runtime.init();
    runtime.spawn(|| {
        test_task(1);
    });
    runtime.spawn(|| {
        test_task(2);
    });
    runtime.spawn(|| {
        test_task(3);
    });
    runtime.spawn(|| {
        test_task(4);
    });
    runtime.run();
    assert_eq!(FINISHED_TASK_COUNT.load(Ordering::SeqCst), 4);
}

fn test_task(task_id: usize) {
    println!("TASK {} STARTING", task_id);
    for i in 0..4 * task_id {
        println!("task: {} counter: {}", task_id, i);
        r#yield();
    }
    FINISHED_TASK_COUNT.fetch_add(1, Ordering::SeqCst);
    println!("TASK {} FINISHED", task_id);
}
