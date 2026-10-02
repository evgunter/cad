---
id: viewer-refusals-speak-the-node
kind: unit
title: The viewer's own refusal types speak the node with its label, built where the frame owns the document
status: closed
pr: 3806
opened: 2026-10-01
closed: 2026-10-02
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

## Ruled (implementer, 2026-10-02; revised in review): a kept refusal is a sentence made once, at the end of its batch, and the next accepted act retires it

Not a fork. The status line holds a `frame::Message`, a string made once per frame. Two rules keep the label it says the document's:

- **Made at the end of the batch.** `frame::batch_refusal` speaks the batch's refusal again from the committed document the batch leaves (`Refusal::respoken`), so a rename later in the same batch is the label it says. A batch that replaced the document (`Open`, `NewDocument`) leaves the refusal as raised: the new document's ids say nothing about it. `Refusal::Edit` cannot be spoken again yet (the kernel door speaks `EditError` at the refusal), filed as `edit-error-respeaks-from-a-later-version`.
- **Retired by the next accepted act.** `frame::batch_status` clears the line on any batch that acts (`frame::acts`), and `SessionOp::SetLabel` acts, so the frame that renames a node clears a refusal that said its old label. This holds only while every label-changing op answers `acts` true; the viewer README's channel section says so.

Re-speaking from a later version is sound because within one document's history an id names one node: an id is the head of the mint chain at its insert, so two versions that part from one value mint different ids from there on (`Refusal::respoken`'s doc comment; `node_labels::an_undo_then_a_different_insert_mints_a_different_id`).

Each of the viewer's own refusal types is spoken at the raise, from the document the raising door holds: `Refusal`'s `NoSuchSlot`, `WrongNodeKind` and `ProfileEditStale` (the committed document), `FaceFrameFault` and `DuplicateFault` (the landed document their evaluation was taken of), `AdmissionFault` and `SlotUnitFault` (the document the admission or slot test reads), `MateToolError` (the landed pair `MateTool::proposal` is handed), and `HeldRefusal` (the document `held_loops` reads). A kernel `Say` value they carry whole (`InterrogateError`, `NodeStanding`, `NodeErrorKind` in `RefusedBoolean`) keeps the `HeldNodes` its sentence names (`held_by`), as `EditError::MateRefused` does. Each has `respoken(doc)`. The mate tool's notice is spoken again from the session's document (`MateToolError::respoken`), the one its panel line speaks, so a rename that has not landed reads the same in both. The at-rest badge speaks `AssemblyError` from the landed document (`AssemblyError::spoken`), and the product badge and `SceneError::NoProduct` speak the gather's refusal (`viewer-product-badge-speaks-the-node`).

## Split

- `viewer-pick-path-refusals-speak-the-node`: `PickIndexError`, `EdgeNameFault`, `EdgeNamesRefused`, `PickError`, `BlendEvent`/`BlendTarget`, `IdAnswer`. The pick path has no document, so these take `Say`.
- `viewer-panes-speak-the-kernel-refusals-they-draw`: `Standing`'s verdict, `app::indeterminate_wording`, `line_in_part`/`PartFault::spoken`, and the remaining kernel reads.
- `viewer-product-badge-speaks-the-node`: `frame::product_badge` and `scene.rs`'s `NoProduct`, carried by this row's PR once #3794 merged.
- `edit-error-respeaks-from-a-later-version`: `Refusal::Edit` inside its own batch.
