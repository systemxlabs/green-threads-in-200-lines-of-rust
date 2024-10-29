# Green threads / stackful coroutine implemented in 200 lines of Rust
Port "Green threads explained in 200 lines of Rust" tutorial (original [post](https://cfsamson.gitbook.io/green-threads-explained-in-200-lines-of-rust) and [repo](https://github.com/cfsamson/example-greenthreads) have been deleted) to RISCV64 Linux and refactor code for better readability.

## Get started on Ubuntu 24.04
1. install risc-v toolchain
```
sudo apt install gcc-riscv64-linux-gnu g++-riscv64-linux-gnu libc6-dev-riscv64-cross
```

2. install qemu
```
sudo apt install qemu-system-riscv64
sudo apt install qemu-user-static
```

3. add target
```
rustup target add riscv64gc-unknown-linux-gnu
```

4. build and run
```
cargo run
```

Reference
1. [Rust target riscv64gc-unknown-linux-gnu](https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html)
2. [RISC-V Calling Convention](https://riscv.org/wp-content/uploads/2015/01/riscv-calling.pdf)
3. https://github.com/ziyi-yan/green-threads
4. https://github.com/wonbyte/green_threads
5. https://github.com/rcore-os/rCore-Tutorial-v3/blob/29db2e2d9fe4dc1f8db09c8520e97e9713dee102/user/src/bin/stackful_coroutine.rs