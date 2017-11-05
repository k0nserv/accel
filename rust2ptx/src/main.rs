
extern crate glob;

use std::path::*;
use std::io::Write;
use std::{fs, env, process};
use glob::glob;

const PTX_BUILDER_TOML: &'static str = r#"
[package]
name = "ptx-builder"
version = "0.1.0"
[profile.dev]
debug = false
"#;

const PTX_BUILDER_XARGO: &'static str = r#"
[dependencies.core]
git = "https://github.com/japaric/core64"
"#;

const PTX_BUILDER_TARGET: &'static str = r#"
{
    "arch": "nvptx64",
    "cpu": "sm_20",
    "data-layout": "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v16:16:16-v32:32:32-v64:64:64-v128:128:128-n16:32:64",
    "linker": "false",
    "linker-flavor": "ld",
    "llvm-target": "nvptx64-nvidia-cuda",
    "max-atomic-width": 0,
    "os": "cuda",
    "panic-strategy": "abort",
    "target-endian": "little",
    "target-c-int-width": "32",
    "target-pointer-width": "64"
}
"#;

const PTX_BUILDER: &'static str = r#"
#![feature(abi_ptx)]
#![no_std]

#[no_mangle]
pub extern "ptx-kernel" fn foo() {}
"#;

fn generate_ptx_builder(work: &Path) {
    let save = |fname: &str, s: &str| {
        let mut f = fs::File::create(work.join(fname)).unwrap();
        f.write(s.as_bytes()).unwrap();
    };
    save("Cargo.toml", PTX_BUILDER_TOML);
    save("Xargo.toml", PTX_BUILDER_XARGO);
    save("nvptx64-nvidia-cuda.json", PTX_BUILDER_TARGET);
    save("src/lib.rs", PTX_BUILDER);
}

fn ready_rustup(work_dir: &Path) {
    let nightly = "nightly-2017-09-01";
    process::Command::new("rustup")
        .args(&["toolchain", "install", nightly])
        .stdout(process::Stdio::null())
        .status()
        .unwrap();
    process::Command::new("rustup")
        .args(&["override", "set", nightly])
        .current_dir(work_dir)
        .stdout(process::Stdio::null())
        .status()
        .unwrap();
}

fn compile(work_dir: &Path) {
    process::Command::new("rm")
        .args(&["-rf", "target"])
        .current_dir(work_dir)
        .status()
        .unwrap();
    process::Command::new("xargo")
        .args(
            &[
                "rustc",
                "--release",
                "--target",
                "nvptx64-nvidia-cuda",
                "--",
                "--emit=asm",
            ],
        )
        .current_dir(work_dir)
        .status()
        .unwrap();
}

fn get_ptx_path(work_dir: &Path) -> PathBuf {
    let pattern = work_dir.join("target/**/*.s");
    for entry in glob(pattern.to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => return path,
            Err(_) => unreachable!(""),
        }
    }
    unreachable!("");
}

fn main() {
    let work = work_dir();
    if !work.exists() {
        fs::create_dir_all(&work).unwrap();
        fs::create_dir_all(work.join("src")).unwrap();
    }
    generate_ptx_builder(&work);
    ready_rustup(&work);
    compile(&work);
    let ptx = get_ptx_path(&work);
    println!("PTX path = {}", ptx.display());
}

fn work_dir() -> PathBuf {
    let home = env::home_dir().unwrap();
    let work = home.join(".rust2ptx");
    work.into()
}
