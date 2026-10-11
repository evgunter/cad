---
id: main-slow-set-reds-on-two-rows-since-4553
kind: issue
title: Main's slow set is red on two rows since PR 4553 (knot algebra off the ring): the sweep serial-golden digest and the arc-loft import volume bits
status: open
opened: 2026-10-11
priority: P0
---


Two rows of the slow set (the nightly's; CI's per-PR `ci` profile
filters them out) fail on main since PR 4553's merge (98e620a195,
`nurbs/knot-algebra-off-the-ring`). INTENT's stage 3 A lane reported
that both pass on its parent. The INTENT orchestrator reproduced both
failures on main at 80fa908950 with
`cargo nextest run -p sweep -p step-import -E 'test(the_walk_matches_the_serial_golden_at_four_threads) | test(arc_loft_natively_computes_its_rational_volume)'`:
2 run, 2 failed.

- `sweep::all mass_props_are_thread_count_invariant::the_walk_matches_the_serial_golden_at_four_threads`:
  the serial-golden digest moved
  (`crates/sweep/tests/mass_props_are_thread_count_invariant.rs:471`,
  `check_against_golden`).
- `step-import::all nurbs_import::arc_loft_natively_computes_its_rational_volume`:
  the imported solid's certified volume is no longer bit-identical to
  the native one (`crates/step-import/tests/nurbs_import.rs:373`).
  The row pins bit identity on purpose (R1 MINOR-2), so the reader's
  reconstruction and the native build now run different quadrature
  bits. That is a finding, not a re-pin, until the divergence is
  explained.

Owed: root-cause both against 4553's change. If a digest moved for a
stated reason, re-baseline it and say what moved. If the bits now
differ because the two sides compute the knots differently, make them
compute them the same way.

The serial-golden row's `Sym` lines also move under PR 4662
(nurbs-weights-and-knots-are-the-scalar u2): the `validate_geometric`
rows' `frozen` column is +2 (`sym_arc_loft`) and +1 (`sym_thin_strip`)
against main, because the carriers' knots and weights are held as nodes,
with decisions, discharges, shapes and verdicts unchanged and the `f64`
lines byte-identical to main's red output. Re-cut that golden on a tree
that carries PR 4662.

PR 4708 (`nurbs-weights-and-knots-are-the-scalar` u4) moves the
serial-golden row's `Sym` lines again, against its merge base
`fe323f24d0` (whose own lines had already moved under main's
`from_computed` loft lift, 9618d84fd8). `sym_arc_loft validate_geometric`
goes from `sz=28 num=624 frozen=3706` to `sz=40 num=612 frozen=3732`, and
`sym_thin_strip validate_geometric` from `sz=24 num=636 frozen=3430` to
`sz=36 num=624 frozen=2792`. Twelve more decisions per row discharge as
theorems, because topo now reads the carriers' held domain ends.
Decisions, shapes and verdicts are unchanged, and the `f64` lines are
byte-identical. Re-cut the golden on a tree that carries PR 4708.

PR 4726 (`the-skin-computes-its-values-in-t`) moves the serial-golden
row again, in two ways.
- **One `f64` line moves.** `arc_loft_1e9eps` goes from
  `v=4022487ed6a9da05 a=403995a3b53c3c54 vpad=3eb0783c2ee00000
  apad=3fb5fb0bfc379cc0` to `v=4022487ed6a9d899 a=403995a3b53c3c57
  vpad=3eb0783424400000 apad=3fb5fb0bfc12a660`, with its verdicts
  unchanged. The skin's arc conversion used to call `f64`'s inherent
  `cos`/`sin_cos`, which go to the system libm. Generic over the
  scalar, it now calls `Real`'s, which at `f64` is the pure-Rust
  `libm` that D9 prescribes. The inherent call was a latent D9
  platform dependence, and the ulp it moves is that dependence.
- **The `Sym` lines' `frozen` column falls.** `sym_arc_loft
  validate_geometric` goes from 1244 to 1060 and `sym_thin_strip` from
  1268 to 1177, because the walls are computed in `T` instead of
  lifted as opaque unknowns. Decisions, `sz`, shapes and verdicts are
  unchanged.

The import row (`arc_loft_natively_computes_its_rational_volume`)
builds the same arc loft, so it may move for the same libm reason. Re-cut
on a tree that carries PR 4726.
