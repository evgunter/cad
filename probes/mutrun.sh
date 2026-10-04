S=/tmp/claude-0/-home-user-cad/b7f60dd6-dba8-53f8-9cf2-54b2db674bcf/scratchpad
export CARGO_TARGET_DIR=/home/user/tgt-rev4023/mut CARGO_INCREMENTAL=0 CARGO_PROFILE_RELEASE_DEBUG=0
cd /home/user/rev4023-mut
for m in M1 M2 M3 M4 M5; do
  git checkout -q crates/geom-core/src/linalg/lsq.rs
  python3 $S/mutate.py $m
  echo "=== $m"
  cargo test --release -q -p geom-core --lib lsq 2>&1 | grep -E "test result|FAILED|panicked" | head -6
  cargo test --release -q -p geom --lib fit::tests 2>&1 | grep -E "test result|FAILED" | head -4
  (cd probes/banded4023 && cargo run --release -q 2>&1 | grep -E "Tally" | cut -c1-200)
done
git checkout -q crates/geom-core/src/linalg/lsq.rs
echo MUTDONE
