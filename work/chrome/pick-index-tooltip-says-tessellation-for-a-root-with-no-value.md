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

The `Node` arm now claims only what is true of every payload: *"root N
could not be indexed: {error}"*. Why (no value, not a body, no such
body, a tessellation or indexing refusal) is the payload's own sentence.
Splitting the standing out alone would have left the `NotABody` arm
saying *"could not be tessellated or indexed: … so there is nothing to
tessellate and index"*.

"Which node has no value" has one home, `PickIndexError::standing`
(`crates/viewer/src/pickindex.rs`), over `PickIndexError::restated`: a
match naming every variant of `PickIndexError`, `NodePickError` and
`NameLookupError`. It covers the name doors' `NameLookupError::Standing`
as well as the build's. `frame::downstream_root` and
`frame::index_refusal_as_drawn` both read it and no longer carry nested
matches of their own. That also makes a name-door standing badge as
downstream of its cause, as the build's already did.

Pinned word for word in `tests/frame_policy.rs`: the failed-root tooltip
and the never-ran label. `tests/error_display.rs`,
`pick_index_error_says_only_that_its_root_was_not_indexed` (every arm)
and `pick_index_error_reads_a_standing_at_the_build_and_at_the_name_doors`.
