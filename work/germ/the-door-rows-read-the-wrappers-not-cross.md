---
id: the-door-rows-read-the-wrappers-not-cross
kind: issue
title: "The fixed door rows read the four test-only wrappers, not cross: re-aim them and delete the wrappers"
status: open
priority: P3
cost: M
opened: 2026-11-25
---

Filed by GERM U4b, which retired the four root-door wrappers from the
API: `line_quadric_roots` (`crates/geom-brep/src/curve_face/line_quadric.rs`),
`conic_quadric_roots` (`conic_quadric.rs`), `circle_torus_roots`
(`circle_torus.rs`) and `ellipse_torus_roots` (`ellipse_torus.rs`) are
`#[cfg(test)]` readers now, with `circle_roots::door_shape`,
`circle_torus::torus_wrapper` and `cross::Walked::on_span` behind them.
No production code reads them: the sweep reads `curve_face::cross` and
the face reading.

The safety fuzzes run a `cross` leg beside the wrapper leg
(`conic_oracle::cross_reading`): `conic_quadric/in_band_sweep`,
`conic_quadric/shallow_sweep`, `ladder_lane_fuzz`,
`circle_torus/in_band_sweep`, `circle_torus/shallow_sweep` and
`ellipse_torus`'s `certified_torus_answers_hold_against_the_true_distance`.

## What remains

About sixty fixed door rows still read only the wrappers, asserting in
their `CircleRoots` / `LineQuadricRefusal` / `ConicQuadricRefusal`
shape: `cone_rows`, `ellipse_rows`, `circle_wall_rows`, `circle_sphere_rows`,
`window_rows`, the rest of `in_band_sweep`, `line_quadric`'s and
`circle_torus`'s own rows, and `ellipse_torus`'s.

## What would close it

Each row re-aimed at `cross` (its meetings, windows and pose), judged by
the same oracle, and the four wrappers, `door_shape`, `torus_wrapper` and
`Walked::on_span` deleted with the refusal types only they read
(`LineQuadricRefusal`, `ConicQuadricRefusal`'s wrapper arm). If U4d's
spline work rewrites these modules, it can take this row.
