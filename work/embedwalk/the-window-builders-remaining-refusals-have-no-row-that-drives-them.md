---
id: the-window-builders-remaining-refusals-have-no-row-that-drives-them
kind: issue
title: No row drives window_of's remaining Unsupported refusals end to end: an inexact knot domain, a carrier or moved carrier that does not certify, an unrefining chart, a non-finite window
status: open
opened: 2026-11-25
priority: P3
cost: M
---



## Finding

Found by lane `embedwalk-trimmed` (PR 7787). Once every NURBS face has
a window, `topo/tests/clearance_windows_on_a_spline_chart.rs` pins no
refusal, and no test in the workspace reaches any of `window_of`'s
remaining `EngineRefusal::Unsupported` arms (`crates/topo/src/clearance.rs`)
through a body:

- `knot_domain`: "a free-form face whose knot domain is not exact
  structure";
- "a face whose carrier does not certify" and its moved twin "…, read
  through its motion, does not certify";
- `refines`: "a face whose interval chart does not refine …";
- "a face whose carrier window is not a finite rectangle".

The only `Unsupported` row is `m10_5_r2_probes_interval.rs`'s, which
builds the value by hand to pin its rendering. Searched: each carrier
string over `crates/*/tests` and `crates/*/src`; the hits outside
`clearance.rs` are other doors' strings with the same words.

## What is needed

A row per arm that builds the body which reaches it (an interval knot
of width, a carrier lifted from a refused scalar, a chart whose halves
do not narrow) and asserts the refusal names its carrier. An arm no
valid body reaches is worth saying so at the arm.
