---
id: a-sketch-traces-window-is-a-metre-when-its-body-has-no-vertex
kind: issue
title: The sketch trace's window is a metre wide whenever its body's reach reads nothing, and a body with no vertex reads nothing
status: open
opened: 2026-11-25
priority: P3
cost: E
---



Found by the VGEOM substitution sweep (`vgeom/substitution-sweep`,
`work/vgeom/viewer-substituted-value-class-is-crate-wide.md`), in the
type-default/`unwrap_or` pass over `crates/viewer/src`.

**What.** `DocSession::sketch_trace` (`crates/viewer/src/session.rs`)
sizes the square it samples the trace across as
`TRACE_WINDOW_MARGIN * body_reach(ev, face.node).unwrap_or(1.0)`.
`body_reach` answers `None` for a node with no usable value, a value
that is not one body, and a body whose vertices all sit on its own
`xy` origin; it reads only VERTICES, so a body whose reach is carried
by its faces rather than its vertices (a closed periodic face — whether
this kernel's sphere or torus has a vertex was not checked) answers
from whatever vertices it has, or `None`. Every `None` becomes a
window of one metre, whatever the body's size — a number the function
did not compute, in the shape of a measured reach.

**Why it matters.** On a 5 mm part, one metre spreads the
`SKETCH_TRACE_RAYS` rays two hundred times wider than the body, and
nearly all miss: the trace shown has a handful of points, or none,
with nothing saying the window was a stand-in. The same fallback on a
body tens of metres wide draws a window cut short of it. `body_reach`
also drops a vertex whose `readback::vertex_point` refuses
(`filter_map(... .ok())`) and, through `f64::max`, any coordinate that
is `NaN`; both shrink the window silently rather than refuse it.

**Fix shape.** Read the reach the way `editor-core::mate::reach`'s
`face_reach` does (faces, not vertices), and answer the door with the
refusal when there is none rather than a metre — the
door-answers-Option/Result shape of `vgeom/refusal-floor` (#3000).
VSEAM's log (2026-10) already routed `body_reach` to this slate as a
key holder beside `frame_off`
(`the-viewers-numeric-gestures-read-a-frame-back-off-geometry`); this
row is the separate value defect, not that one.
