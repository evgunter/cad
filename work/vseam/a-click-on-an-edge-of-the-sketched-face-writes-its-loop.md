---
id: a-click-on-an-edge-of-the-sketched-face-writes-its-loop
kind: unit
title: Sketching on a face, a click on one of its edges writes that face's loop (FACELOOP U3, the viewer's half)
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: a-faces-loop-or-closed-trace-cannot-become-a-profile-loop
blocked_on: [a-profile-on-a-face-holds-that-faces-outline-or-a-hole]
---


The viewer's half of the face-loop build (FACELOOP,
`work/faceloop/plan.md`), filed on VSEAM because it is a gesture the
viewer holds on behalf of the document, on VSEAM's ground. It waits on
FACELOOP's U2, `a-profile-on-a-face-holds-that-faces-outline-or-a-hole`,
which lands the document form this click writes.

## The ruled text

`docs/DESIGN.md` `[variables.kinds]` (PR 7284), and the ruling's
gesture, from the parent row
(`work/faceloop/a-faces-loop-or-closed-trace-cannot-become-a-profile-loop.md`,
"Weighed"): sketching on a face, a click on one of its edges writes
that face's loop. A click on an edge of the outer loop writes `Outer`;
a click on an edge of a ring writes `Through(that edge)`. A rim click
never offers both a face loop and a trace (B's final text). Latching
another face to trace, today's gesture, is unchanged.

## Done when

- Ev's elbow by the viewer: **New profile on a face** on the pipe's end
  cap, click the outer rim, click the inner rim, Revolve. The profile
  holds two loops and no number. Whether the revolve then builds is
  LATHE's row for one heading (below), and joining it to the pipe is
  HELDRECORDS's P0.
- A second click on an edge of a loop the profile already holds writes
  nothing and says so.
- A click on an edge of another face (not the one sketched on) is not
  this gesture; a click on an edge whose carrier the profile vocabulary
  lacks writes the loop and the profile's badge shows U2's typed
  refusal.
- `crates/viewer/README.md`, "New profile on a face", says it.

## Where, by name

- `crates/viewer/src/session/op.rs`: `SessionOp::DrawOnFace { face,
  traced, chains }`; the chains are `pncad::trace::Chain`
  (`crates/pncad/src/trace.rs`), lowered by `pncad::trace::chain_loops`
  to `LoopProgram<Formula>`. A face loop is not a chain, so either
  `Chain` or `DrawOnFace` grows a face-loop entry.
- `crates/viewer/src/session.rs`: `Session::draw_on_face` (it reads
  picks through `picked_body` and `pick_read`), and the edge-pick path
  of the profile editor (edge selections are handled in
  `session/select.rs` and `session/author.rs`; which one routes a click
  while a profile on a face is open was not traced).
- For a profile already open in the editor, the click is a
  `SetProgram` through the editor's own door
  (`SessionOp`'s "write the profile editor's program" arm).

## First check, before building

**Which loop is the outer one.** `ProfileProgram::loops` is "outer
first, then holes". If Ev clicks the inner rim first, does validation
pick the outer loop by containment, or refuse on order? Read
`ProfileError::MultipleOuterLoops` and the validate path. If the order
is the program's, the click must place `Outer` first whatever the
click order, which is a rule the ruling does not state: **stop and
report** rather than choose.

## Ground crossed, and who is told

`crates/viewer/src/session.rs` (CHROME, DOORS, OFFER, VSEAM),
`session/op.rs` (CHROME, VSEAM), `crates/viewer/README.md` (CHROME),
`crates/pncad/src/trace.rs` (PYDOOR: a `pncad` façade change announces
there).

## Rows it closes or changes

With U2, closes `a-faces-loop-or-closed-trace-cannot-become-a-profile-loop`
(FACELOOP).

## Review

**Single (STYLE)**, difficulty **M**: a gesture over a door U2 has
already proven; the reviewer checks the click's routing and the README.
