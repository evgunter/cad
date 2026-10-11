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
row again. Measured against main's own output for that row
(`work/nurbs/the-arc-loft-thread-count-digest-is-red-on-main.md`'s
table at ε = 1e-9), not the committed digest:
- **`arc_loft_1e9eps`**: `v` and `a` are main's (`…d899`, `…3c57`), and
  only the pads move, `vpad` `3eb0783423f55555` → `3eb0783424400000`
  and `apad` `3fb5fb0bfc12a5e0` → `3fb5fb0bfc12a660`. Its verdicts are
  unchanged. The likely cause is that the skin's arc conversion now
  calls `Real`'s sines and cosines, the pure-Rust `libm` D9 prescribes,
  where it called `f64`'s inherent (platform libm) ones. That moves an
  arc's weight or tangent pole by an ulp on a few per cent of sweeps.
- **The `Sym` lines' `frozen` column** reads 1060 (`sym_arc_loft
  validate_geometric`) and 1177 (`sym_thin_strip`) at the PR's head,
  against the committed 1244 and 1268. Main's own counts were not
  re-measured here. The walls are computed in `T` instead of lifted as
  opaque unknowns, and the fixture placements are constants rather
  than `from_computed` unknowns. Decisions, `sz`, shapes and verdicts
  read as the committed ones.

The import row (`arc_loft_natively_computes_its_rational_volume`)
builds an arc loft too, so it may move for the same libm reason.
Re-cut on a tree that carries PR 4726.
