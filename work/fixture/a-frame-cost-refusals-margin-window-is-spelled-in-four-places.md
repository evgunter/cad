---
id: a-frame-cost-refusals-margin-window-is-spelled-in-four-places
kind: issue
title: a frame-cost refusal's predicate-and-margin-window test is spelled in four places across three crates
status: open
opened: 2026-11-22
priority: P4
cost: E
---


PR 7131's cost rows each check that a refusal named a predicate with its margin's upper end in `(lo, hi]`. The check has four spellings:

- `crates/sweep/tests/common/frame_cost.rs`, `escalated`. Its module doc says it lives there "so each spells it once", but it is sweep-test-only.
- `crates/geom-brep/src/pcurve_cache/frame_cost.rs`, `envelope_escalates` (`pub(super)`).
- `crates/topo/src/boxes.rs`, `sector_at_its_band`: an inline `bound > 1.0e-12 && bound <= 1.1e-12` on a `KernelLimit`.
- `crates/topo/tests/a_seamless_pipe_cut_by_an_oblique_plane_measures.rs`, `pipe_at_interval`'s `within` closure (GREEN PR 7402).

`geom_core::test_support` already holds `at_the_frame_cost_band` and `upper`, which every one of these suites can reach. Its natural home is beside them: one `upper_in(margin, (lo, hi))` or `escalated(cause, predicate, window)`, with the four callers moved onto it.

Found by the review of PR 7402 (style Q1). (GREEN, U8)
