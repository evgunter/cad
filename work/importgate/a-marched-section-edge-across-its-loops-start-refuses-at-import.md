---
id: a-marched-section-edge-across-its-loops-start-refuses-at-import
kind: issue
title: step-import: TAIL_TURBINE's cylinder × sphere section edges refuse once its two cylinder records are one (a built spline carrier whose end reads at or before its start)
status: open
opened: 2026-11-19
---


Found by `importgate/snap`, whose round one-record
(`crates/step-import/src/one_record.rs`, `one_round_record`) joins
`TAIL_TURBINE`'s two records of one cylinder (`#152`, `#563`), so its
import gets past the two-records refusal.

## What

`tests/fixtures/wild/stepcode/TAIL_TURBINE.stp` then refuses on the two
edges its cylinder `#152` and its sphere `#506` (off the cylinder's axis)
share, `#170` and `#217`, whose printed carriers are quintic splines:

- at ε 1e-9 and 1e-12, `#217`: "a built spline carrier whose end reads
  at or before its start" (`crates/step-import/src/build.rs`, `params`).
  The marched section's loop is built whole (`served_pair`,
  `whole_loop`), and the edge's built ends are read on it by
  `spline_end_param`; an edge that runs across the loop's start reads
  its end before its start. This is a guess from the shape of the
  refusal, not measured.
- at ε 1e-6, `#170`: "no intensional description certifies …
  geometry attachment gate".

`tests/tier_gate.rs`'s `TAIL_TURBINE` row pins the 1e-9 refusal.

## What would close it

An edge on a periodic marched loop read as the arc between its built
ends whichever way the loop's start falls, and `TAIL_TURBINE` importing
(or refusing on its next frontier).
