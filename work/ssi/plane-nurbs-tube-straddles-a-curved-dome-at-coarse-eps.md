---
id: plane-nurbs-tube-straddles-a-curved-dome-at-coarse-eps
kind: issue
title: ssi: limb 3 refuses TubeStraddles on the plane × curved-dome cuts at ε 1e-6
status: closed
opened: 2026-10-02
priority: P1
cost: M
closed: 2026-10-02
branch: ssi/dome-tube
---


Found by `ssi-a-seed-refined-off-the-chart-is-marched`'s lane. This is
one of the two refusals that
`plane-nurbs-ssi-does-not-certify-a-curved-dome` leaves unexplained.

## What

At ε 1e-6, limb 3 refuses the tilt cut of the dome `W(d)` with
`TubeStraddles { verdict: Zero(margin 0.0) }` at every d measured from
0.5 to 3 (52 to 137 boxes). This holds once the seed off the chart no
longer reaches the fit. Before that fix it showed at d = 0.5 alone,
because the other rows refused limb 1 first. The level loop refuses
the same way at every d. At ε 1e-9 the same cuts get past limb 3 and
refuse limb 2 instead.

So limb 3's transversality enclosure over the tube chain straddles
zero on a branch that the march traced without any
`ssi_transversality` refusal. It is not yet measured, but the likely
reading is that
the tube boxes at ε 1e-6 are wide enough that the wall-normal interval
over one box spans the plane normal's direction. If so, it is a
resolution limit of limb 3 at coarse ε, not a sliver (F6), and the
refusal's text misnames it.

## Repro

`crates/geom-brep/tests/m5_pr7_ssi.rs`,
`a_seed_settled_off_the_walls_chart_is_no_branch`, at
`CAD_TOLERANCE_EPS=1e-6`. It pins this refusal at d = 1 and d = 2.

## Next

Measure the straddling box (its chart extent, and both normal
enclosures) before choosing between refining the tube chain where it
straddles and renaming the refusal.

## Closed

**It was an enclosure artefact, not a resolution limit and not a
sliver.** The dome is one quadratic Bézier patch: knots
`[0,0,0,1,1,1]` both ways, so one span cell. `NurbsBoxes::deriv_box`
read the quotient rule's hull off every span cell the window touched,
**whole**. So every tube window, at every rung, got the whole patch's
derivative net: `S_u ∈ (1, [−2d, 2d], 0)`, `S_v ∈ (0, [−2d, 2d], 1)`.
Against the tilt normal `(1,1,0)/√2` that gives `φ_u ∈ [−0.71, 2.12]`
and `φ_v ∈ [−1.41, 1.41]` at d = 1. The transverse component spanned
about `[−2.5, 1.5]` in all 76 windows, at all 14 rungs, from 0.125 m
down to the 1.5e-5 m floor. The pads were 0.056 down to 6.8e-6 chart
units. The margin was exactly 0 every time. Nothing about ε is
involved, and no other sheet of the locus or boundary is near.

At ε 1e-9 limb 3 is not reached on these cuts: limb 2 (or 1) runs
first and refuses. The exception is the tilt at d = 0.5, which
straddled at 1e-9 too. So the earlier reading, that "the same cuts get
past limb 3 at 1e-9", had the limb order backwards.

**Fix** (`crates/geom-brep/src/ssi/enclose.rs`): `deriv_box` also cuts
each cell's net to the window's part of the span (`CellNet::cut`). It
blossoms the homogeneous net `(w·(P − c), w)` to the window's Bézier
block by de Boor's recurrence in certification arithmetic, applies the
same paired quotient-rule form, and **meets** that box with the whole
cell's.

The meet is load-bearing. The cut's rounding is not always smaller
than what it saves:
- On a net constant along the cut whose `P − c` does not round
  exactly, the cut alone reads a few ulps around zero where the whole
  cell reads an exact zero. That turned `WallConstantAcrossLocus` into
  a margin-0 straddle.
- On a window a few ulps wide, `degree / (b − a)` amplifies the cut's
  rounding far past the whole cell's box.

So the box is never wider than the whole cell's, and exact zeros
stay exact. A cell the window covers whole reads exactly as before,
so `chart_speeds` is bit-identical. `rect_box`, which the exhaustiveness sweep and seeding
read, keeps the whole-cell box (`cell_deriv_box`). Moving it is
`the-chart-sweeps-first-order-box-reads-its-derivative-off-the-whole-span-cell`.

| cut | d | ε 1e-6 before | ε 1e-6 after (margin, boxes) |
|---|---|---|---|
| tilt | 0.5 | TS (52) | OK, rung 0.125, 0.651, 52 |
| tilt | 1 | TS (76) | OK, rung 0.125, 0.619, 76 |
| tilt | 1.5 | TS (93) | OK, rung 0.125, 0.570, 93 |
| tilt | 2 | TS (107) | OK, rung 0.125, 0.525, 107 |
| tilt | 3 | TS (137) | OK, rung 0.125, 0.455, 137 |
| level | 0.5–3 | TS (235) | OK, rung 0.125, 0.105 / 0.255 / 0.384 / 0.480 / 0.605, 235 |

At ε 1e-9 the tilt at d = 0.5 goes from TS (295) to OK (0.663). The
other rows refuse limb 1, limb 2 or the fit budget as before, with
identical margins. At 1e-12 the fit budget refuses every row before
the certificate runs. Pinned by `certify::tests::normal_crossing_tests::a_chart_constant_across_the_locus_refuses_on_a_net_that_is_not_one_point`,
by `enclose::tests::the_derivative_box_is_never_wider_than_the_whole_cells`
(a fuzzer),
by `m5_pr7_ssi::a_curved_domes_cuts_prove_their_tube_at_the_widest_rung`
(band 1e-6), by `a_seed_settled_off_the_walls_chart_is_no_branch` (now
`Ok` at 1e-6), and by
`enclose::tests::the_derivative_box_shrinks_with_its_window_below_a_span_cell`.
