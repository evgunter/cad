---
id: a-one-segment-loops-revolve-and-loft-wall-wraps-a-period-with-no-seam
kind: issue
title: A one-segment loop's revolve or loft wall wraps a period no seam is defined on (a torus's tube angle; a spline wall's u), so both verbs refuse it: weigh a new seam kind against splitting the wall
status: closed
opened: 2026-10-06
priority: P1
cost: H
refs: [4175, one-segment-loop-revolves-and-lofts-to-one-wall]
closed: 2026-10-06
---



Found by unit 3 (`one-segment-loop-through-builders`), which stopped on
it as a design fork: D1, the row and the ratified pages do not settle it.

## What

A one-segment loop (D1's full turn) has one segment, so each sweep verb
builds one wall over it, with one strut whose two halves both bound that
wall. The wall wraps a period, and the strut is where it is cut:

- **extrude**: a cylinder wrapping its azimuth `u`. The cylinder's
  `u_ref` is aimed at the vertex (`extrude::side_surface`), so the strut
  IS the chart's seam, and it is described as one. This builds, tiers
  1–3 and closed-form volume (`crates/sweep/tests/one_segment_loop.rs`).
- **revolve** (partial or full): a torus wrapping its TUBE angle `v`,
  cut by the latitude strut at the vertex. A seam in this kernel is the
  `u_ref` meridian only — D1, "A seam is defined SPATIALLY (the u_ref
  half-plane meridian)"; `geom_brep::implicit::seam_frame` meters
  `w·v_ref = 0, w·u_ref ≥ 0` — so nothing describes a cut in `v`.
  Measured on a probe build (far rim first, the strut described at
  rest): the body passes tiers 1–2, then tier 3 refuses
  `CurvedSenseInverted` on the torus face and `mass_properties` reads
  the volume exactly negated (−θRπr² for a θ patch), at every vertex
  phase tried (0, 1, π) and both θ signs. The face's loop is the same
  cycle the two-arc circle's walls carry (strut down, near rim forward,
  strut up, far rim backward); the readers take the strut's two halves
  as one image. STEP import already meets this: `step-import`'s
  `normalize::full_torus` re-mints a one-face torus as two half-faces
  split in `v`, "the splitting a natively revolved torus carries".
- **loft**: one spline wall closing on itself in `u`, its strut both the
  `u = 0` and the `u = 1` edge of that face. A spline chart is not
  periodic and has no seam frame (`seam_frame` returns `None` for
  `Nurbs`). Measured on a probe build (the strut's iso run back, top to
  bottom): `topo::mint_pcurves` refuses `Pcurve(Certify { …,
  ResidualExceeded { check: Envelope } })` on the second half. The
  wall is also the C⁰-creased skin TESS's
  `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`
  names.

Until this is settled both verbs refuse, typed and naming the loop:
`RevolveError::OneSegmentLoop` (after the axis classes, which keep
refusing a full turn that reaches the axis by what is wrong with it)
and `LoftError::OneSegmentLoop` (the geometry door `loft_geometry`
skins the wall; the body assembly refuses).

## Options

1. **A seam on the wrapped period.** A torus seam in `v` (the chart
   gains a reference for where `v` starts, or the seam is allowed at a
   chosen `v` level), and a two-image seam on a closed spline chart.
   It changes D1's seam definition and reaches the description,
   certification, pcurve, flux and material-sign readers, mesh and the
   boolean's seam handling. The body is the one the parent row drew:
   one wall, one strut.
2. **The verb splits the wall.** Revolve cuts the torus wall at the
   antipodal latitude into two faces on one surface key (STEP import's
   normalization, and today's body for a two-arc circle); loft cuts its
   spline wall at an interior knot (TESS's first remedy for the crease
   cuts at every one). The profile stays one segment; the body carries
   a station the profile does not, so the naming lane needs a role for
   it (EMIT).
3. **Keep the refusal** and have unit 4 (`circle-lowers-to-one-segment`)
   keep lowering `circle` to two arcs where a revolve or loft reads it —
   which is the two-form arc the parent's ruling (A2) set out to remove.

Unit 4 waits on this: once `circle` lowers to one segment, every revolve
or loft of a `circle` takes whichever answer this row gets.

## Outcome (#4175)

Decided by Ev on #4175 (2026-10-06): option 1, as D1's **wrap edge** —
a property of a face, placed where the construction cut, with the
closed direction read from the carrier's chart class. Its
implementation is the unit `one-segment-loop-revolves-and-lofts-to-one-wall`;
until it lands, revolve and loft keep refusing `OneSegmentLoop`.
