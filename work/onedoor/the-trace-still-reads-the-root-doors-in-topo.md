---
id: the-trace-still-reads-the-root-doors-in-topo
kind: issue
title: "[topo.homes] arm 1: boolean/trace.rs (ray_crossings, circle_crossings, plane_misses) takes no Body and is still built in topo"
status: open
opened: 2026-11-20
priority: P2
---


Split off `the-conic-quadric-door-is-still-built-in-topo` (GERM, its U2
move, 2026-11-20), whose body said `trace.rs` "follows the door"; the
U2 PR moved only the door, with no behaviour change.

`crates/topo/src/boolean/trace.rs` (`ray_crossings`, `circle_crossings`,
`plane_misses`, `crossing`) takes no `Body` and no arena key. It reads
the curve × face doors that now live in geom-brep —
`geom_brep::curve_face::line_quadric::line_quadric_roots`,
`geom_brep::curve_face::conic_quadric::conic_quadric_roots`,
`geom_brep::curve_face::circle_torus::circle_torus_roots` — each refusal
mapped into its own `TraceFault` through `door_refusal`, and, since the
U2 move, names no `BooleanError` at all. What keeps it in topo is
`crate::validate::decide` (a thin wrapper over
`geom_core::k_stats::decide`) and its callers' placement; its own
closed forms are a circle against a plane (`TRACE_PLANE_DEPTH`, the
plane door's class: `curve_face::plane_crossing`) and the sense and
order of each crossing. Its move is `[topo.homes]` rule 1's question:
whether `trace` is curve × face machinery (beside `curve_face::route`,
reading `cross` once U3 builds it) or one consumer's helper.
