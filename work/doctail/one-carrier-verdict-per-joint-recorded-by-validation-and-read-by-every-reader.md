---
id: one-carrier-verdict-per-joint-recorded-by-validation-and-read-by-every-reader
kind: unit
title: Profile validation records one carrier verdict per joint, by a margin that reads both legs; the sweep, the names, the unproven-coincidence lint and the edit door read that record, and swept::cosurface and cosurface_pairs retire
status: open
opened: 2026-11-20
priority: P2
cost: H
refs: [a-reshaping-that-joins-a-kept-piece-into-a-run-reports-nothing]
---


Decision 1 of `[ev]` 7082, which Ev approved on 2026-11-20 ("great! that'll be nice to unify"). The sweep's `Walls: one per run` (`crates/sweep/README.md`) now reads the run from validation's joint verdict.

## What

Two deciders answer "are these two adjacent profile pieces on one carrier?":

- the sweep's `cosurface` / `cosurface_pairs` / `wall_runs` (`crates/sweep/src/swept.rs`), in swept order;
- validation's `judge_joints` (`crates/profile/src/validate.rs`), in authored order. It keeps the class only for unconstructed joints (`DecidedJoint`).

Both read the far end of one leg against the other leg's carrier, so the margin scales with the ratio of the two legs' lengths. A reversed traversal (a negative extrude depth, or a revolve with θ > 0) swaps which leg is read, and can group a near-collinear joint differently. The designers' example: legs of 1 and 1e-3 at 1e-7 rad read 1e-10 forward (one wall) and 1e-7 reversed (two walls) at ε 1e-9, K 10. Validation also calls a joint "Tangent" between two lines whenever the far-end reading misses but the turn reads Zero, so the `unproven-coincidence` lint can disagree with the body. Programs cannot reach the asymmetry, because the path lattice refuses an undeclared zero-turn joint. Raw loop tables, which reach only the kernel API, can.

## Done when

- Validation records a carrier verdict for every joint, constructed or not, using a margin that reads both legs, so neither traversal reads it differently.
- The sweep builds its runs from that record, and `swept::cosurface` / `cosurface_pairs` are deleted.
- The `unproven-coincidence` lint reads the same record.
- A test shows a raw loop table extruded at +d and −d groups its walls alike at a near-collinear joint where the far-end reading would part them.

## Also from the designers, to check while in here

- A run of three or more raw pieces is chained from pairwise verdicts.
- N1's example has been stale since PR 6682.
- A `[topology]` sentence is garbled.
