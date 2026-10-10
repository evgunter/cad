#!/bin/bash
# r2's mutant runs for PR 4527 at 3cbd7403ab: apply one mutant, run the
# rows that should catch it, revert. Output: mutants-run.txt.
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/tgt-r2 CARGO_INCREMENTAL=0
OUT=review/dm4-unit1-r2/mutants-run.txt
: > $OUT
run() {
  id=$1; filter=$2; file=$3
  python3 review/dm4-unit1-r2/mutate.py $id >> $OUT
  echo "== $id filter: $filter" >> $OUT
  cargo nextest run -p editor-core --no-fail-fast -E "$filter" 2>&1 | grep -E "^\s+(FAIL|SIGABRT|SIGSEGV)|Summary|^error" >> $OUT
  git checkout -- $file
}
run M1 'test(/m4_pr4|docm3_union|dm4_reads|intent_s2_b/)' crates/editor-core/src/eval/wire.rs
run M3 'test(/dm3_indexed|docm3_union|emit_union|emit_nested|lib_g16|placedunion|die_|r2_every/)' crates/editor-core/src/names/emit_union.rs
run M6 'test(/declare|decl|r2_a_declared|docm7|m4_pr5/)' crates/editor-core/src/eval/wire.rs
run M7 'test(/mate|asm|lift|inline|split|refactor/)' crates/editor-core/src/names/role.rs
run M9 'test(/inline|split|refactor|asm|remap/)' crates/editor-core/src/refactor.rs
run M11 'test(/dm3_indexed|persist|golden|corpus_name|plain_reads/)' crates/editor-core/src/operand.rs
echo DONE >> $OUT
