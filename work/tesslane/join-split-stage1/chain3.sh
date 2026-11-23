#!/bin/bash
P="JS_RULES_OFF=1 JS_CHORD_FLOOR=1 JS_FOLD_CURE=1 JS_SPLIT=1"
/home/user/js/scripts/run.sh why-9 default $P JS_OFFERED=1
R=/home/user/js/scripts/runsweep.sh
$R sw-base-6 1e-6
$R sw-probe-6 1e-6 $P
$R sw-base-12 1e-12
$R sw-probe-12 1e-12 $P
echo ALLDONE > /home/user/js/chain3.done
