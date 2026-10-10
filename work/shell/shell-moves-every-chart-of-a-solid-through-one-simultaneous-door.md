---
id: shell-moves-every-chart-of-a-solid-through-one-simultaneous-door
kind: issue
title: shell moves a solid that is neither planar nor axial chart by chart through replace_faces_offset, so each intermediate section lands on a moved fit's window edge; one general simultaneous door owes every edge as its moved surfaces' section and every corner as their root
status: open
priority: P2
cost: H
refs: [a-wall-seam-between-two-fits-has-no-section, a-saddle-walls-offset-fit-stalls-short-of-the-default-eps]
opened: 2026-10-10
pr: 4524
branch: shell/general-door
---


Filed from the wall-seam fork (PR 4515). The designed final state is in `a-wall-seam-between-two-fits-has-no-section`'s `## Designed`; this item is its door half.

## The gap

`shell`'s door ladder (`crates/topo/src/shell.rs`, `enum OffsetDoor` and `offset_door`, around :3476-3512) has three doors:
- `PlanesTogether`, when every face is a plane;
- `ChartsTogether`, for an axial body;
- `PerChart`, for anything else: `replace_faces_offset_staged` one face group at a time (around :1440).

Under `PerChart`, each face moves against held neighbours. So each edge is first derived as the section of one moved surface with one HELD surface, and re-derived when the neighbour moves.

On a lofted wall that intermediate section lies `d·cot φ` (φ the interior dihedral) from the moved fit's row. That changes sign at 90°, so on the twisted loft it falls outside the moved fit's window along part of a seam. This is the measured top-rim `NoBranch` at `d = 5e-10` in `a-wall-seam-between-two-fits-has-no-section`'s `## Measured`. Whether a body shells then depends on the order faces are taken and on intermediate bodies nobody asked for.

## Owed

One general simultaneous door for every solid the ladder now sends to `PerChart`:
- It takes `ChartMove`s for all of the solid's charts and mints every new surface.
- Each edge is derived as the section of its two MOVED surfaces through C5. The old carrier seeds the branch and its sense, as `derive_edge` seeds today. An edge is transported where the pair's relative motion carries it rigidly: two planes moving in parallel, or a G1 pair.
- Each corner is the root of the ≥ 3 moved surfaces that meet it, under `solve_corners`' pairwise-agreement rule with every surface moved. This adds fit × spline-carrier roots to the section lane; today it roots only a plane.
- `offset_planes_together` (Cramer) and `offset_charts_together` (the meridian half-plane) are its closed-form arms.
- `replace_faces_offset` stays the single-face verb, no longer `shell`'s driver.
- O4's "anything else chart by chart" sentence (`crates/geom-brep/README.md`; agent text) is reworded with the code.

The door can land before the NURBS × NURBS arm. Against bodies whose moved pairs route today (planes and fits), the loft's rims and corners derive simultaneously, and its wall–wall seams refuse `NeighborPairUnroutable(Nurbs, Nurbs)` until the arm lands. Re-baseline `encl_curved_loft_shell` and `offd_r1_probes`' seam rows to that.

Every planar and axial body must keep its outcome and its stored bits. That needs a merge-base differential.

## Decided

1. One general simultaneous door replaces `OffsetDoor::PerChart` for every solid that is neither all-planar nor axial. It takes the solid's `ChartMove`s (`offset_together`'s own type) and mints every new surface first.
2. **Edges.** Each edge is the section of its two MOVED surfaces through the C5 table. The old carrier seeds the branch and its sense, as `derive_edge` seeds today. An edge is transported where the pair's relative motion carries it rigidly:
   - two planes moving in parallel;
   - a G1 pair that holds the move;
   - a self-shared seam, which moves with its chart.
3. **Corners.** Each corner is the root of the ≥ 3 moved surfaces meeting it, under `solve_corners`' pairwise-agreement rule, with every surface moved.
   - The section lane today roots only a plane along a spline carrier. Add whatever roots the fixtures need (a fit or spline surface along a spline carrier, i.e. route 2 of unit 15's item), or refuse typed and name it.
   - Do not silently skip a moved surface at a corner.
4. **Closed forms.** `offset_planes_together` and `offset_charts_together` are this door's closed-form arms. Keep them as the fast paths where the ladder sends them today. Whether they fold into the general door's code is the implementer's call, argued.
5. **The single-face verb.** `replace_faces_offset` stays as the verb that moves one face, and stops being `shell`'s driver. The rim lift uses the same door decision as the cavity.
6. **Seams until the arm lands.** Wall–wall seams between two moved fits refuse `NeighborPairUnroutable(Nurbs, Nurbs)` (or Approx) until the NURBS × NURBS arm lands. That arm is a later unit.
   - Re-baseline the twisted-loft rows (`crates/sweep/tests/encl_curved_loft_shell.rs`; `crates/sweep/tests/offd_r1_probes.rs`'s seam rows) to what the door now reaches, at ε 1e-6, 1e-9 and 1e-12.
   - At the default ε the wall's fit still refuses `BudgetExhausted`. That is `a-saddle-walls-offset-fit-stalls-short-of-the-default-eps`, not this unit.
7. **Prove the point.** Build at least one non-planar, non-axial solid that `PerChart` refused or built order-dependently, and that the general door builds. Pin its closed form and its tier-3 validity, or its typed refusal past the door.
8. **O4.** O4's "anything else chart by chart through `replace_faces_offset`" in `crates/geom-brep/README.md` is agent text. Reword it with the code, along with the `shell.rs` and `replace_face.rs` module docs.
9. **No regression.** Every planar and axial body keeps its outcome and its stored bits, shown by a merge-base differential over geom-brep, topo, sweep, editor-core, `demos/tour` and the corpus documents, at the default ε and at `CAD_TOLERANCE_EPS=1e-12`, every moved row explained.
