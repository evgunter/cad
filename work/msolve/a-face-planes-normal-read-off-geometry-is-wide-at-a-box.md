---
id: a-face-planes-normal-read-off-geometry-is-wide-at-a-box
kind: issue
title: A face's plane read off geometry has a normal far wider than the box at Interval
status: open
opened: 2026-10-18
priority: P3
cost: M
---

A `PoseDef::Plane { face }` read is evaluated by `read_off` (`crates/editor-core/src/eval/wire/pose.rs:250`, `:499`). It returns the face's normal as a `UnitVec3` normalised from the carrier's geometry. A box lane cannot cancel a length against itself, so a cap whose normal is exactly ±z comes back with exact x and y but a wide z:
- ±8.75e-9 against a parameter half-width of 6e-11;
- ±4e-7 on the tour chain's link at its ceiling.

The INTENT stage 3 D m10 lane measured these. The resulting map is mathematically the identity but holds ±1e-7…1e-6 on its yy and zz entries, so the at-rest verdict goes undecided and the body is moved.

D works around the axis-aligned case in the solve: `mate::solve::tightened` (`crates/editor-core/src/mate/solve.rs`) reads a direction two of whose coordinates are exactly zero as that axis. A tilted face's normal stays as wide as the readback makes it.

**Done when** the pose readback keeps a unit normal's dependency, normalising once at the carrier's own representation (a plane carrier's stored unit normal, say) rather than from its points. Add a row: a tilted cap's plane read at Interval is as wide as the box moves it, and D's `tightened` workaround can retire.
