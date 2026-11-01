---
id: curves-review-m5-pr4-f1-costs-50x-by-seed
kind: issue
title: tcost: the fast-set row curves_review_m5_pr4_adversarial::f1_fit_bound_dominates_dense_sampling_randomized costs 0.07–3.9 s by CAD_FUZZ_SEED, so a seed fails the 1 s fast-set bar
status: closed
opened: 2026-10-22
priority: P2
cost: E
closed: 2026-10-26
---


## Found (PR 5582, 2026-10-22)

`crates/geom/tests/curves_review_m5_pr4_adversarial.rs`
`f1_fit_bound_dominates_dense_sampling_randomized` is a fast-set row
(not in `.config/nextest.toml`'s `ci` slow set). It read:

- 0.184 s on PR 5582's run 39718415733;
- 1.776 s on run 39722437704, where `scripts/check-fast-set-times.py`
  failed the `test` job at its 1 s bar for an existing test. The PR
  does not touch `crates/geom`.

PR 5582's two reviewers measured its cost at 0.07–3.9 s across
`CAD_FUZZ_SEED` values, deterministic per seed. The per-PR gate
therefore fails on whichever seed a run draws.

Either cap the cost of one seed (bound the sampled fit's size), or move
the row to the slow set. Owned by `tcost` and `tint` by territory;
filed on `tcost`, the program that owns the gate's time bars.

## Closed (2026-10-26, tracker sweep)

**Fixed on main** with the row's second option. Commit `b1cf6380`
(2026-10-24) moved
`curves_review_m5_pr4_adversarial::f1_fit_bound_dominates_dense_sampling_randomized`
into the `ci` profile's slow set (`.config/nextest.toml:78`). The fast-set
1 s bar no longer reads it. Capping one seed's cost was not done, and
nothing here asks for it now.
