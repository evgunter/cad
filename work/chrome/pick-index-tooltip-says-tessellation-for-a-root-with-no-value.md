---
id: pick-index-tooltip-says-tessellation-for-a-root-with-no-value
kind: issue
title: The pick-index tooltip says a root's bodies could not be tessellated or indexed when the root has no value at all
status: closed
closed: 2026-09-29
pr: 3487
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

## Closed (2026-09-29, `chrome/viewer-small`)

`PickIndexError::valueless_root` (`crates/viewer/src/pickindex.rs`) is
the one home of "this refusal is a root with no value": a `match` naming
every `NodePickError` and every `PickIndexError`. The `Node` arm's
`Display` reads it and says *"root N has nothing to index: {standing}"*,
keeping *"could not be tessellated or indexed"* for the arms where
something was. `frame::downstream_root`, which asked the same question
in its own nested match, now calls it; `frame::index_refusal_as_drawn`
is untouched and still re-reads the standing through the tree before
the sentence is written.

Pinned: `tests/frame_policy.rs`, the failed-root tooltip
(*"pick index: root 5 has nothing to index: node 5 failed, so it has no
value — fix the node's own failure"*) and the never-ran label, word for
word; `tests/error_display.rs`,
`pick_index_error_says_a_root_with_no_value_has_nothing_to_index`.
