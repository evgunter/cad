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
PR 4324), and this unit builds on that answer. No pose is free and none
is defined from nothing: there is no `Seed`, no free `Frame`, no
`BodyFrame` and no "document coordinates". A `Profile` is 2-D shape with
no frame; a construction reads it through a frame it supplies, or
through none, and the one that reads none is the document's base
(`Doc.base`, one per document), built in coordinates of its own that
nothing reads; every other construction's frame slot is required. Every other pose is read off geometry
(`FaceFrame` over a `Face` variable, a carrier's axis or centre), is
coordinates over scalar variables `InFrame` a frame it reads, is a
construction over poses, is a pose of a copy (`OfCopy`), or is an
operation's output (a revolve's axis). The world defines no pose
variable: only a bundle mate reads it. Spaces are kinds decided from the
recipe (the base's `Body`, the product's, a loose copy's), a placement's
copy taking its targets' kind, so a read across two spaces is a kind
mismatch at the door and an evaluator assert elsewhere, never a
refusal an author can reach. Migration: a
document's first profile datum folds into its body's own coordinates,
and every later absolute datum is rewritten `InFrame` over a face frame
of that body, its bits moving by one composition, which the migration
reports. The façade's datum builders take a frame and default to none.
`InFrame` of kind `Frame` and `Offset` stay two spellings, and that is
this unit's call.
