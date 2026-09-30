---
id: viewer-has-no-door-to-accept-a-parts-updated-version
kind: issue
title: viewer: no door emits DocEdit::UpdateReference, so a repaired part's new version cannot be accepted from the GUI
status: open
opened: 2026-09-29
priority: P3
cost: M
---


## What

A part's repair moves its content pin, so every instance of it then
refuses with a pin mismatch whose recourse is to accept the updated
version: `pncad::workspace::PIN_MISMATCH_RECOURSE`
(`crates/pncad/src/workspace.rs`) names `DocEdit::UpdateReference`, or
`workspace::update_to_store` for every site at once.

The viewer has no door that emits either. No `SessionOp` builds a
`DocEdit::UpdateReference`; the one mention in the crate is the
no-op filter in `crates/viewer/src/session.rs` (the `writes_nothing`
match, `DocEdit::UpdateReference { .. }` among "every other edit
submits"), which never sees one. So the refusal a GUI user reads
names a recourse the GUI cannot take, and the user has to leave the
viewer to record it.

Found while building `edit/part-root-carried-refusal`: the part-root
failure's own recourse ("open the part and repair node N") deliberately
stops short of "then accept the updated version" for this reason
(`work/edit/part-root-failure-nests-a-whole-refusal-past-the-budget.md`,
"Put to Ev").

## What would close it

A door on the instance row (or the pin-mismatch refusal's line) that
records `DocEdit::UpdateReference` against the store's current pin,
beside the `Add part…` door (`crates/viewer/src/parts.rs`).
