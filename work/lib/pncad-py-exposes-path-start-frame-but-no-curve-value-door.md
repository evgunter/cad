---
id: pncad-py-exposes-path-start-frame-but-no-curve-value-door
kind: issue
title: pncad-py exposes path_start_frame but no curve value door: a Python caller has the consumer of the point/tangent pair and no producer for it
status: open
opened: 2026-09-21
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
