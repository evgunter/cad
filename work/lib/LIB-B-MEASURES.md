---
id: LIB-B-MEASURES
kind: unit
title: binding census family B-MEASURES
status: open
opened: 2026-09-06
---

Queued mechanical census family (the B-RESOLVE shape): sweep the
family's bindings against the census contract, construct the
previously unconstructible pins where the surface now allows, and
re-cut the census rows honestly. Families share the census/tags/test
files, so at most two run concurrently, staggered.

## Derived scope

`crates/pncad-py/tests/test_binding_census.py` charters `B-MEASURES`
in `FAMILIES` (ERROR-DESIGN E3/E10): the AUTHORING half of
measurement — `MeasureExpr`'s constructors, `MeasurePrimitive`'s four
verbs and `AssertionDir` (`crates/editor-core/src/measure.rs`) onto
`Node.measure` / `Node.assertion` constructors, with `MeasureNodeFault`
as the refusal a caller dispatches on and `MinClearanceRefusal` /
`MeasureUnavailableAt` as the two the fourth verb adds. The READ half
(`Value.measure`, `Value.assertion`) already ships and is deliberately
not in this gap. Seven `NOT_BOUND` entries cite the family
(`AssertionDir`, `MeasureExpr`, `MeasureNodeFault`, `SitedRef`,
`MeasurePrimitive`, `MinClearanceRefusal`, `MeasureUnavailableAt`);
the sweep decides what else it wants bound or listed.

## Home

LIB's (the Python surface is outside M10's fence). Filed 2026-09-06
at the program's reactivation; `LIB-B-FACE-FRAME` and `LIB-B-PART`
named it as "unscheduled alongside". Sequenced after
B-DISTRIBUTIONS if both are taken: the census says this family's
asymmetry is B-DISTRIBUTIONS's without the sharp edge.
