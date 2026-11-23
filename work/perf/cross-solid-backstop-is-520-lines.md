---
id: cross-solid-backstop-is-520-lines
kind: issue
title: sweep_cross_solid_backstop is ~520 lines of closures, nested types and two arms inside a 4500-line census.rs
status: open
opened: 2026-09-13
priority: P4
cost: D
---


## The finding

`sweep_cross_solid_backstop` (`crates/topo/src/census.rs:2765–3246`) is
~480 lines in one function: six closures (`solid_of`, `face_vertices`,
`face_points`, `line_bounded`, `hull`, `reach_box`), a nested `Reach`
struct and a `SolidReach` type alias, two candidate classes built
inline over its own boxes (PERF-12 added ~90 lines for those), and two
arms with their own skip ladders. `census.rs` is 4 582 lines with a
233-line module header. Not PERF-12's alone — the arms predate it — and
a reading-shape finding rather than a defect: nothing here is wrong,
and the reviewers who walked it could, but each arm is a function's
worth of code that has no name.

## What a fix is

The two arms as named functions over a `Reach` slice built by a third;
the closures that only the reach construction uses go with it. No
behaviour moves, so the goldens and the differential suites pin the
refactor for free.
