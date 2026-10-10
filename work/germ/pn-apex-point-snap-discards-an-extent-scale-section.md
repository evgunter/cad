---
id: pn-apex-point-snap-discards-an-extent-scale-section
kind: issue
title: plane×cone serves the apex point for a plane whose real section reaches the extent
status: closed
opened: 2026-10-07
closed: 2026-10-10
priority: P2
cost: M
branch: germ/pn-apex-point-snap
pr: 4478
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

**2026-10-10, closed (PR 4478, DR-136, sequential arm).** `ApexPoint` is served only where `pn_apex_point_reach`, the Hausdorff distance from the apex to the true section, `|δ|/|D|`, reads Zero. Otherwise the off-apex lane builds the circle or the tilted ellipse. The wrong answer was live in `offset_derive`'s planning, which read the ellipse as `NoBranch`; its row is red at main. `chord_join` and `pk_germ_frame` were measured: both refuse typed there.
