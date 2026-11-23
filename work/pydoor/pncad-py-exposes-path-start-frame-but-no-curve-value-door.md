---
id: pncad-py-exposes-path-start-frame-but-no-curve-value-door
kind: issue
title: pncad-py exposes path_start_frame but no curve value door: a Python caller has the consumer of the point/tangent pair and no producer for it
status: open
opened: 2026-09-21
priority: P3
cost: M
---

## What

`pncad-py` publishes the start-frame door — `Frame.path_start_frame(origin,
tangent)` (`crates/pncad-py/src/py/place.rs`, the `#[staticmethod]` over
`geom_core::linalg::frame::path_start_frame`) — but no curve value door
at all: no `eval`, no `deriv`, no `ders1`, and neither `Curve3` nor
`NurbsCurve3` appears anywhere under `crates/pncad-py/src` (`Doc.eval`
is the expression evaluator, `crates/pncad-py/src/py/expr.rs`). A Python
caller therefore holds the CONSUMER of a point/tangent pair and has no
producer for it: the tour's `tube_place` shape — a station frame at
every parameter of a spine — cannot be written from Python.

Found by CURVE3-JET's R1 review lane writing the unit's end-to-end
program in Python and finding the pair unreachable. On the Rust side
the pair is one call (`NurbsCurve3::ders1`, `Curve3::ders1`); whether
the Python surface should carry a curve object with `eval`/`deriv`/
`ders1`, or a narrower `station_frame(path, t)` door that hides the
pair entirely, is LIB's call.

- 2026-11-03 — moved from LIB to PYDOOR by `git mv` when LIB closed, id and body unchanged; banded P3 and priced M in the move (LIB had three P1 rows left behind 124 closed ones; its library-door rows went to PYDOOR, opened for them, and its tag, stub, census and projection rows to BIND; Ev, in chat).


## `path_start_frame` retired (FRAME, PR 6465, 2026-11-03)

`geom_core::linalg::frame::path_start_frame` and its Python spelling `Frame.path_start_frame` retired in PR 6465, with the `FrameInput::{Tangent, ReferenceLadder}` words. A frame across a direction with no stated reference is now a written frame turned onto it: `RigidMotion::turned_onto(self, direction, site, band)` (`crates/geom-core/src/linalg/rigid_motion.rs`), e.g. `RigidMotion::axes_xy(p).turned_onto(t̂, …)` for a sweep's start. Python has no `turned_onto` spelling yet, so the point/tangent consumer this row names is gone and the row's premise has changed. This row's status and title are the owning program's call.
