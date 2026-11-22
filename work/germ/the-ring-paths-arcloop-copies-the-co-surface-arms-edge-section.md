---
id: the-ring-paths-arcloop-copies-the-co-surface-arms-edge-section
kind: issue
title: The ring path's ArcLoop decides a torus loop arc's own loop of its plane section by hand, a copy of the co-surface arm's edge Section
status: open
opened: 2026-11-22
---

Filed by GERM's `germ/co-surface-table` lane (PR 7431).

## Finding

The co-surface arm now holds an edge's own section of its plane
(`geom_brep::curve_pair::on_surface::Section::{Circle, Oval}`, read by
`OnSurface::meet`): which of a torus circle's plane curves, and which
oval of a spiric's plane, a meeting lies on. The ring path keeps its own
copy of that reading: `ArcLoop` and `LoopSplit` in
`crates/topo/src/ring_path.rs` decide, on
`split_ring_path_torus_loop`, which loop of a torus loop arc's plane
section a crossing of that plane lies on (`ring_path::Oval`,
`LoopArc::torus_spiric_arc`), and the ring path reads its pairs through
the 3D curve-pair table (`geom_brep::curve_pair::meet`, `ring_path.rs`'s
`use geom_brep::curve_pair::{Hit, Span, SpanEnd, meet}`), which C5 says
no consumer holding two curves on one surface does.

The unit `co-surface-direction-table-and-spiric-plane-door` names this
copy as going "when the ring path moves"; PR 7431 does not move the
ring path. Moving it reads the ring path's pairs through
`OnSurface::meet` and retires `ArcLoop`'s section reading. Not verified
here: whether every ring-path pair is co-surface (charted from one
record), which `meet` needs.
