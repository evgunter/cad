---
id: step-import-source-debug-in-prose-panics-the-binding
kind: issue
title: step-import renders {source:?} on TransformError and BooleanError, both of which have a Display — a live panic at py::typed_err
status: open
opened: 2026-09-04
refs: [debug-in-prose-at-blend-and-step-import]
priority: P0
cost: E
---


## Split out of `debug-in-prose-at-blend-and-step-import` (2026-09-04)

That item carried two live panics on two programs' ground and its own
`## Home` offered the split: *"the two sites are FILLET's and EXCH's
respectively. Re-home by header edit, or split into two items if the
programs prefer to take them separately."* Split during the
`work/issues/` re-home sweep, because a live panic on a public door
should not wait on the other program's slate. The carrier keeps the
blend half and stays on FILLET's board; **read it for the sweep's
method and its stated blind spots**, which are this finding's evidence
too and are not restated here.

## The finding

`crates/step-import/src/error.rs:455, 461` render `{source:?}` on
`Placement.source` (`topo::TransformError`) and `Instance.source`
(`Box<topo::BooleanError>` — the `graft_disjoint` door's error, NOT
the `EulerOpError` that the sibling `Assembly` arm carries; the
carrier item misnamed this payload and the misnaming was inherited
here), **both of which already have a `Display`**
(`topo/src/transform.rs:170`, `topo/src/boolean/mod.rs:1309`). This is
the sharper of the two sites: the payload can name
itself and the consumer composes a debug rendering anyway, which is the
exact inversion of the standing rule that the layer which raised a
failure names it.

## It is live, not cosmetic — `sure`

Most of `BooleanError` is struct-shaped — `ScaffoldingOperand {
operand, edge }`, `NonMaximalFaces { .. }`, `Escalated { .. }`, the
four `Curved*Unsupported { .. }` arms and more — and `TransformError`
likewise (`Pcurve { .. }`, `Certify { .. }`, `NotRigid { .. }`,
`NullScaffold { .. }`, …), so `{source:?}` yields `ScaffoldingOperand
{ operand: .., edge: .. }` and its kin, carrying the field-brace
fingerprint `" { "`. `crates/pncad-py/src/errors.rs`'s
`reads_as_prose` rejects
that fingerprint and `py::typed_err` asserts it on **every** raise,
live under release. `crates/pncad-py/src/py/value.rs:1341` raises
`err.to_string()` through `typed_err` under a comment stating that
*every* arm of `StepImportError` is reachable there.

So a STEP import whose assembly placement or disjoint graft refuses
panics the Python binding where that arm means to refuse gracefully —
the same panic PR 1779 closed in the tier-3′ door, in another one.

## What it does not wait on

The durable fix for the class is a mechanical guard that renders every
`Display`-reachable refusal at every struct-shaped payload variant and
asserts `reads_as_prose`; it is cut as FIX's
`prose-gate-has-no-mechanical-guard`. **This point fix is worth taking
before that guard lands rather than after** — the carrier item's own
conclusion, and it holds here.

## Home

`work/exch/` — `crates/step-import/` is EXCH's territory and
code-quality Track U's, which EXCH claims for its STEP/STL rows.
