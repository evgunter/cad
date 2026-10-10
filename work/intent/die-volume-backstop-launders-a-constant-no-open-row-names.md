---
id: die-volume-backstop-launders-a-constant-no-open-row-names
kind: issue
title: die's volume_backstop re-values away from p1 with every topo site fixed, and no open launders row names the constant
status: closed
opened: 2026-10-10
priority: P0
cost: M
refs: [a-computed-value-re-enters-as-a-constant, the-from-f64-gate-reads-every-generic-crate, topo-keeps-computed-values-in-t]
closed: 2026-10-10
---


The census candidate `topo-keeps-computed-values-in-t` carried
(`revalue_corpus.rs`'s `revalue_census`, `REVALUE_TIER=off
REVALUE_DOC=die`, release) does not move when every topo site is fixed.
Before and after that unit, the census reads the same:
`die` in step, compared 10548, laundered `{volume_backstop}` at decision
#10548 (p0 `0`, re-valued `-3.116226563567004e-17`, p1
`-2.3316116359393167e-16`), parted at `expr_non_finite`. With the topo unit's sites fixed, #10548 re-values to the same bits;
the two decisions that went opaque on the first cut were the zero
quadrature pad of `MassProperties::enclosure`, which the unit then kept
exact.

What the unit measured about it:
- `die` is planar (a cube less 21 rectangular pips, `tests/corpus/die.rs`),
  so no face takes the quadrature lane: `volume_pad` is `0`, every
  `MassProperties::reading` is `Exact`, and `props.rs`'s `ac`, `fc` and
  `volume_pad` never reach this margin.
- The round count is not the difference. A stderr probe in
  `bound_holds` over the census run: six rounds open, at the same lower
  ends in both builds, and every one `moved=false` (nothing refines).
- The gap is rounding-sized (2e-16) under the in-step move, which is a
  dilation of a document of lengths. A laundered LENGTH would be off by
  about `1e-7` of the length; one of this size is a quantity a dilation
  leaves fixed (a unit normal, a frame, a rotation's entries) re-entered
  as a constant and rounding one ulp apart from what the `p1` build
  computes. None of the open `launders` rows names such a planar input,
  which rules those rows out and leaves three homes, likeliest first: a
  crate the gate does not scan (`editor-core`'s evaluator and
  transforms, `profile`'s lowering); a row of the four scanned crates
  classed `constant` that is not one; or one of the gate's stated blind
  spots (a helper taking an `f64` it lifts, a point-free lift bound to a
  local, a macro body).
- The round-count reading came from a probe that is not in the tree (a
  stderr line per open round in `bound_holds`); it is reproducible by
  adding one and re-running the census.

Owed: name the constant (list the `Lit` leaves of decision #10548's DAG,
or bisect the document's nodes), then fix it where it lives, or
classify it under `the-from-f64-gate-reads-every-generic-crate` if it is
one of that row's sites.

## Closed

There was no laundering. The census mis-paired two different margins.

Decision #10548's margin DAG, diffed node by node between the builds:
- **At `p0`** it is `Mul(num, Inv(area))`, where `num` is the full sum in `T` and is exactly `0`.
- **At `p1`** it is `Mul(Lit −1.128e-14, Inv(area))`.

The cause is in topo `boolean/ops.rs`'s `bound_holds`. #10547, `volume_backstop_violation` at the exact band, comes out differently:
- **At `p0`** it answers `Zero`: the planar subtract's volume identity cancels exactly.
- **At `p1`** it answers `Negative`: the margin is −3.1e-17 of rounding, and `revalue` reproduces that value bit for bit.

So the two builds take different arms:
- `p1` takes arm 1 and decides `volume_backstop` over `lo / lever`. That `lo` is `T::from_f64(margin.lo())`, the real computed-value site; it is `topo-keeps-computed-values-in-t`'s and becomes `from_computed` on #4585.
- `p0` decides arm 2's `hi / lever` under the same name.

The census paired the two by position and name.

The fix is in the instrument (PR #4596, `the-from-f64-gate-reads-every-generic-crate`):
- `revalue::Decision` records each decision's sign.
- `revalue_corpus.rs`'s `tally` keeps a mismatch after the first sign flip in `after_flip`, never in `laundered`, and the gating rows assert `after_flip` is empty.
- Pin: `a_decision_answered_with_another_sign_keeps_what_follows_apart`.

The census now reads `die` in step: compared 10548, laundered `{}`, `after_flip {volume_backstop}` at flip #10547.
