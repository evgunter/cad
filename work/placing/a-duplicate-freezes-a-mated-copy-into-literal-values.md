---
id: a-duplicate-freezes-a-mated-copy-into-literal-values
kind: issue
title: Duplicate writes a copy's solved motion back as literal values and drops its mates, so the duplicate stops following the copy's variables
status: open
opened: 2026-11-02
priority: P1
cost: M
design: true
---


Found by the PLACING sweep Ev asked for on PR 6281 (2026-11-02): "can you see if anything similar also got missed?" by the INTENT refactor. Read at `3bf15bef9c`; traced in code, not run unless said.

**Ruled.** `[placement.placement]`: "A placed copy's frame is not a variable: it is the construction its bundle states." The Stage-3 helpers ruling: no constructor "reads geometry or writes a number its caller did not write". Also `[intent.stored]`.

**Survives** (sure).
- `DocSession::add_duplicate` (`crates/viewer/src/session.rs`) goes through `duplicate_seats` to `combine::copy_motion` (`crates/viewer/src/combine.rs`). That function reads the evaluation's `motion` for the copy, then `moves_of` writes it as `atan2` and nominal `Formula::literal`s.
- So duplicating a mated copy gives an unmated copy frozen at today's numbers, which no longer follows the source copy's variables or mates.
- Pinned by `crates/viewer/tests/duplicate_beside.rs::a_duplicate_of_a_mated_copy_is_drawn_beside_it_unmated`, whose module doc states the snapshot rule.
- Stale prose: the `DuplicateTool` doc in `combine.rs` ("the step lands in the translation slots of the transform the gesture authors"). Also `crates/viewer/README.md`, *A new body lands unplaced*, which states the snapshot rule and then says "Nothing a place gesture writes reads the copy's geometry".

**Design question.** What a duplicate of a placed copy is. For example, a further copy of the copy slid by the step, or a second placement of the same body whose bundle is the copy's own, with the step added. `a-feature-gesture-re-points-every-copy-of-its-target` already says Duplicate can become a second placement once that row lands. This goes to designers before a lane builds it.
