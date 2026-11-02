---
id: the-mate-snap-overwrites-held-values-with-measured-literals
kind: issue
title: committing a mate snaps every held value to a measured literal, so a value that read a variable stops reading it
status: open
opened: 2026-11-02
priority: P1
cost: M
rides_with: mate-tool-commits-one-mate
---


Found by the PLACING sweep Ev asked for on PR 6281 (2026-11-02): "can you see if anything similar also got missed?" by the INTENT refactor. Read at `3bf15bef9c`; traced in code, not run unless said.

**Ruled.** `[intent.stored]` and the helpers ruling. Fork row 232's decided final state (`mate-tool-commits-one-mate`): "A number typed on a freedom that survives is kept".

**Survives** (likely). `SessionOp::AddMate` (`crates/viewer/src/session.rs`) calls `snap::snapped` (`crates/viewer/src/session/snap.rs`).
- `snapped` drops every held value (`held.into_iter().filter(|c| c.value().is_none())`).
- It then writes the slides and spin as `number(...)` literals read off the evaluation.
- So `x = px` on a freedom the new mate leaves becomes `x = 0.0347…` and stops reading `px`. The variable is silently unlinked.

The bundle editor's Apply must not do this. This row rides with that unit, which closes it: a row there commits a mate beside a value reading a variable, and asserts the variable is still read.
