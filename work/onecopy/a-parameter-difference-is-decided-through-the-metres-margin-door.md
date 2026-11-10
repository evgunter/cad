---
id: a-parameter-difference-is-decided-through-the-metres-margin-door
kind: issue
title: A curve parameter difference is decided through Margin::of, whose doc admits only metres
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by PR 6709's FULL review (S2). Not fixed there.

`geom_core::Margin::of` (`geom-core/src/predicate.rs:907`) is the door for a quantity that "IS metres": a length, a signed projection of a metre vector onto a unit direction, a difference of lengths, and so on. A curve's parameter is no length. On a spline it is a knot coordinate, on a circle an angle. Yet two sites decide a parameter difference through it:
- `geom-core/src/k_stats.rs` `param_order`, `prove(row, Margin::of(s − t))`, read by `geom-brep`'s `curve_pair::compare`: the curve pair's walks order two parameters, and test two for the same foot;
- `topo/src/boolean/edge_join.rs:494` `crease_read`, through `param_order`: whether a join edge's end parameter lies on its spline's crease.

Both read at the least band (`prove`), so no band's metres meaning enters the sign. But the recorded margin goes to the K stream labelled as metres, and the dimension audit's convention (D4, clause (i)) is that every recorded margin is a length.

**Options.**
1. A parameter door on `Margin` (`Margin::parameter`, recorded as dimensionless or tagged), read by `param_order`.
2. Meter the difference by the carrier's speed: a lever, through `Margin::metered`, as `pcurve_cache`'s placements do. This turns it into metres along the curve, though the exact comparison does not need it.
3. Rule that an exact parameter comparison at the least band is no margin, and record it under its own kind.

The sweep for further instances is part of the fix: `Margin::of(` over a `t`, `s`, `u` or `param` difference.

## The sweep's classes, and (c) landed

The ruling (orchestrator, 2026-11-10) applies D4 (iii) as written. The
sweep sorted the band-decided parameter differences into four classes:
- **(a)** `param_order` and its two readers (`curve_pair::compare` and
  `edge_join`'s crease reader): still open;
- **(b)** the choose sites (the period's `run_from`, and `in_window` ×2):
  still open;
- **(c)** decided against the band, with a rate in hand or a ledger row.
  Landed on branch `onecopy/param-difference`:
  - `nurbs_iso::iso_boundary_row` (`iso_row_at_domain_end`) is metered at
    the chart's sup stretch along the axis;
  - `slit::parting_order` (`PARTING_ORDER`) is metered at the sup of the
    carrier's `speed_range`;
  - `replace_face`'s `iso_row_runs_against` has no certifiable inf rate,
    so it rides `decide_flagged` under ledger row F41;
- **(d)** already metres, so nothing to do: `offset_derive`.

A probe moved no decision at default ε or 1e-12: 29
`iso_row_at_domain_end` readings and 112 `parting_order` readings at
each.
