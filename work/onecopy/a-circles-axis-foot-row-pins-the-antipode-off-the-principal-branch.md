---
id: a-circles-axis-foot-row-pins-the-antipode-off-the-principal-branch
kind: issue
title: curve_pair span_rows' probe-gated axis row pins its foot at 7π/3 and reads 4π/3 since the foot carries its parameter
status: open
opened: 2026-11-04
---


Filed by the PCERT main-reds lane (`pcert/main-reds-e2-e5`), found while
classifying which rows go red only under `--all-features`.

`geom-brep curve_pair::span_rows::a_point_on_a_circles_axis_is_off_with_no_place_read`
(`crates/geom-brep/src/curve_pair/span_rows.rs`) is `#[cfg(feature = "probe")]`,
so no CI row runs it (the class is ciw's
`probe-gated-lib-unit-tests-never-run-in-ci`). Under
`cargo nextest run -p geom-brep --lib --features probe` it fails at its
first assertion:

    assertion `left == right` failed: the middle's foot is its antipode
      left: 4.1887902047863905      (4π/3)
     right: 7π/3

The file was last changed by PR 6483 (`fe70d5d08`, "a foot has no bare
parameter … `Foot::nearest`"), which repointed the `nearest()` callers.
`c.foot(centre, 4π/3).nearest()` now answers the parameter on the seed's
own branch, 4π/3, where the row pins the antipode 7π/3 (= 4π/3 + π).
Decide which is right: whether the axis point's foot is still read
antipodal to the seed (the row's premise, that `atan2(+0, −0) = π`), or
whether the row's premise moved with the foot. Then re-pin or fix the
row, and run it under `probe`.
