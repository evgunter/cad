---
id: node-labels-are-document-data
kind: unit
title: A node's label is document data the kernel stores and speaks: SetLabel, one Label type, kind+label+tag in every sentence
status: open
opened: 2026-10-01
priority: P1
cost: H
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

- **Refusal values hold a bare `RecipeNodeId`** and print `node <tag>`, with no kind: every kernel refusal that names a node (`EditError`, `RangeRefusal`, `MateFault`, `AssemblyError`, `PartFault`, `NodeStanding`, …) and the viewer's own refusal types — `MateToolError` (`matetool.rs`, `NotAnInstancePick`, `SamePick`), `HeldRefusal` (`sketch.rs`), `SlotUnitFault` (`props.rs`), `Refusal` and `FaceFrameFault` (`session/refuse.rs`), `DuplicateFault` (`combine.rs`), `AdmissionFault` (`display.rs`). The ruling builds the spoken node at the raise site; that is a type change on each variant that names a node, and it lands here with the label, which those sentences need too.
- **The memoized `NodeError` keeps a bare id** (`node <tag> failed: …`); its node frame speaks it once the label exists.
- **The tag is the id's HIGH 48 bits**, its 12-hex prefix — the `DocRef` pin-prefix rule the ruling cites — switched in PR 3594 when ids became a digest prefix (`u64::from_be_bytes` of the hash head). One home on each side: `spoken.rs`'s `write_tag`, and the Python suite's `tests/spoken.py::tag` (read by `test_assembly_eval.py`, `test_assembly_author.py` and `test_document.py`); expected texts spell a tag through `test_utils::refusal::tag`, and a fixture that forges ids by hand takes them from `test_utils::refusal::tagged`, whose tag reads its argument.

Off-question findings to file with it: Python `Doc(label=...)` seeds a `DocumentId`, so rename it (e.g. `seed`) so that "label" means one thing; `Attr::Label` is read by no surface.
