---
id: viewer-own-refusals-are-held-to-no-shape-guard
kind: issue
title: chrome: the viewer's own Refusal arms are held to no refusal-shape guard
status: open
opened: 2026-09-29
---


(Filed by EDIT's recourse unit for `edit-refusals-short-of-the-shape-guard`,
from the second pass of its sweep: the pattern it swept was the edit
roster's zero-recourse check, and what that pattern cannot see is a
refusal the status line draws that is not an `EditError`.)

## What

`viewer::session::Refusal` (`crates/viewer/src/session/refuse.rs`) has
25 arms. `crates/viewer/tests/refusal_concision_edits.rs` holds only
`Refusal::Edit` to `test_utils::refusal::problems`, and
`crates/viewer/tests/part_root_carried.rs` only the part-root lines; no
suite renders the other 24 arms through `problems`, so none of them is
checked for the budget, a stage prefix, a `Debug` struct, an arena key
or a recourse.

One instance, by reading: `Refusal::NoSuchParam` renders
"no document parameter named {name} — {UNDECLARED_PARAM_RECOURSE}".
Its sibling at the edit door, `EditError::DocParamNotDeclared`, labels
the same const `Recourse:`; the viewer's arm states it bare, so the
shape guard would count no recourse there, and the two doors that
`UNDECLARED_PARAM_RECOURSE`'s doc says converge on one recourse spell
it two ways.

## Repair shape

A roster over every `Refusal` arm except `Edit` (which the edits suite
owns), on a representative payload, held to `problems` with the same
must-fire admission lists; then each arm rewritten at its source.
