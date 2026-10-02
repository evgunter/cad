---
id: viewer-refusals-speak-the-node
kind: unit
title: The viewer's own refusal types speak the node with its label, built where the frame owns the document
status: open
opened: 2026-10-01
priority: P2
cost: M
parent: node-labels-are-document-data
---


Split from `refusal-values-speak-the-node-with-its-label` (its PR spoke `EditError` and the `NodeError` headline, which the feature tree's failed rows and carried lines now draw through `NodeError::spoken` and `CarriedLevel::line_in`). The rule is DESIGN.md Band 1, "Node labels": where the frame owns the document, the refusal holds the spoken node, built at the raise.

## The types (from the parent row, with the sweep's hits)

- `MateToolError` (`matetool.rs`, `NotAnInstancePick`, `SamePick`; its `Display` near `face_of`).
- `HeldRefusal` (`sketch.rs`).
- `SlotUnitFault` (`props.rs`).
- `Refusal` and `FaceFrameFault` (`session/refuse.rs`).
- `DuplicateFault` (`combine.rs`).
- `AdmissionFault` (`display.rs`).
- Also hit by the same sweep: `blend.rs` and `pickindex.rs` (`Display`s), `session.rs` (`set_program_of`), `drafts.rs` (`profile_edit`).

The sweep matched `"…node {…}"` in non-test `src/` at that PR's merge base. A second pass listed the viewer's `pub enum …Error`/`…Fault`/`…Refusal` types holding a `RecipeNodeId` field, whatever noun prints it. It found the six types above, plus `EdgeNameFault` and `PickIndexError` (`pickindex.rs`) and `Standing` (`session/select.rs`).

## A kept refusal freezes its label

`Refusal::Edit` (`session/refuse.rs`, its `Display`, "the edit was refused: …") holds the kernel's `EditError` value. A refusal the viewer keeps on screen (the status line) therefore says the label as it stood when the edit was refused. A rename after that does not move it. That matches the ruling ("read off the document when the sentence is made"), but it is a sentence that outlives its moment. This row decides whether a refusal the viewer keeps is re-spoken when drawn or is cleared by the next edit.

## Kernel refusals the viewer draws

Added by `selection-door-refusals-speak-the-node`. These kernel types hold bare ids (memoized, or raised from an evaluation alone), and each now has `spoken(doc)`, its sentence with each node as `doc` holds it. Their own `Display` says the tag. The viewer frame holds the document, so where it draws one of them it should speak it: `NodeStanding`, `NodePickError`, `NameLookupError`, `HitTestError`, `UnnamedEntity`, `SelectRefusal`, `InterrogateError`, `ResolveError`, `Diagnosis`, `ResolveIndeterminate`, `AssemblyError`, `MintRefusal`, `ChecksError` and pncad's `ExportError`.

Added by `memoized-refusals-speak-inner-nodes-through-the-frame`: `NodeErrorKind`, `MateFault`, `PoseRefusal`, `LeverRefusal`, `FaceRefusal`, `OffsetCheck`, `NamingError` and `SelectionRefusal` write their sentence over `spoken::Speaker` too (`spoken_by(value, doc)`, or `MateFault::spoken`/`PoseRefusal::spoken`). The tree already speaks a failed row (`NodeError::spoken`) and a carried level (`CarriedLevel::line_in`). `CarriedLevel::line_in_part(part, tol)` and `PartFault::spoken(doc_ref, part, tol)` have no callers yet: a frame that holds the resolved part would speak a part's level and fault through them. A status line that prints a kind's `Display` (the refused union's `Refusal`, `session/refuse.rs`) still says tags.

Added by `product-refusals-speak-the-node`: `ProductError` has `spoken(doc)` too, and `PartFault::PartProduct` now holds the gather's refusal whole, so `PartFault::spoken` says its nodes from the part. The viewer draws a gather refusal's `Display` in `frame::product_badge` (the session's `product_fault`) and in `scene.rs`'s `NoProduct`; both hold the document.

The viewer reads them in `pickindex.rs` (most), `tree.rs`, `frame.rs`, `pane/properties.rs`, `pane/create.rs`, `pane/viewport.rs`, `session.rs`, `session/refuse.rs`, `session/select.rs`, `matetool.rs`, `blend.rs`, `combine.rs`, `idpass.rs` and `app.rs`. Not every read prints a sentence; the ones that do are this row's to speak.
