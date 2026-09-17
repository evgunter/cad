---
id: rotation-about-re-divides-a-decided-axis
kind: issue
title: Mat3::rotation_about normalizes its axis internally, so a caller holding the unit witness pays the divide twice — a witness-taking rotation constructor is the next member
status: open
opened: 2026-09-15
---



## Where this came from

The fix pass of `unit-vector-witness-in-geom-core` (SCALAR):
`editor-core`'s `eval/wire.rs` `unit()` now mints
`geom_core::UnitVec3<T>` — the transform axis and the pattern
direction are unit as a property of the type — and `transform_map`
takes the witness. It then calls `Mat3::rotation_about(axis.get(),
angle)`, which divides the axis by its own norm a second time.

## The site

`crates/geom-core/src/linalg/mat.rs`, `Mat3::rotation_about` (`:135`):
`let n = axis.normalize();` on a bare `Vec3<T>`, then Rodrigues on
`n`. The callers in this tree that already hold a decided axis:
`editor-core/src/eval/wire.rs` `transform_map` (the witness, `.get()`
at the call), `editor-core/src/placement.rs` `rotate_then_translate`
(the same), and `Affine3::rotation_about_axis` (`affine.rs`), which
`stepped_rule_map` reaches with the datum's witness. On a unit input
the second divide changes no bit the format holds exactly, and every
consumer's bits are pinned unchanged by the unit that filed this — so
this is not a defect in the output. It is a decision made once and a
divide paid twice, with the type that could carry the decision
stopping at the door.

## The next member

A rotation constructor that takes `UnitVec3<T>` — the existing door
re-typed, or a sibling beside it — with the bare-`Vec3` door kept
only for callers that make no decision.
`work/props/normalize-overflow-yields-zero-axis.md` is that bare
door's own row: an overflowed axis normalizes to zero and the rotation
collapses to a scaling, which is exactly the case the witness
constructor refuses before the divide, so the two rows close together.
`Affine3::rotation_about_axis` follows the same shape. Not changed by
the filing unit: `Mat3` is PROPS' ground and the bits argument (no
change on a unit input) is what makes the second divide harmless
today, so which door retires is this program's decision, not a
one-line take.
