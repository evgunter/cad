---
id: a-tube-with-a-negative-major-radius-refuses-as-a-profile-across-its-axis
kind: issue
title: A tube whose major radius is negative refuses as a profile crossing the revolve axis, not as the size it is
status: open
opened: 2026-11-22
priority: P3
cost: E
---

Found by LATHE's one-side-of-the-axis lane (branch
`lathe/one-side-of-the-axis`), sweeping the readings that decide by the
side of a directed axis.

`sweep::revolve::tube::build` (`crates/sweep/src/revolve/tube.rs`) places
the tube's circle at `c_sk = (major_radius, 0)` in a sketch frame whose
axis is `(0,0)→(0,1)`, and classifies it with
`revolve::axis::classify_loop` directly: it does not pass through
`revolve_construction`, so it does not take the revolve's side reading
(`revolve::axis::lies_left`). Nothing decides `major_radius` before that.
So `tube_along_arc(frame, -3.0, window, 1.0, tol)` (the circle wholly
at `x ∈ [−4, −2]`) refuses `TubeError::Revolve(VertexCrossesAxis)`,
whose sentence says vertex 0 "lies definitely on the other side of the
revolve axis from the profile's widest reach, so the profile crosses
the axis" (`RevolveError::write`, the `VertexCrossesAxis` arm), a fact
the input does not have, with a recourse ("move the profile to one side
of the axis") the caller cannot act on: the door's input is two radii.
Not run; read from the code.

The tube door's inputs are intent sizes ("the numbers the caller gives
are the numbers the body stores", the module docs), so the likely
answer is the size rule (Ev, #3551: "a size an operation covers is
positive"): the door decides `major_radius` definitely positive and
refuses a non-positive one as such, before the frame. The window is
already metered the same way (`arc_window`).
