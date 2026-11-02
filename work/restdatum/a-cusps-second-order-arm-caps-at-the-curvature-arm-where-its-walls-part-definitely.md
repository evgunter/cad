---
id: a-cusps-second-order-arm-caps-at-the-curvature-arm-where-its-walls-part-definitely
kind: issue
title: A cusp's second-order arm caps at the walls' curvature arm, so walls that part definitely within their extent refuse in band
status: open
opened: 2026-10-19
priority: P3
cost: M
design: true
---


Found by designer A on the cusp fork (#5354): "Witness 1 parts by
13–18ε at its far end, which is definite. Tier 3 still refuses it at
8.5ε because it levers at r = 1. If that is wrong, it is wrong in tier
3's arm, not in the profile." Filed by `paths/joint-reads-its-turn`,
which pins the refusal
(`crates/sweep/tests/a_tangent_joint_on_near_concentric_carriers.rs`,
`a_join_whose_walls_part_in_band_refuses_its_sliver`).

**The witness.** An arc of the unit circle into the joint `(1, 0)`
and an arc of the circle about `(17ε, 0)` of radius `1 − 17.5ε` back
out through 1.6 rad, closed by a line: a decided tangent cusp that
validates at every ε row, its second arc's far end 13–18ε clear of the
first (the profile's far-end table,
`crates/profile/tests/rejections.rs`,
`a_tangent_cusps_far_end_draws_its_line`). Extruded to depth 1 it
refuses `ExtrudeError::SliverJoin` at vertex 1 on
`tangent_second_order`, margin 8.5ε, at `f64` and `Interval`, at
1e-6, 1e-9 and 1e-12. The second-order margin is `κ_rel·arm²/2` with
the arm from `geom_brep::folded_lever_arm`
(`crates/geom-brep/src/dihedral.rs`): `min(curvature arms, reach)`,
and the cylinders' curvature arms are 1, so the arm is 1 and the
sagitta `17ε/2`, in band.

**Why it may over-refuse.** The two walls run 1.6 m along the arcs
from the strut, and over that run they part by `κ_rel·s²/2`, about
22ε at the far end: definitely, inside the faces. The cap at the
curvature arm is the fold's estimate of how far the sagitta formula
holds, and it is the conservative side for a first-order wedge; for a
cusp's second-order verdict it reads a pair as undecided that the
faces' own extent decides. Tier 3's check 4 reads the same arm, so the
at-rest gate would refuse the body the extrude declined to build.

**The open part** (`design: true`): whether a cusp's second-order arm
should be the faces' reach along the arcs rather than capped at the
curvature arm, and on which surfaces that holds (two cylinders here;
a cone or torus wall's arm is not its curvature radius,
`geom_brep::curvature_lever_arm`'s docs). Ground: tier 3's check 4 is
RESTFRONT's; the fold's home is `dihedral.rs`, SELECT's file; declared
cusps' wedge-0 arm is TANG's (`work/tang/declared-cusps-second-order-wedge-arm.md`).
