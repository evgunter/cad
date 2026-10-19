---
id: sliver-gate-refusal-typing-for-an-unclaimed-wall-has-no-test
kind: issue
title: gate_sliver_orientation's typing of a wall face with no reach (WallOutlineUnsupported / PartialConeFace) has no test: it needs an at-rest sliver operand bounded by a spiric or spline edge
status: open
opened: 2026-10-18
priority: P3
cost: M
---

## What

PR 5261 made `census::face_reach` answer `None` for a cylinder or cone
face with a boundary edge that has no axial claim (no sound box, or a
spiric). `crates/topo/src/boolean/reduce.rs` `gate_sliver_orientation`
then types that refusal as `PointInSolidError::WallOutlineUnsupported`
(cylinder) or `PartialConeFace` (cone), through
`census::boundary_unclaimable`, instead of `CorruptFace`. Nothing pins
that typing.

## The fixture it needs

An operand that passes the at-rest gate as a sliver (`AtRestBody::slivers`),
or one with no verdict (a dual's), holding a cylinder or cone wall whose
boundary includes an edge on a spiric carrier (a plane's section of a
torus seated on the wall) or a spline edge. Then any boolean against
it should refuse `BooleanError::Containment(WallOutlineUnsupported { face })`
or `PartialConeFace`. A dual-scalar operand may be the cheaper route,
since every one of its solids is read.
