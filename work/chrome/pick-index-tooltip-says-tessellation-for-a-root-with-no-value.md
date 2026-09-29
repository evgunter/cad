---
id: pick-index-tooltip-says-tessellation-for-a-root-with-no-value
kind: issue
title: The pick-index tooltip says a root's bodies could not be tessellated or indexed when the root has no value at all
status: dispatched
branch: chrome/viewer-small
opened: 2026-09-29
priority: P3
cost: E
---


Found on PR 3477 (`chrome/poisoned-panels`) in review. The defect predates
that PR.

`PickIndexError`'s `Display` (`crates/viewer/src/pickindex.rs`,
`impl Display for PickIndexError`, the `Node` arm) opens every `Node`
refusal with *"root N's bodies could not be tessellated or indexed: …"*.
For `NodePickError::Standing` that is false. The root has no value, so
nothing was tessellated, and the standing that follows says why.
`frame::index_badge` shows that sentence in its tooltip under the label
*"pick index: waits on feature M, which failed"*, which the tree
draws.

The fix belongs in the `Node` arm's own wording: spell the standing arm
apart (for example *"root N has no value to index: {standing}"*), or let
the payload open the sentence. `frame::downstream_root` already reads
the standing arm as a different case from tessellation and indexing
refusals.
