---
id: instance-value-rows-do-not-say-their-axis
kind: issue
title: an instance's Properties value rows say slide/spin value without the axis
status: closed
closed: 2026-11-02
opened: 2026-10-31
priority: P2
cost: E
---

## Finding

Found driving the viewer (2026-10-31; Ev: file it). Selecting an instance shows its placement's six free values as rows reading only `slide value` (three) and `spin value` (three). The feature tree names the same constraints `constraint 3: slide along z value`, but Properties drops the axis, so the rows can only be told apart by order. The row label is `SlotId::ValueSlide`/`ValueSpin`'s name (`crates/editor-core/src/node.rs`, `"slide value"`, `"spin value"`).

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`.

```text
window 1280 1600
click "Assemble parts"
click "Add part…"   # in an assembly saved beside a part
click "add" 1
click "InstantiatePart 1 *"
dump
```

## Closed

Properties names a value row by the freedom it sets: `SlotRow` carries a
`label` read through `Node::slot_label` (`crates/editor-core/src/node.rs`),
which names a value's number by `Coord::value_label`
(`crates/editor-core/src/mate.rs`), the same `Coord::name` the placement
editor's rows and the tree's constraint rows read, so the axis wording
stays in one place. `variable_label` reads it too. Row:
`bundle_editor::a_value_row_in_properties_names_its_axis`. The refusals
that name a value slot with no node at hand are filed as
`a-refusal-naming-a-value-slot-drops-its-axis`.
