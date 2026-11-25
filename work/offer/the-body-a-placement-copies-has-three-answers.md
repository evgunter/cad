---
id: the-body-a-placement-copies-has-three-answers
kind: issue
title: viewer/editor-core: the body a placement copies has three answers that differ at a named member chain and at a pick
status: open
opened: 2026-11-23
priority: P2
---

Found in review of PR 7356 (the relations unit). Three sites each answer "the body this placement copies", and they disagree in two cases.

The three answers:
- **`Doc::copied_node`** (`crates/editor-core/src/doc.rs`, over `copy_chain`/`source_root`) walks every source step down to the first read that is no source. A pick's copy copies nothing. `Doc::placements_of`, `placements_of_var`, `ReadIndex::placements_of` and so the tree badge read it.
- **`world::copy_source`** (`crates/viewer/src/world.rs`) reads `through_sources(source.read)`, which stops at a member read's family source. For `xs = Row`, `m = xs[1]`, `Place(m)` it answers `xs`, not Row's body. The re-point offer and the way to the original body read it.
- **The placement editor's "copy of"** (`crates/viewer/src/pane/create.rs`, `copy_of_row`) reads `through_sources(source).read` and then `operation_of`, which follows a pick to the placement picked from. A pick's copy reads "copy of: <that placement>", while the badge says it copies no body of it.

Where they differ:
- **A copy of a member of a named family source.** `copied_node` answers Row; `copy_source` stops at `xs`.
- **A pick's copy.** `copied_node` answers none; the editor's "copy of" answers the placement picked from.

Done when one kernel query answers the body a placement copies, the other two read it, and each case is decided once and pinned. Rows near it: `lands_unplaced::a_copy_of_one_piece_reads_the_placement_and_copies_none_of_it` and `editor-core named_source::a_copy_of_a_named_familys_member_copies_the_body_the_family_reads`.
