# Green threads / stackful coroutine implemented in 200 lines of Rust
Running on RISC-V 64 Linux using Qemu.

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

3. build and run
```
cargo run
```

参考：
1. https://doc.rust-lang.org/nightly/rustc/platform-support/riscv64gc-unknown-linux-gnu.html
2. https://github.com/ziyi-yan/green-threads
3. https://github.com/wonbyte/green_threads
4. https://github.com/rcore-os/rCore-Tutorial-v3/blob/29db2e2d9fe4dc1f8db09c8520e97e9713dee102/user/src/bin/stackful_coroutine.rs