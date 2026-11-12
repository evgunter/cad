---
id: a-carried-plane-spline-chords-at-rest-certificate-costs-half-a-second
kind: issue
title: A carried plane x spline chord's at-rest certificate costs about half a second, and the spline census tripled
status: open
opened: 2026-11-12
priority: P2
cost: M
---

Filed by the `sectarms/join-reads-spline-implicit` lane's fix pass.

## Measured

Debug build, one run each, timers around the pipeline's stages
(`ops::through_the_join`'s glue, reduction, guard and connect; the
section table's classification and whole reading; the Euler op's
`certify_stated` call), on a box shared with other builds:

- **A brick across the loft prism's flat wall, wholly inside the
  wall's face** (`x ∈ [−0.25, 0.25]`, `y, z ∈ [0.75, 1.25]`), union: 7.30 s. The
  four pairs' classifications and whole readings take 0.56 s; the
  reduction 0.94 s in all; the join's 12 chord edges each pay the
  at-rest certificate (`policy_lane::certify_stated`, the plane × NURBS
  lane) 0.49 to 0.65 s, 6.4 s in all. It then refuses at the ring lane
  (`a-ring-on-a-spline-wall-closed-by-carried-chords-has-no-winding`).
  On main it refused at the germ frame in about 0.4 s.
- **A brick across the flat wall at its rim**
  (`z ∈ [−0.25, 0.25]`): 7.5 s. Three whole readings take 0.38 s, the
  reduction 0.65 s, and 12 at-rest certificates 0.48 to 0.68 s each,
  6.6 s in all; it refuses at containment.
- **A ball of radius 0.25 across the flat wall**: the one whole reading
  takes 7.0 s, of which the pad (`spline_implicit::pad_of`) 1.8 s; it
  refuses in the reduction.
- **The nightly census** `spline_subtract_intersect_census` took 6 min
  34 s on main and 17 min 31 s on the branch (debug, local, one run
  each; the main run shared the box with a build): the runs that now
  reach the join pay the whole reading and the chord certificates.

So the cost of a run that reaches the join is the at-rest certificate
of each carried chord, not the whole reading. The chord's carrier is
the whole reading's own certified fit; the plane × NURBS lane
re-derives its certificate from scratch at rest.

## Done when

A carried plane × spline chord certifies at rest in a time comparable
to the whole reading's per-branch cost, or the census returns within
a factor of two of main's.
