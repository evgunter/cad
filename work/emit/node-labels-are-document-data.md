---
id: node-labels-are-document-data
kind: unit
title: A node's label is document data the kernel stores and speaks: SetLabel, one Label type, kind+label+tag in every sentence
status: review
opened: 2026-10-01
priority: P1
cost: H
pr: 3713
branch: emit/node-labels
---


## Scope (ruled on PR 3565, DESIGN.md Band 1 "Node labels")

- `Doc` gains a map from node id to `Label`, beside `appearance`, `placements` and `witnesses`; never a field of `Node`, so a label is in neither the mint preimage nor any content key.
- One validated `Label` type (non-blank, one line, no control characters), also adopted by `Attr::Label`.
- One edit, `SetLabel { node, label: Option<Label> }`. It refuses a node that is not live, and `DeleteNode` drops the label. `InsertNode` carries no label; a labelled creation is two edits in one undo step. The label is in the save file and the pin.
- Split and inline carry labels on the nodes they move; an instance left by split is unlabelled.
- One spoken-node type (id, kind, label), built by the frame that owns the document when a sentence is made (edit refusals at the raise site; evaluation's node errors in the node frame), never inside a value the evaluation memo reuses. One `Display`: `Extrude "base plate" (3fa9c1d2a0b1)`, `Extrude 3fa9c1d2a0b1` when unlabelled, `node 3fa9c1d2a0b1` when gone. The kind noun moves from `viewer::tree::node_kind` into editor-core.
- Python: `NodeId` repr is the full 16 hex digits; `doc.label(node)`, `DocEdit.set_label`, `label=` on insert helpers.
- Viewer: a labelled row's headline is its label, with kind and tag muted beside it; an unlabelled row shows kind, pose and tag. Create forms propose an editable "Kind N", stored only when committed. Rename is a `SetLabel`.

Sequencing: PR 3631 landed the unlabelled slice first (the spoken node, the tag, the full id on machine channels, the viewer's rows and pickers), before `sibling-branches-mint-one-node-id-for-different-nodes` unit 2 (PR 3594), and took that much of its unit 3. The rest of this row — the label and everything below — lands after PR 3594.

## Carried from PR 3631

The refusal values that still hold a bare `RecipeNodeId`, and the memoized `NodeError`, moved to `refusal-values-speak-the-node-with-its-label` when PR 3713 split the row (that row carries the list). The tag's rule stays as PR 3594 left it: one home on each side, `spoken.rs`'s `write_tag` and the Python suite's `tests/spoken.py::tag`; expected texts spell a tag through `test_utils::refusal::tag`.

Off-question findings to file with it: Python `Doc(label=...)` seeds a `DocumentId`, so rename it (e.g. `seed`) so that "label" means one thing — done in PR 3713 (`Doc(seed=...)`); `Attr::Label` is read by no surface — filed as `attr-label-has-no-reader`; `sentence::Labels` is a third meaning — filed as `sentence-labels-is-a-second-meaning-of-label`.
