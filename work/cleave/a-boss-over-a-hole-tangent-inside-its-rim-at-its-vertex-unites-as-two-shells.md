---
id: a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells
kind: issue
title: A boss over a hole or pocket tangent inside its rim, both vertices at the tangent point, unites as two shells sharing the crescent
status: open
opened: 2026-10-14
priority: P0
cost: M
---



Found by PR 5159's dual review (SECT's in-face one-site arm, taken by
PATHS), measured on its fix pass (branch
`paths/in-face-section-self-loop`) and on `main` at 26bdf137b1. Pinned
by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose`.

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
order they only touch, and `sectors.rs` `sector_overlap` (≈1816) reads
no overlap: no germ reaches the join, the section never sees the
crescent, and read at first order alone the union keeps the operands
whole (through: S2 F10 R2 E18 V12; pocket: S2 F11 R1 E18 V12) at the
closed-form volume, tier 2 and the certificate passing, tier 3′
failing. One order down the
sectors overlap: the hole's rim bends into the floor's side faster than
the boss's rim does, `(κ_b − κ_a)·û > 0` (`sectors.rs` `crescent`,
≈1915).

`union` now reads that second order at every vertex pair whose sectors
lie on two opposed plane faces and refuses `TangentCrescentUnsupported`
(`sectors.rs` `crescent_refusal`, ≈1856, called from `boolean/mod.rs`
after `pair_search`, ≈5891). ∩ is empty and both differences are
`SOUND`, as they were. Building the union is
`a-union-across-a-tangent-crescent-refuses-unbuilt`.

## The poses around it (every op both orders; default ε, 1e-6, 1e-12 alike)

| pose | one-segment | two arcs (vertices at az and az + π) |
|---|---|---|
| tangent inside, both vertices at the tangent point | ∪ `TangentCrescentUnsupported` (was two shells, wrong) | ∪ `TangentCrescentUnsupported` (was `UnpairedLooseEnds{4}`) |
| tangent inside, the boss's vertex at 1 or π | ∪ `TangentCrescentUnsupported` (was `UnpairedLooseEnds{2}`) | same (was `{4}`) |
| tangent inside, the hole's vertex at 1 or π | ∪ `TangentCrescentUnsupported` (was `UnpairedLooseEnds{2}`) | same (was `{4}`) |
| tangent inside, no vertex at the point | `CurvedPierceUnsupported` in every op | same |
| tangent outside, both vertices at the tangent point | ∪ `SectionLoopMixed` | `SOUND` |
| tangent outside, vertices elsewhere | `CurvedPierceUnsupported` in every op | `SOUND` |
| hole inside, not tangent | `SOUND` (pocket: two shells, the cavity) | `SOUND` |
| concentric, the boss's radius | ∪ two shells, tier 3′ `UnrecordedContact` on the rims | same |
| concentric, larger (`r = 0.4`) | `SOUND`, two shells | `SOUND`, two shells |

∩ and both differences are `EMPTY ok` and `SOUND` on every line that
does not refuse in every op. Through and pocket read alike. The equal
concentric hole's two solids are right, and their records name only
the vertex pair: `a-union-touching-along-a-coincident-rim-records-only-its-vertex-pair`.

## Why P0

Production does not run tier 3′ on a boolean's result:
`boolean/ops.rs` `BooleanBody` says the door gates at tier 3 and does
not run the census over its contacts, and tier 3 is a local battery
that two solids sharing a face area pass. Once PATHS unit 4
(`work/profile/circle-lowers-to-one-segment.md`) lowers `circle` to one
segment, every `circle()` puts its vertex at azimuth 0, so a boss over
a pocket tangent inside its rim on the `+x` side reaches this pose.
