---
id: tier-3-passes-a-cone-face-whose-loop-holds-two-vertices-at-its-apex
kind: issue
title: Tier 3 passes a cone face whose loop holds two distinct vertices at its apex, two cones of material meeting only there, that tier 3′, the mesher and STEP re-import refuse
status: open
opened: 2026-11-25
priority: P2
cost: M
---


Filed by LATHE (`lathe/pinched-cone-loop`, PR 7585's review fix pass), on RESTFRONT's slate: what tier 3 passes at rest that it should refuse.

## What

The cone of radius and height 1, `Revolution::Partial(5.5)` (`sweep::test_support::revolved_about_y`, profile `(0,0), (1,0), (0,1)`), less the box `[−0.3, 0] × [0.95, 1.2] × [−0.3, 0.3]`. What remains of the tip is two wedges of material touching along the axis above `y = 0.95`. The cone face's loop passes the apex point at two DISTINCT vertices, one per wedge: the pinch `topo::loop_points` names (`crates/topo/src/loop_points.rs:11`, two distinct vertices on one point key).

At PR 7585's first head (`acb07be56`), props measured that loop lobe by lobe. The body then built at its right volume and passed tier 3 (`validate_geometric`). It was refused by:

- tier 3′, `validate_pseudomanifold` (`BoundaryCrossesItselfUndecided`, `NoTangentPlane`);
- the mesher, `TessellateError::PinchWedge` (`crates/mesh/src/curved.rs:372-376`), which reads `loop_pinches`;
- STEP re-import, `TierInvalid` (a VertexVertex contact, and an EdgeEdgeOverlap on the axis).

Check 9's pinch reading (`crates/topo/src/validate.rs:2148`, `PinchCornerCrossed`) asks only whether another pass of the loop lies inside a corner's wedge. The two tip wedges are disjoint, so it finds nothing. Nothing in tier 3 asks whether two distinct vertices of one loop at one point are a slit's parted end, which tier 3 admits (`loop_points.rs:12-14`), or two cones of material meeting at a point that tier 3′ cannot decide.

The Boolean now refuses this result at the gate: props reads the apex passes by vertex identity and refuses `props_cone_apex_vertices` (`crates/geom-brep/src/props/curved.rs`, `require_apex_lobes`). That holds only on a cone face, and only because props measures that face. A body whose pinch sits on a face props does not read so (a planar or spline face, or a cone pinched off its apex) passes tier 3 the same way.

Pinned: `crates/sweep/tests/a_cone_apex_on_a_plane_through_it.rs`, `a_wedge_cut_along_two_rulings_through_its_apex_refuses_two_apex_vertices`. It pins that `w ∖ b` refuses at the gate and that `w ∪ b` builds clean.

## Where to look

Whether tier 3's "cycles simple and disjoint" reading should refuse a loop pinch that is not a slit's parted end, the way tier 3′ and the mesher already do. That boundary is DESIGN.md's distinct-entity touch (`docs/DESIGN.md:816`): representable only as a typed success carrying its tier-3′ records, and this one has none that decide.
