---
id: persist-door-refusals-speak-the-node
kind: unit
title: The load and save doors' refusals (SnapshotError, PersistError) speak the node from the document they judge
status: closed
opened: 2026-10-01
closed: 2026-10-01
priority: P2
cost: M
branch: emit/persist-speak
pr: 3741
parent: node-labels-are-document-data
---


Split from `kernel-door-refusals-beyond-edit-speak-the-node`, whose PR spoke `RootFault` at every door. The rule is DESIGN.md Band 1, "Node labels", which makes no exception for loading. The load and save doors share one validator, `persist::check::validate_snapshot(doc: &ProfileDoc, …)`. It judges a deserialized document whose labels `Label`'s `Deserialize` has already admitted (`Label::new`). The `LabelOnMissingNode` check runs late (after the node walk, and after every walk `validate_document` runs before `validate_snapshot`), and that order is not load-bearing: `Doc::spoken` reads a label only for a node the document holds, so a label keyed to a missing node is never spoken. So each refusal it raises holds a `SpokenNode` built with `doc.spoken(..)`, which is `absent` for an id the document does not hold, as everywhere else. A machine channel (a Python payload) keeps the full id through `SpokenNode::id`.

## The arms (node fields, at the parent PR's merge base)

`SnapshotError` (`persist/check.rs`):
- `NodeNotMinted` 1, `StepIds` 1, `DanglingInput` 2, `ForwardInput` 2, `DeclareInput` 2.
- `WitnessSite` 1, `WitnessOnMissingNode` 1, `LabelOnMissingNode` 1.
- `PlacementSite` 1, `PlacementNonFinite` 1, `PlacementImproper` 1, `PlacementNonRigid` 1, `PlacementNotGauge` 2, `PlacementRule` 1, `MateAlignment` 1.
- `SlotDimension` 1, `SlotUnknownDocParam` 1, `SlotDocParamDimension` 1, `PayloadUnknownDocParam` 1, `PayloadDocParamDimension` 1.
- `MeasureRefs` 1, `InputList` 1, `AssertionTarget` 2, `AssertionBound` 2.

Also:
- `SnapshotError`'s two `StableName` fields and `NonFiniteSite`'s one, which speak through `Doc::spoken_name`.
- `PersistError::ProfileProgram` (`persist/mod.rs`, its `fmt_labelled`), raised from the same validator (`persist/check.rs`).
- `FrameSite::subject`, which the load door calls with `SpokenNode::absent` today.

`mint.rs`'s `Minted` `Display` is a mint-log entry, not a refusal raised over a node; say whether it changes.

The Python payloads for `SnapshotError` (`pncad-py/src/tags.rs`, and the persist payload projection) keep the full id.
