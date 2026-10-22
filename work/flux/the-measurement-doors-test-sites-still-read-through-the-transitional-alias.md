---
id: the-measurement-doors-test-sites-still-read-through-the-transitional-alias
kind: issue
title: The measurement door's test sites in sweep and editor-core still read through the transitional alias unmigrated_measure
status: open
opened: 2026-10-22
priority: P2
cost: M
---


Filed by FLUX unit F1 (`flux/typed-door-f1`). F1 types the measurement
door (`mass_properties` and `classify_shells` take `&AtRestBody`, Ev's
decision on PR 4822) and migrates topo and the small crates. sweep's and
editor-core's test call sites, which F2 migrates, read through
`topo::test_support::unmigrated_measure` / `unmigrated_classify` in the
meantime.

The alias means **"not yet migrated"**, never "measures an invalid body
on purpose". That second meaning is `measure_unvalidated` with a one-line
reason at the call. The alias is test-support only (`cfg(any(test,
feature = "test-support"))`, `doc(hidden)`), so no production caller
reaches it.

The census `scripts/gates/unmigrated-measure-sites.sh` counts its call
sites and pins the count F1 leaves. A rise reds the gate.

## What closes it (F2)

Each counted site becomes `finished(...)` once per body (or the op's own
`AtRestBody`), or `measure_unvalidated` with its reason. Then the count
reaches 0, the alias and the gate's pin are deleted, and editor-core's
`product.rs` keeps the verdict its at-rest gate reaches, as `split` now
does.
