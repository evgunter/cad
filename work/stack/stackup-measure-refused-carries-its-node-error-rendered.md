---
id: stackup-measure-refused-carries-its-node-error-rendered
kind: issue
title: stackup's MeasureRefused carries the node error rendered as a String, where the drive now carries it typed
status: open
opened: 2026-10-01
priority: P4
cost: M
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

## Re-homed from FLUX to FLUXHOLD (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXHOLD holds FLUX's rows on the D10 hold (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`). Each waits on the INTENT unit that rebuilds its ground, named in `blocked_on`. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.

## Re-homed from FLUX to STACK (2026-10-10)

(FLUX orchestrator) The row was parked on the D10 hold at FLUX's cut,
on `measure-is-an-operation`. That unit had already closed (PR 4355),
and `MeasureRefused { node, cause: String }` survives it in `stackup.rs`
unchanged. The asymmetry with the drive's typed `MeasureRefusalClass`
is not D10's vocabulary, so the row is open. It goes to STACK, which
claims `stackup.rs` and whose log this row already said to announce on.
FLUX no longer claims `stackup.rs`.
