---
id: record-binding-and-the-part-store-spell-the-failed-nodes-twice
kind: issue
title: eval/mod.rs's RecordBinding::of and eval/parts.rs each spell 'the nodes a build FAILED' as the same filter over a result map
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN (`green/censuses`, PR 6921), from the review of that PR.

## What

"The nodes a build FAILED" is the same expression in two places:
- `crates/editor-core/src/eval/mod.rs`, `RecordBinding::of`: `record.nodes.iter().filter(|(_, r)| matches!(r, NodeResult::Failed(_))).map(|(&id, _)| id)`.
- `crates/editor-core/src/eval/parts.rs`: `evaluation.nodes.iter().filter(|(_, r)| matches!(r, super::NodeResult::Failed(_))).map(|(&id, _)| id).collect()`, the refusals a part build's record carries.

Both came in with PR 6735 (`onecopy/dual-tier3`). The second one writes the set that the first one reads back, so if either spelling changes alone (for example, poisoned nodes counted on one side only), the record no longer binds what it recorded. `node_standing`'s `READERS` counts both as listed readers.

## Owed

One function over a result map that answers its failed nodes, which both call. `node_standing`'s counts for the two files move with it.
