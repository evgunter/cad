---
id: a-clearance-bound-shape-is-a-row
kind: issue
title: A clearance's bound shape is a row on the requirement, not the create button's choice
status: open
opened: 2026-11-03
priority: P2
cost: M
parent: a-seated-tools-values-have-no-home-after-insert
---


Ev on `[ev]` PR 6406 (2026-11-03): "i'd also lean editable row, but this seems very easy to change, so do whatever". The create rule in `crates/viewer/README.md` now says the shape is a row on the requirement, and that the create gesture inserts "at least".

Today the clearance form asks for the shape (`ClearanceBoundChoice` in `crates/viewer/src/pane/create.rs`). The section "The seated tools whose values have a home" describes that, and so does #6414's `SlotId::ClearanceBound` rows.

The build:
- one Clearance button that inserts `AtLeast 0`;
- a shape choice row on the requirement, which rewrites the claim's bounds through an edit door and keeps the ends it can (Low keeps Low, a band gains a High);
- deleting the form's shape choice;
- updating that section.

Changing the shape changes which slots exist, so the door must say what happens to a slot read by a formula when its end goes away.
(FORMS orchestrator)
