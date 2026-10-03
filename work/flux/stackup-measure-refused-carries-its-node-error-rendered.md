---
id: stackup-measure-refused-carries-its-node-error-rendered
kind: issue
title: stackup's MeasureRefused carries the node error rendered as a String, where the drive now carries it typed
status: open
opened: 2026-10-01
---


(PROPS' recourse-grammar lane, from
`work/props/measure-refused-reduces-the-typed-refusal-to-its-name`,
which names this as a sibling one crate-file over.)

## What

`crates/editor-core/src/stackup.rs`'s `MeasureRefused { node, cause: String }`
carries the node error RENDERED, on the stated ground that
`NodeErrorKind` is neither `Clone` nor `PartialEq`.

Its sibling one file over is now typed: PROPS' unit replaced
`drive::RefusalReason::MeasureRefused`'s `class: &'static str` with
`drive::MeasureRefusalClass`, a closed type holding
`clearance::ClearanceRefusal` itself for the engine's arm. So the drive's
hop is typed and the stackup's is not, and the asymmetry now has no
reason beyond the two missing derives.

## What would close it

Either `NodeErrorKind` gains `Clone` and `PartialEq` (which is the work
the stated ground names, and which a reader of the stackup report needs
anyway to match an arm), or `stackup.rs` carries the same
`MeasureRefusalClass` the drive does, narrowed to what the stackup's own
producer can refuse with.

## Why it is filed here

`crates/editor-core/src/stackup.rs` is claimed by `flux` and `stack`
(and was PROPS', which closes with the unit that found this). FLUX is
PROPS' successor, so it lands here; a lane taking it should announce the
seam on `stack`'s log.
