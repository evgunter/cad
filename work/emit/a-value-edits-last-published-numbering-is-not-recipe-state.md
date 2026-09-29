---
id: a-value-edits-last-published-numbering-is-not-recipe-state
kind: issue
title: A profile's last published numbering is not recipe state, so a value edit through an unreadable state strands names it could carry
status: closed
opened: 2026-09-24
priority: P1
cost: D
closed: 2026-09-25
pr: 3223
---


## The finding

A value edit carries the names spelled in a profile's canonical
numbering from the numbering before the edit to the numbering after it
(`reanchor_report` / `numbering_move` in `crates/editor-core/src/edit.rs`,
PR #3180). The names, though, are spelled in the numbering the profile
was **last published in**: the last state whose numbering could be read.
The two differ whenever the document passes through a state whose
numbering cannot be read. There are three such states:
- a program that does not replay (`hole_r` = 0);
- loops that cannot be ordered (a hole tied with the outer loop for the
  largest area);
- a loop with no readable sense (zero area).

## The measurement: the last published numbering is not a function of the document

This was a scratch row, not committed:
- **Path A:** mint a frame on `Lateral{1,0}` at `hole_r` = 0.3. The circle
  is the hole, loop 1.
- **Path B:** mint the same spelling at `hole_r` = 1.5. The circle
  encloses the square, so loop 1 is the square.
- Both paths then park at `hole_r` = 0, where the profile does not replay.
- `persist::save(doc, &[], tol)` of the two parked documents is
  **byte-identical**.
- At the publishing states the spelling denotes different walls:
  - path A: corners (0.7,1)–(1.3,1), a circle half;
  - path B: corners (0,0)–(0,2), a square side.

Why nothing already stored can recover it:
- `apply` is pure over `Doc`, and `Doc` holds no history. `persist/canon.rs`
  says "history is not state".
- A saved log can recover it by replay. A snapshot saved with an empty log
  cannot.
- A viewer session's landed evaluation holds it, but not across a reload
  in an unreadable state.
- The names alone do not say which numbering they are spelled in.

## What #3180 ships (the interim, no ruling needed)

- A value edit whose result cannot be read on either side strands every
  name spelled in the profile's numbering. The stranding is reported, and
  the edit stays legal. This is `SetProgram`'s own rule for an old
  program whose spans cannot be read.
- No name is ever renamed silently.
- The cost is here: a round trip through an unreadable state leaves the
  names stranded, with `Rebind` or undo as the repair. The row
  `edit_set_program::a_round_trip_through_a_zero_area_loop_strands_the_loops_names`
  pins it, so that row is the one that turns when this is decided.

## The fork for Ev

**A. Persist the last published numbering: new recipe state.**
- Per profile, store `Vec<(program_loop, reversed)>` in `Doc`.
  - Every edit whose result is readable writes it.
  - Every value edit and every `SetProgram` translates from it, not from
    the pre-edit state.
  - An unreadable result leaves it, and the names, pending.
- Closes the round trip. All three unreadable states get one treatment.
- Also makes the old side free to read (see
  `the-value-edit-numbering-check-costs-a-replay-per-swept-profile`).
- Costs:
  - A field in the save format, the validator and canon.
  - It must stay out of content keys, because it is naming metadata and
    not geometry.
  - A load rule for older files: derive it from the snapshot if the
    snapshot is readable; otherwise it is unknown, and the names strand at
    the first readable state.

**B. Doc-param edits refuse an unreadable result, as slot edits already
do (`check_profile_after_slot_edit`).**
- Every reachable state is readable, so the pre-edit numbering is always
  the published one.
- Changes documented door behaviour: V1 class 2 says "refusing programs
  may exist at rest". It also changes these rows:
  - `edit_set_program::a_program_that_no_longer_replays_has_no_spans_so_every_name_on_it_strands`
    lands `hole_r` = 0 through `SetDocParam`;
  - the tie row;
  - the zero-area row.
- Loaded or pre-existing unreadable documents still strand at their first
  readable edit.
- Removes an authoring move: parking a value in a bad state while editing
  another.

**Recommendation: A.**
- It is the only option under which a round trip is lossless.
- It keeps V1 class 2.
- It removes most of the value door's cost.

## Folded into the stable-name question (2026-09-24)

This row, the other `needs_ev` EMIT row on profile numbering, and EDIT's
`a-slot-edit-through-a-zero-fit-renumbers-a-loops-live-names` share
one cause. A profile name spells a position that is recomputed from
current state, so any edit that moves the positions re-denotes it. All
three are put to Ev as one question: name a profile piece
by an id minted when its step is authored (`names/README.md`, "N1, the
profile pieces"), or keep positions and persist a rename ledger. Under
the id rule this row's fork does not arise, because no numbering is
left to carry.

## Linked

`a-child-documents-rebind-leaves-the-parents-held-names-in-the-old-numbering`
(P0, executed) is the cross-document case of the same question. A parent's
names are spelled in a child's numbering, whose history the child does not
keep, so a pin update cannot translate them. Option A here records the
last published numbering per profile, which is the per-document half of
the rename ledger that row needs. Decide the two together.

## Ruled (2026-09-25)

Ev ruled that profile pieces are named by minted step ids (N1, "the
profile pieces"). Under that rule nothing renumbers, so this row is
fixed by building it: parked on `profile-pieces-are-named-by-minted-step-ids`.
