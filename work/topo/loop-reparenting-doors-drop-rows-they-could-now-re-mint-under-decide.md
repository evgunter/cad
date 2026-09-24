---
id: loop-reparenting-doors-drop-rows-they-could-now-re-mint-under-decide
kind: issue
title: kfmrh, mfkrh, ring_move, mef's run and kef drop a moved loop's rows onto a minted analytic face where the Decide mint-site walk could now re-mint that face
status: open
opened: 2026-09-24
---


Found by `half-edge-minting-euler-ops-leave-a-minted-curved-face-incomplete`
(branch `topo/mint-rows-at-the-mint-site`), whose sweep re-read every
sentence that justified a door's drop by the derivation bound.

`Body::drop_rows` (`crates/topo/src/euler_ring.rs`) justified dropping
a moved loop's rows by "every derivation door in `crate::pcurves`
carries the `PcurveFittedLane` bound, which the `Decide` doors that
call this do not have". That stopped being true: `pcurves::site_rows`
re-derives a complete face on an ANALYTIC chart under `Decide` — the
chart image (`chart_pcurve`), the branch walk (`walk_cycle`) and the
certification (`PcurveCache::certify`) — and the three half-edge-minting
Euler operators run it before they mutate. The loop-re-parenting doors
(`kfmrh`, `mfkrh`, `ring_move`) and the run doors (`mef`'s moved run,
`kef`'s remnant) still drop rows across a chart change, leaving a
minted destination half-minted
(`kfmrh_onto_a_curved_face_leaves_it_incomplete_and_tier_three_says_so`,
`ring_move_onto_a_curved_face_leaves_it_incomplete` in
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`) where the same
walk could now re-mint the destination whole. The drop's docs were
re-worded to say so and to point here.

Shape: at such a door, when the destination face's rows are complete
and its chart is analytic, re-mint it through `site_rows` (every loop
`Kept` after the move) instead of dropping; a spline destination keeps
the drop. What it would change: the two `leaves_it_incomplete` rows go
complete, and the loud-to-silent trade
(`work/pcert/validate-pcurves-cannot-tell-a-never-minted-face-from-an-emptied-one`)
shrinks to the spline charts.
