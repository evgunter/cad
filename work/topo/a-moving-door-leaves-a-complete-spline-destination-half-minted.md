---
id: a-moving-door-leaves-a-complete-spline-destination-half-minted
kind: issue
title: The moving doors keep the drop on a complete spline destination, which leaves it half-minted where mev refuses typed
status: open
opened: 2026-09-30
priority: P3
cost: M
design: true
---

Found by `loop-reparenting-doors-drop-rows-they-could-now-re-mint-under-decide`
(branch `topo/reparent-remint`, PR 3531), which built it this way on
its brief.

Ev's ruling on PR 2527 (recorded in
`half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`,
`## Ruled`) makes a half-minted face a state no door can produce, with
"a typed refusal at the fitted frontier". `mev`, `mef` and `mekr` hold
that: on a complete SPLINE face they refuse `PcurveMint { SplineChart }`
before mutating (`pcurves::site_rows`). The doors that move a loop or
run onto a face (`kfmrh`, `ring_move`, `mfkrh`, `kef`, their
`_minting` twins, and `mef`'s new face across a chart change) do not: on a complete spline destination
whose moved rows do not stand, they drop the moved rows and leave the
destination half-minted (`SiteFace::moved`, `site_rows`' spline arm),
because the item's shape said "a spline destination keeps the drop".
The reason given at the site: the doors that move a loop are the ones
that fuse and merge bodies arriving minted, which have no "move before
minting" to take as a refusal's recourse.

**Measured** (probes on every moving door, not committed): no moving
door reaches a complete spline destination in sweep's `ci` profile;
in topo's suite only the rows that pin it
(`loop_reparenting_pcurve_rows::a_spline_destination_keeps_the_drop`,
the deep-copied `…_across_one_payload` rows).

**The fork.** Refuse typed, as `mev` does (one answer at the fitted
frontier; the move fails on a cached spline face); clear the
destination (unminted, a legal state, and silent to tier 3); or keep
the drop (today: half-minted and loud, contrary to the ruling's
reading). `SCALAR`'s note on the ruling's item — a derivation door
taking `Option<FittedLane<T>>` under `Decide` — is a fourth: the doors
that hold the fitted lane could derive the rows.
