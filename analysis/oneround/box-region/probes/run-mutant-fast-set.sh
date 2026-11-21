#!/bin/bash
cd /home/user/frozen
export CARGO_TARGET_DIR=/home/user/tgt-frozen
[ "$1" != "0" ] && export R2_MUTANT=$1
timeout 3000 cargo nextest run --profile ci -p sweep -p topo -p editor-core --no-fail-fast \
  -E 'not test(review_r2)' > /home/user/fast-$1.log 2>&1
echo "== mutant $1"; grep -E "^\s+Summary|^\s+(FAIL|TIMEOUT|SIGABRT|SIGSEGV)" /home/user/fast-$1.log | sort -u | cut -c1-230
