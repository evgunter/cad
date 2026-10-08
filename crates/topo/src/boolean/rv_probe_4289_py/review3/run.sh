#!/bin/bash
# run.sh BIN tag set eps
BIN=$1; tag=$2; s=$3; e=$4
out=out_${tag}_${s}_${e}.txt
RV_CASES=cases_$s.txt RV_OUT=$out RV_EPS=$e $BIN --exact boolean::sectors::rv_probe_4289::rv_probe_cases --test-threads 1 >/dev/null 2>&1 || echo "FAILED $tag $s $e"
RV_EPS=$e python3 cmp.py $s $out 0 | head -1 | sed "s/^/$tag $s $e: /"
