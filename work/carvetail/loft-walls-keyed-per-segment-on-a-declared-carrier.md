---
id: loft-walls-keyed-per-segment-on-a-declared-carrier
kind: issue
title: loft: a declared continuation's two walls get two NURBS surface keys, so no merge rung can join them
status: open
opened: 2026-09-25
priority: P2
cost: M
design: true
---


Found by BAND's `subdivided-profile-side-coplanar-walls-gate` rows
(`crates/sweep/tests/band_subdivided_side_walls.rs`,
`lofted_continuation_walls_carry_one_key_per_segment`).

Loft two copies of a square whose bottom side is a declared straight
continuation. The two walls on that side get two distinct NURBS
surface keys (one per segment, from `assemble` in
`crates/sweep/src/loft.rs`), and neither carries a shared
`GeomSource`, so `merge_coplanar_faces` finds no run. Extrude and
revolve share one key across the same continuation (the cosurface run
structure in `crates/sweep/src/swept.rs`). The loft does not, so the
author's declaration is lost here.

This is latent today: the boolean refuses the lofted body's spline
edges before its maximal-faces gate is reached. It becomes a
`NonMaximalFaces`-shaped wall, or a numeric-only coplanarity no rung
can merge, on the day NURBS edges are admitted. The fix has one of two
shapes. The skin can build one surface over a same-carrier run in
every section, or the walls of such a run can carry a shared
`GeomSource` so the declared rung joins them.

## Parked on the D10 hold (CARVE, 2026-10-06)

The row is about a DECLARED straight continuation and the declared merge rung that would join its walls. Declared continuations are on the hold's ground, and D10 retires the declaration in favour of structural identity (one construction, canonical carrier forms), so the fix's two shapes are both questions D10's stage 4 answers. It waits on that build.

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: a declared continuation and the declared merge rung, which stage 4 replaces with canonical carrier forms at the door. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-09)

E removes both rungs this row's fix named. `GeomSource` is deleted, so walls cannot share one. The declared continuation now changes nothing, because the merge reads each pair by its margin, declared or not (`crates/topo/src/merge_faces.rs:2255`, `faces_continue`). The defect stands: the two lofted walls are NURBS on two keys, and the merge's margin rung has no NURBS carrier. `face_carrier` answers `None` for a net (`crates/topo/src/boolean/carrier_pair.rs:243`), `carriers_continue` reads that as "no merge" (`merge_faces.rs:2354`), and so the walls stay split. What remains is the first shape, one skin surface over a same-carrier run, or a NURBS arm in the carrier ladder. It is still latent behind the boolean's spline-edge refusal.
