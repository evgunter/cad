---
id: a-strand-row-says-a-kept-steps-old-row
kind: issue
title: A SetProgram strand row says a kept step at the row it held before the edit
status: open
opened: 2026-10-03
---


Found by PR 3886's fix pass, deciding the review's question (M3) on `Maintenance::Strand` spoken from `before`.

`edit.rs`'s `stranded_steps` speaks each strand row's name from `before`, the document the `SetProgram` replaced (`before.spoken_name(name)`). For a name on a step the reshaping DROPPED this is right: the step's old row (`loop 0 step 5`) is the only way a person can tell which step went, since its id shows on no screen, and the row sits in a sentence that says the edit removed it.

For a name on a KEPT step whose piece the reshaping stopped drawing (`undrawn_kept_pieces`), the step is still in the program, usually at a new row (a fillet inserted before its leg moves it down). The row spoken from `before` is its old row, which in the program now on screen may be another step's.

**The fix.** Speak a strand row's name from `doc` for its kept steps and from `before` for its dropped ones: `before.spoken_name(name)` with each kept step re-read from `doc` (`HeldNodes::respoken` already re-reads a kept step and lets go of a dropped one; the row wants the reverse for a dropped one, so a dedicated speaker or a `respoken` that keeps a dropped step's old row for this caller). Pin with a strand row over a kept-but-undrawn piece after an insert above it.
