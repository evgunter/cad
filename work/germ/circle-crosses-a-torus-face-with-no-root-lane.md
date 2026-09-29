---
id: circle-crosses-a-torus-face-with-no-root-lane
kind: issue
title: A circle edge against a torus face has no root lane: the lily's stem seam refuses CurvedPierceUnsupported at the arch's torus wall
<<<<<<< HEAD
status: dispatched
=======
status: closed
branch: germ/circle-torus-root-lane
pr: 3375
>>>>>>> origin/main
opened: 2026-09-26
priority: P1
cost: H
refs: [torus-operand-gate-admission]
closed: 2026-09-29
---


## What

Sub-item (b) of `torus-operand-gate-admission`, left open when the rest of
that row landed (PR 3265). The arch's outer-equator seam crosses the stem's
carrier at 29° along the arch, outside the stem face's window. With the
torus admitted, the lily's wall 1 now stops there as a circle×torus
`CurvedPierceUnsupported`. That is an undecided enclosure: there is no root
lane for a circle against a torus. The item's honest negative certificate
maps the arc into the stem's chart.

## Home

GERM: the torus operand lane. `wall_crossing` answers line×torus since
PR 3265, and this is the circle×torus crossing beside it.

## The lane (branch `germ/circle-torus-root-lane`)

**Measured first.** On the base the lily's stem glue refused
`CurvedPierceUnsupported { operand: A }` at the stem's INNER equator
seam (radius 4.94 about `(-5, 0, 0)`) against the arch's tube wall —
not the arch's outer seam this item's prose names. Once that pair had a
lane, the next refusal was the arch's outer equator seam (radius 1.152)
against the stem's wall: the pair this item describes, a STRADDLE (one
end inside the stem's tube at the weld, one far outside) whose one
crossing lies on the stem's carrier past the stem face's 22° window.

**The polynomial is a quartic.** `|q|²` and `q·â` are first harmonics
along a circle, so the torus's implicit `F` composed with the circle is
a degree-2 trigonometric polynomial; the tangent half-angle gives a
QUARTIC (Bézout's eight loses four to the circular points). It is
solved by the ray lane's certified ladder, factored out of
`solid_contain::line_torus_roots` as `depressed_quartic_roots` with its
own predicate rows (`bool_circle_torus_*`), in
`topo::boolean::circle_torus`. The half-angle pole is anchored at the
arc's antipode and certified off the torus (`bool_circle_torus_pole`),
which makes the monic division sound and certifies the one unreachable
parameter is not a root; a coaxial carrier is decided first, in metres.
The parallel-axes pose (the lily's) has its own closed form decided on
lengths — circle × contour — because the quartic's discriminant also
meters the COMPLEX roots and read the lily's 8 mm near-miss as a graze
at ε = 1e-6 (hosted CI, `tour_runs_green_at_eps_1e_6`). `wall_crossing` takes the circle's roots and meters its end gaps as
arc length; every root is examined.

**The certificate chosen: the roots, not an arc-to-chart map.** The
chart map cannot clear either lily pair: both arcs END at the weld,
exactly on the partner face's window edge. The roots sit well clear of
it (1.7° and 6.5° along the stem seam), and each is placed `Out` by the
face's own trim — the negative certificate, read through the chart,
root by root.

**The straddle.** A straddle whose every interior root is placed `Out`,
with no root at an end, is now no event (`SpanVerdict::Elsewhere`);
before, the straddle arm read it as a contradiction. That arm is shared
with lines, so a line straddle moves too: the merged teapot cup's
subtract now reaches the join (evidence on ZIP's
`an-edge-lying-in-a-cutter-face-past-its-end-wall-leaves-loose-ends-unpaired`).

**What the lily does now**: wall 1 refuses
`GermFrameUnsupported { Plane, Torus }` at the join — the stem's weld
cap against the arch's wall (evidence on CURVED's
`c5-plane-torus-cone-cylinder-arms`). No body, so nothing to measure.

**Fix pass (both reviewers NOT-MERGEABLE-AS-IS, adjudicated union).**
The half-angle pole is now required to be well conditioned, not just
non-zero: `|F(pole)| ≥ κA` (κ = 1/16, `A` the harmonic amplitude bound)
keeps every root `κ/2` rad from the pole and the monic coefficients
below `6/κ`, over 32 candidate anchors, which Parseval makes complete
unless `F ≡ 0` (the pole's residual decision refuses that case). The
machinery is surface-generic (`half_angle_roots`), for VERBS-CONE's U2.
The parallel arm decides on the RESIDUAL at the carrier's two
extremes, with the admitted tilt charged; the lever is
`min(2ρ, R + r)`; a covered circle keeps the frontier door at the
circle rung, so the declared-cover arms are line-only by construction.
