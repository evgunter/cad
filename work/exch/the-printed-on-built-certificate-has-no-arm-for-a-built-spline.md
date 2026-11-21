---
id: the-printed-on-built-certificate-has-no-arm-for-a-built-spline
kind: issue
title: the printed-on-built certificate has no arm for a built spline other than the printed one's twin, a spline on an ellipse or spiric, or a line on a curve; those edges are still read at the printed carrier's ends and middle
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Found by EXCH unit D (`exch/adopt-by-description`, PR 7304).

`on_built::printed_on` (`crates/step-import/src/on_built.rs:93`)
certifies the printed carrier on its built one over the whole trim for
these pairs:

- any printed carrier on a built line (its trim cover);
- a printed spline on a built circle;
- a printed arc on a built circle;
- a printed conic on a built conic, a circle on an ellipse included;
- a spline on its bitwise twin.

The iso rung adds a straight column (`build::segment_gap`) and a curved
one (`build::circle_column_match`), both covering any printed spline.
Every other pair answers `None`, and `build::description_gap`
(`crates/step-import/src/build.rs:1358`) falls back to the farthest of
the printed carrier's ends and middle. The pairs left:

- **a printed spline on a built spline that is not its twin**:
  - the spline section of a plane × wall or wall × wall pair (27 corpus
    edges at the default ε);
  - a cap's rational rim row (`build::cap_row`'s column arm);
  - a marched branch;
- **a printed spline on a built ellipse or spiric**;
- **a printed line on a built circle, ellipse or spline**:
  `Conic::of(Line)` is `None`;
- **a printed circle on a built spline**: a cap row, a marched branch;
- **on the iso rung's curved column, a printed ellipse or line** (only a
  circle or a spline is matched there), and a printed spline trimmed short
  of its domain (the arc residual reads whole carriers).

On those edges a printed curve that bulges off its description between
the three samples still imports, as every edge did before unit D.

- The curve-on-spline pairs need a curve-to-curve certificate the kernel
  does not have: a spline has no implicit form.
- The ellipse needs an elliptic-cylinder composite in
  `geom_core::spline::compose`.
- A printed line on a curve is the cover arm read against that curve's
  site, which `geom::cover` offers only for a point or a line.
