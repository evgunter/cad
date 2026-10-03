#!/bin/bash
export CARGO_TARGET_DIR=/home/user/tgt-ver3999
cd /home/user/ver3999
for m in "$@"; do
  git checkout -q -- crates
  python3 /home/user/mut3999/mutants.py $m || { echo "$m APPLYFAIL" >> /home/user/mut3999/summary; continue; }
  git diff --stat | tail -1 > /home/user/mut3999/$m.diffstat
  cargo nextest run -p geom-brep -E 'test(/ssi_limb3_one_arc|ssi::/)' --no-fail-fast > /home/user/mut3999/$m.log 2>&1
  rc=$?
  fails=$(grep -E "^\s+(FAIL|TIMEOUT|SIGABRT|SIGSEGV)" /home/user/mut3999/$m.log | sed -E 's/.*geom-brep(::[a-z_0-9]+)? //' | sort -u | tr '\n' ' ')
  echo "$m rc=$rc $(grep Summary /home/user/mut3999/$m.log) :: $fails" >> /home/user/mut3999/summary
done
git checkout -q -- crates
echo DONE >> /home/user/mut3999/summary
