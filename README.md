# misra-rust-poc

Small, runnable evidence for two questions in the **MISRA C++:2023 → Rust** mapping
([issue #575](https://github.com/Safety-Critical-Rust-Consortium/safety-critical-rust-coding-guidelines/issues/575),
[PR #1226](https://github.com/Safety-Critical-Rust-Consortium/safety-critical-rust-coding-guidelines/pull/1226)).

Everything here compiles and runs. Reproduce it all with:

```
./run-evidence.sh
cargo test
```

Verified on `rustc 1.96.0 (ac68faa20 2026-05-25)`, edition 2024, aarch64-apple-darwin.

---

## Rule 10.4.1 — "The `asm` declaration shall not be used"

The mapping rationale currently says:

> In safe rust the use of inline assembly is not possible, therefore this rule
> doesn't apply to safe rust.

Testing the three ways to write assembly in Rust gives a more nuanced picture:

| Mechanism | Needs an `unsafe` block? | Rejected by `#![forbid(unsafe_code)]`? |
|---|---|---|
| `asm!` | yes | yes — *"usage of an `unsafe` block"* |
| `global_asm!` | **no** | yes — *"using this macro is unsafe even though it does not need an `unsafe` block"* |
| `#[unsafe(naked)]` + `naked_asm!` | **no** | **no — compiles cleanly** |

Three conclusions:

1. **The quoted sentence is false as written.** `global_asm!` compiles with no
   `unsafe` block anywhere (`src/bin/a1_global_asm_no_unsafe_block.rs`).

2. **The unsafe-only classification is still defensible** — but the criterion
   should be `forbid(unsafe_code)`, not impossibility. The compiler states
   outright that `global_asm!` *is* unsafe code that merely doesn't need the
   block syntax (`negative/n1_global_asm_forbidden.rs`). That is a precise,
   checkable criterion; "not possible" is not.

3. **Naked functions pass `#![forbid(unsafe_code)]` entirely**
   (`src/bin/a2_naked_passes_forbid.rs`). A whole function body of raw machine
   code compiles inside a crate that has declared itself free of unsafe code.
   Naked functions were stabilized in Rust 1.88, after the lint gained its
   `global_asm!` coverage, so this may simply be a gap in `unsafe_code` worth
   reporting upstream. Either way it means **naked functions are assembly that
   no standard safety gate currently catches**, which is relevant both to how
   10.4.1 is worded and to the consortium's Clippy lints goal.

## Rule 11.6.2 — "The value of an object must not be read before it has been set"

`src/bin/b1_uninit_heap_read.rs` reads heap storage that was never written:

```rust
let p = unsafe { alloc(layout) } as *mut u32;  // p IS initialized
let v = unsafe { *p };                         // never written -> UB
```

Two points for the rationale:

- It **separates 11.6.2 from 11.6.1**. No *variable* is uninitialized here — `p`
  is a perfectly good pointer. What is unset is the *object* it addresses, which
  is exactly the noun 11.6.2 uses. (`MaybeUninit` is a poor example for this,
  because the ratified 11.6.1 rationale already names `MaybeUninit` as the reason
  11.6.1 applies to unsafe Rust — both rules are engaged there, not just one.)
- It **printed `0`**. Fresh pages from the OS are usually zeroed, so this UB
  routinely looks like it works. Reading uninitialized memory is UB even for
  integer types — not "you get a garbage value" — which is the part most often
  misunderstood, and the reason `MaybeUninit` exists.

---

## What this does *not* establish

These are facts about **rustc**, not about MISRA. Whether they change a
classification depends on MISRA C++:2023's rationale and amplification text for
10.4.1, which the author does not have access to — only the public MathWorks
headline listing. Corrections welcome.

## Layout

```
src/bin/a1_global_asm_no_unsafe_block.rs   global_asm! with no unsafe block   -> compiles
src/bin/a2_naked_passes_forbid.rs          naked fn under forbid(unsafe_code) -> compiles
src/bin/b1_uninit_heap_read.rs             reads unwritten heap storage       -> UB, prints 0
negative/n1_global_asm_forbidden.rs        global_asm! under forbid           -> must fail
negative/n2_asm_forbidden.rs               unsafe block under forbid          -> must fail
tests/negative.rs                          asserts the two negatives fail
run-evidence.sh                            runs everything, prints the table above
```
