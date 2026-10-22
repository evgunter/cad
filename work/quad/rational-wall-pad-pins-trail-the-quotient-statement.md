---
id: rational-wall-pad-pins-trail-the-quotient-statement
kind: issue
title: the rational-wall pad pins read 2.0016e-6 after the quotient statement tightened the f_uu hulls
status: open
priority: P1
cost: E
opened: 2026-10-22
refs: [5582]
---

Two slow-set rows are red on main at the default ε, at the
`MEASURED` assertion that holds the measure the pad ceiling's comment
cites:

- `crates/sweep/tests/cert5_offgrid_knot_rational.rs`
  `a_linear_skin_with_six_offgrid_knots_certifies` (blade-8) —
  `BLADE_8_PAD_AT_DEFAULT_EPS`;
- `crates/sweep/tests/m8_3_rational_volume.rs`
  `tier3_admits_the_rational_wall_body_and_its_volume_brackets_the_extrusion`
  (arc prism) — `ARC_PRISM_PAD_AT_DEFAULT_EPS`.

Both read 2.2106e-6 against a pin of 2.0016e-6 (+10.4%). Bisected on
main's first-parent line: green at `aaef9f58` (PR 5580), red at
`9d680e33` (PR 5582, the rational quotient statement), red at
`b7ffb5fc` (PR 5598).

**Cause: a tighter hull, read through the fixed schedule's stop rule.**
`rational_patch_face`'s `f_uu` block hulls now come from
`MeasureCells::block_hulls` (the quotient statement's coefficient
ratios) instead of the hand quotient rule. On the wall's 8×14 blocks
they are ~12× narrower: `[11.31, 15.83]` against `[-8.40, 45.52]`.
The schedule stops at the first round under `target_len` (1.024e-6),
so the wall now converges at round 4 (128 pieces, width_len 9.97e-7,
0.973× target) where it ran to round 6 (512 pieces, 9.01e-7, 0.88×).
The face's flux width ratio 1.326e-5 / 1.201e-5 = 1.104 is the pad
ratio. Sound (the hulls are the statement's, checked under an exact
oracle in 5582's review) and expected: the pad is what the schedule
achieves. The old pin was not over-tight. It was the overshoot of
two rounds the looser hull forced.

`cell_midpoint`'s enclosure moves nothing here: collapsed back to a
point at `9d680e33`, the pad reads identical to 17 digits.

**Fix:** re-pin both to 2.2106e-6 and update the ceiling comments'
"1.95 times the target" to 2.16. Drop the origin-lever figures, which
were taken under the old hulls.
