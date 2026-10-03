---
id: measure-refused-reduces-the-typed-refusal-to-its-name
kind: issue
title: RefusalReason::MeasureRefused carries a measure's typed refusal as its name, mixing the wiring's and the clearance engine's vocabularies
status: review
opened: 2026-09-24
branch: props/recourse-grammar
pr: 3942
---


`RefusalReason::MeasureRefused { node, class: &'static str }`
(`crates/editor-core/src/drive.rs`, the arm at ~:540) is built by
`box_independent_measure_class` (~:2073) from a `NodeErrorKind`, and the
`class` it keeps is a string drawn from two vocabularies: the wiring's own
`"selection_kind"` for `MeasureSelectionKind`, and the clearance engine's
`ClearanceRefusal::name()` for `MeasureClearanceRefused`. So one hop after
the measure layer started carrying the engine's typed refusal
(`work/port/min-clearance-refusal-stringly-twin.md`, PR #3188 — PORT has since closed; recover it with `git show 6a173492f:work/port/min-clearance-refusal-stringly-twin.md`), the drive
reduces it back to its name; a reader of `RefusalReason` that wants the
arm matches a string, and nothing stops the two vocabularies colliding.
The serialized form (`drive.rs` ~:1938, `measure_refused {node} {class}`)
reads only the name, so a typed payload need not move the goldens.

`RefusalReason` derives `Debug, Clone, PartialEq` (~:491), and
`ClearanceRefusal` derives the same, so carrying a typed class — an enum
naming which `NodeErrorKind` arm, holding the engine's refusal for the
clearance arm — costs no derive.

A sibling one crate-file over, same shape: `stackup.rs`'s
`MeasureRefused { node, cause: String }` (~:225) carries the node error
rendered, on the stated ground that `NodeErrorKind` is neither `Clone` nor
`PartialEq`.

Found by PR #3188's review. That PR's sweep for the shape grepped
`pub (class|kind|name): &'static str` — a struct-variant field has no
`pub`, so the pattern could not match this arm.

## Resolved (props/recourse-grammar)

`RefusalReason::MeasureRefused` now carries
`drive::MeasureRefusalClass`, a closed type with the two vocabularies
apart: `SelectionKind` is the wiring's own arm, and
`Clearance(clearance::ClearanceRefusal)` holds the engine's typed
refusal itself. `box_independent_measure_class` returns that type.

The serialized form is unchanged — `measure_refused {node} {class}`
reads `MeasureRefusalClass::name()`, which is the wiring's own name for
its arm and `ClearanceRefusal::name()` for the engine's — so no receipt
golden moved.

Not taken here, and filed: `stackup.rs`'s `MeasureRefused { cause: String }`,
whose stated ground is that `NodeErrorKind` is neither `Clone` nor
`PartialEq` — typing it is that derive work, not this hop.
