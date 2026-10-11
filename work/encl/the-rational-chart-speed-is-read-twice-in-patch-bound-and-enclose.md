---
id: the-rational-chart-speed-is-read-twice-in-patch-bound-and-enclose
kind: issue
title: the rational chart speed is read twice, by patch_bound's rational cells and ssi::enclose's speed_sup, and enclose's reading costs 57% more on the meters' cells
status: open
opened: 2026-10-11
priority: P3
cost: M
design: true
refs: [the-meters-sup-side-still-reads-boxes-where-the-cell-carries-no-coefficient-form]
---


## Found (PR 4687's review, ENCL meters-sup lane, 2026-10-11)

Two rigid-invariant upper bounds on a rational patch's `‖S_u‖`, one per
consumer:

- `crates/geom-brep/src/patch_bound.rs` `rational_cells`: the quotient
  rule over vector norms, `(sup‖Ã_u‖ + sup‖S − c‖·sup|w_u|)/inf w`, read
  off the cell's refined `Ã_u` coefficient vectors and `P − c`.
- `crates/geom-brep/src/ssi/enclose.rs` `NurbsBoxes::speed_sup` →
  `CellNet::derivative_norm_sup` → `pair_norm_sup`: the numerator's
  pair terms evaluated at each control point, `max ‖term(P_j)‖` over
  the weight's lower bound.

## Measured

`enclose`'s reading was wired into `rational_cells` through
`CellNet::of_spans` on the cell's refined block and timed against the
`patch_bound` reading on the bowed patch's rational twin (weights
`1 + 0.15·(k mod 5)/4`, the fixture of `offset_fit.rs`'
`the_vector_upper_bounds_do_not_move_under_a_rotation`), with one
release reading of `patch_cells_refined` plus both meters:

| | `patch_bound` reading | `enclose` reading |
|---|---|---|
| rung 16 (4096 cells) | 208 ms | 327 ms |
| rung 64 (65536 cells) | 4.36 s | 5.45 s |
| speed lever at rung 16 | 1.160044 | 1.150562 |

The box reading's lever was 1.150676. `enclose`'s reading is never
above the box at rest (measured `1 + 2.3e-14` over
`patch_bound.rs`' `the_meter_sups_bound_every_sampled_magnitude`
fixtures), where `patch_bound`'s is up to `1.125`× it. Under callgrind,
40% of the rational arm's time with `enclose`'s reading was
`CellNet::derivative_norm_sup`: `PairTerm::about` with `Affine::per`
(17.5%, the per-term interval affine forms), `pair_norm_sup` (14.7%)
and `pair_terms` (6.9%).

## What is open

One reading for both consumers. Either `enclose`'s, once the per-term
affine forms cost about what `patch_bound`'s hull reads do (it is
the tighter one), or a shared core both call. The cost criterion is
the meters' rung: within a few percent of `patch_bound`'s reading.
