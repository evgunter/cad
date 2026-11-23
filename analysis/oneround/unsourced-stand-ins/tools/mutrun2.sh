#!/bin/bash
# mutrun.sh <outfile> <eps> <filterexpr|ALL> mutant...
SP=/tmp/claude-0/-home-user-cad/3e284829-abc8-5122-95cb-dce386300ca9/scratchpad
out=$1; eps=$2; filt=$3; shift 3
cd /home/user/dold
for m in "$@"; do
  python3 $SP/mutants2.py $m || { echo "$m APPLYFAIL" >> $out; continue; }
  if [ "$filt" = ALL ]; then fa=(); else fa=(-E "$filt"); fi
  CAD_TOLERANCE_EPS=$eps CARGO_TARGET_DIR=/home/user/tgt-dold cargo nextest run --profile ci -p geom-brep -p topo --no-fail-fast "${fa[@]}" > $SP/mut-$m-$eps.txt 2>&1
  rc=$?
  fails=$(grep -E '^\s+(FAIL|TIMEOUT|SIGABRT|SIGSEGV)' $SP/mut-$m-$eps.txt | sed 's/.*] //' | sort -u | tr '\n' ';')
  summ=$(grep -E 'Summary' $SP/mut-$m-$eps.txt | tail -1)
  echo "$m eps=$eps rc=$rc | $summ | $fails" >> $out
  python3 $SP/mutants2.py $m revert
done
echo DONE >> $out
