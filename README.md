# Green threads / stackful coroutine implemented in 200 lines of Rust
- Port "Green threads explained in 200 lines of Rust" tutorial to RISCV64 Linux (original [post](https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust) and [repo](https://github.com/cfsamson/example-greenthreads) have been deleted)
- Refactor code for better readability
- Add another way (`global_asm!`) to implement switch function

Chinese blog post: [200 行 Rust 代码实现绿色线程 / 有栈协程](https://systemxlabs.github.io/blog/green-threads-in-200-lines-of-rust/)

## Get started on Ubuntu 24.04
1.install risc-v toolchain
```
sudo apt install gcc-riscv64-linux-gnu g++-riscv64-linux-gnu libc6-dev-riscv64-cross
```

2.install qemu
```
sudo apt install qemu-system-riscv64
sudo apt install qemu-user-static
```

3.add target
```
rustup target add riscv64gc-unknown-linux-gnu
```

4.build and run
```
cargo run
```

output
```
RUNTIME: spawning task on green thread 1
RUNTIME: spawning task on green thread 2
RUNTIME: spawning task on green thread 3
RUNTIME: schedule next thread 1 to be run
TASK 1 STARTING
task: 1 counter: 0
RUNTIME: schedule next thread 2 to be run
TASK 2 STARTING
task: 2 counter: 0
RUNTIME: schedule next thread 3 to be run
TASK 3 STARTING
task: 3 counter: 0
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 1 to be run
task: 1 counter: 1
RUNTIME: schedule next thread 2 to be run
task: 2 counter: 1
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 1
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 1 to be run
task: 1 counter: 2
RUNTIME: schedule next thread 2 to be run
task: 2 counter: 2
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 2
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 1 to be run
task: 1 counter: 3
RUNTIME: schedule next thread 2 to be run
task: 2 counter: 3
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 3
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 1 to be run
TASK 1 FINISHED
RUNTIME: schedule next thread 2 to be run
task: 2 counter: 4
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 4
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 2 to be run
task: 2 counter: 5
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 5
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 2 to be run
task: 2 counter: 6
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 6
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 2 to be run
task: 2 counter: 7
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 7
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 2 to be run
TASK 2 FINISHED
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 8
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 9
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 10
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 3 to be run
task: 3 counter: 11
RUNTIME: schedule next thread 0 to be run
RUNTIME: schedule next thread 3 to be run
TASK 3 FINISHED
RUNTIME: schedule next thread 0 to be run
All tasks finished!
```

Reference
1. [Rust target riscv64gc-unknown-linux-gnu](https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html)
2. [RISC-V Calling Convention](https://riscv.org/wp-content/uploads/2015/01/riscv-calling.pdf)
3. https://github.com/ziyi-yan/green-threads
4. https://github.com/wonbyte/green_threads
5. https://github.com/rcore-os/rCore-Tutorial-v3/blob/29db2e2d9fe4dc1f8db09c8520e97e9713dee102/user/src/bin/stackful_coroutine.rs