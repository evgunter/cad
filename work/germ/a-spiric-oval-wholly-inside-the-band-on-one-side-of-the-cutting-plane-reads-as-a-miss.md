---
id: a-spiric-oval-wholly-inside-the-band-on-one-side-of-the-cutting-plane-reads-as-a-miss
kind: issue
title: The spiric × plane door answers a miss for an oval wholly inside the band on one side of the cutting plane: both extremes of its height in (0, escalate) leave the count at none on all three lines
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [a-spiric-plane-near-miss-shift-is-measured-in-the-ovals-plane-not-off-the-cutting-plane, 7571]
---


Filed from the review of PR 7571 (`germ/spiric-near-miss-off-q`). Not a
regression: main answers the same way.

## Finding

`geom_brep::curve_face::spiric_plane::spiric_plane_method` reads the
oval's count on `L = P ∩ Q` and on the two lines of the spiric's plane
`P` the band's escalation off the cutting plane `Q`, and refuses where
either differs. An oval that lies wholly on one side of `Q` with both
extremes of its height off `Q` in `(0, escalate)` meets none of the three
lines: all three counts are 0, and the door answers `Isolated` with no
meeting. The whole oval is within the band of `Q`, so that is a definite
miss the band cannot hold.

The pose needs the oval's width across `L` to be under `arm`, the radius
of the span's cover ball, and the tilt sine `s` between `escalate/arm`
(the parallel rung's edge) and `escalate/width`.

The review's example: torus `R = 5`, `r = 0.1`, the spiric at offset
4.85, span `(−3.1, 3.1)`, `s = 2.2e-8`, the oval's extremes at 0.02 and
0.46 of the escalation off `Q`.

Re-read on the PR's branch at the default band `(1e-9, 1e-8)`: the plane
with normal `(1, 0, s)` through `(4.85, 1, −0.1) − 0.02·escalate·n̂`
reads `Isolated` at `s = 2.3e-8`, `3e-8` and `4e-8`, with the oval's
heights sampled from `Curve3::eval` at 0.02 to 0.48, 0.62 and 0.82 of the
escalation. At `s = 2.2e-8` that construction refuses on the parallel
rung instead (its levered tilt reads 9.9e-9).

## Settles when

Every gap inside the band refuses, an oval wholly within the band on one
side of the cutting plane included, with a row holding that pose.
