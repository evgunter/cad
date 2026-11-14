---
id: pcurves-mint-holds-the-chart-facts-every-pcurves-file-imports
kind: issue
title: pcurves/mint.rs holds two concerns, the chart facts and edge accessors every pcurves file imports and the mint pass, so decision.rs and mint.rs import each other
status: open
opened: 2026-11-14
priority: P4
cost: M
---


Filed by the ONEDOOR orchestrator on 2026-11-14, from the review of PR 6939 (the `pcurves.rs` split).

After the split, `decision.rs` and `mint.rs` each import 11 of the other's items:
- decision reads mint's chart facts: `DescribedChart`, `ChartPeriods`, `net_reach`, `shift_v_branch`, `sphere_twin`;
- mint reads decision's walk.

`mint.rs` and `lift.rs` also import each other: `loop_rows` one way, `edge_record` and `entry_vertex` the other.

The cause is that `mint.rs` holds a sixth concern beside the mint pass: the chart facts and edge accessors that every pcurves file reads. Moving them into their own file, for example `pcurves/chart.rs`, would leave the files importing in one direction. The four chart-branch movers the MAKERS guard lists are split today: `lever_periods` and `branches_astride_as` in decision, `shift_v_branch` and `sphere_twin` in mint. They would land together.

P4: no behaviour depends on it. The glob re-exports (`pub use {decision,lift,mint,site,tier3}::*`) also hide each file's real sibling imports; that hiding would go with it.
