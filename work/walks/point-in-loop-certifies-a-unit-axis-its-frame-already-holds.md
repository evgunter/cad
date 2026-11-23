---
id: point-in-loop-certifies-a-unit-axis-its-frame-already-holds
kind: issue
title: point_in_loop certifies a unit normal its Frame parameter already holds unit, so OffPlaneCause::NormalNotUnit has no producer
status: open
opened: 2026-11-22
---


Filed by `three-answers-to-is-this-loop-inside-that-one`'s lane, as the
residue of that unit's change to the walk's parameter.

`splitting::containment::point_in_loop` (`crates/topo/src/splitting/containment.rs`)
now takes the loop's plane as a `geom::Frame`, whose axis is unit as a
property of the type (`geom::Frame` holds a `RigidMotion`). Its
precondition certifier `certify_plane` still decides the axis's length
as its first datum (`POINT_IN_LOOP_NORMAL`, refusing
`OffPlaneCause::NormalNotUnit`), so that refusal has no producer: the
sweep row that reached it (`crates/sweep/tests/point_in_loop_arc_cap.rs`,
"a normal of length 2") cannot be written against the new signature and
was dropped.

What follows: drop the unit datum from `certify_plane` and retire
`OffPlaneCause::NormalNotUnit`, a public variant read by
`validate.rs`'s refusal text, `census.rs`, `test_support_samples.rs` and
the sweep row's exhaustive match. Left out of the walks unit because retiring a public variant
is its own change across those readers.
