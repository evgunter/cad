---
id: a-reshapings-values-strand-what-a-value-edit-would-not
kind: issue
title: A SetProgram reports a piece its new VALUES leave undrawn (a Zero-fit run), where the same values through SetParam report nothing
status: open
opened: 2026-10-02
priority: P3
cost: E
design: true
---


## The finding

Found while building
`a-reshaping-that-shadows-a-kept-steps-piece-reports-nothing`.
`DocEdit::SetProgram` now reports a name on a kept step's piece that
the old program draws and the new one does not, both under the current
parameters (`undrawn_kept_pieces`, `crates/editor-core/src/edit.rs`;
REFERENCES.md DM7). The one authority it reads,
`ProfileProgram::pieces`, answers for structure and values together:
the replay's record does not say WHY a piece is undrawn (another piece
took its segment, or a `Zero` fit suppressed it). So a `SetProgram`
whose new values alone suppress a piece reports it too — for example
`edit_set_program.rs`'s `filleted_square` at `r = 0.3` rewritten whole
at `r = 2` with every step kept strands a name on `{fillet, RunOut}` —
while the same radius written by `SetParam`
(`a_zero_fit_piece_vanishes_and_comes_back`) reports nothing, as DM7's
value-edit clause says. The viewer's path editor lands its number
moves as one `SetProgram` (`profile_edit.rs`,
`a_moved_number_is_one_edit_and_undoes`), so a radius dragged there
through a `Zero` fit counts on the Apply button, and the same radius
typed into the parameter panel does not.

DM7 states only the structural rule (a kept piece the new program
does not draw is reported); the value-only behaviour is as built and
pinned by
`edit_set_program::a_reshapings_values_strand_what_a_slot_edit_of_them_would_not`,
and the question is whether it is the right answer.

**The count case.** A `circle_split` hole's count `4 → 3` through a
keep-all `SetProgram` reports the names on its `Piece(3)`, and only
those (the same row pins it). The count `n` is a `u32` in
`LoopProgram::CircleSplit`, not an expression slot (`SlotId::Profile`
has no count argument; `SlotId::dimension`'s `Profile` arm: "none is
Count"), so a `SetStructuralParam` aimed at it refuses (pinned
too) and `SetProgram` is the count's only door. Unlike the radius
there is no value-edit twin that reports nothing, and the options
below land differently on it: the second would stop reporting it (no
other piece takes `Piece(3)`'s segment; the split just draws fewer
pieces), and the third reports it only if the count is structure
rather than a value.

## The options

- Leave it: a reshaping states its values, and the edit that removed
  the referent reports it, transient or not.
- Report only a piece undrawn because another piece took its segment.
  That needs the replay record (`profile::ReplayStructure`) to say why
  a piece is absent — the one authority grown by a field, not a second
  derivation in the edit door.
- Treat a `SetProgram` whose structure (values erased) equals the old
  program's as a value edit and report nothing for it; a structural
  reshape that also moves values still reports a value-suppressed run.
