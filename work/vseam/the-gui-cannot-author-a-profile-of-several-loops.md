---
id: the-gui-cannot-author-a-profile-of-several-loops
kind: issue
title: The GUI offers no way to author a profile of several loops; the bore preset is the only one
status: closed
opened: 2026-11-02
closed: 2026-11-02
pr: 6295
priority: P0
cost: M
---

**Reported against an older build** (Ev, in chat, 2026-11-02: "i had
accidentally built a slightly old version of the gui"). Reproduce on
current main before building anything; if main no longer shows it, close
the row saying so.

Reported by Ev (in chat, 2026-11-02): "it is not clear how to actually
create a profile with multiple loops in the gui". Filed by the VSEAM
orchestrator.

## What is there

A profile's program holds any number of loops, and containment decides
which are holes. `validate` orients each loop as outer or hole
(`crates/profile/README.md`, V3). The library authors these freely, and
a plate with three bolt holes is one profile. The GUI cannot:

- The add-profile form (`crates/viewer/src/pane/create.rs`, the
  profile section) offers one shape: circle, rectangle or a path chain.
  The only way it makes a second loop is the circle's "with bore"
  checkbox.
- The profile editor (`crates/viewer/src/pane/profile.rs`) labels loops
  ("loop 0", "loop 1") when there are several, but offers no way to add
  or remove one.

## Why P0

"Something the GUI cannot author at all" (`work/README.md`, Priority).
A profile with holes other than one concentric bore cannot be drawn,
and booleans of extrusions are the only workaround. It also gates
`the-rectangle-and-circle-templates-reopen-as-paths`: Ev ruled on PR
6143 to drop the bore preset ("if profiles are already multiple loops,
then there's no need for a union of profiles"). Dropping it before this
row lands would remove the GUI's only way to make a hole.

## What a fix needs

An "add loop" (and "remove loop") in both the create form and the edit
door. Each loop picks its own starting shape, using the same presets
the form offers for the first loop. The edit door's per-loop rows
already exist, and loops are one program entry each. Neighbours:
`a-profile-cannot-refer-to-its-own-earlier-parts` (a hole centred on
its boss), and `the-profile-editor-grew-past-a-small-window` (more loops
make a taller pane).

## 2026-11-02 — closed: both doors add and remove loops

Reproduced on main first: neither door had an add-loop control.

- **The create form** (`ViewerBehavior::added_loops_ui`,
  `crates/viewer/src/pane/create.rs`) draws, under the first loop's
  fields, one header per added loop (`loop 1`, `loop 2`, … with its own
  circle / rectangle / path choice and `Remove loop`) and that shape's
  fields, then the add-loop row: `Add circle loop`, `Add rectangle
  loop`, `Add path loop`. Added loops are `Drafts::profile_more`;
  `Drafts::profile_loops` appends them, so the preview, the commit and
  a sketch on a face all take every loop. A new loop starts as
  `drafts::fresh_loop`: its template at half the form's defaults,
  centred on the origin.
- **The edit door** (`edit_door_ui`, `crates/viewer/src/pane/profile.rs`)
  puts `Remove loop` on each loop header when there is more than one,
  and the same add-loop row (`add_loop_row`) under the loops; a template
  added there is held as its path steps (`sketch::shape_steps`).
  `ProfileEdit::add_loop` / `remove_loop` keep `loaded_as` the shape of
  the loops, so the kept loops keep their step ids, their expressions
  ride `carry_unmoved` as before, and a name on a removed loop's step
  is counted on Apply and reported in the outcome, as a removed step's
  is.
- The circle's "with bore" checkbox stays, and is now redundant: a bore
  is a second circle loop. Removing it is
  `the-rectangle-and-circle-templates-reopen-as-paths`.

Filed: `a-traced-chain-is-the-profiles-only-loop` (the traced door
withholds its commit while loops are added) and
`a-template-rectangle-has-no-centre` (closed as covered by
`the-rectangle-and-circle-templates-reopen-as-paths`).
