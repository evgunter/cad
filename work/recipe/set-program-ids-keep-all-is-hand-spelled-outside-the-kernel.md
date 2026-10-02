---
id: set-program-ids-keep-all-is-hand-spelled-outside-the-kernel
kind: issue
title: SetProgram's keep-every-step ids live in the viewer, and the kernel's tests spell them by hand five times
status: closed
opened: 2026-09-30
priority: P4
cost: E
branch: recipe/set-program-undrawn-names
pr: 3879
closed: 2026-10-02
---


## The finding

"Every step of this program kept where it is" is a fact about
`DocEdit::SetProgram`'s `ids`: per loop, per step, `Some` of the step's
own id. Its one named spelling is the viewer's
`sketch::kept_in_place` (`crates/viewer/src/sketch.rs`, added by
AUTH-6, #3446), with `sketch::is_committed` beside it for "this
`SetProgram` would write nothing". The kernel has neither, and its own
tests spell the ids by hand:

- `crates/editor-core/tests/corpus/reshaped_rod.rs`, `bump_ids` (near
  line 95).
- `crates/editor-core/tests/asm_parent_held_names.rs`, the `kept` read
  near line 151, and `with_leg` near line 240.
- `crates/editor-core/tests/asm4_split_inline.rs`, near line 1594.
- `crates/editor-core/tests/edit_set_program.rs`, `keep_all` (near line
  136) — a named test-local copy.

Found with `rg -n 'map\(Some\)' crates/editor-core/tests`; the pattern
cannot match a spelling that builds the ids another way (a loop that
pushes `Some(id)`, say).

## The fix

A kernel door beside `SetProgram` (a `ProfileProgram` method, say)
that the five test sites and the viewer both call, with the viewer's
`kept_in_place` retired onto it. Whether `is_committed`'s no-op rule
belongs beside it (the edit door has a no-op of its own) is EDIT's
call. The viewer's copy is the one live consumer today, so nothing is
wrong yet; this is where a second spelling would drift.

## Built (2026-10-02, PR 3879)

`ProfileProgram::kept_in_place` is the door. The viewer's
`sketch::kept_in_place` is retired onto it; `is_committed` stays in the
viewer and calls the door. The five listed sites use it now, and so do
two sites the row missed: `run_wall_offers.rs` and
`msolve6_part_extent.rs`. The PR body carries the sweep's hit list and
its second pass.
