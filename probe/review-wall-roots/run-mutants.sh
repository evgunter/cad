#!/usr/bin/env bash
# Each mutant (and the base's two spellings) applied in turn, the rows
# the PR names run against it, the tree restored after.
set -u
cd /home/user/cad
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/home/user/review-wall-target CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only
F='test(quadratic_rows) | test(nearest_rows) | test(wall_root_rows) | test(line_cone_rows) | test(a_ray_from_far) | test(sphere_rows) | test(door_rows)'
for m in "$@"; do
  echo "=== $m"
  if [ "$m" != none ]; then python3 -I probe/review-wall-roots/mutants.py "$m"; fi
  cargo nextest run -p geom-brep --lib --no-fail-fast -E "$F" 2>&1 | grep -E "^\s+(PASS|FAIL|SIGABRT|TIMEOUT)|Summary|error(\[|:)" | sed 's/\[ *[0-9.]*s\] //'
  git checkout -q -- crates
done
echo "=== done"
