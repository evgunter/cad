#!/bin/bash
S=/tmp/claude-0/-home-user-cad/632e7b18-4656-5a40-9f73-5afebc684e10/scratchpad
w=$1; B=$(grep -o "/tmp[^)]*all-[0-9a-f]*" $S/sweepall-$w.log | tail -1)
cd $S/$w
$B pinch_runs_battery --ignored --nocapture --test-threads 1 > $S/batt-pinch.$w.txt 2>&1
RCW_SHARD=21/84 $B rc_wide_battery --ignored --nocapture --test-threads 1 > $S/batt-rc21.$w.txt 2>&1
RCW_SHARD=62/84 $B rc_wide_battery --ignored --nocapture --test-threads 1 > $S/batt-rc62.$w.txt 2>&1
echo DONE > $S/batt-$w.done
