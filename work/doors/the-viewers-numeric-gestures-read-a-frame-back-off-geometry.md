---
id: the-viewers-numeric-gestures-read-a-frame-back-off-geometry
kind: issue
title: The viewer's split/pattern and transform gestures read a frame back off geometry and write atan2's angles, the heuristics the helpers ruling retired
status: open
opened: 2026-10-23
priority: P2
cost: M
---


Found by D′2's second implementer lane (PR 5613), sweeping for the
retired helpers' shape rather than their names.

**What.** D′2 retired `frame_at`, `place_at`, `place_against` and
their heuristics (`docs/INTENT-STAGE3-SPEC.md` §5′.2, the helpers
ruling): a caller that meant a frame writes the moves it meant. The
viewer kept two of those heuristics under its own names in
`crates/viewer/src/session.rs`:

- `frame_off` (asked by `DocSession::read_off`, the split and circular
  pattern gestures' typed plane and axis): it evaluates the document,
  picks the body's first line-or-conic edge by id and a vertex off its
  line (`picked_frame`'s rule), and moves that frame to the typed
  numbers by `moves_of`.
- `moves_of` (asked by `frame_off` and by `gesture_moves`, the
  transform gesture's rotation about an axis that is not one of the
  space's three): it reads a rotation matrix and writes `atan2`'s
  angles, a quarter turn rounded to `turn/4`, as numbers. A rotation
  angle that reads a variable stops driving the copy once its axis is
  not a space axis.

The test fixtures carry the same two rules for the rows that state a
pose as a matrix: `editor_core::test_support::frame_on` /
`frame_on_in` (the first edge and vertex by id, moved by `atan2`'s
angles) and `moves_at`, both over the one reading
`test_support::euler_turns`. They are scaffolding, not a door, but a
row written through them states its pose as numbers read back.

Neither is in `mate/pin.rs`, so
`scripts/gates/bundle-constructor-reads-no-evaluation.sh` does not see
them, and the D′2 grep for the retired names does not either.

**Not a defect of D′2's combine or the tour.** `in_one_space` (the boolean
gesture's re-placement of a moved copy) also read the solved pose back
through `moves_of`; PR 5613 replaced that with a re-target that keeps
the copy's values as written. The tour (`demos/tour/src/world.rs`)
carried a third copy of both rules; PR 5613's fix pass rewrote its
callers to write the values they mean and to read a split's plane and
a pattern's axis off named corners (`corner_frame`, `moved_frame`), so
the tour holds neither.

**To decide.** The split and pattern gestures take numbers in the
space's coordinates, and since D′2 no pose reads a space's frame, so a
typed plane needs some geometry to stand on. Whether the gesture
offers a pick (a face, an edge) instead of numbers, or writes a frame
off a named entity the user chose, is a viewer design question; the
transform gesture's arbitrary axis likewise (write a direction mate and
a spin, or offer only the three axes). Until then the two heuristics
are the viewer's, and their numbers are the evaluation's, not the
author's.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to DOORS's charter (the viewer's authoring doors). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
