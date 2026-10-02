---
id: pi-seam-between-two-operands-has-no-declaration
kind: issue
title: Build BooleanCoincidence::Seam: a declared G1 seam between two operands' distinct carriers (ruled, PR 3756)
status: open
opened: 2026-10-02
priority: P0
cost: H
---


## What

At a G1 rim between two operands on distinct carriers (a hemisphere
on a tube, the lily's torus chain), the edges leaving the rim graze the
partner wall tangentially: the tube's rulings against the sphere, and
the cap's meridian against the tube. C4 says a graze is reached only
through structure or a declaration. None of today's declarations fits:
`Tangent` requires opposed senses, and `Continuation` requires one
carrier. MATE-7's "no declaration needed" for wedge π cannot stand
beside that rule. Measured by PR 3746: every declaration is refused,
each under a different name (`RimSeamNotDeclarable`,
`ContactContradicted`, `UnsupportedDeclarationClass`).

## The question for Ev

Both designers recommend `BooleanCoincidence::Seam`: the aligned-sense
twin of `Tangent`, as `Continuation` is of `Rest`. It is declared on a
boolean node only, verified along the locus with the sense bit
reversed, and zipped as the smooth seam. The alternative is a
sense-agnostic `Tangent` at the boolean seat. The text is on the
`[ev]` PR (branch `tang/ev-rim-licence`). A G1 joint authored inside one
profile stays the structural form and needs no declaration.
`torus-declared-rest-lane-banked` item 3 and the lily's stem point
here.

## Ruled (Ev, PR 3756, 2026-10-02)

Build `BooleanCoincidence::Seam` as C4 now states it: declared on a
boolean node only; verified by the `Tangent` witness lane along the
locus with the sense bit reversed; opposed senses contradict it; it is
routed by the material wedge and zipped as the smooth seam carrying
`TangentIntersection`. It is a cover source in C4's one-sided-cover
list.
