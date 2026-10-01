---
id: analysis-door-refusals-speak-the-node
kind: unit
title: The analysis doors' refusals (range, drive, mc, stackup, clearance, report, product) speak the node with its label
status: open
opened: 2026-10-01
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
