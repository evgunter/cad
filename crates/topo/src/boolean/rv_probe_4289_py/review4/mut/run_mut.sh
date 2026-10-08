#!/bin/bash
# run_mut.sh NAME... : per mutant, build, run sectors+vtxfac unit rows and cone_fuzz, and the review-4 harness
R=/home/user/probe/crates/topo/src/boolean/rv_probe_4289_py/review4
O=$R/mut/results; mkdir -p $O
cd /home/user/probe
export CARGO_TARGET_DIR=/home/user/tgt2
for m in "$@"; do
  git checkout -q crates/topo/src/boolean/sectors.rs
  python3 $R/mut/mut.py $m || { echo "$m: apply failed" >> $O/summary.txt; continue; }
  cargo test -p topo --lib --no-run > $O/$m.build 2>&1 || { echo "$m: build failed" >> $O/summary.txt; continue; }
  BIN=$(ls -t /home/user/tgt2/debug/deps/topo-* | grep -v '\.d$' | head -1)
  $BIN boolean::sectors boolean::vtxfac --include-ignored > $O/$m.unit 2>&1
  fails=$(grep -E "^test .* FAILED$" $O/$m.unit | sed 's/^test //; s/ \.\.\. FAILED//' | tr '\n' ' ')
  cd $R
  h=""
  for s in s2e1e-9 s3e1e-9 s2e1e-6 s2e1e-12; do
    e=${s#*e}; e=${e#*e}; e=1e${s##*e1e}
    RV_CASES=cases_adv_$s.txt RV_OUT=mut/results/out_${m}_$s.txt RV_EPS=$e $BIN --exact boolean::sectors::rv_probe_4289::rv_probe_cases --test-threads 1 >/dev/null 2>&1
    d=$(cmp -s out_adv_$s.txt mut/results/out_${m}_$s.txt && echo same || echo "$(diff out_adv_$s.txt mut/results/out_${m}_$s.txt | grep -c '^>') moved")
    w=$(python3 adv.py check $s mut/results/out_${m}_$s.txt $e 2>/dev/null | head -1 | grep -oE "'(WRONG|FLIP)_[a-z]+': [0-9]+" | tr '\n' ' ')
    h="$h [$s: $d $w]"
  done
  for c in dl fac; do
    RV_CASES=cases_$c.txt RV_OUT=mut/results/out_${m}_$c.txt RV_EPS=1e-9 $BIN --exact boolean::sectors::rv_probe_4289::rv_probe_cases --test-threads 1 >/dev/null 2>&1
    ref=out_$c.txt; [ $c = fac ] && ref=out_fac_head.txt
    d=$(cmp -s $ref mut/results/out_${m}_$c.txt && echo same || echo "$(diff $ref mut/results/out_${m}_$c.txt | grep -c '^>') moved")
    h="$h [$c: $d]"
  done
  cd /home/user/probe
  echo "$m: unit fails: ${fails:-none} | harness:$h" >> $O/summary.txt
done
git checkout -q crates/topo/src/boolean/sectors.rs
echo MUTDONE >> $O/summary.txt
