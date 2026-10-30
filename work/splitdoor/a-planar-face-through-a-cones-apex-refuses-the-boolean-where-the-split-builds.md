---
id: a-planar-face-through-a-cones-apex-refuses-the-boolean-where-the-split-builds
kind: issue
title: A Boolean whose planar face passes through a cone's apex refuses NormalAtConeApex where the split through the same plane builds
status: dispatched
branch: splitdoor/cone-apex-boolean
opened: 2026-10-29
priority: P1
cost: H
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
