---
id: a-selected-node-deleted-is-said-by-tag-where-the-tools-say-its-label
kind: unit
title: A selection whose node is deleted says it by tag, where the seats, mate and blend tools say the label it had when picked
status: open
opened: 2026-10-02
priority: P3
cost: M
parent: node-labels-are-document-data
refs: [viewer-panes-speak-the-kernel-refusals-they-draw]
---


Found in review of `viewer-panes-speak-the-kernel-refusals-they-draw` (PR 3827).

**The case.** A picked face or edge whose minting node is deleted gets the selection verdict (`pane::properties::standing_verdict`, `Resolution::Failed`), which says `ResolveError::NodeGone` from the landed document. That document no longer holds the node, so the sentence says `node <tag>`. `pane::properties::verdict_tests::a_pick_verdict_says_its_nodes_from_the_landed_document` pins this tag case. A selected node that was deleted (`Standing::Node { present: false }`) reads the same way: `standing_ui` draws `tree::node_label` over the committed document beside "deleted", and that document says it by tag.

**The tools say the same fact another way.** They snapshot the node when it is picked and keep its `SpokenNode`:
- `blend::BlendEvent::TargetLost`, whose `node` is "the target's node as the document spoke it when the target was picked";
- `seats.rs`'s held `SpokenNode`s, for the face-frame and datum seats;
- `matetool.rs`.

So a deleted node is said by its last label in the tools and by its tag in the selection.

**The fix.** `session::select`'s `Selection` (`FaceSelection`, `EdgeSelection`, and `Selection::Node`) snapshots the picked node's `SpokenNode` from the document it was picked in, when it is selected, as the tools do. `standing_verdict` and `standing_ui` then say a node the landed or committed document no longer holds by that snapshot, and a node it does hold from the document, which is the label now. Selections are made by `SessionOp::Select` from several doors (viewport pick, tree click, the checks window), and each must hand it the document it picked in.

The code is viewer ground (`crates/viewer/src/session/select.rs`: chrome, polish, vseam), on emit's node-labels slate.
