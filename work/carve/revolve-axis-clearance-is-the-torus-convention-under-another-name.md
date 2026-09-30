---
id: revolve-axis-clearance-is-the-torus-convention-under-another-name
kind: issue
title: carve: revolve's axis_arc_clearance decides the ring-torus convention R - r under its own name, ending 'not supported' when definite and the declare menu in band
status: open
opened: 2026-09-30
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
  arc at loop {i} segment {j} clears the revolve axis is too close to
  call: {source}", the whole `Indeterminate`, so it ends in
  `COINCIDENCE_RECOURSE` (a declaration no revolve takes).

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
