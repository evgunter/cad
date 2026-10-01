---
id: kernel-door-refusals-beyond-edit-speak-the-node
kind: unit
title: Kernel refusals raised at a door that holds the document, beyond EditError, speak the node with its label
status: review
opened: 2026-10-01
priority: P2
cost: M
parent: node-labels-are-document-data
pr: 3735
branch: emit/kernel-refusals-speak
---


Split from `refusal-values-speak-the-node-with-its-label` (its PR spoke `EditError` and the `NodeError` headline). The rule is DESIGN.md Band 1, "Node labels": a refusal raised by a door that holds the document holds the spoken node (`SpokenNode`, `Doc::spoken`), built at the raise; its typed field keeps the id through `SpokenNode::id`, and a machine channel (a Python payload's `node`) keeps the full id.

## The hits (sweep of `"…node {…}"` in non-test `src/`, at that PR's merge base)

Each needs a ruling first: is the value raised at a door that holds the document, or is it memoized or raised where no document is at hand (then it keeps the tag, `SpokenNode::absent`)?

- `RootFault` (`roots.rs`, `RootFault`'s `Display`): raised by `roots::check` at the edit door (`EditError::Roots`) and by the load door (`SnapshotError::Roots`). The edit door's recourse words in `edit.rs` (`EditError::render`, the `Roots` arm: "drop root …", "list node …") print its ids bare too.
- `RangeRefusal` (`range.rs`), `AssemblyError` (`assembly.rs`), `NodeStanding` (`eval/mod.rs`; its `Display` is also the Python `poisoning` message's opening, `pncad-py/src/py/value.rs`).
- `stackup.rs` (the `Display`s near `StackupError` and `render`/`render_sensitivity`), `product.rs` (`fmt_labelled`), `clearance.rs`, `mc.rs` (`fmt`, `render`), `report.rs` (`render`), `drive.rs`.
- `resolve/mod.rs`, `resolve/pick.rs`, `resolve/hit.rs`, `names/emit.rs`, `names/geompred.rs`, `names/role.rs`.
- `refactor.rs` (`SplitError`, `InlineError`): split and inline hold the source document.
- `edit.rs` `Maintenance`'s `Display` (`Strand`, `OrphanedDeclare`): a report the edit door makes, not a refusal, but it names nodes the same way.
- `pncad/src/export.rs`, `pncad-py/src/py/checks.rs` (`__repr__`, `new`).
- `StableName`'s `Display` (`names/role.rs`): "face name minted by node <tag>". It prints inside about 14 `EditError` sentences: every arm that forwards a `name` (`DeclareNamesMissingNode`, the `Rebind…` and `Appearance…` arms, `NameStepNeverMinted`, `Meta…`). So an edit refusal still names a minting node by a bare tag there. A `StableName` is a stored reference with no document behind it, so the fix is a rendering that takes the speaking document, the same shape as the memoized row's.
- `persist/check.rs`, `persist/mod.rs` (`fmt_labelled`), `mint.rs`: the load door reads bytes that are not a document yet. These most likely keep the tag; say so on the row when it is built.

The grep matches a format string with `node {…}` on one line. It cannot see a node printed under another noun (`instance {}`, `mate {}`, `gauge {}`, `root {}`) or through a helper. So a second pass listed every `pub enum` named `…Error`/`…Fault`/`…Refusal`/`…Standing` that holds a `RecipeNodeId` field. Its kernel hits, with the number of such fields:

- Door-side, for this row: `MintRefusal` 2 (`assembly.rs`), `SelectionRefusal` 1 (`clearance.rs`), `DriveRefusal` 2 and `RefusalReason` 1 (`drive.rs`), `McRefusal` 1 (`mc.rs`), `NamingError` 4 (`names/emit.rs`), `SelectRefusal` 1 (`names/geompred.rs`), `ProductError` 6 (`product.rs`), `RangeRefusal` 4, `SplitError` 11 and `InlineError` 6 (`refactor.rs`), `NodePickError` 2 (`resolve/pick.rs`), `RootFault` 5, `LiftRefusal` 1, `SensitivityRefusal` 2 and `StackupRefusal` 2 (`stackup.rs`), `NodeStanding` 5, `ExportError` 2 (`pncad/src/export.rs`).
- Load door, where the tag likely stays: `SnapshotError` 30, `PersistError` 1.
- Node-local faults that a door renders in its own words: `InputFault` 1 and `AssertionBoundFault` 2 (`node.rs`). The edit door already speaks them through `EditError`.
- Memoized, owned by `memoized-refusals-speak-inner-nodes-through-the-frame`: `NodeErrorKind` 13, `PartFault` 4, `MateFault` 19, `FaceRefusal` 2, `LeverRefusal` 2.

## Built

This row's PR speaks the edit door's family:

- **`RootFault`** holds `SpokenNode`s. `roots::check` takes the door's `speak`. The edit door speaks from the document it was handed: a node the edit is minting by kind and tag, an id neither document holds as `absent` (`edit::spoken_after`). The `Roots` recourse words speak the node too.
- **A name an `EditError` arm forwards** (15 arms) is a `SpokenName`: the `StableName` beside its minting node spoken (`Doc::spoken_name`). It reads `face name minted by Extrude "base plate" (3fa9c1d2a0b1)`, and `node <tag>` when the node is not held. `StableName`'s own `Display` keeps the tag, because a stored reference has no document behind it.
- **`Maintenance`'s `Strand`, `StrandedAppearance` and `OrphanedDeclare`** speak their nodes and names from the document the edit was applied to, which still holds a deleted minting node.

## The load door: it keeps the tag

`SnapshotError` (30 node fields), `PersistError` (1), `persist/check.rs`, `persist/mod.rs` (`fmt_labelled`) and `mint.rs` (`Minted`'s `Display`) stay as they are. This follows the rule already written in `spoken.rs`'s module doc: the bare tag is for "a load door reading bytes that are not a document yet". DESIGN.md's "the label is read off the document when the sentence is made" has no document to read at that door. The label store is itself among what the load door judges (`LabelOnMissingNode`, a blank label), so a sentence quoting a label from the bytes would quote unvalidated text. `RootFault`, which both doors share, is spoken by the load door through `SpokenNode::absent`, so its sentence there is `node <tag>`.

## Split off (each a row, `parent: node-labels-are-document-data`)

- `split-and-inline-refusals-speak-the-node`: `SplitError`, `InlineError`. It also records a defect: `SplitError` prints a cut/kept end as a decimal `u64`.
- `analysis-door-refusals-speak-the-node`: range, drive, mc, stackup, clearance, report, product.
- `selection-door-refusals-speak-the-node`: resolve, pick, hit, names, `NodeStanding`, `AssemblyError`/`MintRefusal`, export, `py/checks.rs`.
- `a-cluster-act-speaks-its-gauges-by-tag`: `ClusterMaintenance`'s sentence. The sweep found it, and no row listed it.
