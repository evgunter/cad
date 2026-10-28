---
id: measure-clearance-refused-carries-arms-its-door-never-raises
kind: issue
title: MeasureClearanceRefused carries EngineRefusal, three of whose seven arms the measure door never raises
status: closed
opened: 2026-10-28
closed: 2026-10-28
pr: 5928
priority: P4
cost: E
---


Found by SYMCLEAR, which narrowed `NodeErrorKind::MeasureClearanceRefused`
(`crates/editor-core/src/eval/mod.rs`) from `ClearanceRefusal` to
`topo::clearance::EngineRefusal`. Its one producer is
`ClearanceLane::min_separation` (`crates/topo/src/clearance.rs`), whose
door raises four arms: `EmptyScope`, `NoAdmittedPair`, `Unsupported` and
`PoisonEnclosure` (`min_separation`'s `# Errors`: "never a width, and
never a budget"). `Budget`, `Sliver` and `WitnessUnverified` belong to
the bound sweep and cannot reach this carrier.

They still cost a reader: `drive.rs`'s `box_independent_measure_class`
classes all three as "not reached", and the refusal-concision census
renders a row for each (`refusal_concision_chains.rs`, the
`MeasureClearanceRefused/` rows).

## Fix

Give `min_separation` its own refusal type with the four arms, and carry
that. It changes a `topo` signature and its callers in the M10-6 suites,
which is why it did not ride with SYMCLEAR.

## Closed (2026-10-28, #5928)

`topo::clearance::MinSeparationRefusal` carries the four arms;
`min_separation` and `ClearanceLane::min_separation` return it, and
`MeasureClearanceRefused` and `MeasureRefusalClass::Clearance` carry it.
`box_independent_measure_class` matches its four arms with no
"not reached" arm, and the refusal-concision census renders four rows.
