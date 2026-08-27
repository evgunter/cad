#!/usr/bin/env bash
set -uo pipefail
W=/home/user/cad/.claude/worktrees/agent-a65ff4267d7598c57
cd "$W" || exit 1
run() {
  echo "=================== MUTANT: $1 ==================="
  "$W/local-scripts/with-build-slot.sh" -- cargo test -p bvh --test all 2>&1 \
    | grep -E "^test (ray|ray_r2)::|test result:|^error\[" | sed 's/^/  /'
  git checkout -- crates/bvh/src
}

# M1 again, now with the R2 tie-break row present.
python3 - <<'PY'
p='crates/bvh/src/tree.rs'
s=open(p).read()
s=s.replace("out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter).then(a.item.cmp(&b.item)));",
            "out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter));")
open(p,'w').write(s)
PY
run "M1 sort loses the index tie-break"

# M4': the NaN axis now WITNESSES disjointness (an empty interval).
python3 - <<'PY'
p='crates/bvh/src/ray.rs'
s=open(p).read()
old="""    if t0.is_nan() || t1.is_nan() {
        return None;
    }"""
new="""    if t0.is_nan() || t1.is_nan() {
        return Some((f64::INFINITY, f64::NEG_INFINITY));
    }"""
assert old in s
s=s.replace(old,new)
open(p,'w').write(s)
PY
run "M4' NaN axis witnesses disjointness"

# M6: the closed verdict becomes strict (a graze is pruned).
python3 - <<'PY'
p='crates/bvh/src/ray.rs'
s=open(p).read()
s=s.replace("(t_min <= t_max).then_some(t_min)","(t_min < t_max).then_some(t_min)")
open(p,'w').write(s)
PY
run "M6 closed verdict becomes strict"

git status --short crates/
