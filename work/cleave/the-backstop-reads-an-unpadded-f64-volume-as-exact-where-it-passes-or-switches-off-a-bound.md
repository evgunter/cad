---
id: the-backstop-reads-an-unpadded-f64-volume-as-exact-where-it-passes-or-switches-off-a-bound
kind: issue
title: The volume backstop passes a violated bound whose f64 margin rounds to holding, and its operand gate and +V posture read the unpadded f64 sum as exact too
status: closed
opened: 2026-10-21
priority: P1
cost: M
refs: [the-backstops-magnitude-arm-meters-the-f64-margin-after-the-interval-has-cleared-it, a-planar-face-sums-its-area-about-a-far-carrier-origin]
pr: 5842
closed: 2026-10-27
---


Found by PR 5576's review (FLUX #2, `flux/far-frustum-pad`; R1's probes on
`review/5576-017Nyiv2`, `crates/topo/src/boolean/ops.rs` modules
`review_5576*`; R2's on `review/5576-011tM9s6`). This is the sibling of
`the-backstops-magnitude-arm-meters-the-f64-margin-after-the-interval-has-cleared-it`.
That row's arm 2 is a false refusal; this row's (a) is a silent pass,
which is worse.

Every reader below takes a closed-form body's volume (`volume_pad == 0.0`,
so `SignReading::Exact`) as the exact value, where it carries the closed
forms' rounding. That rounding reaches far past the band where a face's
carrier is stored far from it
(`fluxtail/a-planar-face-sums-its-area-about-a-far-carrier-origin`).
The fix shape is the same for each: before a held or in-band read decides
anything, re-derive in interval arithmetic
(`PastTarget::interval_volume`, as arm 1 does before a refusal), or
decide on an enclosure of the sum.

## (a) A violated bound reads held: `Posture::read`, the Bound arm

`crates/topo/src/boolean/ops.rs:2436`, `Posture::read`. For an exact
margin, `held = upper.is_ok()` (`:2448`–`:2449`): an `f64` margin that
reads non-negative is `Reading::Held`. `bound_holds` (`:2530`, `:2539`)
then returns `Ok` with no re-derivation, because arm 1 re-derives only
a `Violated` reading.

Witness, R1's `review_5576_violation_hidden_by_rounding`: A is a
1 m × 1 m × t slab at home, and R is the same slab `(1 + k)` times as
thick with its planes anchored `far` along themselves. So
`vol(A ∩ B) ≤ vol(A)` is truly violated by `k·t` m³. **3 of 60 rows pass
silently, at every ε:**

| far | t | k | exact margin | f64 margin |
|---|---|---|---|---|
| 1e4 | 1e-2 | 1e-9 | −1.0e-11 | +6.4e-9 |
| 1e4 | 1e-2 | 1e-7 | −1.0e-9 | +1.06e-9 |
| 1e5 | 1e-3 | 1e-7 | −1.0e-10 | +4.8e-8 |

The arm-2 fix proposed on the sibling row does not reach this: no arm
ever reads the interval margin here.

## (b) The operand gate: `bounded`

`ops.rs:2320`–`:2336`, `volume_backstop`. The bound arms run only
against an operand whose `volume_lo / surface_area` decides positive at
the band (`decide_invariant("volume_backstop_operand", …)`), and for a
closed-form operand `volume_lo` is the `f64` sum. A valid far operand
whose sum reads in band switches off **every** bound arm, and a valid
near-band operand whose sum reads past the band switches them on.

Witness, R1's `review_5576_operand_gate_reads_f64`: A is a 1 m² slab of
thickness 3e-8 m, valid, and the planted "result" of `A ∩ A` is a whole
1 m³ cube.
- At `far = 0` and `1e2`, the sum reads 3e-8, and the backstop refuses
  `vol(A ∩ B) ≤ vol(A)`.
- At `far = 1e5`, the same slab sums to 1.0e-8, in band. The gate
  switches the arms off and the backstop returns `Ok` for a result
  3·10⁷ times too large.

R2's `p3_operand_gate` (`review/5576-011tM9s6`, `topo` `tier3_tests`)
shows the flip the other way. It plants an inside-out 1 mm² slab
(exact `V/A` from −5e-12 to −5e-8) anchored 5 km to 100 km away. Its
`f64` sum reads positive, so the gate reads `Positive` and switches the
bound arms on against a complement operand. Here the planted `A ∩ B`
refuses `vol(A ∩ B) ≤ vol(A)` against a bound of +1.2e-12. Tier 3
refuses that operand at rest, so the public door does not reach this
case. So the gate errs in both directions; it does not merely "err
toward checking more".

## (c) The +V posture: `Posture::PlusV`

`ops.rs:2461`: `Posture::PlusV` reads `crate::validate::plus_v_read`,
which is `props::read_role`, metered against the band. It skips
`props::certify_role`, so it never re-derives.

Witness, R1's `review_5576_plus_v_hidden_by_rounding`: an inside-out
1 mm² × 100 nm slab anchored 5 km along itself, as the result of `A ∩ A`.
Its `f64` sum reads **+1.2e-12**, so `vol(A ∩ B) ≥ 0` passes. Check 7
refuses the body downstream (`NegativeVolume`), so this one is caught
later, not silently returned. At 1e4 and 1e5 the sums read negative and
the arm refuses.

## What closes them

Each held, in-band or role read in the backstop decides on the interval
re-derivation, or on an enclosure that states the sum's rounding (as
`props::certify_role` does with `rounding_bound` where there is no lane).
The four readers (arm 2 on the sibling row, and (a), (b) and (c) here)
share one cause. They may be one fix, a margin type that is never "exact"
at `f64`. `SignReading::Exact`, `SignReading::exact()` and `is_exact()`
invite exactly these reads: "exact" there means "one value, no
quadrature bracket", not "exact" (R1, style).

## Closed (2026-10-27)

The volume backstop left the boolean's operation path (`[ev]` PR 5780: the volume relations are a test oracle the suites run on their own results, `topo::test_support::volume_relations`), so nothing refuses or passes on it. Its three readers (the Bound arm's held read, the `bounded` operand gate, the +V posture) went with it. The far-anchored closed form's rounding the row leans on is NEARPOINT's (`a-planar-face-sums-its-area-about-a-far-carrier-origin`), and the oracle's docs name it as a flag's possible cause.
