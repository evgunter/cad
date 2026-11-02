---
id: a-direction-mate-on-a-curved-face-reads-its-plane
kind: issue
title: a direction mate picked on a cylinder's face reads its plane, which refuses, not its axis
status: closed
closed: 2026-11-02
opened: 2026-11-02
priority: P3
cost: E
---

## Finding

Found by `placing/mate-kinds`. The mate tool reads a `Direction` off a
pick through the first of two readings whose seat admits the entity
(`matetool::MateReading::of`, `crates/viewer/src/matetool.rs`): a
face's plane through its `Normal` role, then an edge's axis through its
`Direction` role. A face therefore always reads as its plane, so a
direction picked on a cylinder's, a cone's or a torus's face refuses
where the placement evaluates (the plane read's non-planar refusal),
though its carrier's axis has a direction the kernel would read
(`PoseDef::Axis { of: face }` admits a face, `crates/editor-core/src/pose.rs`).

Which reading a face takes is a fact of its surface, which the tool
does not read before the commit (the module's rule: whether a face has
the plane or the axis is the placement's refusal, never a pre-check).

**Done when** a direction picked on a cylinder's face reads its axis's
direction, chosen by something the tool may read (the surface kind the
pick index already holds, or a second choice on the panel), and a row
mates a peg's side parallel to an edge.

## Closed

A face picked for a direction reads its carrier's axis where its carrier
is no plane: `MateReading::at_site` (`crates/viewer/src/matetool.rs`)
asks the kernel's tag read `face_carrier_kind` at the pick's site and,
for a face that is not planar, takes the first admitted reading that is
not its plane. A tag that does not read keeps the plane reading, and the
placement states its refusal. Row:
`mate_kinds::a_cylinder_face_mated_as_a_direction_runs_parallel`, a
peg's side mated parallel to a block's edge.
