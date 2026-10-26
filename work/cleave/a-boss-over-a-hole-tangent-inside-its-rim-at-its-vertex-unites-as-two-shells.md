---
id: a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells
kind: issue
title: A boss over a hole or pocket tangent inside its rim, both vertices at the tangent point, unites as two shells sharing the crescent
status: closed
opened: 2026-10-14
closed: 2026-10-26
priority: P0
cost: M
---

Found by PR 5159's dual review (SECT's in-face one-site arm, taken by
PATHS), measured on its fix pass (branch
`paths/in-face-section-self-loop`) and on `main` at 26bdf137b1.

## The pose

The boss is the one-segment cylinder `r = 0.3`, `z ∈ [0, 1]`, its
vertex at azimuth 0. The plate is `[−1, 1]² × [−0.5, 0]`, holed by the
one-segment circle `r = 0.1` about `(0.2, 0)`, its vertex at azimuth 0,
through (`z ∈ [−0.5, 0]`) or as a pocket (`z ∈ [−0.25, 0]`, cut by
`subtract`). The two rims are tangent inside each other at
`(0.3, 0, 0)`, where both vertices lie. The boss's floor and the
plate's top face lie on each other, normals opposed, over the crescent
of the floor the hole does not take; the true union glues them there
into one shell (through) or one shell and a cavity touching it at the
point (pocket).

## The cause

At the vertex pair the floor's sector and the top face's sector share
the rims' common tangent and lie on opposite sides of it, so at first
order they only touch: `sectors.rs` `pair_search` reads no overlap, no
germ reaches the join, and the section never sees the crescent. Read at
first order alone, the union keeps the operands whole (through: S2 F10
R2 E18 V12; pocket: S2 F11 R1 E18 V12) at the closed-form volume, tier 2
and the certificate passing and tier 3′ failing, and the door, which
does not run tier 3′, ships it. One order down the sectors overlap: the
hole's rim bends into the floor's side faster than the boss's rim does,
`(κ_b − κ_a)·û > 0`.

## Closed

2026-10-26, PR 5687. `pair_search` hands the coplanar pairs it reads as
touching to `sectors.rs` `crescent_refusal`, which under union reads
the second order (`crescent`) on two opposed plane faces and refuses
`TangentCrescentUnsupported`. ∩ and both differences build. The pose
table, every op both orders through and pocket, one segment and two
arcs, at the default ε, 1e-6 and 1e-12, is
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose`. What stays open:

- the union itself:
  `a-union-across-a-tangent-crescent-refuses-unbuilt` (P1);
- a refusal of a union `main` builds, where short arcs either side of
  the tip carry the section:
  `a-crescent-refusal-fires-where-short-arcs-carry-the-section` (P1);
- the one-segment forms that refuse where the two-arc forms build:
  `one-segment-tangent-rims-refuse-where-their-two-arc-forms-build` (P1);
- the concentric hole of the boss's radius, a right body whose records
  name only the vertex pair:
  `a-union-touching-along-a-coincident-rim-records-only-its-vertex-pair` (P2).
