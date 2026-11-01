---
id: a-patterns-value-rows-read-the-member-index-outside-a-member
kind: unit
title: a pattern's Properties value rows evaluate a member-indexed slot outside any member and show = ?
status: review
opened: 2026-11-01
priority: P1
cost: M
parent: an-instances-pattern-draws-nothing
branch: spaces/pattern-value-rows
pr: 6208
---


Split out of `an-instances-pattern-draws-nothing` (its last paragraph):
selecting an unfused pattern's placement shows its first slide row as
`= ?`, with `slide value: the definition of variable #38:… refused:
index #38:… has a value only in one member's evaluation, and it was read
outside one` (`ExprError::IndexOutsideMember`,
`crates/editor-core/src/expr.rs`). The Properties pane
(`crates/viewer/src/pane/properties.rs`, `crates/viewer/src/props.rs`)
evaluates the slot's expression once, outside any member, and a slot
that reads the member index has no single value there.

The refusal is right; the pane asking for one value is the defect. Done
when a member-indexed value row shows what the slot means (its
expression, and/or its value per member) with no refusal, and a row in
`crates/viewer/tests/` drives an unfused pattern, selects its
placement, and asserts the rows carry no refusal. Sweep every reader in
the viewer that evaluates a placement's or a family's slot outside a
member (the Properties pane may not be the only one), and record the
hit list.

Review tier: single, STYLE plus a correctness arm on the sweep's
claim (a refusal hidden instead of a meaning shown is the failure mode
to falsify).
