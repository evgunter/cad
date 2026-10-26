---
id: one-segment-tangent-rims-refuse-where-their-two-arc-forms-build
kind: issue
title: A boss over a hole whose rims are tangent refuses in its one-segment form where the two-arc form builds
status: dispatched
opened: 2026-10-26
priority: P1
cost: H
branch: cleave/one-segment-tangent-rims
---


Found measuring
`a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`.
Pinned by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose`, at the default ε, 1e-6 and 1e-12, alike.

The boss `r = 0.3`, `z ∈ [0, 1]`, over the plate `[−1, 1]² × [−0.5, 0]`
holed through or pocketed by a circle `r = 0.1`. Each rim is a
one-segment circle, or two arcs with vertices at the given azimuth and
opposite. Where the two-arc form builds `SOUND` in every op, the
one-segment form refuses:

| pose | one segment | two arcs |
|---|---|---|
| tangent outside (hole about `(0.4, 0)`), both vertices at the tangent point | ∪ `Join(SectionLoopMixed)`; ∩ and both differences build | every op `SOUND` |
| tangent outside, the boss's vertex at 1 | `CurvedPierceUnsupported` in every op | every op `SOUND` |

With no vertex at the tangent point at all (the boss's at 1, the hole's
at 2, tangent inside), every op refuses `CurvedPierceUnsupported` in both
forms: the crossing layer reads the tangency of a rim with the other's
wall.

Once PATHS unit 4 (`work/profile/circle-lowers-to-one-segment.md`)
lowers `circle` to one segment, every `circle()` puts its vertex at
azimuth 0, so these are the forms `circle()` authors.
