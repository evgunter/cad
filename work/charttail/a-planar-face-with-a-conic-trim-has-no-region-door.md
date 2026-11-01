---
id: a-planar-face-with-a-conic-trim-has-no-region-door
kind: issue
title: chart_region refuses a conic trim in a plane chart (NonPlanarTrim), so no recorded seat on an arc-bounded planar face verifies, and no region-confined census rung can back an event on one
status: open
opened: 2026-10-10
priority: P2
cost: H
---

Found by CONTACTHOLD (`contacthold/census-edge-pass`), measuring
`census-edge-pass-reads-no-line-conic-crossing`.

`chart_region.rs`'s loop extraction (`loop_uv_polygon`'s plane-chart arm,
the `geom_brep::chart_pcurve(..)` → `pcurve_entry` path) refuses a circle
or ellipse edge of a planar face `NonPlanarTrim`: the arc's chart image
is a sinusoid `Harmonic`, outside the module's stated trim inventory
(module docs: "conic trims in a plane chart — refuses NonPlanarTrim").
That is the stated envelope, not a wrong answer, but its reach is wider
than the envelope sentence suggests:

- **No recorded seat on a cylinder cap verifies.** Measured on a
  half-disc post (`tier3_tests::half_disc(0.25, 0.5, (0.3, 0.2, 0.0))`,
  cap at `z = 0.5`) under the shelf `[0, 0.9] × [0, 0.30] × [0.5, 0.54]`
  with a `PatchContact` (cap, shelf underside): the confirm pass's Door 2
  refuses `CensusUnsupported { FacePair(cap, underside),
  ChartRegion(NonPlanarTrim { face: cap, half_edge: <the arc> }) }`.
  Since INTENT's E (booleans glue on Zero), an undeclared flush seat
  makes the same records a declared one did, so every at-rest seat on a
  round cap or a planar face with a round hole lands here.
- **No region-confined census rung can back an event on such a face.**
  `census::pair_region_verified` reads the same Door 2, so the crossing
  rung (`ee_cross_backed`) and `ef_bound_backed`'s face-pair arms answer
  `false` for any pair one of whose faces carries an arc. In particular a
  line edge crossing a cap's rim (`CutAt::CurvedCrossing`, once `ConicCrossing`) can have no
  face-pair backing rung until this door reads the arc (its edge-edge
  record backs it, as tier 3′'s pass 5′ owes it one): the crossing point lies on
  the arc-bounded face, which is always one face of the pair.

Wanted: a plane chart reads a conic trim exactly — a circle or ellipse
arc's region contribution (signed area by the closed-form sector, the
point-in-region walk by `splitting::containment::carrier_loop`'s
`ConicArc` rows, which the census's crossing cuts already read) — so
`classify_chart_region`'s overlap on two planar faces does not need a
polygonal trim. A witness row: the half-disc seat above with its patch
record verifies (`PositiveArea`), and the census's `EdgeFaceOverlap` on
the shelf edge between its two rim crossings is then the only finding
left for a line × conic crossing rung to take.
