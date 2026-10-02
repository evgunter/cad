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

- `ProductError` (`product.rs`), seven `RecipeNodeId` fields and one `StableName`'s minting node: `PlacedUnderTwoRoots` (`placed`, `first`, `second`), `Naming` (`node`, and `name`'s minting node), `Graft` (`node`), `ContactLineage` (`node`); and `SourceFinding::node` under `RootInvalid`. Its sentences are in `ProductError`'s `fmt_labelled` (`Staged`) and `SourceLine::subject` ("root N output M").
- `ProductError::Root(NodeStanding)` carries `NodeStanding`, which is `selection-door-refusals-speak-the-node`'s.

## Why it needs a ruling first

One type, raised at two kinds of site:

- **At a door that holds the document**: `product` / `product_recorded` called by a caller (`checks.rs`, `pncad`'s export, `pncad-py`'s `product_memo.rs`, which memoizes only a gather that succeeds), and by the assembly gate (`assembly.rs`, `AssemblyError::Product`), which crosses into `EditError` (`edit.rs`) and Python's payloads (`pncad-py/src/edit_payload.rs`, `py/assembly.rs`).
- **Inside a part's evaluation**: `eval/parts.rs`'s `product_fault` renders `error.sentence()` into `PartFault::PartProduct { message }`, which lives in `NodeErrorKind` and is memoized. A label spoken into that string would go stale on a rename.

So `ProductError` cannot simply hold `SpokenNode`s. Either the part's evaluation renders a bare-tag sentence (and `memoized-refusals-speak-inner-nodes-through-the-frame` speaks it later), or the error keeps ids and gets a rendering that takes the speaker. The payloads keep the full id either way.

That choice is a design fork, so the row carries `design: true`. It is weighed by a designer pair first, then goes to Ev, before a lane builds it.


## Forward sites inside spoken sentences

Added by `selection-door-refusals-speak-the-node`. These sentences now speak their own nodes, but forward a `ProductError` by its tag `Display` (`{e}`). When this row gives `ProductError` a speaker rendering, they should forward `Said(e, by)`:

- `AssemblyError::Product` (`assembly.rs`, `AssemblyError`'s `say`, the `Self::Product(e)` arm).
- `AssemblyError::Space`'s `refusal` (`assembly.rs`, the same `say`): "the own space of the group rooted at … does not gather: {refusal}".
- `ExportError::Product` (`pncad/src/export.rs`, `ExportError`'s `say`).

`ChecksError::Product` carries a `reason` string, not the error, so it cannot be spoken later. That sentence is made by `Subject::refused` (`checks.rs`, `source.to_string()`), which holds no document; `run_checks` does, so the speaker would have to reach it there.

## Ruled (orchestrator, 2026-10-02): not a fork; PR 3760's pattern settles it

PR 3760 already decided this shape for `AssemblyError` and `ExportError`. Each is a type raised both at doors that hold the document and at sites that don't. Each keeps bare ids and gains `Say`, and the frame that hands it out speaks it with `spoken(doc)`. `ProductError` is the same case, so it follows the same rule:
- `ProductError` (and `SourceFinding`) keep `RecipeNodeId`s and gain `Say`. `Display` is the sentence said by tag.
- `PartFault::PartProduct` stops holding a pre-rendered `message` string. It holds the `ProductError` itself, so the frame can speak it. A part's ids keep their tags unless the frame holds the part, which is `PartFault::spoken`'s pinned-part rule from PR 3782.
- The forward sites listed above forward `Said(e, by)`.
- `ChecksError::Product` gets its sentence from a speaker that `run_checks` builds over the document it holds. Otherwise it keeps the error, not a string.
- Payloads keep the full id.
