---
id: a-planar-face-through-a-cones-apex-refuses-the-boolean-where-the-split-builds
kind: issue
title: A Boolean whose planar face passes through a cone's apex refuses NormalAtConeApex where the split through the same plane builds
status: closed
branch: splitdoor/cone-apex-boolean
opened: 2026-10-29
priority: P1
cost: H
closed: 2026-11-02
pr: 6266
---

Found while measuring the cone witness of
`a-slit-tip-on-a-curved-face-or-at-a-cone-apex-reads-its-full-turn-as-a-spike`
(branch `splitdoor/slit-tip-readers`), on main `c295cb56` at ε 1e-9.

## What

The cone revolved about y from the triangle `(0, 0)`, `(√3, 1)`,
`(0, 1)` (half-angle 60°, apex at the origin), intersected with a slab
whose top face is a plane through the apex (normal `+z`, and
`(1, 0, 0.3)` normalized), refuses

```
NormalAtConeApex { operand: A, face: FaceKey(3v1), band: Band { zero: 1e-9, escalate: 1e-8 } }
```

where `split` by the same planes builds. The slit-to-apex cone of
`crates/topo/tests/a_slit_tip_at_a_cone_apex_reads_its_full_turn.rs`
refuses the same way against a slab under each of its three split
planes (`y = 0`, `(1, 0.5, 0)`, `(0.3, 1, −0.2)`) at 1e-6, 1e-9 and
1e-12, while each split builds at its closed form.

The split's walk reads a cone face at its apex as its rulings
(`sector_face::VertexFace::Apex`, `sector_shape::apex_shape`); the
Boolean's sector lane refuses the apex by name. The Boolean side of a
plane through an apex has no arm.

## Where to look

`boolean::sectors::orbit_corners` and its normal at the vertex: whether
the Boolean can read an apex sector through the split's
`apex_corner_shape` as it reads other corners through `corner_shape`.

## Decided (2026-11-01, designer pair, two rounds)

The pair converged. Round 1 split on scope. In round 2 each moved to the same position; neither moved to the other's earlier one. The final state of the code is one where every apex pose builds. **This row owns its first unit:**

- `BoolSector`'s `normal: OutwardNormal` becomes `at_vertex: VertexFace`, the shared face-at-a-vertex type. Every reader matches it.
- The pierce lane reads an apex corner against the pierced plane through the split's shared one-plane reading, `apex_corner_shape`, which is subdivided at the plane's extremal rulings. The six pierce-lane readers each say what they read at an apex:
  - entry sides;
  - coplanar tilt;
  - the germ: the section ruling, one trace with the split's `apex_germ`;
  - its bend: zero along a ruling;
  - `fate::bisector` / `BoolSector::middle`: the mid-azimuth ruling;
  - `tangent_lump`: `ConeApex::material_inside`.
- `build_sectors` takes its subdividing planes as a list, so the vertex–vertex unit can pass the partner's faces without a signature change.
- The refusal moves out of `boolean::sectors::sector_face`, which both lanes call, into the vertex–vertex pair search. It is renamed to say that the vertex–vertex pair algebra at a cone's apex is unbuilt, and it points at the owed row below.
- `NearAxis` stops being refused under the apex's name. It refuses under a name of its own, or reads as the split does (the needle row).
- A graze is a tangency between distinct carriers, under the Boolean's standing declaration rule.
- Pins use the one-wrap-edge form of Ev's PR 6014 ruling, with the apex as one full-turn corner. Measure first at 30°, 45° and 60°, plus the Euler-built notched cone, at three ε.
- Review tier: DUAL.

**Owed next:** `a-cone-with-its-apex-against-its-copy-refuses-normal-at-cone-apex`, widened to P1 / H. In the vertex–vertex lane, sectors become arcs of great or small circles on the direction sphere. One route: cut each apex sector at the union of its partner faces' extremal rulings, keeping bound-coded sectors. Fan × flat reuses the `plane_cone_apex` trace. Fan × fan is two small circles meeting, at most two points, in closed form. Its witnesses: the copy cone, a cone tip on a box edge and on a box corner, an hourglass union, and an apex on a sphere.

There is no `[ev]` PR. The designers agree, and no ratified text changes: the 2026-10-28 line this supersedes was orchestrator text, from commit e1890f2c. That follows the 2026-10-28 precedent. The record is on `analysis/design-fork/boolean-at-a-cone-apex`. Sequencing: built after `the-boolean-sectors-read-a-wrap-tip-as-a-one-edge-slit-tip`, by the same lane, because the full-turn apex corner is that row's form. (SPLITDOOR orchestrator)
