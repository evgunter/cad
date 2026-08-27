#!/usr/bin/env bash
set -uo pipefail
W=/home/user/cad/.claude/worktrees/agent-a65ff4267d7598c57
cd "$W" || exit 1
echo "=== BASELINE ==="
"$W/local-scripts/with-build-slot.sh" -- cargo test -p bvh --test all 2>&1 | grep -E "test result:|FAILED"
python3 - <<'PY'
p='crates/bvh/src/tree.rs'
s=open(p).read()
s=s.replace("out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter).then(a.item.cmp(&b.item)));",
            "out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter));")
open(p,'w').write(s)
PY
for i in 1 2 3; do
  echo "=== MUTANT M1 (index tie-break dropped), run $i ==="
  "$W/local-scripts/with-build-slot.sh" -- cargo test -p bvh --test all 2>&1 \
    | grep -E "^test (ray|ray_r2)::.*(FAILED)|test result:" | sed 's/^/  /'
done
git checkout -- crates/bvh/src
git status --short crates/bvh/src
