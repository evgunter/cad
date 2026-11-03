---
id: revolve-axis-clearance-is-the-torus-convention-under-another-name
kind: issue
title: carve: revolve's axis_arc_clearance decides the ring-torus convention R - r under its own name, ending 'not supported' when definite and the declare menu in band
status: closed
opened: 2026-09-30
priority: P1
cost: M
closed: 2026-11-03
---


(TOPO, the fix pass of PR 3506: the review's MINOR-4, a D4 ¶1 (iv)
fork on CARVE's ground, filed and not fixed.)

## What

`sweep::revolve::axis::classify_segment` (`crates/sweep/src/revolve/axis.rs`,
the arc arm whose centre is definitely off the axis) decides
`axis_arc_clearance`, `Margin::of(rc − radius)`: the swept circle's
distance from the axis less its radius, which is the minted torus's
`R − r`. That is the ring-torus convention's ring half, decided under
another name:

- definitely `Zero | Negative`: `RevolveError::UnsupportedToroid`,
  rendered "the arc at loop {i} segment {j} would sweep a horn or
  spindle torus (its circle reaches the axis), which is not supported.
  Recourse: keep the arc's circle clear of the axis"
  (`crates/sweep/src/revolve/mod.rs`);
- in band: `RevolveError::SliverAxisClearance`, rendered "whether the
  arc at loop {i} segment {j} clears the revolve axis is undecided:
  {payload}. {ending}". For `axis_arc_clearance` the ending is
  `COINCIDENCE_RECOURSE` ("move the geometry") with the length's
  tolerance arm (`ClearanceDecision::ArcClearance`); the sphere-class
  span (`axis_arc_span`) is lever-only.

Two stories for one decision, and neither is the convention's: PR 3506
gave the convention its closed type, `geom_brep::TorusConvention`
(`crates/geom-brep/src/torus_convention.rs`), whose `Ring` half ends
both arms in "make the tube radius clearly smaller than the ring
radius", with the tolerance a positive in-band margin gives; the
Boolean's pierce and tier 3 read it. A horn or spindle is also not "not
supported" but a shape with no representation (D1: "`UnsupportedToroid`
is permanent … spindle tori have no representation").

`carve-refusals-short-of-the-shape-guard.md` lists `SliverAxisClearance`
among the forwarded menus; this row is the fork between it and its
definite sibling.

## Repair shape

Decide the clearance as `TorusConvention::Ring` (`geom::ring_torus` on
`(rc, radius)`, keeping its reporting margin), and end both arms from
`TorusConvention::Ring.sized()` and `.refused(...)`, phrased at the
revolve's own lever (the profile arc's distance from the axis) if the
convention's words do not fit a profile; drop "which is not supported".

- 2026-11-02 — moved from CARVETAIL to LATHE by `git mv` when CARVETAIL merged into LATHE, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and CARVETAIL had fallen far under it).

## Closed (2026-11-03): the decision retired with the spindle sheet

SPIRICARM's `the-spindle-sheet-away-from-its-apexes` (piece 2) made the
revolve read the arc: `axis_arc_clearance`, `ClearanceDecision::ArcClearance`
and `RevolveError::UnsupportedToroid` are gone. The margin `rc − radius`
is now `axis_arc_reach` (`ClearanceDecision::ArcReach`), which picks the
kind rather than refusing one: positive a ring torus, zero or negative a
`Surface::Spindle` sheet with `R = rc`. Only its band refuses, as the
near-horn sliver (`SliverAxisClearance`, the sized ending), as the
ratified arc rule (`crates/topo/README.md`'s structural conventions)
states. No "not supported" text remains, and there is no definite
refusal left to end in the ring convention's words.
