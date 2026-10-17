---
id: the-rational-chart-speed-is-read-twice-in-patch-bound-and-enclose
kind: issue
title: the rational chart speed is read twice, by patch_bound's rational cells and ssi::enclose's speed_sup, and enclose's reading costs 1.57× at the meters' first rung and 1.25× at the second
status: closed
opened: 2026-10-11
priority: P3
cost: M
design: true
refs: [the-meters-sup-side-still-reads-boxes-where-the-cell-carries-no-coefficient-form]
branch: encl/one-chart-speed
closed: 2026-10-13
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

The box reading's lever was 1.150676. `enclose`'s reading is above
the box by at most `2.3e-14` relative (measured over
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

## Answered (design pair, 2026-10-12)

Both designers reached the same final state, so this did not go to Ev: the two readings are one reduction of the same data, not two methods.
- `patch_bound`'s speed is the triangle relaxation `max‖k‖ + max|g|·max‖q‖` of `enclose`'s `max_{i,j} ‖k_i − g_i·q_j‖`.
- Here `k_i` are `Ã_d`'s coefficients about the centre, `g_i` are `w_d`'s, and `q_j = P_j − c`.
- The joint reading is centre-free, rigid-invariant and at or below the box. It gives a lever of 1.150562 against 1.160044 today.

The row's 1.57× measured rebuilding the terms from quotient points. Fed the vectors `rational_cells` already holds, the joint reduction with an `f64` screen costs +0–15% (two independent measurements).

The unit, in final state:
- **One per-cell rational quotient-numerator form**, homed in `patch_bound`. Its inputs are the terms `k_i`, `g_i`, the offsets `q_j` and the weight hull.
- **The speed** is the screened joint maximum, taken through one function that both `patch_bound::rational_cells` and `ssi::enclose` (`CellNet::derivative_norm_sup`, `transverse_readings`) call.
- **The integral arm** is the same form with `g ≡ 0`.
- **The signed first-order box** is the per-term hull, which is the tighter quotient rule. It feeds the regularity floor.
- **The order** stays divide-by-the-weight-floor before the norm.
- **The cross bound `m_sup`** keeps its relaxation, which is a separate question.

Follow-ups are for the lane to file:
- the two rational-cell substrates: `CellGrid::refined` vs `refine_equal_split`, and `pair_terms`/`pair_scales` re-spelling `TensorCoeffs::diff_u`;
- `topo/src/shell.rs` `spline_speed`'s relaxed quotient rule;
- `props/quad.rs`'s rational cross as a candidate consumer.

## Closed: superseded by NURBS's quotient ruling (2026-10-13)

NURBS ruled at 03:00Z on 10-13 (`work/nurbs/log.md`) that every rational derivative bound reads one Bernstein quotient bound on exact dyadic pieces:
- PR 4954 moved `ssi::enclose` onto it;
- PR 4984 (unit 2) moves `patch_bound` `rational_cells`.

That gives one home for the chart speed. It replaces this row's joint pair-term reduction, which PR 4891 had built. PR 4891 is closed unmerged. The findings it had filed are handed to NURBS on `work/nurbs/log.md`.
