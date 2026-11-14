---
id: a-whole-body-measure-over-a-member-read-refuses-other-body
kind: issue
title: A measure or clearance seat naming a whole body over a member read xs[i] refuses OtherBody
status: open
opened: 2026-11-10
priority: P3
cost: M
---

Found by the review of PR 6555 (F10). `crates/editor-core/src/edit.rs` `mint_selection`'s whole-body branch (`on_body = !body.is_indexed() && …`) returns the body read's variable for a whole body named at a measure or clearance seat, so the seat reads that body. A member read is no variable: `xs[i]` with its whole-body name refuses `SelectionFault::OtherBody`, which says the wrong thing (the body named IS the body read).

A whole member at a measure or clearance seat needs a read the seat can hold: a selection variable of the `Body` kind holding the member read, or the seat itself taking a `BodyRead`. Done when a measure of the whole member `xs[i]` evaluates, or refuses in words that say a member is read only by its entities.
