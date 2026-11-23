---
id: pi-seam-between-two-operands-has-no-declaration
kind: issue
title: Build BooleanCoincidence::Seam: a declared G1 seam between two operands' distinct carriers (ruled, PR 3756)
status: closed
opened: 2026-10-02
priority: P0
cost: H
closed: 2026-10-02
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

## Closed (2026-10-02, TANG, branch `tang/pi-seam`)

`BooleanCoincidence::Seam` is built as ruled:
- It is declared on a boolean node only.
- It is verified by `verify_seam_declaration`: the conformal screen,
  then the `Tangent` witness lane (`tangent_locus_relation`) along the
  DEV-1 line or the shared rim circle with B's sense bit reversed, then
  the material wedge.
- It is refused as `SeamContradicted`, and a seam with no locus as
  `UnsupportedDeclarationClass { class: Seam }`.
- A verified seam is a one-sided cover source where its kinds certify a
  global side (`seam_certifies_side`). The covered circle arm accepts an
  end definitely off on either side, and refuses two ends on opposite
  sides.
- A `Tangent` claim on a wedge-π rim is contradicted
  (`contact_tangent_rim_seam`) and steered to `Seam`.
  `RimSeamNotDeclarable` is retired.

Outcomes:
- The sphere-capped tube builds with its walls declared `Seam`, in both
  orders: the tube and the half ball exactly, census (5, 8, 5, 1), and
  the rim minted `TangentIntersection`. That is the same body as the
  capsule revolved from one profile with its joint authored, which
  needs no declaration.
- Undeclared, it refuses at the crossing layer, naming the recourse.
  Declared `Tangent`, `Rest` or `Continuation`, each is contradicted
  under its own type.
- The transverse dome declared `Seam` is contradicted
  (`contact_tangent_parallel`), as is the kissing torus pair
  (`seam_senses_aligned`).
- The lily's stem chain verifies as a seam and stops at the crossing
  layer, on two pieces filed as
  `a-torus-seam-graze-needs-the-rim-root-deflated` and
  `a-torus-meridian-lying-on-a-torus-is-unsettled`.

Also filed:
- `a-declared-line-seam-stops-at-the-rest-zip`;
- `a-declared-seam-subtract-and-intersect-stop-at-the-fallback-extent`;
- bind's `python-has-no-door-to-declare-a-seam`.
