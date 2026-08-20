//! Rule 11.6.2 - "The value of an object must not be read before it has been set."
//!
//! Separates 11.6.2 from 11.6.1 ("All variables should be initialized"):
//! here NO variable is uninitialized. `p` is a perfectly good pointer.
//! What is never written is the heap storage it addresses.
//!
//! This is undefined behaviour. It usually *looks* fine, because fresh pages
//! from the OS are typically zeroed, which is exactly what makes it dangerous.
use std::alloc::{Layout, alloc, dealloc, handle_alloc_error};

fn main() {
    let layout = Layout::new::<u32>();

    let raw = unsafe { alloc(layout) };
    if raw.is_null() {
        handle_alloc_error(layout);
    }
    let p = raw as *mut u32; // p IS initialized

    let v = unsafe { *p }; // reads bytes never written -> UB
    println!("b1: read {v} from heap storage that was never written (UB)");

    unsafe { dealloc(raw, layout) };
}
