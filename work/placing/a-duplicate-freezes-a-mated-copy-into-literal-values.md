---
id: a-duplicate-freezes-a-mated-copy-into-literal-values
kind: issue
title: Duplicate writes a copy's solved motion back as literal values and drops its mates; it retires in favour of a second placement of the body
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

## Ev's direction (2026-11-01, in a session)

The design question above is answered: **Duplicate retires**. It is
superseded by making more than one placement of a body, in the
placement editor (PR #6300). It does not retire automatically with
Transform, so this row tracks it.

**What the retirement still needs to decide (goes to designers):**
- **Where the second placement is made.** The editor's Create door
  opens only on a pick that is no copy (`MateTool::pick`,
  `crates/viewer/src/matetool.rs` on `placing/bundle-editor`). A body
  that is already placed is not drawn (README, "the picture draws
  bodies, not placements"), so its one route to a second placement in
  the GUI is Duplicate today. The editor needs a door that opens
  Create on a placed body: from its tree row, or as "place again"
  beside the Edit door on a pick of its copy.
- **Its starting draft.** Whether a second placement starts with no
  mates where the body stands, or loaded from the picked copy's bundle
  as written (which avoids this row's literal snapshot).
- **The coupling** with the parked
  `a-feature-gesture-re-points-every-copy-of-its-target`. Two
  placements of one body share it, so a feature on either takes over
  both. That is why Duplicate makes a body of its own today.

**What goes:** `SessionOp::Duplicate`, `DocSession::add_duplicate`,
`duplicate_seats`, `combine::duplicate_step`/`copy_motion`/
`duplicate_moves`, the `DuplicateTool` and its panel, the duplicate
tests (`crates/viewer/tests/duplicate_beside.rs`), and the duplicate
sentences in the README's "A new body lands unplaced".
