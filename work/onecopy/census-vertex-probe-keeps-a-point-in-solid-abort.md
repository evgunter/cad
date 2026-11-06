---
id: census-vertex-probe-keeps-a-point-in-solid-abort
kind: issue
title: census's vertex probe keeps a PointInSolidError that aborts, and ranks first-of-any-kind, beside the standing that type states
status: open
opened: 2026-11-06
priority: P2
cost: M
---


Found by `kept-refusal-loops-the-ladder-row-never-listed`'s sweep
(`onecopy/refusal-standing`).

## What

`census::sweep_cross_solid_backstop`'s `probe` closure (the vertex loop
keeping `refused.get_or_insert(Undecided::of_point_in_solid(&e).what())`)
is a ladder under `Any`: `SolidContainment::In` at any vertex answers,
and `Out` and `OnBoundary` pass (`Out` is remembered for `Probe::Clear`).
A refusal is kept first-of-any-kind, in vertex order. A refusal at a
vertex that `answered_in_band` covers is dropped, which is the census's
own exemption.

`PointInSolidError` already states its standing,
`PointInSolidError::refusal` (`Result<Refusal, Aborts>`). The probe
ignores it in two ways:
- **An abort is kept** and later readings go on. So an `In` after a
  refusal of the whole query answers today, and the abort is reported
  only where nothing answers.
- **The kept refusal ranks first-of-any-kind**, not by `Evidence`.

## What closing it takes

Port the loop onto `climb` (`Any`), reading each vertex through
`rung_or_abort(e.refusal(), …)` with the exemption as `Rung::Passes`.
**This moves outcomes:**
- an abort now stops the probe;
- a limit outranks an earlier in-band refusal.

Measure both and pin each for a ruling. Check also whether
`Undecided::of_point_in_solid(..).what()` is the payload the ranked
refusal should carry.
