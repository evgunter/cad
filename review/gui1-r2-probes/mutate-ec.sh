#!/usr/bin/env bash
set -uo pipefail
W=/home/user/cad/.claude/worktrees/agent-a65ff4267d7598c57
cd "$W" || exit 1
run() {
  echo "=================== MUTANT: $1 ==================="
  "$W/local-scripts/with-build-slot.sh" -- cargo test -p editor-core --test all -- gui1 2>&1 \
    | grep -E "^test gui1.*(FAILED)|test result:|^error" | sed 's/^/  /'
  git checkout -- crates/editor-core/src
}

# ME1: the early-out skips exact ties.
python3 - <<'PY'
p='crates/editor-core/src/resolve/pick.rs'
s=open(p).read()
old="""            if let Some(b) = &best
                && b.t < cand.t_enter
            {"""
new="""            if let Some(b) = &best
                && b.t <= cand.t_enter
            {"""
assert old in s
open(p,'w').write(s.replace(old,new))
PY
run "ME1 early-out skips exact ties (< becomes <=)"

# ME2: triangle boundaries become OPEN (a shared edge/vertex hits neither face).
python3 - <<'PY'
p='crates/editor-core/src/resolve/pick.rs'
s=open(p).read()
s=s.replace("let u_inside = (0.0..=1.0).contains(&u);","let u_inside = (0.0..1.0).contains(&u) && u > 0.0;")
s=s.replace("let v_inside = v >= 0.0 && u + v <= 1.0;","let v_inside = v > 0.0 && u + v < 1.0;")
open(p,'w').write(s)
PY
run "ME2 triangle boundaries become OPEN"

# ME3: the tie-break prefers the LATER target/triangle.
python3 - <<'PY'
p='crates/editor-core/src/resolve/pick.rs'
s=open(p).read()
old="t < b.t || (t == b.t && (target_pos, cand.item) < (b.target_pos, b.tri_pos))"
new="t < b.t || (t == b.t && (target_pos, cand.item) > (b.target_pos, b.tri_pos))"
assert old in s
open(p,'w').write(s.replace(old,new))
PY
run "ME3 tie-break prefers the LATER patch"

# ME4: target standing checked lazily (only the winner's node).
python3 - <<'PY'
p='crates/editor-core/src/resolve/pick.rs'
s=open(p).read()
old="""    for target in targets {
        match eval.nodes.get(&target.node) {"""
new="""    for target in targets.iter().take(0) {
        match eval.nodes.get(&target.node) {"""
assert old in s
open(p,'w').write(s.replace(old,new))
PY
run "ME4 up-front target-standing check removed"

git status --short crates/editor-core/src
