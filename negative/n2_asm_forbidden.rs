// MUST FAIL to compile. `forbid(unsafe_code)` rejects an unsafe block.
#![forbid(unsafe_code)]
use std::arch::asm;
fn main() { unsafe { asm!("nop") } }
