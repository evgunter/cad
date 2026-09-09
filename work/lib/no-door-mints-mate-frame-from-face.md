---
id: no-door-mints-mate-frame-from-face
kind: issue
title: No door mints a mate's alignment frame from a selected face
status: open
opened: 2026-08-23
github: 944
refs: [938]
needs_ev: true
---

## From GitHub issue 944

Opened 2026-08-23; 0 comments.

Found by the ASM-DEMO exit walk (#938).

## The shape of it

A11 makes the constructive solve structural on purpose — "no geometry inspection, no numerics beyond decided predicates" — so `Alignment`'s two `MateFrame`s are numbers the author writes. That is the right design for the SOLVE. The consequence at the authoring door is that a mate's frame and the geometry it is meant to sit on are two independent sources of truth, with nothing tying them together.

Concretely, in `demos/tour/src/assembly.rs` the stand's mates carry the post's cap height as a literal (`POST_SEAT`, built from `POST_HEIGHT`) and the shelf's seating points as literals. Edit the post document so its top cap moves, and the mate keeps pointing where the face used to be. Nothing warns; the assembly simply stops fitting, and the only signal is the at-rest gate refuting the declaration afterwards — which the demo's update walk provokes on purpose and reports.

## What is missing

`face_frame(&ev, node, &name)` already answers a named face with a `Pose`. Nothing consumes a `Pose` into an `Alignment`/`MateFrame`. So the natural authoring gesture — "mate THIS face to THAT face" — has no spelling; the author reads the pose (or knows the model's numbers) and retypes them.

## Mitigation a user learns, and why it is not a fix

Model each part from the datum it mates on, so the mated face sits at the part origin and a size change never moves it. The demo says this at `stops`' doc comment. It only helps for the parts you control and the one face you chose.

## Note on scope

Deriving a frame from a face at AUTHORING time does not weaken A11: the solve still reads authored data. What would need deciding is whether the derived numbers are frozen at authoring (a materialized selection, matching the `select` doors' own materialize-then-store rule) or re-derived — the second reintroduces geometry into the solve and is presumably not wanted.

— Claude (ASM-DEMO lane)

## Home

S-MATE's `keep_out` names this issue by number as its own, to be taken with LIB's hand-off; `crates/editor-core/src/mate.rs` is in its territory.

## Question for Ev (2026-09-09, LIB orchestrator; `[ev]` PR)

The authoring gesture "mate THIS face" has no spelling: `face_frame`
answers a named face with a `Pose` (origin, axis, an optional `u_ref`;
`crates/topo/src/readback.rs:88`), `MateFrame` is three authored
vectors (origin, axis, reference; `crates/editor-core/src/mate.rs:115`),
and nothing maps one into the other. Two decisions, then a shape.

**1. Frozen or re-derived?** A11 makes the solve structural — it reads
authored numbers and inspects no geometry. A door that derives a frame
from a face can either MATERIALIZE the pose into numbers at authoring
time (the `select` doors' own materialize-then-store rule: the mate
stores three vectors exactly as if the author had typed them, and the
solve is untouched), or record the face and re-derive the frame at
solve time (geometry re-enters the solve, which A11 forbids and the
item already presumes is not wanted). Recommendation: **frozen**. It
removes the retyping, not the drift: a face that moves after an edit
still leaves the stored frame behind, and the at-rest gate remains the
signal, as today.

**2. The reference axis when the face gives none.** `placement`
refuses a `reference` parallel to `axis`, and a `Pose` carries
`u_ref` only for faces with a canonical in-plane direction. So the
door needs a rule for the rest: **(i)** take an explicit `reference`
argument that is used when `u_ref` is absent and refused with a typed
error when both are missing — recommended; **(ii)** invent one (any
perpendicular), which makes the clocking arbitrary and silent.

**3. The shape**, given 1 and 2(i): **(A)** one kernel door,
`MateFrame::from_pose(pose, reference: Option<[f64; 3]>)` in
`editor_core::mate`, plus the composition at the façade and in Python
(`MateFrame.from_face(evaluation, node, name, reference=None)`),
refusing as a typed `MateFrameError` when neither source gives a
reference — recommended, mechanical once ruled; **(B)** the same door
that also records the face's `StableName` on the `MateFrame` as
provenance, so a later check can compare the frozen frame against the
face's current pose and report drift — a new field on a persisted
struct and a new advisory check, which is a second unit, not a
refinement of the first; **(C)** leave it: the mitigation (model each
part from the datum it mates on) stays the documented answer.

**4. Whose.** The Home line names S-MATE, which has no tracker
directory today; LIB can take (A) as a mechanical unit in this wave,
or hand it to the solver's program. Recommendation: LIB takes (A).
