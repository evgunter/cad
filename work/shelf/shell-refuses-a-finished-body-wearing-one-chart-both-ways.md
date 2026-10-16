---
id: shell-refuses-a-finished-body-wearing-one-chart-both-ways
kind: issue
title: shell refuses ChartSenseMixed on a finished body that wears one chart with both senses within one solid, which the same geometry on two charts shells
status: open
opened: 2026-10-08
priority: P3
cost: M
---


`topo::shell_open` (`crates/topo/src/shell.rs`, "Decide: each solid's
charts, each with ONE orientation") refuses `ShellError::ChartSenseMixed`
when one solid wears a chart with faces of both senses, because the
offset doors move a chart as one and such a chart has no single inward.
A finished body reaches it:
`crates/step-import/tests/shell_reads_a_chart_worn_both_ways.rs`. The
Z-step prism (`[0,2]×[−1,0] ∪ [1,3]×[0,1]`, height 1) has two faces on
y = 0 that face opposite ways. When the file cites one PLANE for both,
one face `.F.`, `step_import::adopt`'s `attach_surfaces` dedups them onto
one surface key. The body passes `AtRestBody::validate` and `shell(_, 0.1)`
refuses it. The same body written on two planes shells (volume 1.568).
Files from other systems that reuse one plane entity across faces of
opposite sense reach this path through the importer.

The geometry is shellable. The verb could re-key a mixed-sense group
per sense on its own clone before moving it, so each sense moves inward
as its own chart, or it could move each sense's wearers by their own
signed distance. Either way the cavity and the result keep keys the
caller can name, so the choice touches the naming record (`ShellNaming`)
and the open designation's chart-completeness gate
(`OpenFaceChartPartial`).
