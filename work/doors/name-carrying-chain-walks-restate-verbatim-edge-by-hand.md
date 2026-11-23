---
id: name-carrying-chain-walks-restate-verbatim-edge-by-hand
kind: issue
title: Walks over name-carrying edges restate verbatim_edge and copied_read by hand, and disagree on member reads
status: open
opened: 2026-11-02
priority: P3
cost: M
---


Found by the PR 6264 re-verify 3 (its item 6, the wider class of the style finding S-2). PR 6264 gave "what does this copy copy" one spelling, `Doc::copied_read` (`crates/editor-core/src/doc.rs`), and "which nodes carry a body's names as they are" one walk over the compiler-enforced census `names::verbatim_edge` (`VerbatimEdge::carried_read`, `crates/editor-core/src/names/role.rs`; the takeover door's `chains`, `crates/editor-core/src/take_over.rs`). Other walks still hand-match `Node::Place` (and `Node::InstantiatePart`) upward instead:

- `crates/editor-core/src/edit.rs`: `reach` and `inside_part`;
- `crates/editor-core/src/refactor.rs`: around `part_names_at` and the walks beside it;
- `crates/editor-core/src/mate/solve.rs`: the copy walk near its member reads.

They also disagree on member reads. Some filter `!source.is_member()` (a `Place` reading one member of a family), as `doc.rs`'s own read-port walk does, while `copied_read` answers the family's read for a member read too. Which of the two a walk wants is its question, and today each answers it in its own spelling.

**When dispatched:** route each walk through `verbatim_edge` / `copied_read`, with the member rule stated once (a `copied_read` that says whether the read is a member, or a sibling that answers only whole-body copies), and each walk choosing it by name.
