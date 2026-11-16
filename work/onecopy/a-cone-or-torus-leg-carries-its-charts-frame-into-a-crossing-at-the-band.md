---
id: a-cone-or-torus-leg-carries-its-charts-frame-into-a-crossing-at-the-band
kind: issue
title: A cone's ruling legs, and the frame a surface curve carries in its section and plane, still answer a crossing at the band's escalation by the chart's frame (narrowed: circle legs start at the foot)
status: closed
opened: 2026-11-09
priority: P3
cost: H
design: true
closed: 2026-11-10
branch: onecopy/leg-frame-crossing
pr: 6762
---


Filed from `the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart` on closing it.

## What

The region doors read a query's foot point in closed form, on no chart (`geom::Surface::foot`), so the boundary pre-pass answers the same on every frame. The doors' legs are still built from the foot's chart coordinates (`SurfaceCurve::parallel`, `ruling`, `meridian` and `from_apex`, taking `foot.uv()`, in `crates/topo/src/boolean/ruled_region.rs` and `torus_region.rs`). Their circle or line therefore carries the chart's frame in its last ulps.

A crossing about 1e-8 along a leg, at the band's escalation, is answered by the frame. On the measurement in `the-ruled-and-torus-doors-read-the-querys-foot-back-through-the-chart`, 135 of 2400 cone queries and 133 of 2400 torus queries at ±1e-8 off an edge answer differently on different frames (141 and 142 when re-measured on main at this row's narrowing, below):
- cone: `bool_ruled_region_at`, the crossing's place along the leg;
- torus: `bool_torus_region_component`, `plane_crossing_root_in_span` and `bool_torus_region_at`.

The sphere door's rays are great circles about `p̂ × t`, and its corner legs circles about the query; the 6676 measurement shows the sphere none, and neither the cylinder. Since the narrowing below, the sphere's legs also start at the points the door holds (the foot, the corner), so no door's circle leg starts at a chart round trip.

## The question (on C5)

The legs could be built from the closed-form foot instead:
- a parallel about the axis through the foot;
- a ruling through the apex and the foot;
- a torus meridian about the tube centre in the foot's radial plane.

Each of those is the same curve with no frame in it. But README C5 says, at `crates/geom-brep/README.md:917-918`, "A curve on a surface is built from chart coordinates on a surface record". C5 is ratified, a page `docs/DESIGN.md` lists, and Ev engaged on this wording (39aa626fd, "the co-surface rule's how lives in C5 only", PR 6567). `on_surface`'s module docs repeat it for a `SurfaceCurve`. That rule is what keeps a curve on its surface by construction.

*Corrected:* an intermediate edit of this row said the rule was only `on_surface.rs`'s module doc, unratified, and dropped `design: true`. That was wrong: the sentence wraps across lines in README C5 and a search missed it. The rule is C5's.

**What C5 leaves open, and what it settles.**
- A curve's **locus**, its centre, axis and radius, or a line's origin and direction, is what "built from chart coordinates on a surface record" governs. That is what puts the curve on its surface.
- Where a circle's parametrisation counts from, its `θ = 0`, moves no point of its locus. Placing it at a point the door holds on the locus applies C5 rather than changing it. That is option A, shipped (below).
- B and C (below) build the locus itself from the closed-form foot: a ruling through the foot, a parallel's centre and radius from it. Those change what C5 governs, so they are Ev's.

**What keeping the rest costs.** After A, 35 of the row's 2400 cone queries and 31 of the torus's still answer by frame, every one at `±1e-8`, the escalation, and none at the band's zero or off it. A query counts where any frame answers otherwise than the first, over five frames for the cone and six for the torus (the seam turned to four other azimuths, and the axis flipped), so these are a floor on how often a frame decides, not a rate.

## Narrowed (`onecopy/leg-chart-frame`)

**Why the leg carried the frame.** It re-read a foot another door already holds. Every leg started at `surface.eval(foot.uv())`, the chart's round trip of the foot's coordinates, which is exactly what PR 6676 took out of the boundary pre-pass. `QueryFoot` holds the closed-form point `foot.at()` (`geom::Surface::foot`). A circle's `θ = 0`, `(start − centre).normalize()`, carried the round trip's ulps into `Round::place` and `arc`.

**Ruled (ONECOPY orchestrator, option A, as an application of C5).** A circle leg's parametrisation starts at a point the door holds on it (`SurfaceCurve::starting_at`), and its centre, axis and radius stay the chart's (C5, README:917). `θ = 0` is the point's offset from the centre with its axial component projected out, so the frame stays square to the axis.
- The doors that use it: the cone and torus doors start their circle legs at the query's closed-form foot. The sphere door starts its great circle at the foot and its circle about the query at the corner it holds.
- C5's text is unchanged. `on_surface`'s module doc says the same at its scope: a curve's **locus** is built from chart coordinates on its record, so no curve off the surface can be built as one; its parametrisation may start at a point the caller holds on it.

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
- C, centre and radius from the foot. Both change the locus, which is what C5 governs ("built from chart coordinates on a surface record", `crates/geom-brep/README.md:917`), so they need an argument for what keeps such a curve on its surface.
- The frame a `SurfaceCurve` carries in its section and plane: `Section::HalfPlane`'s `toward` azimuth of `u`, a line's plane normal `f.axis × f.radial(u)`, and the crossings read through them. This is the 18/26 that C leaves.

## Closed on the measurement (2026-11-10)

**What is left is the band's edge, not a frame reaching a decision.** Every disagreement the pins count is a query at `±1e-8`, the band's escalation. There one frame decides and another escalates: the reading flips between decided and escalated at the edge of the band, as any band does. None of them is at the band's zero or off the band, so no frame turns one decided answer into another. `ruled_region_rows::a_cones_circle_legs_read_no_chart_frame_at_the_escalation` (35) and `torus_region::tests::a_torus_faces_circle_legs_read_no_chart_frame_at_the_escalation` (31) pin the counts, so a regression shows as a count that rises.

**Measured on main before closing**, on the two pins' queries (scratch edits, not kept):

| variant | C5 | cone | torus |
|---|---|---|---|
| main (A: circle legs start at the foot) | applied | 35 | 31 |
| B′: a ruling's origin moved to the foot's projection onto the chart's own line (the locus kept, the line analogue of A) | applied | 32 | n/a (no rulings) |
| B′ and the ruling's plane normal from its own direction (`axis × dir`, not `axis × radial(u)`) | applied | 32 | 31 |
| B′ and the half-plane's `toward` from the foot's azimuth, not the chart's `u` (cone ruling, torus meridian) | applied | 32 | 31 |
| B: the ruling's origin at the foot itself (the line moves) | changes the locus | 18 | 31 |
| C: B and a parallel's or meridian's centre and radius from the closed-form foot | changes the locus | 18 | 26 |

What A leaves sits in the locus itself, in its last ulps. It is not in the parametrisation, the plane or the section.

**Considered and not taken:**
- **B′:** a line `starting_at` for 3 of the cone's 35 queries.
- **B and C (option 2):** they build the locus from `geom::Surface::foot`, which re-reads C5 ("a curve on a surface is built from chart coordinates on a surface record", `crates/geom-brep/README.md:917`). They would need an argument for what keeps such a curve on its surface, and they halve the residue at best (cone 35 → 18, torus 31 → 26).

Neither is worth a C5 amendment for a residue that lives only at the band's edge.
