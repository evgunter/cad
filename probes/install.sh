#!/usr/bin/env bash
# Install the review probe into a tree: `install.sh head <tree>` or
# `install.sh main <tree>` (main = 446031aa, the PR's merge base).
set -euo pipefail
which=$1; tree=$2; here=$(cd "$(dirname "$0")" && pwd)
if [ "$which" = head ]; then
  dst=$tree/crates/topo/src/boolean/conic_quadric/probe_review.rs
  cp "$here/door_fuzz.rs" "$dst"
  sed -i 's/use super::BooleanError;/use super::super::BooleanError;/; s/use super::circle_roots::CircleRoots;/use super::super::circle_roots::CircleRoots;/' "$dst"
  grep -q 'mod probe_review;' "$tree/crates/topo/src/boolean/conic_quadric/mod.rs" || \
    sed -i 's/^mod ellipse_rows;/mod ellipse_rows;\n#[cfg(test)]\nmod probe_review;/' "$tree/crates/topo/src/boolean/conic_quadric/mod.rs"
else
  dst=$tree/crates/topo/src/boolean/probe_review.rs
  python3 - "$here/door_fuzz.rs" "$dst" <<'PY'
import sys
s=open(sys.argv[1]).read()
a=s.index("// DOOR_CALL_BEGIN"); b=s.index("// DOOR_CALL_END")
s=s[:a]+'''fn door_call(
    c: &geom::Curve3<f64>,
    t0: f64,
    t1: f64,
    s: &geom::Surface<f64>,
    band: Band,
) -> Result<CircleRoots<f64>, BooleanError> {
    match (c, s) {
        (geom::Curve3::Circle { .. }, geom::Surface::Sphere { .. }) => super::circle_sphere::circle_sphere_roots(c, t0, t1, s, band),
        (geom::Curve3::Circle { .. }, _) => super::circle_cylinder::circle_cylinder_roots(c, t0, t1, s, band),
        _ => super::ellipse_roots::ellipse_roots(c, t0, t1, s, band),
    }
}
'''+s[b:]
open(sys.argv[2],'w').write(s)
PY
  grep -q 'mod probe_review;' "$tree/crates/topo/src/boolean/mod.rs" || \
    sed -i 's/^mod ellipse_roots;/mod ellipse_roots;\n#[cfg(test)]\nmod probe_review;/' "$tree/crates/topo/src/boolean/mod.rs"
fi
