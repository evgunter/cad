---
id: at-rest-stored-contradictions-and-unminted-sides-panic-once-import-validates-everything
kind: issue
title: geom-brep/topo: at-rest stored contradictions and unminted sides panic once the import door validates everything
status: parked
opened: 2026-10-19
blocked_on: [can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches]
---


(Filed by the ENCL implementer of `a-proven-zero-is-definite-yet-ends-as-no-size-at-rest`.)

## What

`[tolerance.recourse]` (iv), as ratified on PR 5134, says a stored contradiction and an unminted refused side read over a body at rest past the import door are a kernel bug and panic (`[failure]`). The code keeps a typed ending there, `KERNEL_OR_FILE_DEFECT_ENDING`, because a damaged file can still reach the validator: adoption keeps the file's carriers verbatim, and `step-import-reads-a-negative-surface-radius-unchecked-before-edge-certification` is open (`can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches`).

The sites that turn into the panic:

- `crates/geom-brep/src/recourse.rs`, `SizedDecision::words`: the `(StoredContradiction | Unminted, ReadAt::Run(Reading::AtRest))` arm picks `RefusedReading::ContradictsStoredData`. Its readers at rest include `crates/topo/src/validate.rs` `classify_certify` (`e.ending(Reading::AtRest)`) and the validator's own decision table (`CertDecision`'s `recourse(arm, Reading::AtRest)` rows).
- `crates/geom-core/src/predicate.rs`, `Shows::ending`: the `RefusedReading::ContradictsStoredData` arm, which would become the panic (or go, if `words` panics first).
- `crates/geom-brep/src/pcurve_cache.rs`, `PcurveCheck::recourse`: `ParamSpan`, `AzimuthPeriod` and `TubePeriod` route to the edge decisions, so a pcurve span or period refusal at rest panics with them (this settles `pcurve-span-refusals-name-a-damaged-file-for-a-kernel-derived-cache`).

## The validator cannot tell the import gate's call from a later one

The import's shared at-rest gate (`[import]` step 4) calls `topo::validate_geometric(body, tol)` (`crates/step-import/src/lib.rs`, the gate after assembly), the same function with the same arguments a native caller uses, by design ("import holds no idea of validity of its own"). Every ending the validator composes reads `Reading::AtRest`, never `ReadAt::File`. So the validator cannot know whether it is the import door or a later read.

That leaves two ways to land the panic:

1. EXCH's adoption certifies everything the validator checks (every stored contradiction and unminted side is refused at adoption, typed, naming the file), so the validator during import sees only kernel bugs too. Then the panic needs no door distinction.
2. The validator takes the door it reads at (a `ReadAt` parameter, or an import-gate entry point), so a refusal at the gate ends as the file's and a later one panics. That reverses `[import]`'s "same function, same tiers".

Option 1 is what `[import]` already says; the trigger is `can-an-import-reach-a-state-only-a-kernel-bug-otherwise-reaches` closing with it true.

Note (PR 5464, 2026-10-20): Ev ruled on that PR that the import door names the file for every defect ending a bad file reaches. The refusals the import door renders at `ReadAt::File` now do: adoption's certification refusals, the join (`topo::JoinRefusal::render`), the placement (`topo::TransformError::render`) and the pcurve re-mint (`topo::PcurveMintError::render`). The validator verdicts `StepImportError::TierInvalid` carries from the import gate are read at rest (`topo::validate_geometric`), so they still end in `KERNEL_OR_FILE_DEFECT_ENDING` there. Until this row lands, that gate is where a bad file is still told "kernel defect or damaged file" at the import door. (ENCL implementer)
