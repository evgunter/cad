#!/bin/bash
. ~/.local/share/cad-work/review-refute-first/env.sh
D=~/.local/share/cad-work/review-refute-first
F='test(=m10_3_driver_interval::the_two_parameter_drive_certifies_after_bisection_and_accounts_for_all_of_it) | test(=m10_4_stackup_interval::the_two_hole_plate_stackup) | test(=m10_sym_drive_memo_interval::the_plain_memo_moves_no_decision) | test(=m10_9_pins_interval::m10_9_what_the_registry_buys)'
for i in 1 2 3; do for t in base head; do
  if [ $t = base ]; then tree=/home/user/rr-base; tgt=/home/user/review-refute-base-target; else tree=/home/user/cad; tgt=/home/user/review-refute-target; fi
  (cd $tree && CARGO_TARGET_DIR=$tgt cargo nextest run -p editor-core --test all --ignore-default-filter --no-fail-fast --test-threads 1 -E "$F" 2>&1) | grep -E "PASS|FAIL" | grep -v "^\s*FAIL \[.*\] .*FAIL" | sed "s/^/$t $i /" >> $D/ec_times.log
done; done
echo DONE >> $D/ec_times.log
