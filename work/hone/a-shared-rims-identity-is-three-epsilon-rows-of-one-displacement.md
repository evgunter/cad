---
id: a-shared-rims-identity-is-three-epsilon-rows-of-one-displacement
kind: issue
title: rim_wedge::same_circle decides a shared rim's identity as three separate ε rows (radius, centre, levered axis tilt), so a point of one rim can stand ~3ε off the other with every row Zero
status: open
opened: 2026-10-10
priority: P2
cost: M
---


## What

`crates/topo/src/boolean/rim_wedge.rs::same_circle` matches two faces'
boundary circles by deciding `rim_circle_radius`, `rim_circle_center`
and `rim_circle_axis_parallel` (tilt levered at the diameter) each
within ε on its own. The three terms displace one circle in the same
directions: a point of B's rim stands off A's by up to |Δr| + |Δc| +
tilt·R. So a rim can match with each row Zero while its points sit about
3ε apart. That is the shape `decide_across` (one `datum ± swing` reading)
exists for, as C4's `Rest` already uses it.

The match feeds `shared_rim`, which hands face A's boundary circle to
`tangent_locus_relation` as the carrier of a seam across the rim, on
the declared door and the glue door (`glue.rs`) alike.

## Where it was found

Both designers on CONTACTHOLD's tangent-tolerance fork (2026-10-10)
found it independently, by reading. It is the one genuine "terms of one
reading decided apart" on the tangency paths. Unmeasured: build two rims
offset by ε/2 in each term and check the match reads Zero while a rim
point stands >ε off.

Filed on HONE (owner of `rim_wedge.rs` with CLEAVE) by CONTACTHOLD.
(CONTACTHOLD orchestrator)

**One displacement margin exists.** `geom_brep::curve_pair::pose`'s
one-curve reading (`one_curve`, row `curve_pair_pose_one_curve`) reads
`one_curve_gap`, which bounds the Hausdorff distance between two conics,
tilt included: `|c_e − c_f| + ‖L_e − L_f·Q‖`, with
`L = Conic::semi_axis_vectors()` and `Q` the way-up flip and Procrustes
rotation (PR 5058, moved into geom-brep with stage (c) of
`the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)). It decides one curve for every
conic pair in one plane, circles included. `same_circle` is a second
spelling of that question and can read the same reading.
