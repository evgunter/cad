#!/bin/bash
# Reviewer r1: apply each mutant, run the anchored rows, record red/green.
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/tgt-head
OUT=${OUT:-/tmp/r1_mutants.txt}
F_GEOM='test(/conic|curves::|anchor|seam|ellipse|circle|cover|param_near/) - test(/fit::/)'
F_BREP='test(/anchor|cone_section|far_centred|running|resolution|conic_oracle|review_anchored/)'
F_MESH='test(/curve_key|memo/)'
for m in ${MUTANTS:-$(python3 analysis/oneround/anchored-conic/review-r1/mutants.py --list)}; do
  python3 analysis/oneround/anchored-conic/review-r1/mutants.py $m || { echo "$m APPLY-FAILED" >> $OUT; continue; }
  r=""
  for spec in "geom|$F_GEOM" "geom-brep|$F_BREP" "mesh|$F_MESH" ${EXTRA}; do
    p=${spec%%|*}; f=${spec#*|}
    log=/tmp/r1_mut_${m}_${p}.log
    cargo nextest run -p $p --no-fail-fast -E "$f" > $log 2>&1
    s=$(grep -E "^\s+Summary" $log | tail -1)
    fails=$(grep -E "^\s+FAIL \[" $log | sed 's/.*) //' | sort -u | tr '\n' ';')
    r="$r [$p: ${s##*tests run: } FAILS: $fails]"
  done
  echo "$m :: $r" >> $OUT
  git checkout -q crates/geom/src
done
echo DONE >> $OUT
