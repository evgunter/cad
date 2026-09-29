---
id: mev-null-leaves-a-complete-curved-face-half-minted
kind: issue
title: mev_null adds two rowless half-edges to a face whose pcurve rows are complete, the one Euler operator still returning a face half-minted
status: open
opened: 2026-09-29
priority: P2
cost: D
---


Found by `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`
(branch `topo/mint-rows-at-the-mint-site`, PR 3160), whose ruling is
that no Euler operator returns a face half-minted.

`mev`, `mef` and `mekr` now re-mint, before they mutate, a face whose
rows are complete (`pcurves::site_rows`). `Body::mev_null`
(`crates/topo/src/null.rs`) shares their surgery
(`mev_fan_execute`, `mev_lone_execute`) and hands it no plan: a null
edge has no carrier, so no chart image can be derived for its halves.
On a face whose rows are complete it therefore returns the face
half-minted, two `MissingCache` findings — the one Euler operator that
still does. The posture table carries it as `Neither` with that
sentence (`pcurves::staleness_posture::DECLARED`, the `mev_null`
entry), and `PcurveMintError::MissingCache`'s docs name it among the
doors that can still produce the state, so the unit's
"kernel-bug detector" reading of that arm excludes it.

It is transient by construction (tier 2 refuses a null edge at rest,
and the boolean's closing mint re-derives the body), but the state
exists between `mev_null` and that mint, and `set_edge_curve`, which
gives the scaffold its carrier, mints no row either.

Shapes, for whoever takes it: (a) `mev_null` gives a complete face the
same answer `site_rows` gives a face the closed-form lane cannot mint
— it stores nothing (`SiteRows::Clear`), unminted rather than
half-minted; or (b) `set_edge_curve`, on the null edge's first
description, re-mints a face whose rows are complete except for that
edge's two halves. (a) is one call at the two `mev_null` arms and
costs the boolean nothing it keeps (its closing mint clears the map
first); (b) keeps the rows but needs a rule for "complete except for
these halves". Measure first which intermediate step of the boolean
reads a row between the scaffold and its description (`split_edge`'s
restriction does), since (a) changes what that step sees.
