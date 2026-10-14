---
id: the-tools-pick-time-snapshots-never-follow-a-rename
kind: issue
title: The seat, blend and mate tools keep the label a node had at the pick, not the last it had
status: closed
opened: 2026-10-06
closed: 2026-10-06
priority: P3
parent: node-labels-are-document-data
pr: 4093
branch: emit/tools-respeak
---


Found in review of `a-selected-node-deleted-is-said-by-tag-where-the-tools-say-its-label` (PR 4086).

**The case.** The tools speak a picked node once, at the pick, and
never again:
- `Seats::pick` (`crates/viewer/src/seats.rs:308`);
- `BlendTool::fix` (`crates/viewer/src/blend.rs:559`), whose `HeldTarget::node` is what `BlendEvent::TargetLost` says;
- `MateTool::pick` (`crates/viewer/src/matetool.rs:522`) and the instance-pick refusal in `matetool.rs:191`.

So if a node is renamed after it is picked and then deleted, the tool
says the label it had at the pick, while the selection says the last
label it had. PR 4086 makes the selection's kept nodes
(`DocSession::selection_said`) and the face-frame form's
(`Drafts::datum_face_said`) follow every later document. That
mechanism re-speaks only those two and does not reach the tools' held
`SpokenNode`s.

**The fix.** Re-speak each tool's held `SpokenNode`s from the shown
document after each operation (`SpokenNode::respoken`), at the same
place `Drafts::respeak` runs in `ViewerApp`'s op loop (`app.rs`), or
from a single respeak over every open tool.
