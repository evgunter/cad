---
id: a-refusal-naming-a-value-slot-drops-its-axis
kind: issue
title: a refusal naming a placement's value slot says slide/spin value without the axis
status: open
opened: 2026-11-02
priority: P3
cost: E
---


## Finding

Found by `placing-easy-tail`'s sweep for
`instance-value-rows-do-not-say-their-axis`. Properties and the
variable label now name a value row by its freedom
(`Node::slot_label`, `crates/editor-core/src/node.rs`, through
`Coord::value_label`), but every refusal that renders a slot through
`SlotId::label` / `SlotId`'s `Display` still says `slide value` or
`spin value`: the slot's address holds only the constraint id, and those
sentences are composed with no node at hand. The evaluator's
`the expression at slot slide value failed to evaluate`
(`crates/viewer/tests/msolve3_placer_refused.rs`,
`the_mate_row_names_the_direction_and_not_a_dangling_head`'s expectation) is one; the edit
door's slot refusals (`crates/editor-core/src/edit.rs`, the `EditError`
`Display` arms naming `slot.label()`), `range.rs`'s,
`persist/check.rs`'s and the viewer's `session.rs` / `props.rs`
`slot.label()` sentences are the rest of the hit list.

**Done when** a refusal that names a value slot names its freedom
(`slide along z value`), either by carrying the label from where the
node is read or by speaking through a reader that holds the node, and a
row reads it on a placement's slide.
