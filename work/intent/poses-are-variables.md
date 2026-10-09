---
id: poses-are-variables
kind: issue
title: D10 stage 3 PR A: a pose is a defined variable of its kind, read off geometry or constructed; one Subgroup; the revolve's axis is a 2-D line on the node
status: parked
opened: 2026-10-08
priority: P0
cost: H
blocked_on: [select-defines-face-and-edge-variables]
refs: [explicit-placement-frames-hold-floats, intent-stage3-is-built]
---

INTENT stage 3, PR A. Spec: `docs/INTENT-STAGE3-SPEC.md` §2. Built on FORK-S3P (fork log row 95, PR 4324).

`VarDef::Pose` defines a pose variable, and no pose is free or defined from nothing:

- read off geometry: a face reads as a plane, and an edge or carrier gives an axis or centre; no definition reads a carrier's reference direction;
- written `InFrame` by scalar coordinates in a constructed frame;
- constructed: `Through`, `Meet`, `Flip`, `Standoff`;
- projected;
- an operation's output.

Pose definitions are bound mid-evaluation by `eval_pose`, which replaces `wire_datum`. `AxisInPlane` becomes `Revolve`'s 2-D axis line with an `axis: Axis` output (FORK-1b). `FaceFrame` retires: its readers read the face's plane, and a sketch on a face becomes a placement in D. `Split`'s tool reads a `Plane` of its operand's root. `PoseSymmetry` moves onto `PoseValue`, with `Point` and `Direction` arms, so one `Subgroup` serves poses and mates (#4222). Geometry is bit-equal.

Absolute datums survive A as the operations that define the poses constructions read. D retires them, once constructions read no frame and a placement can say where a body is.

Waits on stage 2 E (a face's plane reads a `Face` variable). Independent of B.
