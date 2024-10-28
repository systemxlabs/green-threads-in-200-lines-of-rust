#![feature(naked_functions)]

use std::arch::naked_asm;

const DEFAULT_STACK_SIZE: usize = 4096;
const MAX_THREADS: usize = 5;

/// Pointer to our runtime, we're only setting this variable on initialization.
static mut RUNTIME: usize = 0;

/// Runtime schedule and switch threads.
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
    // ra (return address)
    x1: u64,
    // sp (stack pointer)
    x2: u64,
    // s0 - s11 (callee saved registers)
    x8: u64,
    x9: u64,
    x18: u64,
    x19: u64,
    x20: u64,
    x21: u64,
    x22: u64,
    x23: u64,
    x24: u64,
    x25: u64,
    x26: u64,
    x27: u64,
    // new return address
    nx1: u64,
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
    /// Initialize with a base thread.
    pub fn new() -> Self {
        let base_thread_id = 0;
        let base_thread = Thread::new_with_state(base_thread_id, State::Running);

        let mut threads = vec![base_thread];
        let mut available_threads = (1..MAX_THREADS).map(|i| Thread::new(i)).collect();
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
        println!("All threads finished!");
    }

    /// Return function that we call when the thread is finished.
    ///
    /// The user of our threads does not call this, we set up our stack so this
    /// is called when the task is done.
    fn t_return(&mut self) {
        // If the calling thread is the base_thread we don't do anything our `Runtime`
        // will call yield for us on the base thread.
        //
        // If it's called from a spawned thread we know it's finished since all
        // threads have a guard function on top of their stack and the only place
        // this function is called is on our guard function.
        if self.current != 0 {
            // Set state to `Available` letting the runtime know it's ready to be assigned a new task.
            self.threads[self.current].state = State::Available;
            // Immediately call `t_yield` which will schedule a new thread to be run.
            self.t_yield();
        }
    }

    /// Schedule a new thread to be run.
    #[inline(never)]
    fn t_yield(&mut self) -> bool {
        let mut pos = self.current;
        while self.threads[pos].state != State::Ready {
            pos += 1;
            if pos == self.threads.len() {
                pos = 0;
            }
            if pos == self.current {
                return false;
            }
        }

        if self.threads[self.current].state != State::Available {
            self.threads[self.current].state = State::Ready;
        }

        self.threads[pos].state = State::Running;
        let old_pos = self.current;
        self.current = pos;

        unsafe {
            switch(&mut self.threads[old_pos].ctx, &self.threads[pos].ctx);
        }
        // Prevents compiler from optimizing our code away on Windows.
        self.threads.len() > 0
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

            available.ctx.x1 = guard as u64; //ctx.x1  is old return address
            available.ctx.nx1 = f as u64; //ctx.nx2 is new return address
            available.ctx.x2 = s_ptr.offset(-32) as u64; //cxt.x2 is sp
        }
        available.state = State::Ready;
    }
}

fn guard() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_return();
    }
}

pub fn yield_thread() {
    unsafe {
        let rt_ptr = RUNTIME as *mut Runtime;
        (*rt_ptr).t_yield();
    };
}

#[naked]
#[no_mangle]
unsafe extern "C" fn switch(old: *mut ThreadContext, new: *const ThreadContext) {
    // a0: _old, a1: _new
    naked_asm!(
        "
        sd x1, 0x00(a0)
        sd x2, 0x08(a0)
        sd x8, 0x10(a0)
        sd x9, 0x18(a0)
        sd x18, 0x20(a0)
        sd x19, 0x28(a0)
        sd x20, 0x30(a0)
        sd x21, 0x38(a0)
        sd x22, 0x40(a0)
        sd x23, 0x48(a0)
        sd x24, 0x50(a0)
        sd x25, 0x58(a0)
        sd x26, 0x60(a0)
        sd x27, 0x68(a0)
        sd x1, 0x70(a0)

        ld x1, 0x00(a1)
        ld x2, 0x08(a1)
        ld x8, 0x10(a1)
        ld x9, 0x18(a1)
        ld x18, 0x20(a1)
        ld x19, 0x28(a1)
        ld x20, 0x30(a1)
        ld x21, 0x38(a1)
        ld x22, 0x40(a1)
        ld x23, 0x48(a1)
        ld x24, 0x50(a1)
        ld x25, 0x58(a1)
        ld x26, 0x60(a1)
        ld x27, 0x68(a1)
        ld t0, 0x70(a1)

        jr t0
    ");
}

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
}

fn test_task(task_id: usize) {
    println!("TASK {} STARTING", task_id);
    for i in 0..4 * task_id {
        println!("task: {} counter: {}", task_id, i);
        yield_thread();
    }
    println!("TASK {} FINISHED", task_id);
}