---
id: a-foot-reads-its-clearance-through-the-three-way-door-and-accepts-one-sign
kind: issue
title: A foot reads its clearance through the three-way door and accepts only Positive
status: open
opened: 2026-11-24
priority: P3
cost: E
---


Found by a designer of
`an-undecided-at-rest-decision-climbs-the-ladder-on-a-margin-that-is-not-zero`
(2026-11-24), read in the code and in M11's item 5.

`geom`'s `Foot::on_carrier` (`crates/geom/src/curves/on_carrier.rs`)
reads `curve_carrier_clear` through `decide_reported` and then accepts
only `Positive`: a `Zero` ends in the same refusal as undecided. The
funnel has a door that says so, `k_stats::decide_positive`; as written
the pass set lives in a `match` the funnel cannot see, so the funnel
asks the symbolic tier for a `Zero` nobody can use (683 s of the 983 s
of climb M11 measured, before the refutation of PR 7698).

Fix as written: read through `decide_positive`, and sweep for the shape
(a `decide`/`decide_reported` whose `match` passes one sign) in the
same crate. Ground is CONIC's and ONECOPY's; filed here because the
box drive's cost found it. (BOXFINISH orchestrator)
