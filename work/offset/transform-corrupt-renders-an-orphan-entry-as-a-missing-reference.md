---
id: transform-corrupt-renders-an-orphan-entry-as-a-missing-reference
kind: issue
title: topo: TransformError::Corrupt renders an orphan curve entry as 'the body references a missing {what}'
status: closed
opened: 2026-10-13
priority: P4
cost: E
closed: 2026-10-24
pr: 5651
branch: offset/transform-rewritten-secondary-map
---


## Finding

`TransformError::Corrupt`'s `Display` (`crates/topo/src/transform.rs`) renders "the body references a missing {what} (a corrupt body)". The site raising "curve entry referenced by no edge (left unmapped)" is an orphan entry — nothing references it — the opposite fact; two other details read garbled through the sentence ("a missing edge key vanished mid-walk", "a missing he_plus has no end vertex").

Same shape as band's `BodyNotIntact` (PR 5021). Territory: `transform.rs` is claimed by offset, shelf and shell; filed on offset. Found by that PR's §5 sweep.

## Closed

PR 5651. `TransformError::Corrupt { found }` takes a complete clause for what the walk found and renders "the body is corrupt: {found}." Every site was re-worded, the orphan site now reads "a curve entry is referenced by no edge", and `an_orphan_curve_entry_reads_as_unreferenced` pins the rendered text.
