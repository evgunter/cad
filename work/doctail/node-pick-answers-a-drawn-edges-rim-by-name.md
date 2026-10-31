---
id: node-pick-answers-a-drawn-edges-rim-by-name
kind: issue
title: NodePick::boundary_rims answers each drawn edge's rim by name; VSEAM added it in PR 6129
status: open
opened: 2026-10-31
priority: P4
cost: C
---


An announcement, not a defect: VSEAM added one door on this program's
ground, in PR 6129 (`vseam/a-picked-rim-is-the-whole-rim`), on the
VSEAM orchestrator's ruling.

`NodePick::boundary_rims` (`crates/editor-core/src/resolve/pick.rs`,
beside `NodePick::boundary_names`) answers, per boundary polyline in
polyline order, the names of every arc of the closed rim that
polyline's edge is on (`topo::query::rim_of`, keys mapped back through
the node's table), or `None` when the edge is on no rim of two or more.
Its loud arms are the new closed enum `RimNameFault` (`Unnamed`,
`NoBody`, `Stale`), re-exported at `editor_core::RimNameFault`. It
exists so the viewer's pick index can treat a seam-split circle as one
drawn edge without an arena key crossing G1
(`crates/viewer/GUI-DESIGN.md`). Nothing else in editor-core changed.

Close this once DOCTAIL has read it. If the door belongs somewhere else
on this ground, move it then.
