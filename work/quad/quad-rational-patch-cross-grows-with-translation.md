---
id: quad-rational-patch-cross-grows-with-translation
kind: issue
title: quad: the rational patch lane (rational_patch_face, Ladder::cross_num_at over a column slice) assembles A = w·P in the absolute frame from decorrelated hulls, so a rational face's area and flux enclosures grow with its translation (area width 0.124 at 0, 7.49 at 100 m, 66.5 at 1 km)
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [a-translated-arc-prism-panics-the-area-gauge-through-mass-properties, 3863]
---


## Found (review of PR 3863, `ssi/rational-chart-speed`, 2026-10-02)

Measured by that PR's reviewer; not reproduced by the filer. PR 3863's
own §5 sweep missed it.

`crates/geom-brep/src/props/quad/rational.rs`: the rational lane
(`rational_patch_face` → `Ladder::cross_num_at`, read over
`Collapse::Over` windows) forms the homogeneous net `A = w·P` from the
control points as given and assembles the area element's cross
product from separately-hulled terms. Each term carries the net's
distance from the origin times the weight derivative, and the hulls
subtract decorrelated, so that distance stays in the width instead of
cancelling. Its caller, `crates/topo/src/props/quad_lane.rs`
(`trimmed_patch_face_rounds` call site), passes `payload.control()`
unshifted; the `center` that file subtracts elsewhere is the
circle/ellipse trig bracket's, not a recentring of the net.

On a cubic × linear wall weighted 1.8 / 0.7 (the `rational_wall`
fixture in `crates/geom-brep/src/ssi/enclose.rs`'s tests), translated:

| translation | area enclosure width | flux enclosure width |
|---|---|---|
| 0 | 0.124 | 0.023 |
| 100 m | 7.49 | — |
| 1 km | 66.5 | 2.04e5 |

`nurbs_patch_face` refuses `QuadratureBudget` from 1 m.

## What is open

Assemble the cross from translation-free terms, either a cell-local
origin for `A` (as `patch_bound`, `mesh::chords` and `mesh::nurbs_cert`
do) or per-pair control-point differences (as
`NurbsBoxes::cell_quotient_numerator` does after PR 3863), so a
translation moves the area enclosure by its rounding width. The flux
integrand itself depends on the origin, so its width should be judged
against the flux's own magnitude after the fix. Settle whether
`a-translated-arc-prism-panics-the-area-gauge-through-mass-properties`
is this degradation.

## Evidence (2026-10-13, NURBS)

`geom_core::spline::derivative` now reads `S_u × S_v = M̃/W³` as a
Bernstein quotient in the frame of a block's first point
(`cell_quotients(.., area = true)`, PR 4954; second order PR 4984).
A translation moves its pieces by rounding alone, the property this row
wants. It reads span cells on exact dyadic pieces. A quad cell (a
uniform cut of the trim rectangle) would reach it through a blossom
restriction of the cell's window and `CellAxis::Bezier`, giving `g_hull`
as `[inf_norm, sup_norm]`. The pads `∂_d g` are the quotient
`(M̃_d·W − 3·M̃·W_d)/W⁴`, which the statement does not form today. Swept
from `work/nurbs/other-rational-speed-readings-read-relaxed-quotient-recipes.md`.

## Re-measured on main 258dc6c7c2 (QUAD, 2026-10-21)

**Verdict: stands at the door `geom_brep::props::quad::nurbs_patch_face`;
gone through `topo::mass_properties`.** FLUX's `shell_reference`
(PR #5442, `topo::props`) moves each face's net to a vertex of its
shell before the quadrature reads it (`quad_lane.rs`, `about`), so a
body's translation no longer reaches this lane. The lane itself
still assembles in whatever frame its caller hands it.

Probe: `crates/sweep/tests/quad_frame_remeasure.rs`
`row1_rational_wall_translated` (`#[ignore]`d; run command in its
header). It drives `nurbs_patch_face::<f64>` on this row's
`rational_wall` net, translated by `(t, t, t)`, over the full
rectangle, perimeter 3.8, default ε:

| t | result |
|---|---|
| 0 | area [0.81342, 0.93714], width 0.124; flux width 1.43e-6 |
| 1 m | refuses `QuadratureBudget { width_len 8.33e-6, target 1.024e-6, rounds 1 }` |
| 10 m | refuses, width_len 4.56e-4 |
| 100 m | refuses, width_len 1.12e-2 |
| 1 km | refuses, width_len 0.125 |
| 100 km | refuses, width_len 12.7 |

These are the same numbers to every printed digit on main before
#5442 (`ec32861499`). The flux refusal width grows about linearly with
`t`, and `nurbs_patch_face` still refuses from 1 m. The 100 m and 1 km
area widths in the table above are not reachable at this door because
it refuses first.

Through a body, the rational lofted walls of
`a-swept-circle-section-loop-decides-its-volume-sign-only-at-the-origin`
read the same refusal widths to four digits at the origin and at
(100, 0, 0) m and (1000, −700, 400) m on main. Before #5442 the
circle loop's v-degree-3 widths grew 2.8–4× from the origin to 100 m.
What is left open is the door's own frame: a caller that hands it a
net far from the origin still gets these widths.

## Re-measured on `quad/rational-quotient-statement` (QUAD, 2026-10-22)

The lane's Taylor hulls and area pads now read the quotient statement
in the frame of each span cell's first point
(`geom_core::spline::cell_measure_quotients`, unit
`quad-rational-integrand-second-derivative-reads-the-quotient-rule-by-hand`).
`rational_wall` translated by `(t, t, t)`, `nurbs_patch_face::<f64>`
over the whole rectangle, default ε:

| t | main | this branch |
|---|---|---|
| 0 | area width 0.124; certifies at round 7 | area width 0.0504; certifies at round 4 |
| 1 m | refuses, width_len 8.33e-6 | certifies |
| 10 m | refuses, 4.56e-4 | refuses, 2.17e-6 |
| 100 m | refuses, 1.12e-2 (area width 7.49) | refuses, 2.17e-5 (area width 0.0504) |
| 1 km | refuses, 0.125 | refuses, 2.17e-4 |
| 100 km | refuses, 12.7 | refuses, 2.17e-2 |

The area bracket no longer moves with `t` (0.0504 at 0 and at 100 m).
The flux floor still grows linearly with `t`: `N = A·(A_u×A_v) + c·M̃`
carries the frame's `c`, so `f_uu` itself scales with the distance
from the origin. That is the flux's own magnitude, which this row asks
to judge it against; the per-point midpoint reads in the absolute frame
are unchanged.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds at the door, unmeasured since 2026-10-22.
`nurbs_patch_face` (`crates/geom-brep/src/props/quad/patch.rs:141`)
now takes the caller's accept closure. Since PR 5741 there is no
`QuadratureBudget`, so the "refuses" rows above now read as a stop
short of the closure's target (`LaneEnd::Floor` or `Wall`, with the
bracket). The widths are not re-measured. The rational lane is
`rational_patch_face` (`props/quad/rational.rs:360`). Topo still reads
each face about its shell's reference point (`props/quad_lane.rs`,
`about`), so a body's translation still does not reach the lane.
