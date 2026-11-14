---
id: chart-facts-are-a-sixth-concern-inside-pcurves-mint
kind: issue
title: The chart's description and periods are a sixth concern inside pcurves/mint.rs, and decision.rs and mint.rs import each other
status: open
opened: 2026-11-14
---


Found by the review of PR 6939 (2026-11-14), the split of `pcurves.rs`.

`crates/topo/src/pcurves/mint.rs` holds two things. One is the mint:
`PcurveMintError`, a row's derivation (`nurbs_iso_derive`,
`derive_image`, `derive_face`) and the minting doors (`mint_pcurves`,
`mint_pcurves_of`). The other is what a chart is, read by every other
file of the module: `DescribedChart`, `chart_mints`, the metres a
chart coordinate is worth (`chart_u_arm`, `v_meter`, `polar_arm`,
`net_reach`), its periods (`chart_u_period`, `chart_v_period`,
`ChartPeriods`, `VPeriod`, `net_closes_as`) and the movers between
branches (`shift_v_branch`, `sphere_twin`).

So `pcurves/decision.rs` and `pcurves/mint.rs` import each other. The
decision reads the chart facts out of the mint
(`ChartPeriods`, `DescribedChart`, `chart_u_arm`, `net_reach`,
`shift_v_branch`, `sphere_twin`, `v_meter`), with `derive_image` and
`PcurveMintError` for the walk's drivers; the mint calls the walk
(`walk_loop`, `walk_runs`, `certify_walked`).

Owed: a file for the chart facts, which the decision, the lift, the
site doors, tier 3 and the mint then all read, leaving the mint
importing the decision and not the reverse. It cannot be named
`chart.rs` without colliding with `crate::chart` for
`source_walk::fn_body`, which finds a file by its stem. `derive_image`
and `PcurveMintError` are the residue to place: the walk's drivers in
`decision.rs` raise the mint's refusal and derive a row.

Not done in PR 6939, which is a move along the seams its row named.
