---
id: point-in-solid-reads-out-inside-a-tilted-cut-cylinder-cavity
kind: issue
title: point_in_solid answers a false Out at hundreds of points inside a tilted-cut cylinder cavity (brick minus rod, cut at tilt 1.0), a cause outside the wall arm
status: open
opened: 2026-09-26
priority: P0
cost: D
refs: [CONTACT-3]
---


Found by a CONTACT-3 dual reviewer (review of `contact/3-wall-trim` at
`0901f45`). The answers are measured false; the cause is not yet traced.

**Fixture.** A brick with a cylindrical rod subtracted (a cylinder
cavity, reversed-sense wall), split by a plane at tilt 1.0. On the
lower half, `validate_geometric` passes and the volume is 16.0730, the
exact half.

**Measurement.** `point_in_solid` gives 298 false `Out` at the CONTACT-3
head and 353 at base, on a 9³/11³ grid clear of the boundary at 6 rigid
poses. For example, `(−1.422, 1.113, 0.128…1.628)` reads `Out` at the
identity pose. 72 of them occur at each of four poses.

**Not the wall arm.** Forcing CONTACT-3's `Unsupported` wall arm to
refuse every hit leaves the count unchanged at 298, and both cavity
walls are `Unsupported`. The reviewer's suspicion is the planar arm on
the cut face, which is bounded by lines and ellipse arcs. Check this
against CONTACT-4's move of `contfp` onto the carrier walk before
tracing further.

It is a wrong answer, not a refusal, hence P0.
