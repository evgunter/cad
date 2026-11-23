---
id: the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis
kind: unit
title: The revolve mints a Spheroid wall from an ellipse arc centred on its axis with a principal direction along it, full and partial, and the body is at rest
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: an-ovoid-cannot-be-authored
blocked_on: [a-spheroid-is-a-surface-kind-that-nothing-mints-yet, profile-validation-reads-an-ellipse-arcs-pairs-on-the-planar-reader]
---



Unit 5 of `an-ovoid-cannot-be-authored` (Ev, PR 7289), part 2's door:
"minted by the revolve under two margined verdicts (the arc's centre
on the axis; a principal direction along it)". Ruled text: `docs/DESIGN.md`
`[surfaces]` ("it is what a revolve mints from an ellipse arc centred
on its axis with a principal direction along it"). The precedent is
`Surface::Spindle` stage 1 (PR 6473): the revolve reads the arc, picks
the kind, and the body is at rest.

## What is true when it is done

- `sweep::revolve`'s segment classification
  (`crates/sweep/src/revolve/axis.rs`, `classify_segment`, `WallKind`)
  reads an ellipse segment with two margined verdicts: the centre on the
  axis (the reading `ClearanceDecision::ArcCenter` makes for an arc), and
  a principal direction along the axis (a new named margin, levered to
  metres by the semi-axis it turns). Both Positive mints
  `WallKind::Spheroid`, built in `revolve/surfaces.rs`'s `wall_surface`
  as `Surface::Spheroid` with `equatorial` the semi-axis across the
  axis and `polar` the one along it.
- Either verdict definitely off refuses typed (row 2) naming
  SPLINEWALL's `the-revolve-mints-the-exact-nurbs-wall-where-no-kind-fits`;
  either in band refuses as a sliver, as `ArcCenter` does.
- Meridian endpoints on the axis are the spheroid's poles, through the
  sphere's pole machinery (`chart_singularity::singular_points`'
  spheroid arm from unit 1). `radial_extent` and `analyze_contact`
  read an ellipse's interior extremum, not a circle's.
- The readers a revolved body needs at rest learn the spheroid, each
  one the sphere's composed with the stretch where the reading is
  affine-invariant and its own where it is metric (unit 1's census
  says which): the pcurves (`geom-brep/src/pcurve_cache.rs`,
  `pcurve_cache/projected.rs`), the chart region
  (`topo/src/chart_region.rs`, the sphere's region door), injectivity
  (`topo/src/injective.rs`), clearance (`topo/src/clearance.rs`),
  face normals (`topo/src/face_normal.rs`), tier 3
  (`topo/src/validate.rs`).
- Through the public revolve door, each of these builds and is at rest
  (tier 3) at every lane scalar: a full prolate spheroid (a
  half-ellipse closed by the axis), a full oblate one, a partial
  revolve of each (a wedge), a spheroid zone between two plane caps,
  and the lily-shaped meridian (cones and a spheroid belly), one of
  them translated by 1e3.
- `mesh::tessellate`, `measure` and STEP export of such a body refuse
  typed (unit 1's refusals) until units 6 to 8.

## Check first

Whether the at-rest readers can take the spheroid as the sphere's
composition without a new chart arm shape in the pcurve layer. If unit
1's census put the pcurve envelope lemma's speed terms on the metric
side, this unit carries the spheroid's own speed bound
(`|∂P/∂v|` between `min(a, c)` and `max(a, c)`). If tier 3 needs more
readers than the census listed, report the list before building past
it; if it needs a design choice (a pole on a chart whose metric is not
the sphere's), **stop and report**.

## Leaves for later

- Off-axis and tilted ellipse arcs: SPLINEWALL.
- Tessellation, measure, STEP, plane × spheroid: units 6 to 9.
- A blend, offset or shell on a spheroid face stays refused (parity
  with the sphere at first).

## Rows and ground

- Changes no other row.
- Crosses LATHE (`sweep/src/revolve/*`), PCERT (`pcurve_cache*`),
  CHARTTAIL (`chart_region.rs`), RESTFRONT and ONEDOOR (`validate.rs`),
  EMBED (`injective.rs`: `a-spindle-sheet-or-an-approximated-face-has-no-window-in-the-injectivity-engine`
  is the spindle's precedent for a window the engine lacks), CLEAR
  (`clearance.rs`). Announce on LATHE's log: the revolve's
  classification gains a kind.
- Review: **dual**: it mints a new kind through a public door and puts
  a new kind's bodies at rest, the widest correctness surface of the
  build. Difficulty **H**.
