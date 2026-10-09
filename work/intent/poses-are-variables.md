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

FORK-S3-1 was weighed with FORK-S3-6 as FORK-S3P (fork log row 95,
PR 4324), and this unit builds on that answer. No pose is free, none is
defined from nothing, and no construction reads one: there is no
`Seed`, no free `Frame`, no `BodyFrame`, no "document coordinates" and
no base. A `Profile` is 2-D shape whose numbers are read only against
each other. A straight extrude reads a profile and a depth; a slanted
one also reads a direction in the profile's own axes held to one side of
its plane (scalar slots on the node, normalised at evaluation); a
revolve reads a 2-D axis line. Every body is built in coordinates of its
own. A feature on a face is a construction placed against the face by a
bundle of mates (the face's frame or plane, an axis, a point), then
combined; the side its material lies on is the mates' sense, never a
sign on the extrude. Poses are read off geometry (`FaceFrame` over a
`Face` variable, a carrier's axis or centre), written `InFrame` a frame
read off geometry, constructed over poses, a pose of a copy (`OfCopy`),
or an operation's output (a revolve's axis), and only placements read
them. The world defines no pose variable. Spaces are kinds decided from
the recipe (a body's root, the product's, a loose copy's), a
placement's copy taking its targets' root once pinned, so a read across
two spaces is a kind mismatch at the door and an evaluator assert
elsewhere. A copy is pinned when its bundle's residual lies within its
construction's stated symmetry (a revolve's axis, an extruded circle),
never a measured one; this needs `Point` and edge-line mate kinds (the
stage 3 spec's Q5). Migration: every datum a profile read becomes a
placement of the body it built (one mate from the frame it sat on), its
bits moving by one composition, which the migration reports; the
façade's "sketch on a face" writes profile, construction, placement and
combine as one gesture.
