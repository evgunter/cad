---
id: a-cone-or-torus-leg-carries-its-charts-frame-into-a-crossing-at-the-band
kind: issue
title: A cone's ruling legs, and the frame a surface curve carries in its section and plane, still answer a crossing at the band's escalation by the chart's frame (narrowed: circle legs start at the foot)
status: open
opened: 2026-11-09
priority: P3
cost: H
---


Filed from `the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart` on closing it.

## What

The region doors read a query's foot point in closed form, on no chart (`geom::Surface::foot`), so the boundary pre-pass answers the same on every frame. The doors' legs are still built from the foot's chart coordinates (`SurfaceCurve::parallel`, `ruling`, `meridian` and `from_apex`, taking `foot.uv()`, in `crates/topo/src/boolean/ruled_region.rs` and `torus_region.rs`). Their circle or line therefore carries the chart's frame in its last ulps.

A crossing about 1e-8 along a leg, at the band's escalation, is answered by the frame. On the measurement in `the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart`, 135 of 2400 cone queries and 133 of 2400 torus queries at ±1e-8 off an edge answer differently on different frames:
- cone: `bool_ruled_region_at`, the crossing's place along the leg;
- torus: `bool_torus_region_component`, `plane_crossing_root_in_span` and `bool_torus_region_at`.

The sphere door's rays are great circles about `p̂ × t`, read from the closed-form foot, and show none. Neither does the cylinder.

## The question (on `on_surface`'s module doc)

The legs could be built from the closed-form foot instead:
- a parallel about the axis through the foot;
- a ruling through the apex and the foot;
- a torus meridian about the tube centre in the foot's radial plane.

Each of those is the same curve with no frame in it. But `on_surface`'s module docs say a `SurfaceCurve` "is built from a surface record's surface and chart coordinates and from nothing else". That rule is what keeps a curve on its surface by construction.

*Corrected:* this row first cited the rule as C5. C5 in `crates/geom-brep/README.md` is "one total kind-pair table, no runtime fallback". The chart-coordinates sentence is `on_surface.rs`'s module doc, written by agent work (77b8789d) and never ratified, so it is not Ev's.

So the fork is:
- **Keep C5 as it stands.** A crossing at the escalation keeps answering by frame. Every answer there is a refusal or a correct side, and the frames differ only in which.
- **Allow a curve built from a closed-form foot on the record.** That needs a clause on what still keeps such a curve on the surface.

The ONECOPY orchestrator ruled on it (below), since the rule is not ratified.

**Measured cost of keeping C5:** about 5.6% of the queries placed exactly at the escalation off a cone or torus edge answer by frame. None do at the band's zero or off it.

The count depends on how many frames are read: a query counts where any frame answers otherwise than the first, so more frames find more of them. The 135 and 133 are over five frames for the cone and six for the torus (the seam turned to four other azimuths, and the torus's axis flipped). They are a floor on how often a frame decides, not a rate.

## Narrowed (`onecopy/leg-chart-frame`)

**Why the leg carried the frame.** It re-read a foot another door already holds. Every leg started at `surface.eval(foot.uv())`, the chart's round trip of the foot's coordinates, which is exactly what PR 6676 took out of the boundary pre-pass. `QueryFoot` holds the closed-form point `foot.at()` (`geom::Surface::foot`). A circle's `θ = 0`, `(start − centre).normalize()`, carried the round trip's ulps into `Round::place` and `arc`.

**Ruled (ONECOPY orchestrator, option A).** A circle leg's parametrisation starts at the door's closed-form foot (`SurfaceCurve::starting_at`). Its centre, axis and radius stay the chart's. `on_surface`'s module doc now says what is true and protective: a curve's **locus** is built from chart coordinates on its record, so no curve off the surface can be built as one; its parametrisation may start at a point the caller holds on it.

**Measured** (the row's 2400 queries, every disagreement at `±1e-8`):

| variant | cone | torus |
|---|---|---|
| legs started at `surface.eval(uv)` | 141 | 142 |
| A: circle legs started at the foot (shipped) | 35 | 31 |
| B: A plus the rulings' origin at the foot | 18 | 31 |
| C: B plus centre and radius from the closed-form foot | 18 | 26 |

These were measured on a scratch build, which is not kept. Neither A nor B moves any row of topo, sweep or editor-core at either ε.

The pins are `ruled_region_rows::a_cones_circle_legs_read_no_chart_frame_at_the_escalation` (35) and `torus_region::tests::a_torus_faces_circle_legs_read_no_chart_frame_at_the_escalation` (31). Each reads 141 or 142 without the start.

**What remains on this row:**
- B, the rulings' origin;
- C, centre and radius from the foot. Both change the locus, which is what the module doc's rule protects, so they need an argument for what keeps such a curve on its surface.
- The frame a `SurfaceCurve` carries in its section and plane: `Section::HalfPlane`'s `toward` azimuth of `u`, a line's plane normal `f.axis × f.radial(u)`, and the crossings read through them. This is the 18/26 that C leaves.
