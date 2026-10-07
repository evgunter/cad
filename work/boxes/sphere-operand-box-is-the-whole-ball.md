---
id: sphere-operand-box-is-the-whole-ball
kind: issue
title: A sphere face's operand box is the whole ball - the same per-kind box class the cone, cylinder and torus arms have left
status: open
opened: 2026-09-06
refs: [torus-operand-boxes-span-whole-ring, 1907]
priority: P1
cost: H
---


## What

`FaceBoxRule::WholeBall` boxes every sphere face as the whole ball
(`boxes.rs`), reading nothing from the boundary — the class the
VERBS-GATE cone clip, the cylinder's `clip_to_boundary` and the torus
window (PR #1907) each retired for their kind. Named-and-deferred in
#1907's sweep with no file; this is the file. A sphere face's chart
window from its stored `Harmonic` images is the torus construction
with one channel (the sphere has no interior critical point in `v`
beyond the poles, which the walk's pole joint already handles).

## Home

CURVED — the operand boxes are the operand-reach lane's.

## Evidence from the split gate (REACH, 2026-10-02)

The split's carrier gate (`splitting/classify.rs` `carrier_gate`) is
reach-scoped since `reach/split-gate-refuses-a-whole-body-for-one-unarmed-face`,
and the whole ball made it useless for the very fixture that filed it:
a cylinder under a spherical cap (sphere radius 5/4 about `(0, 1/4)`,
cap `y ≥ 1`) cut at `y = 1/2` refused, because the ball spans
`y ∈ [−1, 1.5]`. The gate therefore boxes a sphere face itself
(`classify::gate_face_reach`): when `solid_contain::sphere_chart_trim`
pins a latitude window, the face lies in the zone between its two
extreme latitudes, and the box is `slab_extent` over that axial window
met with the ball. That is a split-local copy of the tightening this
row asks for, now with a side guard (a rectangle's boundary also
bounds its complement). Folding it into `FaceBoxRule` is its own unit,
`split-gate-sphere-zone-folds-into-face-box-rule`.

## Since `reach/split-gate-sphere-azimuth` (2026-10-06)

The rule's sphere arm is no longer the whole ball for the rectangle
class: a sphere face whose boundary is latitude rims and meridians, on
its rectangle's side, is boxed by that rectangle's own support
(`FaceBoxRule::SphereWindow`, `boxes::sphere_reach`), in both box lanes.
The ball is left for faces outside the class: a boundary circle tilted
against the chart (a plane's or another sphere's section that is no
rim), which `topo::boolean::sphere_region` reads for containment but no
box reads yet, and a ringed face.


The rectangle reading also gives up on three shapes inside the class,
each keeping the zone or the ball where a tighter box is sound (REACH
review of PR 4123, `probe_sphere_rect_sweep`, 844 sheets, 0 samples
outside any box):

- **A reflex loop.** An L- or U-shaped loop of rims and meridians is
  not its window's rectangle, and keeps the ball. The likely gate is
  the material-sign read in `boxes.rs` `sphere_window`
  (`boundary_material_sign`, an iso-rectangle reading), but this is not
  traced. Its box could be the union of its rectangles' supports.
- **A rim 1e-4 off a pole.** A face whose rim sits that close to a pole
  falls to the zone or the ball. The radial lever there (`ρ ≈ 0.014`)
  is far above the band, so the pole decisions should not take it. Which
  step of `solid_contain.rs` `sphere_chart_trim` or
  `chord_join::face_azimuth_window` gives up is not traced: trace it
  first.
- **A cap's strut that stops short of its pole.** On a face that wraps
  alone, `solid_contain.rs` `sphere_chart_trim` folds only its rims'
  levels and the poles its vertices reach. A one-rim cap whose strut
  ends short of the pole keeps the ball, though its side is known (the
  strut's tip). No valid body carries one (`ScaffoldingStrutVertex`).
