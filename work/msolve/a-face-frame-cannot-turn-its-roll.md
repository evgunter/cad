---
id: a-face-frame-cannot-turn-its-roll
kind: issue
title: A face frame fixes its roll by the carrier's u_ref, so a FromFace mate cannot be turned about its axis
status: open
opened: 2026-09-24
priority: P1
cost: M
design: true
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

## Weighed (2026-10-01)

Two designers weighed this on `msolve/ev-mate-turn`, where `ASSEMBLY.md`
A3 and A11 (1) state the recommended answer. They converged in two
rounds: the turn is the mate's, carried as
`FrameCoincidence { turn }` and `Coaxial { turn: Option }`, and the
rider is deleted.

Corrections to this row, which both designers found:
- The coincidence rider is not refused statically. It is decided over
  the mate's lever (`mate_clocking_redundant`), so a rider of 0 within
  band is admitted.
- The GUI authors no rider at all: the mate panel hard-codes
  `clocking: None`.
- No edit rewrites a committed mate's datum
  (`DocEdit::writes_a_mates_datum` is true only for `InsertNode`).

## Re-weighed (2026-10-03, round 3)

Ev approved the turn on the primitives on 2026-10-01. On 2026-10-03
Ev ruled PLACE's row 53: each mate side is a base composed with an
offset `Placement`, which may be any rigid motion. That offset already
turns a side about its axis. Given row 53, both designers now
recommend dropping the turn: the sides' offsets carry the roll, and
`Coaxial { roll: Free | Pinned }` says only whether the roll is
pinned. They also lean towards retiring `PlanarRest`'s standoff the
same way. The rider deletions stand. `ASSEMBLY.md` A3 states the
revised question, and the PR asks again.

## Ruled (Ev, `[ev]` PR 3681, 2026-10-03)

"the new plan makes sense". The roll lives in the sides' offsets (row
53), and no turn field is added. `MatePrimitive` names only the
residual subgroup: `FrameCoincidence`, `Coaxial { roll: Free | Pinned }`
and `PlanarRest`. `PlanarRest`'s standoff retires, and the rider,
`Clocking`, `table_gap`, `mate_clocking_redundant` and `Lever::Roll`
are deleted. "Turn 90°" is a verb that composes a rotation onto one
side's offset. The build is an MSOLVE unit. It waits for row 53's
`MateFrame` shape to land, because the deletions ride the offset.

