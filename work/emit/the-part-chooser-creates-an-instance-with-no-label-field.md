---
id: the-part-chooser-creates-an-instance-with-no-label-field
kind: issue
title: An instance created from the part chooser gets no proposed, editable label
status: open
opened: 2026-10-01
priority: P3
cost: E
refs: [node-labels-are-document-data]
---

PR 3713 gave every create form an editable proposed label (`ViewerBehavior::creation_label_row`, `push_labelled` in `crates/viewer/src/pane/create.rs`), except the part chooser: `add_part_ui` pushes a bare `SessionOp::AddInstance { id }` from a listing window with no commit button to hang a field on. An instance — the case PR 3565 named, two instances of one part told apart only by pose — is therefore created unlabelled and renamed afterwards in Properties. Wants a label row in the chooser window (noun `InstantiatePart`) and `push_labelled` at its commit.
