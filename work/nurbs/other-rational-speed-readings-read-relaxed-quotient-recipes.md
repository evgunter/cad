---
id: other-rational-speed-readings-read-relaxed-quotient-recipes
kind: issue
title: Four more rational derivative readings use relaxed quotient-rule recipes, not the one Bernstein quotient bound
status: closed
opened: 2026-10-13
branch: nurbs/remaining-rational-readings
closed: 2026-10-13
---


## What

ENCL handed these over when it closed PR 4891 (`work/nurbs/log.md`, 2026-10-13 18:50Z). That PR's branch had filed them; the filings never reached main. Each is a rational derivative reading that ENCL says uses a relaxed quotient-rule recipe rather than the one Bernstein quotient bound (`geom_core::spline::derivative`, ruling 2026-10-13 03:00Z):

- `crates/topo/src/shell.rs` `spline_speed`: reads a relaxed quotient-rule speed. Its row is `the_spline_speed_bound_covers_a_strongly_rational_arc`.
- step-import `recognize` `derivative_sup`: the same shape, per ENCL. It is not found under `crates/step-import/src` on main at 27c7b42e, so re-locate it. It may have moved, or it may be on `encl/one-chart-speed` only.
- `crates/mesh/src/chords.rs`: the `s1`/`s2` relaxation in `rational_carrier_m_bound`. PR 4984 replaces it with `accel_upper_bound`, so check after 4984 lands.
- `crates/geom-brep/src/props/quad.rs`: the rational cross product, a candidate consumer of the one form (`M̃/W³`).

## Shape of the fix

After PR 4984 (unit 2) merges, sweep for these four. Route each live one through `speed_pieces` / `stretch_pieces` / the area quotient, or record why it stays, for example a conic-only path or an exact form.

ENCL's branch `encl/one-chart-speed` @ adb4df370e also carries pieces worth taking:
- `the_meter_sups_bound_every_sampled_magnitude`'s `SPEED_CEILING` tightened to `1 + 1e-12`;
- the unit-arm `screened_max`/`vectors_sup` f64 screen, about 5% cheaper on the integral arm, with its fuzz;
- `a_part_carried_below_the_normal_range_screens_soundly`.

— (NURBS orchestrator)

## Closed (2026-10-13)

Swept on main at `2639b944` (PR 4984 merged).

- **`crates/topo/src/shell.rs` `spline_speed`: already routed.** `cba52f7d`
  replaced it with `NurbsCurve3::speed_upper_bound`, which reads
  `speed_pieces` (the quotient statement) since unit 1. The row
  `the_spline_speed_bound_covers_a_strongly_rational_arc` reads it. A
  sweep of `shell.rs` for `weights()`, `w_min`, `weight_rate`,
  `reach *` and `/ least` finds no other weight-derivative recipe.
- **step-import `recognize` `derivative_sup`: already routed.**
  `fe8ea48f` replaced it with `NurbsSurface::stretch_sup`
  (`crates/step-import/src/recognize.rs:380`), which reads
  `stretch_pieces`' `sup_norm`.
- **`mesh/src/chords.rs` `s1`/`s2`: closed by PR 4984**:
  `rational_carrier_m_bound` reads `accel_upper_bound`.
- **`props/quad.rs`'s rational cross: stays, on QUAD's ground.** It is
  live (`rational_patch_face`'s area cell: `g_hull` is the interval
  quotient of `cross_num`'s cell hull over `w³`, and the pads
  `pad_d` are `|∂_d cross_num|/w³ + 3·|cross_num|·|w_d|/w⁴`), and
  QUAD already holds the three rows on it:
  `quad-rational-patch-cross-grows-with-translation` (P2),
  `quad-area-pads-read-the-cross-norm-off-per-coordinate-boxes` and
  `quad-rational-integrand-second-derivative-reads-the-quotient-rule-by-hand`.
  Two things keep it from being a routing unit here:
  - quad's cells are uniform cuts of the trim rectangle, not dyadic
    pieces of a knot span, so `cell_quotients` would read each cell
    only after a blossom restriction of its window (`CellAxis::Bezier`
    over the restricted block);
  - the pads are first derivatives of the area norm, the quotient
    `(M̃_d·W − 3·M̃·W_d)/W⁴`, which the statement does not form.

  So `g_hull` alone could route, and that would not close the
  translation row, whose pads and `f_dd` hulls grow too. Evidence
  added to the translation row.
