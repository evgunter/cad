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
   result (here the union stops first: at the rest zip's frontier at the
   witness ε, at a classification invariant at 1e-6, below).

**What would close it.** Decide whether a declared-`Tangent` pair's
in-band approach should mint a crossing at all (the tangency is the
contact; the band's edge is not a crossing), and either suppress the
split under the declaration or take the sector arm from the unsplit
edge. A row that asserts no vertex lands within `sqrt(2·ε·r)` of a
declared tangency, and that the lump reads the wall's own side.

**At `ε = 1e-6` the same fixture's UNION refuses as a kernel invariant.**
`CAD_TOLERANCE_EPS=1e-6`: the difference and the intersection still
answer right, but the union refuses `ClassificationInvariant { what:
"pierce transition on a coplanar sector" }` from
`vtxfac::pierce_germ_dir` (measured on PR 3747's head; at the witness ε
it reaches the rest zip's `ChordBetweenIsolatedPierces` instead). The
union's whole-sector lump takes the Eq. 15.3 ⁻ posture (`In`, see 1.
above) beside an `Out` sector, so an out-run's boundary lands on the
lumped wall sector, which is coplanar with the slab's top by
construction and has no germ line. Either the lump should read the
wall's own side (it would, with the split fixed), or a run boundary on
a lumped tangent sector needs its germ from the locus rather than from
the sector normal. The test stopped asserting the union for this reason.

**Who waits on it.** `m9_3_zip::a_tangent_curved_sector_on_a_face_lumps_whole`
tells `vtxfac`'s whole-sector lump from a per-bound reading only through
this defect, and
`work/tang/vtxfac-tangent-sector-should-descend-per-bound.md` is parked
on it: once it is fixed, re-run that test with `vtxfac` reading per
bound.

Filed from the TANG m9-3 residues unit (item 1's measurement).
