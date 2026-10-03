---
id: a-bridge-union-fuses-a-declared-tangent-rest-into-one-shell-with-an-edgeless-contact
kind: issue
title: A union bridging a declared plane-to-cylinder Tangent rest fuses its two shells into one whose wall touches the face interior along a line no edge carries, and both at-rest gates pass it
status: open
opened: 2026-10-03
priority: P1
cost: M
refs: [tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line, boolean-door-adopts-the-finished-body-type]
---



## What

A rod (r = 0.5, z ∈ [0.25, 0.75], axis at x = 0, y = 2.5) rests on the
side face y = 2 of a 4 × 4 × 1 plate. `union_with` declaring the
wall × face pairs `Tangent` answers what the DEV-1 lane
(`verify_tangent_declaration` in `crates/topo/src/boolean/mod.rs`)
admits: one solid, two shells touching along the ruling x = 0, y = 2.
Its `contacts` hold two vertex-on-face rows, the rod's cap-rim vertices
(0, 2, 0.25) and (0, 2, 0.75) on the plate face, and `curves: []`.

A second union with a box bridging the rod's top to the plate's top
(x ∈ [−1, 1], y ∈ [1, 2.5], z ∈ [0.6, 1.6]) then answers ONE shell,
volume 18.2 + 0.85·π/8 = 18.5337942 (true). The rod wall still lies on
y = 2 along z ∈ [0.25, 0.6], and no edge runs along that ruling. The
material meets itself along a line: the door should refuse.

- **Plain** (`union`, no declarations): the result's `contacts` are
  empty, so by `BooleanBody`'s contract (`crates/topo/src/boolean/ops.rs`)
  it is tier-3 currency, and `validate_geometric` passes it.
  `validate_pseudomanifold` with no records refuses `UndeclaredContact
  { VertexOnFace }` at (0, 2, 0.25): the census's vertex × face pass
  (`sweep_vertex_face` in `crates/topo/src/census.rs`) sees the rim
  vertex, not the line.
- **Carried** (`union_with`, the first result's two records carried as
  `CarriedVf { class: Tangent }`; `Rest` does the same): one record
  survives, and `validate_pseudomanifold(&body, &contacts)` passes. The
  body passes every at-rest gate.

Measured identically at `Interval`. Reproducers:
`crates/sweep/tests/wall_face_tangent_reach.rs`, both rows `#[ignore]`d
and pinning today's answer.

## Where the gap is

- The second union fuses two shells across a declared touch and keeps
  the touch. Nothing at the door asks whether the contact the operand
  carries has become a contact inside one shell.
- The record that lets the census pass is one vertex-on-face row. The
  contact it stands for is a line, and the census has no same-solid
  face × face arm for a curved wall on a planar face. The
  cross-solid backstop (`sweep_cross_solid_backstop`) is the only arm
  that pairs faces, and it pairs faces of different solids.
- The finished-body gate (`work/reach/boolean-door-adopts-the-finished-body-type.md`)
  would refuse the plain variant (its result carries no records) and
  pass the carried one.

## Related

The tier-3 half is
`work/cleave/tier-3-passes-a-curved-wall-touching-a-plane-face-interior-along-a-line.md`.

Also measured on the way: the same chain with the bridge reaching
y = 3.5 (covering the rod's whole section at z = 0.6) answers a body
tier 3 refuses, `RingMeetsOuter` on the bridge's bottom face (the rod's
section circle there is a ring tangent to the outer loop at the
contact's end point). The door ships it because it gates tiers 1–2
only, which is what the finished-body item closes.
