---
id: rational-inf-speed-has-two-recipes-curve-and-surface
kind: issue
title: The rational inf speed has two recipes: the curve's piece assembly and the surface's pair-term floor
status: closed
opened: 2026-10-13
priority: P3
cost: M
branch: nurbs/quotient-bound
closed: 2026-10-13
---

## What

A rational spline's derivative is floored from below by two different
derivations, one per dimension of the parameter domain:

- **Curves.** `geom/src/curves/nurbs.rs`
  `NurbsCurve3::rational_speed_lower_bound` reads
  `rational_piece_assembly`: per span, the rational piece's own
  numerator coefficients, with a degree cap
  (`RATIONAL_PIECE_MAX_DEGREE`, 24) above which it refuses.
- **Surfaces.** `geom-core/src/spline/derivative.rs`
  `rational_stretch_floor` (PR `nurbs/rational-inf-floor`): per
  nonempty cell, the least projected pair term at the cell's active
  points over the cell's weight, the sup's own `RationalPairTerm`
  read from below.

The sup side has one statement for both (`rational_speed_sup`,
`rational_stretch_sup`). The inf side does not. The pair-term floor
has an immediate curve twin (one column, `D = 3`) with no degree cap;
whether it should replace the piece assembly, sit beside it as a
second assembly (`speed_lower_bound` already joins two), or stay
apart is a reach question: neither is known to dominate. The two
should be compared on the curve meter's frontier rows
(`geom/tests/curves/m5_pr7_speed_meter.rs`,
`geom/tests/curves/swaying_corner_meter.rs`) before anything moves.

## Ruling (2026-10-13, orchestrator, designers convergent)

Both rows close into ONE statement: the Bernstein quotient (coefficient-ratio) bound on dyadic pieces, in `geom_core::spline::derivative`. Each derivative quantity is written as a numerator over a power of `W`, both raised to one degree. Then `inf (·)·d ≥ min_k (a_k·d)/b_k` and `sup ‖·‖ ≤ max_k ‖a_k‖/b_k`, read on exact dyadic Bézier pieces, never on refined knot vectors. The full ruling is in `work/nurbs/log.md`, 2026-10-13 03:00Z.

## Closed (2026-10-13)

One statement now serves curves and surfaces, inf and sup:
`geom_core::spline::QuotientPiece`'s `inf_norm` and `sup_norm`, built
by `speed_pieces` (curves), and by `cell_quotients` and
`stretch_pieces` (surfaces).

- The curve meter's integral and rational piece readings are one call,
  `NurbsCurve*::piece_floor`. At equal weights the derivative is read
  over one, so every `b_k` is exactly `1`.
- `speed_upper_bound` reads `sup_norm` on the same pieces.
- `rational_piece_assembly`, its degree cap
  (`RATIONAL_PIECE_MAX_DEGREE`) and the pair-term floors are gone. The
  ceiling is now the statement's own `QUOTIENT_MAX_DEGREE` (56, from
  exact binomials): a rational curve meters to degree 28.
