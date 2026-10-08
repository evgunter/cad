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
needs_ev: true
---

INTENT stage 3, PR A. Spec: `docs/INTENT-STAGE3-SPEC.md` §2.

`VarDef::Pose` defines pose variables: a space's seed (FORK-S3-1), coordinates `InFrame`, `Offset`, the projections, `Through`, and `FaceFrame` over a `Face` variable. Pose definitions are scheduled mid-evaluation (`eval_pose` replaces `wire_datum`, `eval/wire.rs:1333`). `Node::Datum` (`node.rs:877`) retires. `AxisInPlane` becomes `Revolve`'s 2-D slots (FORK-1b). Every reader reads a slot of its kind: profile plane, split tool, pattern axis and direction, the tube frame, and explicit frames, which closes `explicit-placement-frames-hold-floats`. `PoseSymmetry` moves onto the pose value, so one `Subgroup` serves both (#4222).

The migration keeps every datum's scalar ids, and geometry is bit-equal. It waits on stage 2 E because `FaceFrame` reads a `Face` variable. FORK-S3-1 goes to a designer pair before dispatch.

FORK-S3-1 was weighed with FORK-S3-6 as FORK-S3P (fork log row 95) and
went to Ev in an `[ev]` PR; this unit builds on the answer
provisionally. After Ev's comment ("frames only enter at placement, so
there's no need to specifically have blank ones"), no pose is free.
A part's construction is written in the document's own coordinates,
which no variable stands for: a datum is coordinates over scalar
variables in them (`InFrame` reads a frame only when it is relative to
one), so the migration of absolute datums is none. There is no `Seed`, no
free `Frame` and no `OfCopy`. A pose of a copy is a `FaceFrame` over the
copy's `Body` variable, or `BodyFrame { body }`, the body's construction
coordinates as a `Frame` pose (today's `FrameBase::Part`). The world is
one undeletable frame defined in the document's coordinates, read only by
placements' mates and export. A space is a set of copies; reads that
reach copies lie in one space. `InFrame` of kind `Frame` and `Offset` stay
two spellings, and that is this unit's call.
