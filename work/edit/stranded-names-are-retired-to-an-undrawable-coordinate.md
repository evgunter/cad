---
id: stranded-names-are-retired-to-an-undrawable-coordinate
kind: issue
title: SetProgram retires a stranded name to an undrawable coordinate rather than leaving it in place: Ev's call
status: open
priority: P1
cost: E
opened: 2026-09-20
needs_ev: true
refs: [a-committed-profile-program-has-no-whole-program-edit]
---

## What Ev ruled, and what the unit built beyond it

Ev ruled on `[ev]` #2904 that `DocEdit::SetProgram` reports every
strand and rebinds every kept name. The unit that built it
(`edit/program-edit`, PR #2927) does a THIRD thing the ruling did not
spell out: a stranded name's locator is REWRITTEN to a coordinate no
program draws. The ratified pages (`crates/editor-core/REFERENCES.md`
DM7, `crates/profile/README.md` V2) say exactly what #2904 ruled and
nothing more; the retirement is documented on
`DocEdit::SetProgram`'s rustdoc (`crates/editor-core/src/edit.rs`, the
"The retirement" paragraph) and on `RETIRED_FLOOR` beside `SegmentMap`
(`edit.rs:2545`), and this row is where it is put to Ev.

## The mechanism as built

`SegmentMap::retired` (`edit.rs:2701`) spells a stranded locator at
`RETIRED_FLOOR + s` — segment `s` of the loop that continues its
loop — or on loop `RETIRED_FLOOR + l` where no new loop continues loop
`l`, `RETIRED_FLOOR = u32::MAX / 2`. Distinct per old coordinate; a
coordinate already at or above the floor is left exactly as it is by
every later reshaping and reported by none (`already_retired`,
`edit.rs:2549`; DM7's clause is the referent THE EDIT removed). The
`Strand` / `StrandedAppearance` row carries the retired spelling, the
name resolves `Vanished` at every evaluation, and `Rebind` from that
spelling is the repair — the wire round-trips it and the resolver does
no arithmetic on it (`edit_set_program::a_retired_name_round_trips_the_wire_and_rebinds`,
`a_retired_name_resolves_vanished_through_the_resolver_door`).

## The measured cost of "left in place" (why the unit did not)

Measured on the sunk rod (PR #2927's premise 5): a name left at its
old coordinate keeps a coordinate the new program may draw — it then
denotes that segment as if it always had, the DI1 aliasing class
(`crates/editor-core/IDENTITY.md`) the ruling exists to end — and a
name a kept step moves onto the same coordinate collides with it in a
selection and is merged away by re-canonicalization: with the arc step
uncontinued, a fillet on both creases silently shrinks to one. A
frame on a dropped wall would re-anchor to whatever plane the new
program draws at its old index
(`edit_set_program::a_frame_on_a_wall_a_reshaping_dropped_refuses_vanished_at_evaluation`
is the row that reds under "left in place").

## R1's slot-edit finding, and why the floor answers it

The first spelling retired a name ONE PAST its loop's end (`n_new +
s`), claiming "past every drawn coordinate". Reviewer R1 measured that
a loop's segment count depends on its ARGUMENTS: a corner fillet whose
runs reach a `Zero` fit emits nothing (`crates/profile/src/path.rs`,
`emit_fillet_in`'s doc), so a plain `SetParam` on the radius grows the
loop and a retired name one past the old end goes live — silently,
since a slot edit reports nothing. The floor answers it by
construction: a drawn coordinate is a count of segments in memory
(`program_index` narrows exactly such a count), and a loop of
`RETIRED_FLOOR` segments would hold 2^31 vertices before the
validator's pairwise walk ever ran, so nothing a program can draw
reaches the floor under ANY edit
(`edit_set_program::a_retired_name_stays_dead_when_a_slot_edit_grows_the_loop`,
R1's probe made green).

## The alternatives, for Ev

- **A typed tombstone.** A `RoleSeg` (or a `ProfileEdgeRef` arm)
  that SAYS "retired" rather than a coordinate that happens to be
  undrawable. Honest by type; costs a variant across the name
  vocabulary (every exhaustive walk, the wire, the Python text
  alphabet, the selector) for a state a `Vanished` rung already
  reports.
- **A `Strand` that deletes the carrier's reference.** Remove the
  name from the selection / open list / pair / store rather than
  re-spell it. No aliasing, no collision; but the report then names a
  thing the document no longer holds, `Rebind` has nothing to rebind
  from, and a fillet whose only crease strands becomes an empty
  selection — a second refusal the door would have to decide.
- **Report only, name left in place.** Ev's ruling read literally.
  The measured cost above: silent re-denotation and a selection
  collision the door cannot see.
- **The floor (built).** No new type, the repair is `Rebind` from the
  reported spelling, the name is dead under every edit.

## What Ev decides

Whether the retirement stands as built, and if so whether the floor
spelling or a typed tombstone is the form it should take. On a ruling
the unit row's `## Built` and `DocEdit::SetProgram`'s doc are
re-worded to say it is ratified; on a different ruling the change is
confined to `SegmentMap::retired` and `already_retired`
(`edit.rs`) and the rows named above.
