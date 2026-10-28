---
id: a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim
kind: issue
title: A loft whose arc bulge differs per section refuses its arc-wall cap rim at the arc-rim pick
status: open
opened: 2026-10-23
priority: P2
cost: M
---


## Finding

The second shape of
`a-rational-wall-beside-an-integral-one-skins-its-shared-corner-off-unit-weight`
(`work/isotail/`), split out because its mechanism is not that row's. A
square with one arc edge whose bulge differs per section (`tan(π/8)`,
`tan 50°`, `tan 30°`), `loft_body::<f64>` at v-degree 2, z = `[0, 0.8,
2]` or `[0, 1, 2]`, refuses at `topo::mint_pcurves` with

```text
Pcurve(Certify { half_edge: HalfEdgeKey(3v1), error: IsoUnsupported { what:
"no boundary row of this chart, in either u direction, holds the circle at
the interior probe — not an arc rim spanning the chart's u domain" } })
```

raised in `topo::pcurves` `nurbs_iso_derive`'s `arc_rim` closure when
`geom_brep::pick_arc_rim` (`edge_nurbs.rs`) returns `Ok(None)`.

**Measured** (2026-10-23, on `iso/loft-seam-per-row-lane` at `8ca0f5481a`, with `skin_on`
choosing its lane per control row): still refuses, so the per-row lane
does not reach it. The corner columns are exact there; the seam-carrier
compare is not what fails.

What the walls hold, measured on the same head: the arc wall's `u`
structure is the union of the three arcs' piece structures
(`[0,0,0,⅓,⅓,½,½,⅔,⅔,1,1,1]`, nine control points; with bulge
`tan(θ/4)` the arcs are 90°, 200° and 120°), so section 0's arc
reaches the wall refined, not as authored. The wall's `v = 0` row equals compatible section 0 bitwise in
its weights, but two rational control points (indices 2 and 6, weight
`0.8698252360829102`) come back one ulp off in `x`
(`1.361302095513585` against `1.3613020955135853`) — the homogeneous
lane's `(w·x)/w` round trip, which `skin_on` leaves in place for
rational rows. The `v = 1` row is bitwise its section. Not
established: whether the probe's refusal comes from that ulp, from the
refined (multi-span) row against the single-arc circle carrier, or
from something else in `pick_arc_rim`. The next measurement is
`pick_arc_rim`'s own reading at `HalfEdgeKey(3v1)` on this fixture.

`loft_v_is_the_section_set::an_arc_walled_loft_is_the_same_under_every_spelling`
asserts that loft's geometry only, for this reason.

**An interior section reaches the same class** (2026-10-28,
`tess/degree1-loft-slabs`, PR 5923). At v-degree 1 through `k ≥ 3`
sections each interior section is a ring of rims described in the row
`v = 1` of the slab wall below it, and an arc whose sub-arc count is
smaller than another section's reaches that row refined. A square
whose +x side carries bulges `[1.0, 0.2, 1.0]` at z = `[0, 1, 2]`
refuses `LoftError::SectionRim { section: 1, source: None }`
(`geom_brep::pick_arc_rim` → `Ok(None)`), and so do
`[1, 0.2, 0, 0.7, 1]` (section 1) and `[0, 1, 0.3, 0]` (section 2).
These lofts built on `main`, where the degree-1 wall spanned every
section and was unmeshable; the same sections at v-degree 2 build, pass
tier 3 and mesh, which is the refusal's recourse. The arc-rim
certificate (`run_iso_arc_checks`, `pcurve_cache.rs`) compares the
column's knots against the arc's own uniform sub-arc breaks exactly, so
a refined row is refused there too, even where the image's map is right
(knot refinement keeps a single arc's `u ↦ angle` map). Describing the
refined row is one fix for both this case and the two-section cap rim.
Pinned by
`sweep::all degree1_loft_slabs::an_interior_arc_refined_by_its_neighbours_refuses_typed`.
