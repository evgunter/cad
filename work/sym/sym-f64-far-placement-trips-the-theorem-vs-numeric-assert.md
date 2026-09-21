---
id: sym-f64-far-placement-trips-the-theorem-vs-numeric-assert
kind: issue
title: Sym<f64>/Sym<Probe> at a far placement panic in Decide's theorem-vs-numeric debug_assert: the point channel is not a proof
status: open
opened: 2026-09-14
priority: P0
cost: H
---


**Found by SYM-6's r2 review, off the door and pre-existing** (reproduced at
merge-base `c02e1b1e4` and at head `2621bc0a9`).

`impl<T: Decide> Decide for Sym<T>::sign_within` (`crates/geom-core/src/sym.rs`,
the `debug_assert!` "the numeric channel proved this margin nonzero and the
form says it is identically zero") fires at `Sym<f64>` and `Sym<Probe>` on an
ordinary document placed far from the origin: a stadium extruded / the M10-9
washer revolved on a sketch plane at `(d, d, d)` with `d = 1e9` at ε = 1e-9,
and already at `d = 1e6` at ε = 1e-12 (`crates/sweep/tests/sym6_r2_e2e.rs`,
the `Sym<f64>` and `Sym<Probe>` rows, which catch it and print it). The form is
a genuine theorem (the placement cancels exactly); the `f64` channel's rounding
at that magnitude exceeds the band and it answers a definite sign. The
assertion's comment says "the enclosure proves it" — true at `Interval`, where
the numeric channel is a certified bracket, and not at `f64`/`Probe`, where it
is a point comparison. At bare `f64` the same margin is refused typed by the
residual gates (`ResidualExceeded { Surface2Residual }`); at `Sym<f64>` it is a
panic in every profile with debug assertions on.

No shipped lane replays at `Sym<f64>` (the driver replays at `Sym<Interval>`),
so this is a limit of the unit-test lane rather than a shipped defect; it
stands in the way of any fixture-scale row that wants to drive `Sym<f64>` far
from the origin (e.g. one that would count `Disputed` on the M10-10 documents).

## A second mechanism: rule F turns a one-ulp sign error into a 2.0 (SYM-8, 2026-09-21)

Found by SYM-8's two blinded reviews, independently, and reproduced by
the fix pass. The row above is about MAGNITUDE — a far placement whose
`f64` rounding exceeds the band. This one needs no far placement and no
large residual: it is the `copysign` atom itself.

**R2's adversary.** `E = (x + 1)² − x² − 2x − 1 + 1e-30·(1 + y²)` is the
polynomial `1e-30 + 1e-30·y²` as a FORM — a positive constant plus a
non-negative term, which SYM-8's rule F (`manifest_sign`) calls
manifestly positive, so `copysign(1, E)` folds to the constant `1` and
`copysign(1, E) − 1` is the zero form. At `x ≈ 1e8` the `f64`
evaluation of the first four terms is roundoff of order one and comes
out NEGATIVE, so the value channel's `copysign(1, E)` is `−1` and the
margin is a DEFINITE `−2`. `Sym<f64>::sign_within`'s contradiction
`debug_assert!` fires at **6 of 6 sampled points** (`x` ∈ {1e8, 3e8,
5e8, 7e8, 1e9, 1.3e9}), and the panic leaves that thread's session
installed, so the next call refuses to nest — which is why the row runs
each point on its own thread.

`E > 0` for every real `x`, `y`, so the tier's discharge is CORRECT and
the `f64` lift's answer is not: the lift is not an enclosure and has no
clause 1 to refuse with. What rule F adds is the amplification — it is
the first rule that turns a one-ulp error in a SIGN argument into a
whole `2.0` at the margin, because `copysign`'s output is `±1` however
small the argument's error was. Any later rule that folds a sign
decision inherits this.

**R1's residue, the same shape from the other side.** At `f64`,
`copysign(1, 1/(t − 1)²) − 1` at `t = 1` answers `theorem` where the
function is undefined (`1/0` is `+inf`, `copysign(1, +inf)` is `1`, the
margin is `0`). Nothing false is reported, but the f64 lift has no
clause 1 to refuse the pole with — at `Sym<Interval>` the same box
answers `refused Invalid`, which is what the rule's soundness argument
relies on.

**Rows.** `geom-core`'s `sym_rule_f_rows`:
`the_adversary_a_positive_form_whose_f64_channel_reads_negative`
(`#[ignore]`d — it fires the assertion by design),
`the_adversary_at_the_interval_lift_is_a_plain_theorem` (gating: the
enclosure of the margin over `x ∈ [1e8 ∓ 1]` is `[−2, 0]`, the numeric
channel cannot decide, and the tier answers the identity), and
`a_manifestly_positive_form_undefined_inside_the_box` (the pole, both
arms, both boxes). SYM-8 changed nothing here: the assertion and the
`f64` lift are this row's, not that unit's.
