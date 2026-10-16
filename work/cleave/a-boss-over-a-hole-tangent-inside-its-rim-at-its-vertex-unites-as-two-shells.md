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
`paths/in-face-section-self-loop`), and the same on `main` at
26bdf137b1. Pinned as today's behaviour by
`crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_tangent_inside_its_rim`.

## The wrong body

The boss is the one-segment cylinder `r = 0.3`, `z ∈ [0, 1]`, its
vertex at azimuth 0. The plate is `[−1, 1]² × [−0.5, 0]`, holed by the
one-segment circle `r = 0.1` about `(0.2, 0)`, its vertex at azimuth 0,
through (`z ∈ [−0.5, 0]`) or as a pocket (`z ∈ [−0.25, 0]`, cut by
`subtract`). The two rims are tangent inside each other at
`(0.3, 0, 0)`, where both vertices lie.

`union` in either order returns `Ok` with two shells: the boss and the
holed plate, unchanged (through: S2 F10 R2 E18 V12; pocket: S2 F11 R1
E18 V12). They share the crescent of the boss's floor the hole does not
take. The volume is the closed form, tier 2 and the certificate pass,
the mesh is valid, and tier 3′ fails `CensusUndecidable`. The only
contact recorded is the two vertices'. No germ reaches the join: the
section never sees the crescent. The true union is one shell. Measured
at the default ε, 1e-6 and 1e-12, alike.

`∩` is empty and both differences are `SOUND`, as they should be.

## The poses around it (default ε, every op both orders)

| pose | one-segment | two arcs (vertices at 0 and π) |
|---|---|---|
| tangent inside, both vertices at the tangent point | ∪ **two shells, wrong** | ∪ `UnpairedLooseEnds{4}` |
| tangent inside, the boss's vertex off it | ∪ `CurvedPierceUnsupported` (boss first), `UnpairedLooseEnds{2}` (plate first) | same refusals, `{4}` |
| tangent inside, the hole's vertex off it (az 1 or π) | ∪ `UnpairedLooseEnds{2}` (boss first), `CurvedPierceUnsupported` (plate first) | ∪ `UnpairedLooseEnds{4}`, or `CurvedPierceUnsupported` plate first at az 1 |
| tangent outside, both vertices at the tangent point | ∪ `SectionLoopMixed` | `SOUND` |
| tangent outside, vertices elsewhere | `CurvedPierceUnsupported` in every op | same |
| hole inside, not tangent | `SOUND` | `SOUND` |

So the two reviews' readings are both right: a tangent hole refuses
typed unless the hole sits inside the boss's rim with both vertices at
the tangent point, and there the one-segment form builds wrong. On
`main` the outside-tangent aligned pose refuses `SingleSiteSectionLoop`
instead of `SectionLoopMixed`, and the inside non-tangent pose refuses
`SingleSiteSectionLoop`; every other line is the same.

A boss over a concentric hole of the same radius (vertices at 0, or the
hole's at 1) or a larger one (`r = 0.4`) also unites as two shells, in
both forms, with tier 3′ failing `CensusUndecidable` on the curved pair.
The larger hole's two parts are disjoint, which is right; the census
cannot yet say so of a curved pair
(`work/restread/census-cross-solid-curved-pairs-undecidable-on-shell-results.md`).
The equal hole's two parts touch along the whole rim circle, but the
contacts record names only the vertex pair.

## Why P0

Production does not run tier 3′ on a boolean's result:
`boolean/ops.rs` `BooleanBody` says the door gates at tier 3 and does
not run the census over its contacts. Tier 3 is a local battery, and
two solids that share a face area pass it. `editor-core`'s product
gather runs the census across copies only, and keeps rather than raises
what it finds (`editor-core/src/product.rs` module docs). So this body
ships silently from `topo::union`.

Once PATHS unit 4 (`work/paths/circle-lowers-to-one-segment.md`) lowers
`circle` to one segment, every `circle()` puts its vertex at azimuth 0,
so a boss over a pocket tangent inside its rim on the `+x` side builds
this body.
