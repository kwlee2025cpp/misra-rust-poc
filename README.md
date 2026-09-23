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

3. **`global_asm!` can run before `main`** (`src/bin/a3_ctor_runs_before_main.rs`).
   It can emit a constructor entry, so the code executes with no call site
   anywhere in the Rust source and no `unsafe` keyword in the file. A reviewer
   auditing a safe-Rust crate has nothing in the source to look at.

4. **Naked functions pass `#![forbid(unsafe_code)]` entirely**
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

## Run it without cloning (Rust Playground)

Verified on the official Playground, `rustc 1.98.1` (2026-09-01), edition 2024,
x86_64 Linux. The Mach-O demos in this repo use `__DATA,__mod_init_func`; these
use the ELF equivalent `.init_array`.

- **Assembly runs before `main`, no `unsafe` anywhere in the file** — prints
  `[ctor] I ran first.` then `[main] main started.`
  [Run it](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=%2F%2F%20MISRA%20C%2B%2B%3A2023%20Rule%2010.4.1%20counter-example.%0A%2F%2F%20No%20%60unsafe%60%20keyword%20appears%20anywhere%20in%20this%20file%2C%20yet%20assembly%20runs%20before%20main.%0Ause%20std%3A%3Aarch%3A%3Aglobal_asm%3B%0Ause%20std%3A%3Aio%3A%3AWrite%3B%0A%0Aglobal_asm%21%28%0A%20%20%20%20%22.section%20.rodata%22%2C%0A%20%20%20%20%22poc_msg%3A%20.ascii%20%5C%22%5Bctor%5D%20I%20ran%20first.%5C%5Cn%5C%22%22%2C%0A%20%20%20%20%22.text%22%2C%0A%20%20%20%20%22.globl%20poc_ctor%22%2C%0A%20%20%20%20%22poc_ctor%3A%22%2C%0A%20%20%20%20%22%20%20%20%20mov%20rax%2C%201%22%2C%0A%20%20%20%20%22%20%20%20%20mov%20rdi%2C%201%22%2C%0A%20%20%20%20%22%20%20%20%20lea%20rsi%2C%20%5Brip%20%2B%20poc_msg%5D%22%2C%0A%20%20%20%20%22%20%20%20%20mov%20rdx%2C%2020%22%2C%0A%20%20%20%20%22%20%20%20%20syscall%22%2C%0A%20%20%20%20%22%20%20%20%20ret%22%2C%0A%20%20%20%20%22.section%20.init_array%22%2C%0A%20%20%20%20%22.quad%20poc_ctor%22%2C%0A%29%3B%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20std%3A%3Aio%3A%3Astdout%28%29.write_all%28b%22%5Bmain%5D%20main%20started.%5Cn%22%29.unwrap%28%29%3B%0A%7D%0A)
- **`global_asm!` is rejected by `#![forbid(unsafe_code)]`** — *"using this macro
  is unsafe even though it does not need an `unsafe` block"*
  [Run it](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=%23%21%5Bforbid%28unsafe_code%29%5D%0Ause%20std%3A%3Aarch%3A%3Aglobal_asm%3B%0Aglobal_asm%21%28%22.globl%20poc_sym%22%2C%20%22poc_sym%3A%22%2C%20%22ret%22%29%3B%0Afn%20main%28%29%20%7B%7D%0A)
- **Naked functions are also rejected by `#![forbid(unsafe_code)]`** on 1.98.1 —
  *"usage of the unsafe `#[naked]` attribute"*
  [Run it](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&code=%23%21%5Bforbid%28unsafe_code%29%5D%0Ause%20std%3A%3Aarch%3A%3Anaked_asm%3B%0A%0A%23%5Bunsafe%28naked%29%5D%0Aextern%20%22C%22%20fn%20poc_naked%28%29%20-%3E%20u32%20%7B%0A%20%20%20%20naked_asm%21%28%22mov%20eax%2C%207%22%2C%20%22ret%22%29%0A%7D%0A%0Afn%20main%28%29%20%7B%0A%20%20%20%20println%21%28%22naked%20returned%20%7B%7D%22%2C%20poc_naked%28%29%29%3B%0A%7D%0A)

### Correction: the naked-function lint gap was real, and is now closed

Earlier revisions of this repo claimed that `#![forbid(unsafe_code)]` rejects
`global_asm!` but lets naked functions through, and suggested that was a lint gap
worth reporting upstream. That was true on `rustc 1.96.0` (2026-05-25), where
`src/bin/a2_naked_passes_forbid.rs` still compiles and runs. It is **no longer
true on `rustc 1.98.1`**, which rejects `#[unsafe(naked)]` under the same lint
with a dedicated diagnostic. Nothing needs reporting upstream; it has been fixed.

This does not affect the Rule 10.4.1 argument, which rests on `global_asm!`
requiring no `unsafe` block at all, not on the behaviour of the `unsafe_code`
lint. It is also a small illustration of the point the argument is about: a
toolchain two releases old gives a different answer about what is and is not
reachable from safe Rust.

## Why this matters for the classification

MISRA C++:2023 gives 10.4.1 category **Required**, and its rationale concerns the
`asm` declaration being conditionally-supported, producing implementation-defined
behaviour, and harming portability, with intrinsics named as the better modern
alternative. None of it concerns memory safety.

Rust's `unsafe` keyword gates memory safety and nothing else. So "does it need
`unsafe`" does not track what this rule protects, and the harm the rule describes
is reachable from safe Rust via `global_asm!`.

(The standard's text is not quoted here. It is commercially licensed and not
redistributable; the above is a characterisation, not a reproduction.)

## What this does *not* establish

These are facts about **rustc**, not about MISRA. Whether they change a
classification depends on MISRA C++:2023's rationale and amplification text for
10.4.1, which the author does not have access to — only the public MathWorks
headline listing. Corrections welcome.

## Layout

```
src/bin/a1_global_asm_no_unsafe_block.rs   global_asm! with no unsafe block   -> compiles
src/bin/a2_naked_passes_forbid.rs          naked fn under forbid(unsafe_code) -> compiles
src/bin/a3_ctor_runs_before_main.rs        global_asm! ctor runs before main  -> compiles
src/bin/b1_uninit_heap_read.rs             reads unwritten heap storage       -> UB, prints 0
negative/n1_global_asm_forbidden.rs        global_asm! under forbid           -> must fail
negative/n2_asm_forbidden.rs               unsafe block under forbid          -> must fail
tests/negative.rs                          asserts the two negatives fail
run-evidence.sh                            runs everything, prints the table above
```
