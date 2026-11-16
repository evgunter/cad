---
id: a-parameter-middle-can-overflow-and-a-lifted-halving-escapes-the-gate
kind: issue
title: A parameter bracket's middle can overflow on admitted knots, and a halving through a local lift escapes bracket-mid-by-hand
status: closed
opened: 2026-11-09
priority: P3
cost: M
closed: 2026-11-09
pr: 6690
---


Found in PR 6681's review (`onecopy/bracket-mid-2`).

## Finding

- **Lifted halvings escape the gate.** `scripts/gates/bracket-mid-by-hand.sh` reads a halving only as `0.5` or `T::from_f64(0.5)`, so a halving through a local lift goes unseen:
  - `geom-brep::props::quad::cells::cell_midpoint`'s `(pt(lo) + pt(hi)) * pt(0.5)`;
  - `props::quad::trimmed`'s `(pi + qi) * pt(0.5)`.
- **The gate's sum-of-halves arm fires on unpaired halves** (`c + 0.5 * a + 0.5 * b`, a vector's halves). Its method arm can't type an integer receiver, and it doesn't read `0.5_f64`.
- **PR 6681's session-box reason covers coordinates only.** About 15 of its sites are parameter middles: knots, quad cuts, `ParamRange::mid`, the `ssi::four` row bisection. Nothing bounds a parameter's magnitude: `KnotVector::on_domain` refuses an overflowing width but admits `(1e308, 1.5e308)`, whose ends sum to `∞`. And `∞` isn't loud everywhere:
  - `cell_midpoint` falls back to `lo`;
  - `trimmed` takes the coarser bound;
  - the row bisection halves into `(lo, ∞)` and `(∞, hi)`.

## Asked (orchestrator's ruling, option ii)

One admission rule at the parameter door: a knot vector refuses, typed, an end past `f64::MAX / 2`. Then cite that door as every parameter site's reason. Also route the lifted halvings to `mid_param`, and teach the gate the lift shape, paired halves and `0.5_f64`.
