---
id: the-polynomial-arm-of-the-offset-composite-bounds-looser-than-the-rational-arm
kind: issue
title: offset_fit's Composite bounds a polynomial surface looser through its polynomial arm than through the rational arm with weights all 2
status: open
opened: 2026-10-12
---


Found in review of PR 4872 (branch `encl/subnormal-weight-face`).

One polynomial surface, described two ways: with unit weights it is
read by `offset_fit`'s `Composite::build` on the polynomial arm
(`w ≡ 1`, `NetForm::Spatial`). With every weight `2` it is read on the
rational arm (`w` decomposed, `M̃` with its `w_u`, `w_v` terms). The
rational arm certifies tighter cells. The polynomial arm is the
rational arm specialised to `w ≡ 1`, so it should be at least as tight;
it loses tightness somewhere.

- **Reviewer's measurement**, made on main with the PR 4872 review's
  fixture: max cell bound `1.355e-4` with weights ≡ 2 (rational arm)
  against `1.571e-4` with unit weights (polynomial arm).
- **Reproduced on a fast fixture.** The control net of `offset_fit`'s
  test `quarter_cylinder` (degree 2 × 1) with all weights equal, against
  the same net scaled by `1.3` about `z` with unit weights, at
  `d = 0.3`, on the meters' floor:
  - weights ≡ 2: max cell bound `1.0714e-1`;
  - unit weights: max cell bound `1.8295e-1`.

The direction is degree 1 in `v`, so both arms raise it in the ring
(`PatchSpans::elevated`). The suspect is how the polynomial arm forms
`w` — the constant `1` patch from `a[0].constant(Interval::one())` —
against the raised `A` channels, or how `M̃ = A_u × A_v` is assembled
there. Not localised.

The test `offset_fit::tests::a_uniform_weight_scale_moves_no_cell_bound`
pins the weights-2 value (`1.0713619353426333e-1`). It moves if the
arms are reconciled.
