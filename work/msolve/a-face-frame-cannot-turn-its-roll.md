---
id: a-face-frame-cannot-turn-its-roll
kind: issue
title: A face frame fixes its roll by the carrier's u_ref, so a FromFace mate cannot be turned about its axis
status: open
opened: 2026-09-24
priority: P1
cost: D
---


Found by the MSOLVE-9 dual review (PR 2934); filed by its fix pass.

## What

A `MateFrame::FromFace` side is its face's name and nothing else
(`crates/editor-core/src/mate.rs`, `FaceFrame`): the solve takes the
carrier's own in-frame reference (`topo::readback::Pose::u_ref`) as the
roll (`crates/editor-core/src/mate/solve.rs`, `resolve_side`). There is
no key for an author to turn it by, and the wire refuses one
(`msolve9_from_face::both_arms_round_trip_and_a_stray_key_on_either_refuses`).
The only other door to a roll is the clocking rider, and the coset
table refuses a rider on a frame coincidence statically
(`mate_coset`'s `FrameCoincidence` arm, `mate_clocking_redundant`),
which is how a seat is ordinarily authored (the story suite's and the
tour's rests).

So a mate the tool authors — face frames on both sides, since the tool
now authors only face frames — cannot be turned about its axis at all.
The story suite meets this and falls back to authored vectors for the
clocked second sail: `crates/viewer/tests/story_assembly.rs`, the second
sail's proposal, where the wall side is re-authored with
`asm::authored_from_world` at the chosen quarter turn.

## Consequence for the plan

`work/msolve/plan.md` item 16 records that `FromFace` answers half (2)
of `mate-clocking-has-no-gui-path` (turning a mate's roll). It does not:
a face frame's roll is the carrier's. Half (2) is still open, and this
row is where it now lives.

## What it wants

A way to turn a mate's roll that the tool can author: an in-face roll
on the face frame (a turn about the face's axis, authored as an angle
beside the name — a number the author owns, unlike the removed
`reference`, which no carrier could admit), or a rider on the
coincidence row the table decides rather than refuses. A design
question on MSOLVE's ground (`mate.rs`, `mate/solve.rs`) with CHROME's
affordance on top.
