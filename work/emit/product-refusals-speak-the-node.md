---
id: product-refusals-speak-the-node
kind: unit
title: ProductError speaks the node with its label at the doors that hold the document, and keeps the tag where a part's evaluation memoizes its sentence
status: open
opened: 2026-10-01
priority: P2
cost: M
parent: node-labels-are-document-data
---


Split from `analysis-door-refusals-speak-the-node`, whose PR spoke range, drive, mc, stackup and the report forms and left product out, because product needs a ruling the others did not. The rule is DESIGN.md Band 1, "Node labels": a refusal raised by a door that holds the document holds a `SpokenNode` built at the raise (`Doc::spoken`); a value the evaluation memo reuses keeps the bare id, and the frame that hands it out speaks it.

## The hits

- `ProductError` (`product.rs`), 6 node fields: `PlacedUnderTwoRoots` (`placed`, `first`, `second`), `Naming` (`node`, and `name`'s minting node), `Graft`, `ContactLineage`; and `SourceFinding::node` under `RootInvalid`. Its sentences are in `ProductError`'s `fmt_labelled` (`Staged`) and `SourceLine::subject` ("root N output M").
- `ProductError::Root(NodeStanding)` carries `NodeStanding`, which is `selection-door-refusals-speak-the-node`'s.

## Why it needs a ruling first

One type, raised at two kinds of site:

- **At a door that holds the document**: `product` / `product_recorded` called by a caller (`checks.rs`, `pncad`'s export, `pncad-py`'s `product_memo.rs`, which memoizes only a gather that succeeds), and by the assembly gate (`assembly.rs`, `AssemblyError::Product`), which crosses into `EditError` (`edit.rs`) and Python's payloads (`pncad-py/src/edit_payload.rs`, `py/assembly.rs`).
- **Inside a part's evaluation**: `eval/parts.rs`'s `product_fault` renders `error.sentence()` into `PartFault::PartProduct { message }`, which lives in `NodeErrorKind` and is memoized. A label spoken into that string would go stale on a rename.

So `ProductError` cannot simply hold `SpokenNode`s. Either the part's evaluation renders a bare-tag sentence (and `memoized-refusals-speak-inner-nodes-through-the-frame` speaks it later), or the error keeps ids and gets a rendering that takes the speaker. The payloads keep the full id either way.

