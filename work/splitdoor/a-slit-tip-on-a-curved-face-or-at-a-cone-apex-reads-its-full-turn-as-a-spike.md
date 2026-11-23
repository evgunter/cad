---
id: a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike
kind: issue
title: A slit tip on a curved face's section reading, or at a cone's apex, reads the face's full turn as a spike
status: closed
opened: 2026-10-29
priority: P2
cost: M
refs: [a-split-through-a-pocket-floor-touching-the-block-walls-refuses-a-sliver-sector]
closed: 2026-10-30
pr: 6052
---

Found by the sweep of `splitdoor/pocket-floor-sliver`, which taught the
split's neighbourhood walk the Boolean's slit-tip reading
(`sector_shape::SlitTips`): between a slit's two coincident edges at
its tip the face wraps a full turn, and `sector_shape`'s rung 3 reads
the two bounds leaving along one ray, which between distinct edges it
rejects as a spike (`sector_straight`, `Rejection { sign: Positive,
passes: Negative }`). Two readers of the same corner still do not read
the tips. Neither has a witness; both are unmeasured.

## What

- **`vertex_section::entered_faces`** (`crates/topo/src/vertex_section.rs`)
  calls `splitting::wide_entries` for a curved face whose sector holds
  the plane's trace with its slit-tip flag `false`: it has no tips to
  read. Both lanes reach it (`splitting::classify`'s
  `read_sections`, `boolean::reduce`'s `decide_vertex_on_plane`). A
  slit of two coincident line edges on a cylinder's ruling, its tip on
  the plane with both edges on one side, would refuse `SliverSector`
  there. The fix threads each lane's tips (the split's from
  `splitting::reduce`, the Boolean's `BooleanReduction::slit_tips`) to it.
- **`wide_entries`' apex arm** (`crates/topo/src/splitting/neighborhood.rs`)
  passes `one_edge` alone as `apex_shape`'s `full_circle`. At a slit tip
  on a cone's apex the window between the two coincident rulings reads
  `azimuth(unit_own)` = 2π, whose chord is zero, and `decide_positive`
  on `sector_apex_window` rejects it. Passing `one_edge || slit_tip` there
  reads the window as the whole turn, which is what the face does.

- **Known gap, both lanes: `SlitTips` is keyed by `(vertex, face)`, not
  by corner.** At a vertex that is a slit's tip in `face`, every corner
  of `face` there reads as the tip's. A corner of the same face at that
  vertex between two other edges whose bounds leave along one ray (a
  cusp between tangent curves at the slit's far end) would read as a
  full turn rather than refused as a spike. Keying by the corner's two
  edges closes it. Found by the review of PR 5982.

Both readers above call `sector_shape::corner_shape` with a
`CornerBounds`, and the gap is closed in `SlitTips::bounds`.

## Witness wanted

A body at rest with either shape (a groove touching a cylinder along a
ruling, ending on the split plane; a cut touching a cone along a ruling
to its apex), split through the tip, measured at three ε before the fix.

