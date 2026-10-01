---
id: kernel-door-refusals-beyond-edit-speak-the-node
kind: unit
title: Kernel refusals raised at a door that holds the document, beyond EditError, speak the node with its label
status: open
opened: 2026-10-01
priority: P2
cost: M
parent: node-labels-are-document-data
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
- `persist/check.rs`, `persist/mod.rs` (`fmt_labelled`), `mint.rs`: the load door reads bytes that are not a document yet. These most likely keep the tag; say so on the row when it is built.

The grep matches a format string with `node {…}` on one line. It cannot see a node printed under another noun (`instance {}`, `mate {}`, `gauge {}`, `root {}`) or through a helper. A second pass for `RecipeNodeId` fields in `pub enum …Error`/`…Fault`/`…Refusal` types closes that gap.
