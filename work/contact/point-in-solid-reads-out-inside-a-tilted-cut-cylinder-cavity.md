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
`0901f45`). Traced by CONTACT-6.

**Fixture.** A brick with a cylindrical rod subtracted (a cylinder
cavity, reversed-sense wall), split by a plane at tilt 1.0. The lower
half's volume is 16.0730, the exact half. `validate_geometric` does NOT
pass on it: it reports `LoopRoleInverted` on one of the two section
faces.

**Measurement.** `point_in_solid` gives 298 false `Out` at the CONTACT-3
head and 353 at base, on a 9³/11³ grid clear of the boundary at 6 rigid
poses. For example, `(−1.422, 1.113, 0.128…1.628)` reads `Out` at the
identity pose. 72 of them occur at each of four poses. CONTACT-6's grid
(brick `[−2, 2]² × [0, 2.5]`, rod `r = 1`) measures 1274 false `Out` at
the base and at CONTACT-4's head alike.

**Not the wall arm.** Forcing CONTACT-3's `Unsupported` wall arm to
refuse every hit leaves the count unchanged at 298.

**Cause.** The section face on the `+y` side of the bore carries
`sense: false` on a chart whose normal already points out of material:
its loop winds counter-clockwise about that normal. So the ray lane
reads every crossing of it as an entry. The bit came from the split's
promotion (`splitting::finish::split_finish`). The null face was minted
on the reversed cavity wall and inherited that wall's `false`, and the
re-chart onto the section plane kept it. The volume is right because
it is integrated from the loop windings, not the bit.

It is a wrong answer, not a refusal, hence P0.
