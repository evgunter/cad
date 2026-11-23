---
id: unsourced-levers-the-stand-ins-and-the-carrier-terms
kind: unit
title: The unsourced arms that are not a lever at all: a tolerance or rounding standing in for an arm, and a carrier's stored term, each re-posed by the role its length plays
status: closed
opened: 2026-11-21
priority: P1
cost: H
parent: unsourced-levers-await-a-source
branch: oneround/unsourced-stand-ins
closed: 2026-11-23
pr: 7268
---


A cut of `unsourced-levers-await-a-source` (Ev, PR 7141: no fourth source, each site re-posed by the role its length plays, the moved verdicts accepted): families 2 and 4 of the parent row, twelve references by `lever-sources.counts` as `oneround/unsourced-bits` (PR 7227) leaves it. Stacked on that PR.

- **A tolerance or a rounding standing in for an arm** (six): `geom_brep::dihedral` (2: `wedge_reads_zero_at_the_arm` and its `_offer`, `band.escalate()`), `topo::boolean::fate` `sign` (a unit arm on a unit vector's length or a turn count), `topo::boolean::join` `off_conic_slack`, `topo::pcurves::decision`'s joint room (`d − 2ε`), `topo::validate` `resolved`. None is a lever of anything: each is re-posed as what it is (a rounding charge, a resolution, a count with no margin), through the door that says so.
- **A carrier's stored term or a coefficient** (six): `geom_brep::pcurve_cache` (5: the focal `beta`, a map coefficient; the Villarceau `tilt`, whose honest arm is the Villarceau circle's radius; `unit_sign` (2), a speed with no span; `spiric_chart_axis`, the chart's `R`) and `props::curved` `require_cone_circle` `off_axis` (a residual). The designers were unsure of the focal β and `unit_sign`: the lane measures before it re-poses.

`lever-sources.counts` falls by twelve. Each re-pose is measured by a differential on both lanes at two ε; a verdict that moves is reported with its direction.

Not this unit: the sums and arc lengths and `profile` (the sibling unit `unsourced-levers-the-sums-the-arc-lengths-and-profile`), anything that changes what a record holds (Ev's condition), the bare arms of `Margin::over_lever`, deleting the `unsourced` doors.

## Three more sites (2026-11-21)

From the review of PR 7227: `geom_brep::intersect`'s `pn_apex_tangent_reach`, `pn_apex_point_reach` and `pair_bound` (far, near, offset) are this unit's. PR 7227 had spelled them `levered_inv(size, sine).value()` with `size = |apex_gap| + gap_err`, a bare division under a door's name; its fix pass puts them back on `Lever::unsourced` with their pins. They are family 2: the plane's apex distance plus a hand rounding term standing in for an arm.

## Read against the tree (2026-11-21, `oneround/unsourced-stand-ins`)

Each site checked before it was changed; corrections to the classification above:

- **`unit_sign` is not a speed with no span.** `part` is the cosine of the circle's unit tangent against a unit direction, levered at the arm a turn of that tangent moves the circle by: the circle's curvature arm (`circle_curvature_arm`), its radius. The comment's "`R·part` is the carrier's speed" is the same product read as a rate. The circle's radius equals `R` within the band (the `_radius` gate), to the bit on an unmoved Villarceau circle; a moved one (`envelope_lemma_fuzz::a_moved_villarceau_circle_is_refused_typed_and_never_off_within_the_band`) moves the tilt's, the tube rate's and the orientation's bits, and no verdict (PR 7268's review, MINOR-3).
- **The focal `β` is neither a lever nor a coefficient to lever.** `1 − |β|` is the least modulus of the true-anomaly map's `atan2` argument, a point on the circle of radius `|β|` about `(1, 0)`: the map is cut-free exactly where it is positive, so its sign alone is the decision (`Margin::sign_of`, `Band::enclosure_sign`).
- **`spiric_chart_axis` is an orientation sign, not a tilt.** The mint's incidence test has read the oval's whole turn on the ring torus before the axis is read (a spiric's placement is never the edge's alone), so the chart holds the oval and its axis is the carrier's or the opposite: the cosine is `±1` by construction, read by its sign (`Margin::sign_of`, `Band::enclosure_sign`). An arc's extent is no lever of it: levered at a short arc's extent the reading lands in band and refused an oval on its own torus as "perpendicular" (PR 7268's review, MINOR-1; `a_spiric_on_its_own_torus_is_served_over_any_span`).
- **`fate` `sign` is two quantities.** At `bisector` it is the bounds' sum, a sine-like reading with a lever in hand (the bounds' reaches); at `tip_wraps` it is a count of half-turns, an integer plus an `atan2` sum's rounding.
- **The joint room's `d − 2ε` is a length and a stand-off**, not a rounding: the decided quantity is a quarter of the vertex's parallel's chord, `d·sin(step/4)/2` with the step a fixed half or whole period, a length times a constant (`Margin::of`); the ends' `2ε` stand-off is the band's (`Band::padded`).
- **`intersect`'s five quotients, `join`'s slack and `validate`'s `resolved` are not stand-ins for an arm either**: a stand-off (with its rounding) scaled by a dimensionless factor, and a rounding against a share of the band. `dihedral`'s two are a dimensionless sine against the band's own ratio `1/K`. None had a door that names it; decided on PR 7410 (`unsourced-levers-with-no-door-that-names-them`), converted by `the-ten-readings-that-are-not-levers-are-decided-by-what-they-are`.

## Closed (2026-11-23, PR 7268)

Seven of the references are re-posed by what each length is, as "Read against the tree" above says: the Villarceau tilt and `unit_sign` levered at the circle's own radius, the focal β and the spiric chart's axis read by their sign, `fate`'s bisector at the bounds' reaches, the joint room as a length with the band's stand-off, the cone circle's off-axis residual. A single FULL review, a fix pass and a delta read that approves with two minor fixes, taken. What the unit left, each in its own file:
- `unsourced-levers-with-no-door-that-names-them` (P1): the ten references that are no lever of anything, decided by Ev on PR 7410 and converted by `the-ten-readings-that-are-not-levers-are-decided-by-what-they-are` (PR 7442, stacked on this one).
- `test-fixture-arms-pass-through-the-curvature-door`: three test fixtures arm a unit length through `Lever::curvature_radius`, which neither lever gate counts.
- `work/pcert/the-villarceau-tilt-gate-decides-only-which-in-band-refusal-is-named.md` (P3).

The row is `docs/model-compare-log/rows/PR-7268.md`.
