// MUST FAIL to compile on rustc >= 1.98.1.
//
// HISTORY: on rustc 1.96.0 (2026-05-25) this file COMPILED AND RAN, and this
// repo cited that as a lint gap in `forbid(unsafe_code)` worth reporting
// upstream. That was wrong by 1.98.1 (2026-09-01), which rejects
// `#[unsafe(naked)]` under the same lint with a dedicated diagnostic:
//
//     error: usage of the unsafe `#[naked]` attribute
//
// The gap was real when found and has since been closed; nothing needs
// reporting. It is kept here as a negative case, and as this repo's own small
// instance of the thing it is about: a toolchain two releases old gave a
// different answer about what is reachable from safe Rust.
#![forbid(unsafe_code)]

#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
use std::arch::naked_asm;

#[cfg(target_arch = "aarch64")]
#[unsafe(naked)]
extern "C" fn seven() -> u32 {
    naked_asm!("mov w0, #7", "ret")
}

#[cfg(target_arch = "x86_64")]
#[unsafe(naked)]
extern "C" fn seven() -> u32 {
    naked_asm!("mov eax, 7", "ret")
}

fn main() {
    println!("{}", seven());
}
