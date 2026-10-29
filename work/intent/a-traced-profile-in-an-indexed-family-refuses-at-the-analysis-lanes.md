---
id: a-traced-profile-in-an-indexed-family-refuses-at-the-analysis-lanes
kind: issue
title: A profile drawn on a frame inside an indexed family member refuses Undrawn at every lane but f64
status: open
opened: 2026-10-29
---


Found by INTENT stage 6 PR C (`work/intent/a-sketch-continues-tangent-to-a-surfaces-trace.md`).

At a lane other than f64, a profile drawn on a frame resolves its
first pass against traces read off the nominal document's f64
evaluation (`LaneEnv::nominal_results`, `crates/editor-core/src/eval/wire.rs`).
An indexed family's member is evaluated with `nominal_results: None`
(`crates/editor-core/src/eval/members.rs:227`). The nominal results hold
the family, not the member, so no trace of the member can be read off
them. A traced profile inside a member therefore resolves
`TraceStepFault::Undrawn` (`crates/editor-core/src/trace.rs:292`) at the
Dual, Interval and symbolic lanes. At f64 it builds.

No scene writes such a family today; C's scenes are all outside one.

**Done when** a member's first pass reads the traces off the member's
own nominal evaluation. One way is for the member pass to evaluate the
member at the nominal bindings once, as `evaluate_at_descent` does for
the whole document. A test then draws a traced boss inside an indexed
family and evaluates it at the Interval lane.
