#!/bin/bash
D=~/.local/share/cad-work/review-refute-first
cd $D
s=$(date +%s.%N)
BOXFINISH_M11=1 BOXFINISH_M11_NAMES=curve_carrier_clear,curve_carrier_gap /home/user/review-refute-accept-target/debug/demo-tour certified > accept.out 2> accept.err
e=$(date +%s.%N); echo "wall $(echo "$e - $s" | bc)" > accept.done
