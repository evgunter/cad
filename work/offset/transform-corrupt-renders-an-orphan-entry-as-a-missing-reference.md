---
id: transform-corrupt-renders-an-orphan-entry-as-a-missing-reference
kind: issue
title: topo: TransformError::Corrupt renders an orphan curve entry as 'the body references a missing {what}'
status: open
opened: 2026-10-13
priority: P4
cost: E
---


## Finding

`TransformError::Corrupt`'s `Display` (`crates/topo/src/transform.rs`) renders "the body references a missing {what} (a corrupt body)". The site raising "curve entry referenced by no edge (left unmapped)" is an orphan entry — nothing references it — the opposite fact; two other details read garbled through the sentence ("a missing edge key vanished mid-walk", "a missing he_plus has no end vertex").

Same shape as band's `BodyNotIntact` (PR 5021). Territory: `transform.rs` is claimed by offset, shelf and shell; filed on offset. Found by that PR's §5 sweep.
