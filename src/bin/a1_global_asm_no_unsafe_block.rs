//! Rule 10.4.1 — evidence 1 of 3.
//!
//! Claim under test (from the current mapping rationale):
//!   "In safe rust the use of inline assembly is not possible."
//!
//! Result: FALSE as stated. `global_asm!` needs no `unsafe` block.
//! Note this file does NOT set `#![forbid(unsafe_code)]` — see a2/n1 for that.
use std::arch::global_asm;

global_asm!(".global poc_sym", "poc_sym:", "ret");

fn main() {
    println!("a1: global_asm! compiled with no `unsafe` block anywhere in this file.");
}
