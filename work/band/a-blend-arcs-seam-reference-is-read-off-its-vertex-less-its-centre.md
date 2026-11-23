---
id: a-blend-arcs-seam-reference-is-read-off-its-vertex-less-its-centre
kind: issue
title: blend surgery reads radius- and setback-scale directions off body-scale points (the corner/seam arc, the strip frame, the chord frame)
status: open
opened: 2026-11-04
priority: P3
cost: M
---


## What

The blend surgery re-reads a direction it already knew, as the
difference of two points of the body's size that lie a radius or a
setback apart. Each such read keeps only the digits that radius leaves.
At `Interval`, a point enclosure of width `w` becomes a direction of
width about `w / r`, ten times wider at `r = 0.1` on a unit body.

`onecopy/p0-sheared-fillet` (PR 6533) re-posed four sites of this
shape: the band cylinder's `u_ref` (`arms::SupportTrace::foot`), the
ruled arm's, the perpendicular cap circle's (`open::end_face`), and the
foot line's carrier. That last one is `attach_contact`'s
`ContactCarrier::TrimLine` arm, which now carries the stored trimline
instead of `line_between` its vertices. The foot line's read was red at
1e-12 at shear −0.8 (+0.8 peaked at 0.92e-12, green), with envelope 1.01–1.04e-12. These remain:

- **`surgery.rs` `attach_contact`, the `CornerArc` / `SeamArc` arm.**
  It sets `u = (p0 - center).normalize()` and
  `w = (p1 - center).normalize()`, where `p0` and `p1` are stored
  vertices at the radius from `center`. The direction has no other home
  at this site: only the vertices' positions reach it. A re-pose
  carries the foot direction from where the vertex is born (the corner
  ball's `±n_i`, the band's `u_ref`), as `TrimLine` now carries its
  line.
- **`surgery.rs` `strip_frame`.** It sets
  `m = (origin - mid).normalize()`, the trimline origin less the
  edge's midpoint, at setback scale. The arm knows `m`: it is the
  support's in-plane direction from the edge to the foot.
- **`surgery.rs`, the ring-clearance pass's chord frame**
  (`d = (t - o).normalize()`, `off = x - o`,
  `m = (off - d * off.dot(d)).normalize()`). `off` is a foot less its
  station, again at setback scale.
- **`open/end_face.rs` `EndCut::plan`'s `round`.** It reads
  `turn = (foot_a - center).cross(foot_b - center)` for a sign only.
  The shape is the same but harmless: the sign is a radius-scale
  quantity read far from zero.
- **`surgery.rs` `attach_contact`, the `Chord` arm, as the chamfer
  strip's support trimlines reach it.** `planar.rs:564` pushes the
  chamfer's trimlines as `Chord`, and `surgery.rs:5333-5336` re-reads
  τ as `(p1 - p0)/len`, though `chamfer_strip` stores each as
  `Line { foot, tau }`. Both faces are planes, whose envelope is
  affine, and every chamfer row in the PR 6533 delta review's ladder
  was well clear. It is the same copy, though, and the fix is the same:
  carry the stored line as `TrimLine` does.

None is red today at the rows this lane ran (the sheared ladder at
shears ±{0.1…0.8}, `ci` at the default ε and 1e-12). The arc arm is
the first suspect when a corner or seam arc next escalates its
envelope.
