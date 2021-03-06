Accel: GPGPU Framework for Rust
================================

[![Crate](http://meritbadge.herokuapp.com/accel)](https://crates.io/crates/accel)
[![docs.rs](https://docs.rs/accel/badge.svg)](https://docs.rs/accel)
[![CircleCI](https://circleci.com/gh/termoshtt/accel.svg?style=shield)](https://circleci.com/gh/termoshtt/accel)

CUDA-based GPGPU framework for Rust

Features
---------

- Compile PTX Kernel from Rust using NVPTX backend of LLVM (demonstrated in [japaric/nvptx](https://github.com/japaric/nvptx))
- [proc-macro-attribute](https://github.com/rust-lang/rust/issues/38356)-based approach like [futures-await](https://github.com/alexcrichton/futures-await)
- Simple memory management using [Unified Memory](http://docs.nvidia.com/cuda/cuda-c-programming-guide/index.html#um-unified-memory-programming-hd)

Sub Crates
-----------
- [accel-derive](accel-derive/README.md): Define procedual macro `#[kernel]`
- [accel-core](accel-core/README.md): Support crate for writing GPU kernel
- [nvptx](nvptx): Compile Rust into PTX using [LLVM/NVPTX backend](https://llvm.org/docs/NVPTXUsage.html)
- [cuda-sys](cuda-sys/README.md): Rust binding to CUDA Driver/Runtime APIs

Pre-requirements
---------------

- Install [CUDA](https://developer.nvidia.com/cuda-downloads) on your system
- Install [LLVM](https://llvm.org/) 6.0 or later (use `llc` and `llvm-link` to create PTX)
- Install Rust using [rustup.rs](https://github.com/rust-lang-nursery/rustup.rs)
- Install [xargo](https://github.com/japaric/xargo), a sysroot manager

Or, you can use [termoshtt/rust-cuda](https://hub.docker.com/r/termoshtt/rust-cuda/) container whith satisfies these requirements.

```
docker run -it --rm --runtime=nvidia termoshtt/rust-cuda
```

See also [nvidia-docker](https://github.com/NVIDIA/nvidia-docker)

Example
--------

```rust
#![feature(proc_macro)]

extern crate accel;
extern crate accel_derive;

use accel_derive::kernel;
use accel::*;

#[kernel]
#[crate("accel-core" = "0.2.0-alpha")]
#[build_path("~/.rust2ptx")]
pub unsafe fn add(a: *const f64, b: *const f64, c: *mut f64, n: usize) {
    let i = accel_core::index();
    if (i as usize) < n {
        *c.offset(i) = *a.offset(i) + *b.offset(i);
    }
}

fn main() {
    let n = 32;
    let mut a = UVec::new(n).unwrap();
    let mut b = UVec::new(n).unwrap();
    let mut c = UVec::new(n).unwrap();

    for i in 0..n {
        a[i] = i as f64;
        b[i] = 2.0 * i as f64;
    }
    println!("a = {:?}", a.as_slice());
    println!("b = {:?}", b.as_slice());

    let grid = Grid::x(1);
    let block = Block::x(n as u32);
    add(grid, block, a.as_ptr(), b.as_ptr(), c.as_mut_ptr(), n);

    device::sync().unwrap();
    println!("c = {:?}", c.as_slice());
}
```

Licence
--------
MIT-License
