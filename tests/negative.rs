//! Asserts that the files in `negative/` DO NOT compile.
//! A passing test here is the negative half of the 10.4.1 evidence:
//! `forbid(unsafe_code)` really does reject these two assembly mechanisms.
use std::process::Command;

fn must_fail(src: &str, expect: &str) {
    let out = Command::new("rustc")
        .args(["--edition", "2024", src, "--emit=metadata", "-o"])
        .arg(std::env::temp_dir().join("misra_poc_neg.rmeta"))
        .output()
        .expect("failed to invoke rustc");

    assert!(!out.status.success(), "{src} unexpectedly COMPILED");
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(
        err.contains(expect),
        "{src} failed, but not for the expected reason.\n--- stderr ---\n{err}"
    );
}

#[test]
fn global_asm_is_rejected_by_forbid_unsafe_code() {
    must_fail("negative/n1_global_asm_forbidden.rs", "global_asm");
}

#[test]
fn unsafe_block_is_rejected_by_forbid_unsafe_code() {
    must_fail("negative/n2_asm_forbidden.rs", "usage of an `unsafe` block");
}
