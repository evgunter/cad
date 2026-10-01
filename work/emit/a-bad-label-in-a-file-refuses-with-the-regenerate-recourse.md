---
id: a-bad-label-in-a-file-refuses-with-the-regenerate-recourse
kind: issue
title: A save file's bad label text refuses as Unreadable, whose recourse (regenerate the file) is wrong for it
status: open
opened: 2026-10-01
priority: P4
cost: M
refs: [node-labels-are-document-data]
---

A label in a save file whose text breaks the label rule (blank, a line break, a control character) is refused at the parse by `Label`'s `Deserialize` (`crates/editor-core/src/label.rs`), through `serde::de::Error::custom`. That is serde's `Data` class with no typed refusal recorded, so `persist::parse_err` lands it on `PersistError::Unreadable`, whose recourse is `REGENERATE_RECOURSE` — "a file this build cannot read". The file is readable; its label is bad, and regenerating reproduces it. The fix is a typed arm: either a `persist::refusal::record` path for a `LabelFault` (the channel is `DimensionError`-only today), or reading the text unvalidated and refusing in `validate_document` as a `SnapshotError` naming the node and the fault. Found in PR 3713.
