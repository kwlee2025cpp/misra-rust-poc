#!/usr/bin/env bash
# Reproduces every claim in one pass. Run from the repo root.
set -u
echo "rustc: $(rustc --version)"; echo
echo "=========== POSITIVE CASES (these compile) ==========="
for b in a1_global_asm_no_unsafe_block a2_naked_passes_forbid a3_ctor_runs_before_main b1_uninit_heap_read; do
  echo "--- $b"; cargo run --quiet --bin "$b" || echo "  (FAILED)"
done
echo
echo "=========== NEGATIVE CASES (these must NOT compile) ==========="
for n in negative/n1_global_asm_forbidden.rs negative/n2_asm_forbidden.rs; do
  echo "--- $n"
  if rustc --edition 2024 "$n" --emit=metadata -o /tmp/misra_poc_neg.rmeta 2>/tmp/misra_poc_err; then
    echo "  UNEXPECTEDLY COMPILED"
  else
    grep -E '^(error|  = note)' /tmp/misra_poc_err | head -3 | sed 's/^/  /'
  fi
done
