---
id: a-blend-arcs-seam-reference-is-read-off-its-vertex-less-its-centre
kind: issue
title: a blend corner or seam arc reads its seam reference as its start vertex less its centre, a radius-scale offset off body-scale points
status: open
opened: 2026-11-04
priority: P3
cost: M
---


## What

`crates/sweep/src/blend/surgery.rs`, the `ContactCarrier::CornerArc` /
`SeamArc` arm of the contact-edge carrier construction, mints the arc as
`Curve3::circle(center, turn.normalize(), radius, u)` with
`u = (p0 - center).normalize()` and `w = (p1 - center).normalize()`.
`p0` and `p1` are stored vertices at the blend radius from `center`,
whose coordinates are the body's size, so each difference keeps only
the digits the radius leaves: at `Interval` a vertex enclosure of width
`w` reads as a direction of width about `w / r`, ten times wider at
`r = 0.1` on a unit body.

It is the shape `onecopy/p0-sheared-fillet` fixed in the cylinder
band's `u_ref` (`arms::SupportTrace::foot`), where the width cost a
1e-12 pcurve envelope its band. Here the direction has no other home
in the surgery: the arc is placed through its stored vertices and only
their positions reach it, so a re-pose has to carry the foot direction
from where the vertex is born (the corner ball's `±n_i`, the band's
`u_ref`) to this construction.

No red today; filed so that the next envelope escalation on a corner
or seam arc starts here.
