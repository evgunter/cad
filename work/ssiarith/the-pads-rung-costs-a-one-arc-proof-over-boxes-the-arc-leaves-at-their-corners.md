---
id: the-pads-rung-costs-a-one-arc-proof-over-boxes-the-arc-leaves-at-their-corners
kind: issue
title: the pad's rung costs a second one-arc proof, several times the tube's, and a torus or cone pair's carry takes up to twice as long
status: open
opened: 2026-11-25
priority: P2
cost: M
---



Filed by SECTARMS's `sectarms/limb3-halves-cover` lane (the build of Ev's
decision on PR 7056,
`work/sectarms/a-sliver-loop-that-carried-as-a-clamped-fit-refuses-its-tube-as-a-periodic-one-at-a-coarse-band.md`; PR 7776).

## Measured

A marched branch's certificate records two rungs
(`ssi::certify`'s `SsiTube::Spatial`): the tube, the widest that
proves, and the narrowest that proves one arc, which the carry reads
the pad and a loop's winding off (`narrowest_proving`). The second costs one more probe,
its cover's graph margins and one `one_arc_r3` run. On every row
measured the ladder's narrowest rung proves on that first probe, so it
is exactly one run. It is not a cheap one.

Each probe timed alone, ε = 1e-6, debug build, the cone pairs of
`cone_pair_section::a_carried_loop_winds_as_its_claim_on_both_carriers`:

| branch | tube rung, its probe | narrowest rung, its probe |
|---|---|---|
| 246 boxes | 7.8e-2, 0.19 s | 9.5e-6, 0.66 s |
| 243 boxes | 7.8e-2, 0.15 s | 9.5e-6, 0.60 s |
| 195 boxes | 2.0e-2, 0.33 s | 9.5e-6, 7.9 s |
| 166 boxes | 2.0e-2, 0.20 s | 9.5e-6, 3.5 s |

That row runs 18.6 s on main and 45.9 s on the branch.

Whole carries (march, certificate and readers) of
`pair_section::wiring`'s fixtures through `march_over_the_ball`, at the
default ε, where main's tube rounds move nothing, so the difference is
the second rung and the readers' two covers:

| pose | main | branch |
|---|---|---|
| the ball round the donut's axis | 7.7 s | 12.2 s |
| plane × torus `(2, 0.5)`, tilt 0.5, height 0 | 5.9 s | 11.5 s |
| the same, height 1.49 | 7.3 s | 8.9 s |
| tilt 0.9, height 1.31 | 6.1 s | 9.7 s |
| tilt 0.05, height 0.01 | 8.5 s | 15.0 s |

At ε = 1e-6 the same carries are no slower than main's (4.1 s against
7.4 s on the first), because main's tube rounds re-marched there and
the branch does not. Cylinder × sphere carries were not timed on main.

An edge certified at rest asks no second rung (its `narrow` is its
tube's), so this cost is the march's alone.

## Not measured

Why the narrow run is slower. A span's first and last control points
are the carrier's own points and sit on the hull box's faces, at a
corner where the span is monotone along each axis, so with ten microns
of padding the arc crosses the box's boundary within that of an edge
or corner, where `face_roots` cuts deepest. That is a reading of the
code, not a measurement.

## What would answer it

Time `face_roots` per box at the two rungs and read where the cuts go.
If the corners are the cause, the pad barely depends on the rung once
the rung is small against the box (the measurement before the build
halved the pad only by halving the boxes), so a rung some fixed
fraction of the narrowest box would read nearly the same pad for a
cheaper proof. Which rung the pad is read off is Ev's decision on
PR 7056 ("the narrowest rung that still proves one arc"), so that is a
question for Ev and not a tuning.
