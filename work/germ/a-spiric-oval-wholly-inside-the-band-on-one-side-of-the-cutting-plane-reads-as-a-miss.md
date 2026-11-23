---
id: a-spiric-oval-wholly-inside-the-band-on-one-side-of-the-cutting-plane-reads-as-a-miss
kind: issue
title: The spiric × plane door answers a miss for an oval wholly inside the band on one side of the cutting plane: both extremes of its height in (0, escalate) leave the count at none on all three lines
status: closed
closed: 2026-11-23
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

## Progress (germ/spiric-oval-in-band, 2026-11-23)

The oval of the two-oval regime is convex (its curvature's sign is
`ρ·y² + offset²·r·cos v·sin² v`, positive throughout; the proof is in
the module doc), so a line of the spiric's plane meets it twice or not
at all. Where the count is none on all three lines, the door now reads
one point of the oval (any point serves; it reads the span's middle)
across `L` against `escalate/s` (`spiric_plane_oval_in_band`): between
`L` and a moved line, the whole oval lies within the escalation of the
cutting plane and the door refuses as a tangency; past both, the miss
stands.

The row `an_oval_wholly_inside_the_band_off_the_plane_refuses` sweeps
eight tori, three lean directions, tilt sines from 1.05× the parallel
rung's edge up, and stand-offs either side, read against the oval
written out in closed form over its whole turn (`Curve3::eval` checked
against it to 1e-12); the review's poses are pinned beside it. The pose
needs the oval's width across the line under its cover ball's radius,
which is the oval's aspect (about `√(R/r)` as `offset → R − r`), not
the tube's thickness: the five long ovals, from `r = 0.01` to `r = 1`
at `(5, 1, 3.99)`, hold it, and the two fattest tubes and the module's
own oval do not. On main such poses answered a miss in band; on the
branch none do. Past-band poses that refuse do so on the line × torus
ladder's own band (its wide band about the tube's top and bottom
circles), on main and the branch alike; the row excuses a refusal only
where the ladder itself refuses on one of the three lines.

## Closed

2026-11-23, by the GERM orchestrator. Closed by PR 7596 (merge `f1c2ac796f`). The oval is convex in the two-oval regime, so where all three counts are 0, one point read across L decides whether the oval lies between the lines (refuse) or past them, on the row `spiric_plane_oval_in_band`. The review checked the proof at 80 digits and found 0 in-band answers in 47,540 poses on 10 tori.
