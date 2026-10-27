---
id: quad-rational-position-bound-is-a-quotient-of-hulls
kind: issue
title: quad: the rational lane's position bound p_bound divides the homogeneous hull by the weight hull, where the control points' own hull bounds S
status: open
opened: 2026-10-22
priority: P3
cost: E
---


## Found (QUAD, `quad/rational-quotient-statement`, 2026-10-22, §5 sweep)

`crates/geom-brep/src/props/quad/rational.rs` `rational_patch_face`:
`p_bound = (a_hull[0] / g_w).mag() + (a_hull[1] / g_w).mag() + (a_hull[2] / g_w).mag()`
divides the refined homogeneous net's hull by the weight net's hull,
three separately hulled inputs and an L1 fold. With positive weights
`S = A/w` is a convex combination of the control points, so their own
Euclidean norms bound `|S|` directly (D4 ¶2's coefficient-norm
reading), with no quotient of hulls. It enters only through
`boundary_defect · p_bound`, which is zero for minted iso pcurves, so
the width it costs is the defect's. It grows with the net's distance
from the origin by definition (the flux pad is an origin-dependent
quantity), so it is not the translation row's defect.

Not taken by the quotient-statement unit, whose two bounds were the
flux's `f_dd` and the area pad.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds: `crates/geom-brep/src/props/quad/rational.rs:421`, read at
`:603` (`boundary_defect * p_bound`).
