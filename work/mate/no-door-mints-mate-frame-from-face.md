---
id: no-door-mints-mate-frame-from-face
kind: issue
title: No door mints a mate's alignment frame from a selected face
status: open
opened: 2026-08-23
github: 944
refs: [938]
---

## From GitHub issue 944

opened 2026-08-23, 0 comments.

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
