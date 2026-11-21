#!/bin/bash
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/tgt-mut CARGO_INCREMENTAL=0
P=/tmp/claude-0/-home-user-cad/9cb1c3a1-5519-5c6a-823b-eb309be9be71/scratchpad/d/mutants2.py
OUT=/tmp/r2_mutants.txt
FILES="crates/geom/src/curves/conic.rs crates/geom/src/curves.rs crates/mesh/src/memo.rs crates/step-import/src/build.rs"
for m in $(python3 $P --list); do
  for f in $FILES; do cp $f /tmp/bak_$(basename $f); done
  python3 $P $m || { echo "$m APPLY-FAILED" >> $OUT; continue; }
  case $m in
    memo-mark-removed) specs="mesh|test(/curve_key|memo::/)";;
    sig-drops-phase) specs="step-import|test(/carrier|signature/)";;
    *) specs="geom|test(/conic::|curves::tests/) - test(/fit::/)";;
  esac
  r=""
  p=${specs%%|*}; f=${specs#*|}
  log=/tmp/r2_mut_${m}.log
  cargo nextest run -p $p --no-fail-fast -E "$f" > $log 2>&1
  s=$(grep -aE "^\s+Summary" $log | tail -1)
  fails=$(grep -aE "^\s+FAIL \[" $log | sed 's/.*) //' | sort -u | tr '\n' ';')
  [ -z "$s" ] && s="NO-SUMMARY $(grep -a '^error' $log | head -2 | tr '\n' ' ')"
  echo "$m :: [$p: ${s##*tests run: } FAILS: $fails]" >> $OUT
  for f in $FILES; do cp /tmp/bak_$(basename $f) $f; done
done
echo DONE >> $OUT
