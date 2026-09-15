---
id: rate-pair-in-geom-core
kind: unit
title: SupSpeed and InfSpeed beside Margin: metered takes the inf, a sup door for overshoot metering, the three blurred sites typed
status: open
opened: 2026-09-15
---


## What

`D283`'s ruling, route A (PR 2457), first unit. Two types in
`crates/geom-core/src/predicate.rs` beside `Margin`: `SupSpeed` and
`InfSpeed`, meters per parameter unit, one operation each way
(`span · s` to meters, `m / s` to parameter units — bit-identical to the
bare arithmetic, D9), a surface being a pair. `Margin::metered` takes
`InfSpeed` (its doc's promise made a type) and gains the sup-side
sibling for overshoot and escape metering; the three sites that pass a
sup through `metered` today (`pcurve_cache::trim_containment`, the
`pcurve_iso_*` slack meters, `topo::pcurves::pcurve_loop_continuity`'s
v-channel — PROPS' row
`metered-margin-doc-promises-an-inf-bound-three-sites-pass-a-sup` and
TRIM's `loop-continuity-meters-u-through-levered-and-v-through-metered`)
move to it; `PatchRegularity`'s speeds and `plane_nurbs_ssi`'s local
speed become `SupSpeed`. The type is a dimension-and-direction tag, not
a positivity witness: the 0/∞ guards stay at their sites. Out of scope
by the ruling: second-order and param→param rates (mesh sizing,
chords), pointwise jet speeds (the march, Newton acceptance),
`levered_inv`'s curvature uses, `gap_is_noise`'s zero lever. Evidence:
`work/scalar/log.md`, "The rate census". PROPS' and TRIM's ground;
announce. Full v6 dual.
