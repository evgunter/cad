---
id: instance-value-rows-do-not-say-their-axis
kind: issue
title: an instance's Properties value rows say slide/spin value without the axis
status: open
opened: 2026-10-31
priority: P2
---

## Finding

Found driving the viewer (2026-10-31; Ev: file it). Selecting an instance shows its placement's six free values as rows reading only `slide value` (three) and `spin value` (three). The feature tree names the same constraints `constraint 3: slide along z value`, but Properties drops the axis, so the rows can only be told apart by order. The row label is `SlotId::ValueSlide`/`ValueSpin`'s name (`crates/editor-core/src/node.rs`, `"slide value"`, `"spin value"`).

Replay with `cargo run --release -p viewer --features app --example drive -- script.txt`.

```text
window 1280 1600
click "Add part…"   # in an assembly saved beside a part
click "add" 2
click "InstantiatePart 1 *"
dump
```
