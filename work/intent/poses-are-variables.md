---
id: poses-are-variables
kind: issue
title: D10 stage 3 PR A: a pose is a variable of its kind; the Datum node retires into pose definitions over a space's seed, the revolve's axis moves onto the node, explicit pattern frames become Frame variables
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [select-defines-face-and-edge-variables]
refs: [explicit-placement-frames-hold-floats, intent-stage3-is-built]
---

INTENT stage 3, PR A. Spec: `docs/INTENT-STAGE3-SPEC.md` §2.

`VarDef::Pose` defines pose variables: a space's seed (FORK-S3-1), coordinates `InFrame`, `Offset`, the projections, `Through`, and `FaceFrame` over a `Face` variable. Pose definitions are scheduled mid-evaluation (`eval_pose` replaces `wire_datum`, `eval/wire.rs:1333`). `Node::Datum` (`node.rs:877`) retires. `AxisInPlane` becomes `Revolve`'s 2-D slots (FORK-1b). Every reader reads a slot of its kind: profile plane, split tool, pattern axis and direction, the tube frame, and explicit frames, which closes `explicit-placement-frames-hold-floats`. `PoseSymmetry` moves onto the pose value, so one `Subgroup` serves both (#4222).

The migration keeps every datum's scalar ids, and geometry is bit-equal. It waits on stage 2 E because `FaceFrame` reads a `Face` variable. FORK-S3-1 goes to a designer pair before dispatch.
