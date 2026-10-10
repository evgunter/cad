---
id: pn-apex-point-snap-discards-an-extent-scale-section
kind: issue
title: plane×cone serves the apex point for a plane whose real section reaches the extent
status: review
opened: 2026-10-07
priority: P2
cost: M
branch: germ/pn-apex-point-snap
---

## What

PR 4280's review: `plane_cone_section`'s apex lane
(`crates/geom-brep/src/intersect.rs`) serves `ApexPoint` once
`pn_apex_on_plane` reads the apex gap Zero and `pn_apex_section_floor`
reads the discriminant definitely negative. A plane a hair off the apex
whose discriminant is just past the band cuts a small-angle ellipse
whose size is the gap over the discriminant's angle: the review served
`ApexPoint` at `D·extent = −10.7·ε` where the true ellipse reaches
0.056 m. The apex point snaps away a section at the scale of the
extent.

## The shape of a fix

Serve the point only where the ellipse the gap and the discriminant
make stays inside the band (its semi-axes from the gap and `D`), else
route to the tilted-ellipse lane or escalate.
