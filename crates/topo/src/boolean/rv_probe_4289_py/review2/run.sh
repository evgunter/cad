#!/bin/bash
# usage: run.sh tag seed eps  -> reads cases_<seed>.txt, writes out_<tag>_<seed>_<eps>.txt and cmp summary
SP=/tmp/claude-0/-home-user-cad/17be02d4-9d3d-5c93-8bb8-ef3f1ac00587/scratchpad/fz
tag=$1; s=$2; e=$3; cases=${4:-cases_$s.txt}
out=$SP/out_${tag}_${s}_${e}.txt
cd /home/user/r2-mut && RV_CASES=$SP/$cases RV_OUT=$out RV_EPS=$e CARGO_TARGET_DIR=/home/user/r2-mut-target cargo nextest run -p topo --lib rv_probe_cases >/dev/null 2>&1 || echo "TEST FAILED $tag $s $e"
cd $SP && RV_EPS=$e python3 cmp.py ${5:-$s} $out ${6:-0} | head -${7:-1} | sed "s/^/$tag s$s e$e: /"
