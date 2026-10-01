---
id: selection-door-refusals-speak-the-node
kind: unit
title: The resolve, pick, naming, standing, assembly and export refusals speak the node with its label where a door holds the document
status: open
opened: 2026-10-01
priority: P2
cost: H
parent: node-labels-are-document-data
---


Split from `kernel-door-refusals-beyond-edit-speak-the-node`. The rule is DESIGN.md Band 1, "Node labels": a refusal raised by a door that holds the document holds a `SpokenNode` built at the raise (`Doc::spoken`), a name speaks its minting node through `Doc::spoken_name`, and a machine channel keeps the full id. Each type needs a ruling first: is it raised at a door that holds the document, or memoized or raised where no document is at hand (then it keeps the tag)? Several of these are raised during evaluation (`NamingError` lands inside `NodeErrorKind`), and those belong to `memoized-refusals-speak-inner-nodes-through-the-frame` instead; say so here when ruled.

## The hits (node fields counted at the parent row's sweep)

- `resolve/mod.rs`: `ResolveError` (3 names) and `Diagnosis` (1 name), and its `node {…}` sentences (12 format strings).
- `NodePickError` (`resolve/pick.rs`), 2; `resolve/hit.rs`'s one sentence.
- `NamingError` (`names/emit.rs`), 4 nodes and 2 names.
- `SelectRefusal` (`names/geompred.rs`), 1 node and 3 names.
- `NodeStanding` (`eval/mod.rs`), 5. Its `Display` is also the opening of the Python `poisoning` message (`pncad-py/src/py/value.rs`).
- `AssemblyError` and `MintRefusal` (`assembly.rs`): `MintRefusal` holds 2 nodes and 1 name.
- `ExportError` (`pncad/src/export.rs`), 2.
- `pncad-py/src/py/checks.rs` (`__repr__`, `new`): a `repr` is a machine channel and keeps the full id; say which of the two is a sentence.
