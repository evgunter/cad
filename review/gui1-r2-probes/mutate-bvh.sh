#!/usr/bin/env bash
# Mutation checks on crates/bvh (R2 review). Restores the tree after each.
set -uo pipefail
W=/home/user/cad/.claude/worktrees/agent-a65ff4267d7598c57
cd "$W" || exit 1
run() {
  local name="$1"
  echo "=================== MUTANT: $name ==================="
  "$W/local-scripts/with-build-slot.sh" -- cargo test -p bvh --test all 2>&1 \
    | grep -E "^test (ray|determinism)::|test result:|^error" | sed 's/^/  /'
  git checkout -- crates/bvh/src crates/editor-core/src 2>/dev/null
}

# M1: drop the index tie-break from the documented sort order.
python3 - <<'PY'
import re,io
p='crates/bvh/src/tree.rs'
s=open(p).read()
s=s.replace("out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter).then(a.item.cmp(&b.item)));",
            "out.sort_unstable_by(|a, b| a.t_enter.total_cmp(&b.t_enter));")
open(p,'w').write(s)
PY
run "M1 sort loses the index tie-break"

# M2: leaves stop re-testing each item's own box (hull acceptance leaks tree shape).
python3 - <<'PY'
p='crates/bvh/src/tree.rs'
s=open(p).read()
old="""                    for &item in self.items.iter().skip(*start).take(*count) {
                        if let Some(t_enter) = self.boxes.get(item).and_then(|b| ray.slab_enter(b))
                        {
                            out.push(RayCandidate { item, t_enter });
                        }
                    }"""
new="""                    let t_hull = ray.slab_enter(aabb).unwrap_or(0.0);
                    for &item in self.items.iter().skip(*start).take(*count) {
                        out.push(RayCandidate { item, t_enter: t_hull });
                    }"""
assert old in s
s=s.replace(old,new)
open(p,'w').write(s)
PY
run "M2 leaves accept on hull without per-item re-test"

# M3: outward widening removed (widen_down/up become identity).
python3 - <<'PY'
p='crates/bvh/src/ray.rs'
s=open(p).read()
s=s.replace("    v.next_down().next_down().next_down().next_down()\n","    v\n")
s=s.replace("    v.next_up().next_up().next_up().next_up()\n","    v\n")
open(p,'w').write(s)
PY
run "M3 4-ULP outward widening removed"

# M4: the NaN arm stops skipping the axis (NaN can now witness disjointness).
python3 - <<'PY'
p='crates/bvh/src/ray.rs'
s=open(p).read()
old="""    if t0.is_nan() || t1.is_nan() {
        return None;
    }"""
new="""    if t0.is_nan() || t1.is_nan() {
        return Some((f64::NAN, f64::NAN));
    }"""
assert old in s
s=s.replace(old,new)
open(p,'w').write(s)
PY
run "M4 NaN axis no longer skipped"

# M5: the t>=0 domain floor removed (behind-origin boxes may report negative t_enter).
python3 - <<'PY'
p='crates/bvh/src/ray.rs'
s=open(p).read()
s=s.replace("        let mut t_min = 0.0f64;","        let mut t_min = f64::NEG_INFINITY;")
open(p,'w').write(s)
PY
run "M5 the t>=0 domain floor removed"

git status --short crates/
