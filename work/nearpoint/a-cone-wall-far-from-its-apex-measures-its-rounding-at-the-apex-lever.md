---
id: a-cone-wall-far-from-its-apex-measures-its-rounding-at-the-apex-lever
kind: issue
title: A cone wall far from its apex sums its vector area about the apex, so a rigid turn reads the rounding at the apex's lever: a drafted revolve measures 2.6% wrong with pad 0
status: open
opened: 2026-10-15
priority: P1
cost: M
---


Found by `cleave/rigid-turn-sliver-volume`'s sweep (the conic-arc lever
of `work/cleave/a-rigid-turn-reads-a-valid-ellipse-sliver-inside-out`):
the same rounding-times-lever shape, with the cone's apex as the lever.

## What

`geom_brep::props::curved::cone_loops` sums each loop's vector area
`loop_vector_area(edges, apex)`, and `cone_face_closed_form` takes the
flux as `apex·A⃗`. On a wall far from its apex every term is the apex's
distance squared and they cancel to the wall's area, so the rounding of
the differences `p − apex` (at the apex's magnitude) reaches `A⃗` times
that distance, and the flux times the apex's distance again. The
closed form's pad is zero. At rest the rounding lies along the axis and
cancels; a rigid turn moves it off.

## Witness (release, measured on that branch, arc fix in place)

A partial revolve (θ = 1 rad about the sketch's `y` axis) of the
annulus `1 ≤ r ≤ 2`, `0 ≤ y ≤ 1`, its top drawn in by `τ`, so each wall
is a cone with its apex `1/τ` or `2/τ` above. Exact volume
`θ·(3 − τ)/2` (Pappus). Turned 1.3 rad about (1, 2, 3) through
(0.3, −0.2, 0.5), measured by `mass_properties`:

| τ | ε | at rest | turned | pad |
|---|---|---|---|---|
| 1e-1 | 1e-9 | exact | 2e-16 | 0 |
| 1e-3 | 1e-9 | −6e-14 | **+5.8e-8** (slack ε·A = 8e-9) | 0 |
| 1e-5 | 1e-9 | −1.4e-13 | **+3.86e-2** (2.6 %), and `validate_geometric` refuses | 0 |

Summing each loop about its own first point instead gives 1e-11 at
τ = 1e-5, but it is NOT the cure: it regresses
`sweep::all a_curved_keel_in_the_plane` at ε = 1e-6 (the halves of the
keeled annulus sum 6.1e-8 off the whole, which the apex sum reads to
3e-13). That body's cone faces carry in-band gaps (the V's tip within
0.6 ε of the cutting plane), and a loop that does not close has a
vector area that depends on its reference: the apex fan reads a gap
along a generator as nothing, a fan from a point of the loop reads it
times that point's lever. Carrying the own-point sum to the apex by the
closure defect (`A⃗(apex) = A⃗(a) − ½(apex − a)×D`, equal in ℝ to the
apex sum) restores the keel to 4e-15, but then a body turned about a
point 600 m away reads the rounding gaps (`D` at `ulp(600)`) at the
apex lever: 1.9e-6 at τ = 1e-5.

So the cure has to say what a gap means, which is
`work/tally/a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever`'s
question too: closing each loop through its gaps by straight chords,
`A⃗ = A⃗(a) + ½Σ(q_i − a)×g_i` (the chord from each edge's end `q_i` by
its gap `g_i` to the next edge's start), makes the loop closed and so
reference-free; it reads a generator-direction gap as the apex fan does
and a rounding gap at the loop's own size. That is unmeasured.

## Inside the box (PR 5185's review)

The witness above puts the apex 1e5 m away, outside D4 ¶4's 1 km
session box. The review measured it inside: τ = 2e-3 (apexes at 500 m
and 1 km), ε = 1e-12, the turned body reads 6.8e-9 m³ off, about 850×
ε·A, with `Ok` and pad 0, while `validate_geometric` refuses it; at
τ = 4e-3 the miss is about 75× ε·A. So the P1 stands on an in-box body.
