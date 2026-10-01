---
id: an-arc-tangent-to-a-face-at-its-end-is-split-at-the-edge-of-the-band
kind: issue
title: "boolean: an arc tangent to the other operand's face at its end vertex is split where it leaves the zero band, sqrt(2·eps·r) from the tangency, and the sliver collapses the sector arm the tangent descent reads"
status: open
opened: 2026-10-01
priority: P3
cost: M
---

## Finding

Measured on `crates/sweep/tests/m9_3_zip.rs`'s
`a_tangent_curved_sector_on_a_face_lumps_whole`: the upper quarter round
(radius 1, its arc tangent to z = 1 at x = 2, the arc's own end vertex)
resting on a slab whose top face contains that vertex, the wall declared
`Tangent` to the slab's top.

During classification the piercing operand holds a vertex at
x = 2.0000000210734243 on the arc, which the operand as built does not
have (its vertices on z = 1 are at x = 0.5 and x = 2 exactly). The
offset is `2.1073424e-8 = sqrt(2 · 2.22e-16 · 1)`: the point where the
arc's height above the plane, `dx²/2r`, leaves a zero band of
`2.22e-16`. The reduction split the arc where it stops reading ON the
plane, leaving a 2.1e-8 m sliver between that vertex and the tangency.

Two consequences, both observed through a temporary probe in
`vtxfac::classify_vertex_on_face`'s declared-`Tangent` branch:

1. **The sector arm collapses to the sliver.** The wall sector at the
   tangency vertex has `arm = 2.107e-8`, so the second-order margin
   `½·κ·arm²` is the band itself and `tangent_sector_order2` decides
   Zero: the whole-sector lump (`sectors::tangent_lump`) falls to its
   exact-zero arm, the Eq. 15.3 ⁻ posture, instead of the side the wall
   actually curves to (away from the slab, `Out`, at every op). The
   answers the test pins are right (difference = slab, intersection
   empty), so the posture is not wrong here — but it is the posture
   by construction, not by measurement, on every arc tangent at its end.
2. **The sliver is a sliver edge** wherever the piece survives into a
   result (here the union stops at the rest zip's frontier first).

**What would close it.** Decide whether a declared-`Tangent` pair's
in-band approach should mint a crossing at all (the tangency is the
contact; the band's edge is not a crossing), and either suppress the
split under the declaration or take the sector arm from the unsplit
edge. A row that asserts no vertex lands within `sqrt(2·ε·r)` of a
declared tangency, and that the lump reads the wall's own side.

Filed from the TANG m9-3 residues unit (item 1's measurement).
