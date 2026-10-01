---
id: analysis-door-refusals-speak-the-node
kind: unit
title: The analysis doors' refusals (range, drive, mc, stackup, clearance, report, product) speak the node with its label
status: review
opened: 2026-10-01
pr: 3749
priority: P2
cost: M
parent: node-labels-are-document-data
---


Split from `kernel-door-refusals-beyond-edit-speak-the-node`. The rule is DESIGN.md Band 1, "Node labels": a refusal raised by a door that holds the document holds a `SpokenNode` built at the raise (`Doc::spoken`), and a machine channel keeps the full id through `SpokenNode::id`. Each type needs a ruling first: is it raised at a door that holds the document, or memoized or raised with no document at hand (then it keeps the tag, `SpokenNode::absent`)?

## The hits (node fields counted at the parent row's sweep)

- `RangeRefusal` (`range.rs`), 4.
- `DriveRefusal` 2 and `RefusalReason` 1 (`drive.rs`).
- `McRefusal` (`mc.rs`), 1; also `mc.rs`'s `fmt` and `render`.
- `stackup.rs`: `LiftRefusal` 1, `SensitivityRefusal` 2, `StackupRefusal` 2, and the `Display`s near `StackupError`, `render` and `render_sensitivity`.
- `SelectionRefusal` (`clearance.rs`), 1.
- `report.rs` (`render`).
- `ProductError` (`product.rs`), 6, and `product.rs`'s `fmt_labelled`. `ProductError` crosses into `EditError`'s and Python's payloads (`pncad-py/src/edit_payload.rs`, `py/assembly.rs`); those keep the id.

The `pncad/src/analysis.rs` wrappers and the Python analysis bindings (`pncad-py/src/py/analysis.rs`) read these types; their payloads keep the full id.

## Built

Each type was ruled by where it is raised and whether a memo keeps it.

- **Spoken at the raise** (a door that holds the document, nothing memoized): `RangeRefusal`'s four node arms (`derive`; `UnknownNode` is `SpokenNode::absent`), `DriveRefusal` (`drive`), `McRefusal::NominalDoesNotBuild` (`monte_carlo`), `SensitivityRefusal`, `PairingViolation` (three arms the parent sweep's `…Error`/`…Refusal` pattern did not list) and `StackupRefusal` (`sensitivities`, `stackup`). `SensitivityRefusal::VerdictNotOfThisBuild` speaks from the document asked about only when that document's replay holds the node. A node only the drive's record names is spelled in the document the drive ran on, which may be another one, so it stays `absent`.
- **Report values keep the bare id, and their human forms speak.** `Stackup::render`, `render_sensitivity`, `McReport::render` and `LeafHistogram::render` take the document to speak from. A report is a value whose goldening form `ReportCache` serves by content key, and labels are outside every content key. The goldening forms (`serialize`) print the full id. `Stackup::serialize` printed the tag through `render_sensitivity` before. `SensitivityOutcome`, `LiftRefusal`, `McMeasure`, `McAssertion` and `LeafHistogram` keep `RecipeNodeId`.
- **Memoized, kept as a tag**: `SelectionRefusal::NoSuchBody` (`clearance.rs`) rides in `ClearanceRefusal`, which `NodeErrorKind::MeasureClearanceRefused` carries. It is listed on `memoized-refusals-speak-inner-nodes-through-the-frame`.
- **No sentence**: `RefusalReason::MeasureRefused` is leaf-verdict data. Only the goldening form (`render_reason`) prints it, with the full id.
- **Split off**: `ProductError` is `product-refusals-speak-the-node`. A part's evaluation memoizes its sentence (`PartFault::PartProduct`), so it needs its own ruling.

`report.rs`'s `MassBudget::render` names no node.
