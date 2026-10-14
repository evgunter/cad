---
id: topo-calls-k-stats-past-its-own-classification-funnel
kind: issue
title: topo calls geom_core::k_stats doors directly in a dozen files past validate.rs, the crate's one classification funnel
status: open
opened: 2026-10-01
priority: P3
cost: M
---


## What

`crates/topo/src/validate.rs` documents `decide` as topo's single
classification funnel. #3686 (LINALG) added `decide_positive` and
`decide_negative` wrappers to it, and the sector and solid-contain
gates now go through those wrappers. Elsewhere in topo, files still
call `geom_core::k_stats` directly:

- `k_stats::decide` is imported directly in nine files.
- `transform.rs`'s `decide` and `split.rs`'s `decide_reported` bypass
  wrappers that already exist.
- `decide_flagged` and `decide_invariant` are called directly in
  `transform.rs`, `boolean/ops.rs` and `boolean/solid_contain.rs`.
  `validate.rs` has no wrapper for either.

The full list is in #3686's PR body, under "Crate-funnel bypasses in
topo".

## The question

Two answers are possible:

- Make the funnel the only door: wrap the missing gates and route
  every caller through `validate.rs`.
- Admit the funnel is not unique, and correct `validate.rs`'s doc to
  say so.

Either way, the doc and the code should say the same thing.

Filed by the LINALG orchestrator from #3686's delta review.
