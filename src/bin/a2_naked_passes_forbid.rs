//! Rule 10.4.1 — evidence 2 of 3, and the most surprising one.
//!
//! `#![forbid(unsafe_code)]` is the ecosystem's standard way to declare
//! "this crate is safe Rust". It rejects `asm!` and `global_asm!` (see n1/n2).
//!
//! It does NOT reject naked functions — even though the attribute is spelled
//! `#[unsafe(naked)]` and the whole body is raw machine code.
#![forbid(unsafe_code)]

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
    println!(
        "a2: naked function ran under #![forbid(unsafe_code)] and returned {}",
        seven()
    );
}
