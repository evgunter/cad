---
id: a-cone-or-torus-leg-carries-its-charts-frame-into-a-crossing-at-the-band
kind: issue
title: A cone or torus door's leg, built from the chart's coordinates, answers a crossing at the band's escalation by the chart's frame
status: open
opened: 2026-11-09
priority: P3
cost: H
design: true
---


Filed from `the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart` on closing it.

## What

The region doors read a query's foot point in closed form, on no chart (`geom::Surface::foot`), so the boundary pre-pass answers the same on every frame. The doors' legs are still built from the foot's chart coordinates (`SurfaceCurve::parallel`, `ruling`, `meridian` and `from_apex`, taking `foot.uv()`, in `crates/topo/src/boolean/ruled_region.rs` and `torus_region.rs`). Their circle or line therefore carries the chart's frame in its last ulps.

A crossing about 1e-8 along a leg, at the band's escalation, is answered by the frame. On the measurement in `the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart`, 135 of 2400 cone queries and 133 of 2400 torus queries at ±1e-8 off an edge answer differently on different frames:
- cone: `bool_ruled_region_at`, the crossing's place along the leg;
- torus: `bool_torus_region_component`, `plane_crossing_root_in_span` and `bool_torus_region_at`.

The sphere door's rays are great circles about `p̂ × t`, read from the closed-form foot, and show none. Neither does the cylinder.

## The question (design, on C5)

The legs could be built from the closed-form foot instead:
- a parallel about the axis through the foot;
- a ruling through the apex and the foot;
- a torus meridian about the tube centre in the foot's radial plane.

Each of those is the same curve with no frame in it. But C5 (`crates/geom-brep/README.md`) says "A curve on a surface is built from chart coordinates on a surface record", and `on_surface`'s module docs say a `SurfaceCurve` "is built from a surface record's surface and chart coordinates and from nothing else". That rule is what keeps a curve on its surface by construction.

So the fork is:
- **Keep C5 as it stands.** A crossing at the escalation keeps answering by frame. Every answer there is a refusal or a correct side, and the frames differ only in which.
- **Allow a curve built from a closed-form foot on the record.** That needs a clause on what still keeps such a curve on the surface.

C5 is ratified, so the change is Ev's to make. The fork is weighed first (designers, then an `[ev]` PR).

**Measured cost of keeping C5:** about 5.6% of the queries placed exactly at the escalation off a cone or torus edge answer by frame. None do at the band's zero or off it.

The count depends on how many frames are read: a query counts where any frame answers otherwise than the first, so more frames find more of them. The 135 and 133 are over five frames for the cone and six for the torus (the seam turned to four other azimuths, and the torus's axis flipped). They are a floor on how often a frame decides, not a rate.
