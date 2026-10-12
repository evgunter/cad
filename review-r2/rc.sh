#!/bin/bash
S=/tmp/claude-0/-home-user-cad/4933b454-9319-5389-a7ac-536fed6e8288/scratchpad
cd /home/user/head
bin=target/release/deps/all-75a03b9b4cf81d6d
export CAD_PROBE_DUMP=1 CAD_TOLERANCE_EPS=1e-6
$bin --include-ignored --nocapture --test-threads 4 join_rc_probes:: > $S/rc_n.out 2> $S/rc_n.err
CAD_PROBE_REVERSE=1 $bin --include-ignored --nocapture --test-threads 4 join_rc_probes:: > $S/rc_r.out 2> $S/rc_r.err
cd $S
{
echo "outcome: $(diff <(grep -v 'finished in\|running for over' rc_n.out | sort) <(grep -v 'finished in\|running for over' rc_r.out | sort) | grep -c '^[<>]')"
echo "matching: $(diff <(grep 'PROBE matching' rc_n.err | sort) <(grep 'PROBE matching' rc_r.err | sort) | grep -c '^[<>]')"
echo "inband: $(grep -c 'mutual=true' rc_n.err) picks_moved: $(diff <(grep 'PROBE first_pair' rc_n.err | sort) <(grep 'PROBE first_pair' rc_r.err | sort) | grep -c '^<')"
echo ALLDONE
} > rc.result
