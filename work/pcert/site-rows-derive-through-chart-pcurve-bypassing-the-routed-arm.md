---
id: site-rows-derive-through-chart-pcurve-bypassing-the-routed-arm
kind: issue
title: topo::pcurves::site_rows certifies without the fitted door, so a site mint clears a spline carrier's projected row on an analytic face instead of deriving it
status: open
opened: 2026-10-07
---


Found by a spline-carrier designer (PR 4261, fork-log row 85).

`topo::pcurves::site_rows` derives a face's rows through `chart_pcurve` directly, not through `analytic_derive`. So on a face whose class the mint routes elsewhere (today the sphere's general circle, through the fitted lane), a site mint clears the row rather than deriving it. It then relies on the producer's closing mint to restore it.

That is consistent with "doors may drop rows" as long as every public Euler door that reaches a routed face has a closing mint. Check that it does. If one has no closing mint, `site_rows` should go through the routed arm.

Under PR 4261's projected-image route, every such class has one derivation, which removes the asymmetry.

## Narrowed by the projected image (branch `pcert/projected-image`, 2026-10-08)

There is one derivation now. `site_rows` and `analytic_derive` both
derive through `geom_brep::chart_pcurve_over`, so a site mint images a
sphere's general circle exactly as the mint does. A projected circle
reads no fitted door, and its row is derived at the site.

**Still open, for spline carriers.** `site_rows` certifies through
`certify_walked(…, None)` (`crates/topo/src/pcurves.rs`): a `Decide`
door holds no fitted door. A net's projected row reads its hull terms
through that door, so at the site it refuses at check 4 and the face is
cleared, relying on the producer's closing mint. Measured:
`Body::set_edge_curve` re-describing a cylinder rim as a spline leaves
the cylinder face rowless, and tier 3 reports it `Unminted`
(`crates/sweep/tests/reach_split_gate_per_face.rs`,
`a_spline_edge_whose_belly_crosses_the_plane_refuses`). `mint_pcurves`
restores the row.

What a fix owes is one of two things. Either hand the scalar's fitted
door to the site mint from the doors whose bound is already
`AtRestPolicy` (`set_edge_curve` is one), or audit that every public
door reaching a net-carrying analytic face has a closing mint.
