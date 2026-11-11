#!/bin/bash
B=/home/user/fork-svc-a-target/debug/deps/all-e94e0ca4fde1af8f
S=/tmp/claude-0/-home-user-cad/1dc20da4-ec95-5f44-ba30-f41a85f77ec3/scratchpad/af-svc
for i in "$@"; do
  [ -s $S/svc-a-bat-$i.log ] && grep -q "test result" $S/svc-a-bat-$i.log && continue
  SVC_A_I=$i SVC_A_CLEAR=1 SVC_A_UNSHARED_OUT=1 $B join_pierce_runs_sweep::four_pairs_battery --ignored --nocapture > $S/svc-a-bat-$i.log 2>&1
done
