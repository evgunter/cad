---
id: a-two-chart-tube-cannot-prove-one-arc-along-a-chart-boundary
kind: issue
title: "The two-chart tube's one-arc walk is undecided where a cut box holds a solution that is no isolated root of one face: a rim along a chart's boundary row, or a carrier ending on the domain sides of both charts (the sheath grafted on its blade hits both)"
status: open
opened: 2026-10-28
priority: P3
cost: H
refs: [a-plane-section-along-a-fits-window-edge-proves-no-one-arc, a-rim-between-two-spline-walls-is-certified-against-a-fitted-image-not-its-exact-one, a-pair-walk-end-at-a-domain-corner-refuses-undecided]
---

Lily wall 8's next stop, found by SPLICE's rim-exact-image lane once
`edge_nurbs::nurbs_pair_limbs` read the rim's limbs 1 and 2 through
each wall's exact `IsoArc` image. Re-derived 2026-10-28 from the
measurements below, after PR 5907's two reviews disagreed on whether
a rim inside both charts certifies. It is filed here because the
ground is the SSI tube's (`ssi/four.rs`, `ssi/certify.rs`); SPLICE's
lily witness waits on it.

## What

`nurbs_pair_limbs` certifies limb 3 through `ssi::certify::certify_branch`
with the two-chart tube (`probe_tube_pair`). `ssi::four::one_arc_pair`
cuts every chain box to the product of the two charts' knot rectangles
(`Box4::of_charts`). It then asks each cut box for exactly two boundary
roots, each isolated on one face by a Krawczyk test (`face_roots` over
`face_walk`). A solution of the pair that is not an isolated root of a
single face makes `face_walk` run to `EXIT_DEPTH` and return `None`,
which `limb_three` reads as `OneArcRefusal::Undecided`. Two shapes
reach it:

1. **The carrier runs along a chart's boundary row.** The whole arc
   lies on that face of every cut box (`v_a = 0`, axis 1), a curve of
   solutions rather than isolated ones. One chart's boundary is
   enough. Every exact arc-rim image is a boundary row
   (`arc_rim_images` reads boundary rows only), so every rim that
   reads an exact image reaches this.
2. **The carrier ends on the domain sides of both charts.** The end
   state lies on two or more faces of the first or last box at once,
   on their shared edge (`u_a = 0` with `u_b = 0`, or a corner). This is
   `nurbs/a-pair-walk-end-at-a-domain-corner-refuses-undecided`'s shape,
   which that row records as having no consumer; lily is one.

It is conservative, not unsound. Its siblings are
`a-plane-section-along-a-fits-window-edge-proves-no-one-arc` (the chart
tube along a fit's window edge) and the corner row above.

## Measured (2026-10-28)

Rings of revolution about `z`, `turn = 1.2` rad, `m` rational sub-arcs:

- the cylinder `z ∈ [0, 1]` and the annulus `r ∈ [1, 2]` meet along
  their shared `v = 0` row ("boundary");
- the cylinder `z ∈ [−1, 1]` and the annulus `r ∈ [0.5, 2]` meet along
  their interior unit circle ("interior").

The rim is the unit circle over a span centred in the turn.
`face_walk` instrumented with a temporary print, not committed.

| placement | span | m | ε 1e-9 | ε 1e-6 | the walk's failing face |
|---|---|---|---|---|---|
| interior both | 0.03 | 1, 64, 512 | `Ok` (hull 8.9e-10 at m = 1) | `Ok` (m = 512) | — |
| interior both | 0.1, 0.3, 0.6 | 512 | `Ok` | `Ok` | — |
| interior both | 0.9, 1.1, 1.19 | 512 | `Ok` | not run | — |
| interior both | 1.2, the whole turn | 512 | `Undecided` | `Undecided` | box 0, `u_a = 0` with `u_b ≈ 0`: an end on both charts' sides |
| boundary both | 0.03, 0.1, 0.3, 1.2 | 1, 64, 512 | `Undecided` | `Undecided` (m = 512, and 0.6) | box 0, `v_a = 0`: the arc on the face |
| boundary one chart | 0.03 | 1, 64 | `Undecided` | not run | box 0, `v_a = 0` (the cylinder's row) |

Interior and one-chart rows with `m ≤ 64` over spans past 0.03 stop
earlier, at limb 2's `ImageMiss` on the fitted image, and do not reach
the tube.

So the span, ε and the sub-arc count do not decide `Undecided`. They
decide only whether limb 2 lets the carrier reach the tube. What
decides it is where the carrier lies against the cut box: along a
boundary face, or ending on two charts' sides. That is the setup
difference between PR 5907's two reviews. r1's interior control used a
√ε-scaled span, which certifies. r2's interior rows used the whole
turn, so their ends sat on both charts' `u` sides.

**Lily**, the sweep row (`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`)
at ε 1e-9: the rim is the full `v = 0` row of both walls, so it has both
shapes. The walk fails first at shape 2, box 0's face `u_a = 1` with
`v_a ≈ u_b ≈ v_b ≈ 0`, the end at a corner of both charts. Shape 1's
face is behind it in `face_roots`' face order. Payloads:

- `PlaneNurbs(TubeNotOneArc { rungs: 16, cause: Undecided(ssi_tube_one_arc,
  MarginDiag(Invalid)) })` at ε 1e-9 and 1e-12, both orders;
- `rungs: 8` at 1e-6;
- the tour's lily wall 8 at default ε reads
  `TubeStraddles { Zero(margin 0.0), boxes: 16 }` instead. That is a
  rung that was no graph, whose zero lower bound is read as a decided
  verdict (`a-tube-whose-graph-margin-straddles-zero-refuses-as-a-decided-zero`).

Fixing shape 1 alone does not move lily.

## Owed by whoever closes this row

`nurbs_pair_limbs`' exact arm has two readings that no row can guard
while every exact chart refuses here. Each needs a row that reaches
`Ok` through an exact chart:

- **Its limbs 1 and 2.** On an exact chart the lane reports the exact
  certificate's `max_residual` and `envelope`. Zeroing them turns no row
  red today: PR 5907 review r1's `exact_limbs_zero` mutant survives.
- **Its tube image.** On an exact chart the tube is built about a
  polyline through the exact image at the fitted image's knots, so the
  banked tube's pads sit about that polyline, not the fitted feet.
  Swapping in the fitted image turns no row red today, since the feet
  match the exact image to the foot's precision. A row that tells them
  apart needs feet off the exact image's row: a closed chart's other
  boundary row, or a foot settled off the rim.

## What is open

A one-arc proof for a carrier that runs along a boundary face of a cut
box, and for an end on two faces of one. Options:

- prove over uncut boxes against the charts' own extension;
- read a domain face that holds the carrier as a face the arc runs
  along, rather than one it crosses;
- count a root on a shared edge once, as the corner row asks.

Either way, the section's accounting has to bank the same region the
proof reads. Until then, every rim between two spline walls that is a
boundary row of either refuses at limb 3. That covers every loft or
sweep body grafted on another at a shared section.

A question, not a fix (SPLICE orchestrator): when the arc is a shared
boundary row of both charts, can limb 3's one-arc question be answered
from that structure rather than from a box search? C5's "topology
decides first" rung is the echo. Each chart's certified `IsoArc` image
already states that the carrier is that chart's boundary row over the
edge's span. Unasked: whether that, with the two charts' transversality
along it, is enough to show that no other arc of the pair's locus lies
in the tube's region.

## Named, not answered (2026-10-28)

The walk's `Undecided` for this shape now refuses by name
(`the-two-chart-tube-refuses-a-carrier-along-a-chart-side-undecided`):
`OneArcRefusal::AlongSide { chart }` for a carrier along a chart's
domain side, `EndOnSides { sides }` for an end on two or more domain
sides at once. That changes the payload, not the reach: no row the
shape refused now certifies.

## Narrowed (2026-10-28)

A rim that is a row on both walls' chart boundaries no longer reaches
this tube: the boolean recovers it as the walls' shared row, D2's
structural selector, certified per wall with no limb 3
(`the-boolean-recovers-no-structural-selector-for-a-rim-known-in-advance`,
closed). The sheath grafted on its blade (lily wall 8 until 2026-10-29, now
the sweep row `a_sheath_grafted_on_its_blade`) is past this site.

Of the two owed rows:

- **The exact limbs** are now guarded. `nurbs_pair_limbs`' exact arm and
  the shared row's certificate read an exact image's limbs through one
  accessor (`PcurveCertificate::limbs`), and zeroing it reds
  `a_rim_on_both_walls_rows_is_a_certified_shared_row`.
- **The tube's image** stays unguardable, for the reason this row gives:
  every exact chart image is a boundary row, and a witness along one
  refuses here (`AlongSide`), so no exact chart reaches `Ok` in the witness lane.

What stays open is the witness lane's: a rim that is a boundary row of
one wall only (a graft of one body's section across the other's
interior), and shape 2's end on two charts' sides. Neither has a
consumer today.

## Re-banded P0 → P3 (SPLICE, 2026-10-29)

Lily wall 8 no longer waits on this row. PR 5959 gives a rim that is a
domain-side row of two spline walls a structural selector, certified per
wall with no limb 3. Lily's own rim then refuses at a tangency (two
points), and the scene is being re-spelled as one loft. No consumer
reaches this shape today. It is still a real gap for a witness-selected
carrier, so it stays open at P3 until one does.

