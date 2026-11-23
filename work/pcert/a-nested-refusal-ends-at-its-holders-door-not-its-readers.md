---
id: a-nested-refusal-ends-at-its-holders-door-not-its-readers
kind: issue
title: A nested refusal ends at a door fixed by its holder (at_build / at_rest), not at the door that reads the holder
status: open
opened: 2026-11-03
priority: P3
cost: M
---


## Finding

Found by E1e (PR "PCERT E1e: a refusal's Display carries only its payload"). Before E1e these sites ended a nested refusal at the reading that the nested type's `Display` spelled, whatever door read the holder. E1e does not change an ending's meaning, so it keeps each one, spelled now and pinned per file by `scripts/gates/display-payload-only.sh` (`recourse::at_build` / `at_rest`, or a literal reading at a door):

- `topo::TransformError::ApproxRecertify` (`crates/topo/src/transform.rs`, `write`): the fit door's meter ends at a build, also when the map is read at rest or at the import door (step-import's `Placement`).
- `topo::ReplaceFaceError::Fit` (`crates/topo/src/replace_face.rs`, `write`): the same, at a build.
- `topo::EulerOpError::PcurveSplit` (`crates/topo/src/euler.rs`, `text`): a stored row re-certified over a child ends at a build, also when the split is reported at rest.
- `topo::ChartRegionError::Escalated` with a poisoned margin (`crates/topo/src/chart_region.rs`): the defect ending at a build, also when the region query is read at rest. (Its sibling `TrimOffFace` now ends at the reader's door.)
- `topo::ValidationError` (`crates/topo/src/validate.rs`, `write`): `JoinUndecidedAtRest` with an image refusal, `ShellRoleUndecided` with a nested shell refusal, and `OrientationUnread::Ray` end their nested refusal at a build at the at-rest gate; `CensusUnsupportedCause::Containment` likewise.
- The finding lists of a build's result (`validate::write_result_not_at_rest`, `SplitFinishError::ResultInvalid`) end at rest inside a build's refusal; that one is arguably right (they are the at-rest gate's findings).
- `step_import::StepImportError` (`crates/step-import/src/error.rs`, `write`): `TierInvalid` ends the at-rest gate's findings at rest and `Instance` ends the graft's Boolean refusal at a build, at the import door, where a damaged file is the culprit (`ReadAt::File`).
- `step_export::StepExportError::ShellUnclassified`: the shell door's refusal at a build, at the export door, which reads a body at rest.
- editor-core's component-count check (`crates/editor-core/src/checks.rs`, `evidence_story`) and pncad-py's check payload (`crates/pncad-py/src/check_payload.rs`) end a `ShellClassifyError` at a build over a body at rest.
- `topo::props::MassPropsError::Face` ends its `PropsError` at rest inside a build's refusal (a shell classification).

## The question

For each: does the nested refusal end at its reader's door (pass `at` down), or is the holder's door the right one? Passing `at` down is the default the E1e design gives; each site above is a meaning change to be decided, not a rewording.
