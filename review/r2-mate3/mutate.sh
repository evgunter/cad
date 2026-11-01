#!/usr/bin/env bash
# R2 mutation harness for MATE-3 (PR 1423): flip one arm of the verdict
# table, run the suites, record which COMMITTED rows go red.
# Usage: mutate.sh <name>   (applies mutation, runs, restores)
set -u
ROOT=/home/user/cad/.claude/worktrees/agent-a479edbadbd8a58b3
D=$ROOT/crates/geom-brep/src/dihedral.rs
V=$ROOT/crates/topo/src/validate.rs
cd "$ROOT" || exit 1
git stash list >/dev/null

restore() { git checkout -- "$D" "$V" "$ROOT/crates/profile/src/path.rs"; }

apply() {
case "$1" in
  M1_pairing_sign_flip)
    python3 - "$D" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="        Sign::Positive => Ok(MaterialPairing::Aligned),\n        Sign::Negative => Ok(MaterialPairing::Opposed),"
b="        Sign::Positive => Ok(MaterialPairing::Opposed),\n        Sign::Negative => Ok(MaterialPairing::Aligned),"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M2_kappa_drop_negation)
    python3 - "$D" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="    -(kappa_rel * sense_plus)"; b="    kappa_rel * sense_plus"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M3_kappa_drop_sense)
    python3 - "$D" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="    -(kappa_rel * sense_plus)"; b="    -kappa_rel"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M4_decl_ignore_class)
    python3 - "$V" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="        d.class == crate::contact::ContactClass::Tangent\n            && ((d.a == a && d.b == b) || (d.a == b && d.b == a))"
b="        ((d.a == a && d.b == b) || (d.a == b && d.b == a))"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M5_decl_ignore_pair)
    python3 - "$V" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="        d.class == crate::contact::ContactClass::Tangent\n            && ((d.a == a && d.b == b) || (d.a == b && d.b == a))"
b="        d.class == crate::contact::ContactClass::Tangent"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M6_decl_always_true)
    python3 - "$V" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="    declarations.iter().any(|d| {"
b="    let _ = (declarations, a, b); return true;\n    #[allow(unreachable_code)]\n    declarations.iter().any(|d| {"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M7_no_lamina_arm)
    python3 - "$V" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="                    if !jet_determinate {\n                        lamina = true;\n                        None"
b="                    if !jet_determinate {\n                        None"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M8_declared_arm_cusp_only)
    python3 - "$D" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="        matches!(self, Self::Cusp | Self::Slit)"; b="        matches!(self, Self::Cusp)"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M9_lamina_outranks_removed)
    python3 - "$V" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="        if lamina {\n            errors.push(ValidationError::LaminaWedge { edge: edge_key });\n        } else if let Some(wedge)"
b="        if lamina {\n            errors.push(ValidationError::LaminaWedge { edge: edge_key });\n        }\n        if let Some(wedge)"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
  M10_cusp_verb_uses_ang_plus_pi)
    python3 - "$ROOT/crates/profile/src/path.rs" <<'EOF'
import sys
p=sys.argv[1]; s=open(p).read()
a="        Self::from_unit(-self.unit)"
b="        Self::from_angle(self.ang + <T as geom_core::Real>::from_f64(std::f64::consts::PI))"
assert a in s; open(p,'w').write(s.replace(a,b,1))
EOF
    ;;
esac
}

NAME=$1
apply "$NAME" || { echo "APPLY FAILED $NAME"; restore; exit 1; }
OUT=$ROOT/review/r2-mate3/out-$NAME.txt
{
  echo "=== MUTATION $NAME ==="
  cargo test -p topo --lib 2>&1 | tail -60
  echo "--- step-export ---"
  cargo test -p step-export --test all 2>&1 | tail -30
  echo "--- sweep m9_3 ---"
  cargo test -p sweep --test all m9_3 2>&1 | tail -25
  echo "--- profile ---"
  cargo test -p profile 2>&1 | tail -30
} > "$OUT" 2>&1
grep -E "^(test result|failures:|    [a-z0-9_:]+$)|FAILED|APPLY" "$OUT" | head -60
restore
echo "RESTORED"
