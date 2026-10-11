---
id: editor-core-keeps-computed-values-in-t
kind: issue
title: The mate lever arm is computed from the resolved origins' reach and re-enters every levered margin as a constant
status: open
opened: 2026-10-10
priority: P2
cost: M
refs: [the-from-f64-gate-reads-every-generic-crate]
---


`editor-core/src/mate/coset.rs`'s `Arm::of(parts, datum)` sums two inputs:
- the mated parts' reach `R_a + R_b`;
- `Alignment::lever_terms(a.origin_reach(), b.origin_reach())`, the
  `SolveScalar::upper` bounds read out of the resolved origins
  (`mate/solve.rs`'s `lever`).

Both are functions of the document's variables, read out to `f64`. The
arm then re-enters the scalar through `from_f64` at six sites, each
listed `launders(editor-core-keeps-computed-values-in-t)` in
`scripts/gates/from-f64-sites.tsv`:
- `mate.rs`'s `Lever::margin` (two sites);
- `coset.rs`'s `parallel` and `perpendicular`;
- the two step arms that divide by a levered sine.

A symbolic identity `Zero` of `x · arm` is one of `x`, since the arm is
positive. But:
- the band reading of `Margin::levered` reads the arm's value;
- the step arms' `arm / sine` cancels the levered sine's arm only to
  rounding;
- the re-valuation row would flag every levered decision it reached,
  because `p0`'s `Lit(arm)` does not move to `p1`'s.

Owed: keep the arm in `T`, or enter it through `Real::from_computed`.
Keep the two `arm / sine` arms cancelling: one `T` value threaded
through both, not one opaque minted per call. Pin it per site; the
corpus reaches no levered mate.
