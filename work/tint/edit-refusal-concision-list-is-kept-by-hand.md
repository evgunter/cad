---
id: edit-refusal-concision-list-is-kept-by-hand
kind: issue
title: refusal_concision_edits renders 80 of EditError's 92 arms by hand, and no roster ties the list to the enum
status: open
opened: 2026-10-20
priority: P4
cost: E
---


Found by `wire-census2`'s sweep for the class of
`kernel-payload-lists-in-the-concision-census-are-kept-by-hand`.

`crates/viewer/tests/refusal_concision_edits.rs:6` says it renders
"every `EditError` arm" through the status line and holds each to the
budget. Its forwarded faults are welded by `witnesses` chains (`:724`) whose
`next` functions match with no wildcard, but the `EditError` list
itself is kept by hand: a scripted comparison of its constructors
against the enum (`crates/editor-core/src/edit.rs:2131`) finds 12 of 92
arms with no row — `PoseShape`, `IndexedRead`, `LoftSectionsSpelled`,
`SetExtrudeSideOnNonExtrude` (since retired with `Extrude.side`), `SharedVarNeedsName`, `FreshUnheld`,
`FreshKind`, `FreshUnread`, `VarIsAnOutput`, `AppearanceBodyNotARead`,
`MateNamedByValue`, `LabelUnchanged`. Some may be unreachable at a
viewer door; each wants a row or a reason in-file.

## Repair shape

An `f6_variants!` roster for `EditError` and a set difference over the
rows' `Debug` arms (as the chains census's
`every_forwarded_arm_has_a_row` does), then a row per arm it turns up,
or a named reason.
