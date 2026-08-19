// MUST FAIL to compile. `forbid(unsafe_code)` rejects global_asm!.
#![forbid(unsafe_code)]
use std::arch::global_asm;
global_asm!(".global n1_sym", "n1_sym:", "ret");
fn main() {}
