---
id: the-integral-arm-reads-below-the-rational-arm-on-an-equal-weight-net
kind: issue
title: The integral arm reads a little below the rational arm on an equal-weight net, by degree elevation
status: deferred
opened: 2026-10-12
priority: P4
cost: M
---

## What

PR 4824 set both piece schedules to 32 so that an equal-weight net
loses no reach by taking the integral arm. It loses a little. On 1697
random integral nets (degree 1–5, `random_curve` in
`geom/tests/curves/swaying_corner_meter.rs`, seed `0x77`), the same net
with one weight one ulp off (the rational arm) reads higher on 73 (4.3%),
by a median 0.1% and at worst 4.6%. Two causes, both in
`crates/geom/src/curves/nurbs.rs`:

- the rational piece assembly reads `A′w − Aw′`, which at `w ≡ 1` is the
  derivative degree-elevated by `p`: the same direction and a tighter
  hull, so never lower than the integral arm's pieces
  (`tests::the_rational_piece_reading_is_the_elevated_integral_one`);
- (retired by PR 4860, which removed the scan) the rational arm also runs `rational_refined_scan`, whose refined-span
  chords the integral arm does not have.

On the swaying family the two arms agree to about `1e-12`
(`an_equal_weight_curve_meters_no_worse_than_one_ulp_off`).

## Shape

Degree-elevate in the integral piece assembly (cost: rows of degree
`2p − 1` instead of `p − 1`), or give the integral arm the refined
chords. Weigh against `the-rational-meters-refined-scan-costs-190-ms-per-loft`,
which may remove the refined scan instead.

## Update (2026-10-12, `nurbs/refined-scan-cost`)

The rational arm's refined scan, the second cause above, is gone. On
the same 1697 nets (seed `0x77`) the one-ulp-off reading is now higher
on 33 (1.9%, was 4.3%), by a median 0.03% and at worst 0.20% (was
4.6%): degree elevation alone. The shape that remains is the first
bullet's.

## Deferred (2026-10-12, the NURBS orchestrator's ruling)

With the refined scan gone (PR 4860), measured on 4796 random integral
nets (`random_curve`, degree 1–6, every seventh an exact stall, seed
`0x77`) plus the swaying family at degree 2–3, `a` 0.5–8: on the
certified (`Interval`) lane the one-ulp-off reading is higher on 95,
and **0** have a positive rational reading under a non-positive
integral one; the least `span·meter` among them is `0.043`, far above
the bands the consumers decide `Margin::metered` against. (At `f64`
19 degree-2 stall nets read about `1e-17` one ulp off against the
integral arm's exact `0`: rounding inside the meter's stated posture on
a curve whose speed is `0`, not reach.) No decision moves, so the gap
stays, disclosed; the fix to build if a consumer needs it is the
shape above (degree-elevate the integral piece reading).
