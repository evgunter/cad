---
id: the-frame-carries-every-placement-reading
kind: unit
title: geom::Frame gets its own module and every placement reading; doors return the home by reference; the circle and line consumer structs hold geom::Circle and geom::Line
status: dispatched
priority: P1
cost: H
parent: analytic-surfaces-spell-their-frame-field-by-field
opened: 2026-11-03
branch: onecopy/frame-readings
---

Stages 1 and 2 of the placement decision recorded on `analytic-surfaces-spell-their-frame-field-by-field` ("Decided"). Read that section first; it binds this unit.

## Scope

**Stage 1, additive:**
- `Frame` moves from `curves/conic.rs` into its own `geom::frame` module, and `center` is renamed `origin`. The docs say only "a point and a right-handed orthonormal basis, the third axis derived". Each kind's own docs say what its `axis` and `u_ref` mean; "θ = 0 lives here" moves to `Circle` and `Ellipse`.
- Readings, one home each:
  - `v_ref`;
  - local coordinates `(x, y, h)` and their inverse;
  - `axis_line() -> Line`;
  - height above, and foot on, the frame's plane;
  - `reversed`, `translated`, `map_scalar`;
  - the image under a handedness-preserving rigid motion only (improper maps keep their per-kind rule in `topo::transform`/`mirror`, with a test per kind);
  - `square() -> SquareFrame`;
  - `Frame::of_motion(&RigidMotion)` as the one bridge.
- `Curve3::frame()`, `Curve3::circle() -> Option<&Circle>`, and `Surface::frame() -> Option<&Frame>` (one exhaustive match each).
- The azimuth helpers, `Curve3::circle_at` and the cover constructors take `&Frame` and no longer take loose vectors.
- `geom::cover::Frame` (box axes, no point) is renamed.
- Reword `RigidMotion`'s "so the one type is both".

**Stage 2, consumers that need only the stage-1 homes:**
- `surgery::CircleFrame` (deleted in favour of `Curve3::circle()`);
- `ring_path::CircleArc { circle, span }` and `LoopArc::Line { line, span }`;
- `contain::LoopCircle`;
- `chart_point::ChartInverse { frame, arm }`;
- `chart_region::CylFrame`;
- step-import's `type Frame` tuple, and step-export's `axis2_placement`;
- the hand-built `Line { origin, dir: axis }` axis lines;
- the five local line types (`local-point-direction-line-types-restate-geom-line`);
- the inline foot-on-plane and radial-offset spellings (`a-points-foot-on-a-plane-is-spelled-inline`, `a-points-radial-offset-from-an-axis-is-spelled-inline`), which become `Frame` and `Line` readings.

**Not in scope:**
- `rim_wedge::Rim`, which waits on PR 6443;
- the `Surface` kind structs and the gate, which are stage 3, on the parent row.

