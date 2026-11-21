---
id: the-printed-on-built-certificate-has-no-arm-for-a-built-spline
kind: issue
title: the printed-on-built certificate has no arm for a built spline other than the printed one's twin, nor for a spline or conic on an ellipse or spiric; those edges are still read at the printed carrier's ends and middle
status: open
opened: 2026-11-21
priority: P2
cost: M
---

Found by EXCH unit D (`exch/adopt-by-description`).

`on_built::printed_on` (`crates/step-import/src/on_built.rs`) certifies the
printed carrier on its built one over the whole trim for a printed spline
on a built line or circle, a printed line on a built line, a printed arc
on a built circle, a printed conic on a built conic, and a spline on its
bitwise twin. Every other pair answers `None`, and `build::description_gap`
falls back to the farthest of the printed carrier's ends and middle:

- a printed spline on a built spline that is not its twin: the spline
  section of a plane × wall or wall × wall pair (27 corpus edges at the
  default ε), a cap's rational rim row (`build::cap_row`'s column arm), a
  marched branch;
- a printed spline or circle on a built ellipse or spiric;
- a printed conic on a built straight column.

On those edges a printed curve that bulges off its description between
the three samples still imports, as every edge did before unit D. The
curve-on-spline pairs need a curve-to-curve certificate the kernel does
not have (no implicit form for a spline); the ellipse needs an
elliptic-cylinder composite in `geom_core::spline::compose`.
