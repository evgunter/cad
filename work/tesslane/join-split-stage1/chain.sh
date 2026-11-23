#!/bin/bash
S=/home/user/js/scripts/run.sh
P="JS_RULES_OFF=1 JS_CHORD_FLOOR=1 JS_FOLD_CURE=1 JS_SPLIT=1"
$S probe-9 default $P
$S base-6 1e-6
$S probe-6 1e-6 $P
$S base-12 1e-12
$S probe-12 1e-12 $P
echo ALLDONE > /home/user/js/chain.done
