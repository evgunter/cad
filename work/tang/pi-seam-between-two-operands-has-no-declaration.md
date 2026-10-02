---
id: pi-seam-between-two-operands-has-no-declaration
kind: issue
title: A G1 (wedge-π) seam between two operands' distinct carriers has no declaration class: Tangent is opposed-only, Continuation same-carrier-only, so a capped tube or a torus chain cannot be unioned
status: open
opened: 2026-10-02
priority: P0
cost: H
design: true
needs_ev: true
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
