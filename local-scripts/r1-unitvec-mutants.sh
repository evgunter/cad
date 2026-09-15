#!/bin/bash
# R1 mutants on the frozen head; each applied, tested, reverted.
cd /home/user/cad/.claude/worktrees/agent-a6bc830957139e932 || exit 1
export CARGO_TARGET_DIR=/home/user/scalar-unitvec-r1-target CARGO_INCREMENTAL=0
UV=crates/geom-core/src/linalg/unit_vec.rs
FR=crates/geom-core/src/linalg/frame.rs
LOG=/home/user/scalar-unitvec-r1-scratch/mutants.log
: > "$LOG"
run() { # name, filter-expr, packages...
  local name="$1"; local expr="$2"; shift 2
  echo "=== $name" >> "$LOG"
  cargo nextest run "$@" --features geom-core/interval -E "$expr" 2>&1 | grep -E 'Summary|FAIL|PASS.*r1|error\[' | tail -12 >> "$LOG"
  git checkout -- "$UV" "$FR"
}
# (a) witness door re-normalizes before the bare body
sed -i 's|self.0.orthonormal_basis()|self.0.normalize().orthonormal_basis()|' "$UV"
run "a: witness door renormalizes" 'test(orthonormal_basis)' -p geom-core
# (b) underflow question dropped
sed -i 's|if is_underflowed_length(len, v.norm_witness()) {|if false \&\& is_underflowed_length(len, v.norm_witness()) {|' "$UV"
run "b: underflow question dropped" 'test(/unit_vec|underflow|direction/)' -p geom-core -p topo -p editor-core
# (d) ladder mint skips the divide
sed -i 's|Self(v.normalize())|Self(v)|' "$UV"
run "d: after_decided_length skips the divide" 'test(/frame|point_at|path_start|mirror/)' -p geom-core -p editor-core -p sweep
# (e) decided_unit ignores the gate
sed -i 's|definitely_positive(name, v.norm(), v.norm_witness(), band, input)?;|let _ = definitely_positive(name, v.norm(), v.norm_witness(), band, input);|' "$FR"
run "e: decided_unit ignores the gate" 'test(/frame|point_at|path_start|mirror/)' -p geom-core -p editor-core
# (f) sin_cos mint swapped
sed -i 's|Self(Vec3::new(c, s, T::zero()))|Self(Vec3::new(s, c, T::zero()))|' "$UV"
run "f: from_angle_xy swapped" 'test(sin_cos)' -p geom-core
# (g) negation is the identity
sed -i 's|Self(-self.0)|Self(self.0)|' "$UV"
run "g: Neg is identity" 'test(negation)' -p geom-core
# (c) field made pub: the compile_fail doctest must go red
sed -i 's|pub struct UnitVec3<T: Real>(Vec3<T>);|pub struct UnitVec3<T: Real>(pub Vec3<T>);|' "$UV"
echo "=== c: field pub (doctests)" >> "$LOG"
cargo test -p geom-core --doc -- unit_vec 2>&1 | grep -E 'test result|FAILED|failed|compile_fail|unit_vec' | tail -8 >> "$LOG"
git checkout -- "$UV" "$FR"
echo DONE >> "$LOG"
