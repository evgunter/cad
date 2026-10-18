---
id: editor-core-keeps-computed-values-in-t
kind: issue
title: The mate lever arm is computed from the resolved origins' reach and re-enters every levered margin as a constant
status: closed
opened: 2026-10-10
priority: P2
cost: M
refs: [the-from-f64-gate-reads-every-generic-crate]
closed: 2026-10-18
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

**Partly paid by PR 5267 (s3 C).** Its corpus places every part through the place-part gesture, so `revalue_corpus` reaches levered mates, and it flagged `mate_axes_parallel` and `mate_axis_normal_perpendicular` as laundered on every placed document. Those two decisions (`coset.rs`'s `parallel` and `perpendicular`) and `Lever::margin` now take the arm through `Real::from_computed`: opaque, so no identity is proved over it and the row counts it rather than comparing it. What remains owed is the two step arms that divide by a levered sine (`candidate_translation`, the `coset.rs` row of `scripts/gates/from-f64-sites.tsv`): one `T` arm threaded through `parallel`'s sine and both steps, so they cancel.

## Closed (2026-10-18, INTENT stage 4 H)

The two step arms no longer divide by a levered sine. `coset.rs`'s `parallel` decides `|u × v|·arm`, with the arm through `from_computed` as before. It hands out the line `normalize(u × v)` and the sine `|u × v|` (`UnitVec3::new_scaled`), and both steps divide by that sine. So no arm enters a fold's map at any scalar, and nothing is left to cancel. The `coset.rs` row of `scripts/gates/from-f64-sites.tsv` is gone. H needed this because the replay at `Sym` read the arm's opaque atom in every placed carrier.
