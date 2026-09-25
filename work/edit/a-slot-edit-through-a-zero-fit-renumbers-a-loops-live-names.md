---
id: a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names
kind: issue
title: A slot edit through a Zero fit renumbers a loop's live names and reports nothing
status: open
priority: P0
cost: H
opened: 2026-09-20
refs: [a-committed-profile-program-has-no-whole-program-edit, stranded-names-are-retired-to-an-undrawable-coordinate]
---

## The finding

`crates/editor-core/src/eval/anchor.rs`'s module doc used to say "a
parameter edit CANNOT renumber, by construction". It is false for one
class of program. A loop's segment count is a function of its
arguments as well as its structure: a corner fillet whose runs reach a
`Zero` fit emits nothing (`crates/profile/src/path.rs`,
`emit_fillet_in`: "A `Zero` fit emits nothing and springs the arc off
the last vertex"). So a plain `DocEdit::SetParam` on the fillet's
radius changes how many segments the step draws, and every LIVE name
on a later segment of that loop denotes a different wall afterwards —
with no refusal, no `Maintenance` row, and no `Vanished`, because the
old index is still drawn.

Measured by reviewer R1 on the retired coordinate (the first spelling
of the retirement, one past the loop's end, went live under such an
edit — answered by `RETIRED_FLOOR`, see
`stranded-names-are-retired-to-an-undrawable-coordinate`), and
extended here by one row on a LIVE name, pinned as measured:
`edit_set_program::a_slot_edit_through_a_zero_fit_renumbers_a_live_name_and_reports_nothing`
(`crates/editor-core/tests/edit_set_program.rs`). The chain `At,
Toward(+x), Fillet(r), Toward(+y), FarEndTo(2,2), LineTo(0,2),
LineTo(Start)` draws THREE segments at `r = 2` (both runs fit `Zero`)
and FIVE at `r = 0.3`; a derived frame on wall 2 — the LEFT edge
`(0, 2) → (0, 0)` at `r = 2` — keeps evaluating after `SetParam
r = 0.3` on the RIGHT edge `(2, 0.3) → (2, 2)`, `Applied.maintenance`
empty. The frame did not refuse and did not move: the name it holds
now denotes the opposite wall.

## Where it sits in the design

`anchor.rs`'s sentence is corrected to name the exception (this row's
id); DI1's record line in `crates/editor-core/IDENTITY.md` names it as
the same aliasing class one level in, on a held locator rather than a
node id. `SlotId::Profile`'s stability claim (V2: "step indices are
stable under every slot edit") is still true — STEP indices are; it is
the SEGMENT indices the published names carry that move, and a step
that draws a variable number of segments is exactly where the two
numberings come apart.

## What would close it

Not this unit's to fix. The door that knows is `SetParam`'s
`check_profile_after_slot_edit`, which replays the program and could
compare the per-step spans before and after through the same
`SegmentMap` `SetProgram` uses (DM8's checked records on both sides,
the identity provenance): a step whose span length moved under a slot
edit would then report `Rebound` for every name after it, or strand
it, exactly as a reshaping does. That is a widening of what a slot
edit may report — today it reports nothing but cluster maintenance —
and is a design question for EDIT, filed here rather than built.
