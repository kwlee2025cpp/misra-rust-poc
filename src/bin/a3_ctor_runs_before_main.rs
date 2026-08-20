//! Rule 10.4.1 - evidence 3 of 4, and the sharpest one.
//!
//! `global_asm!` is not merely assembly sitting inert until something calls it.
//! By emitting a constructor entry it registers itself to run BEFORE `main`,
//! with no call site anywhere in the Rust source and no `unsafe` keyword in
//! this file.
//!
//! Both writes below are unbuffered, so the printed order is the real order.
use std::io::Write;

#[cfg(target_vendor = "apple")]
std::arch::global_asm!(
    ".section __TEXT,__cstring",
    "Lm: .asciz \"[ctor] I ran first.\\n\"",
    ".text",
    ".p2align 2",
    "_poc_ctor:",
    "    stp x29, x30, [sp, #-16]!",
    "    mov x0, #1",
    "    adrp x1, Lm@PAGE",
    "    add  x1, x1, Lm@PAGEOFF",
    "    mov x2, #20",
    "    bl _write",
    "    ldp x29, x30, [sp], #16",
    "    ret",
    ".section __DATA,__mod_init_func",
    ".p2align 3",
    ".quad _poc_ctor",
);

fn main() {
    let mut out = std::io::stdout();
    out.write_all(b"[main] main started.\n").unwrap();
    out.flush().unwrap();

    if cfg!(not(target_vendor = "apple")) {
        println!(
            "(the constructor half of this demo is written for Mach-O; \
             the ELF equivalent uses .init_array)"
        );
    }
}
