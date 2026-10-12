---
id: the-integral-arm-reads-below-the-rational-arm-on-an-equal-weight-net
kind: issue
title: The integral arm reads a little below the rational arm on an equal-weight net: degree elevation and the refined scan
status: open
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
- the rational arm also runs `rational_refined_scan`, whose refined-span
  chords the integral arm does not have.

On the swaying family the two arms agree to about `1e-12`
(`an_equal_weight_curve_meters_no_worse_than_one_ulp_off`).

## Shape

Degree-elevate in the integral piece assembly (cost: rows of degree
`2p − 1` instead of `p − 1`), or give the integral arm the refined
chords. Weigh against `the-rational-meters-refined-scan-costs-190-ms-per-loft`,
which may remove the refined scan instead.
