---
id: two-step-normalize-and-unasked-lengths
kind: issue
title: two-step v / v.norm() normalizations bypass the overflow prescale, and un-asked lengths read as levers
status: open
opened: 2026-10-24
---


## Where this came from

The sweep for `work/chart/normalize-without-the-length-question-two-more-sites`.
`Vec3::normalize` and `Vec2::normalize` now prescale by a power of two past
the squares' range, so `v.normalize()` of a finite `v` is its direction at
every magnitude. That repair reaches only call sites spelled
`.normalize()`. Two shapes are still exposed. Neither has been executed
end to end: both are argued from the arithmetic. **Execute before fixing.**
That unit's own site 1 is the warning: it was filed with a mechanism that
turned out not to be the one that fires.

## Shape 1: two-step normalization, `v / v.norm()`

Past ~1e154 the squares overflow, `v.norm()` is `∞`, and `v / ∞` is the
zero vector rather than a direction. Below ~1e-162 the squares flush,
`v.norm()` is `0`, and `v / 0` is `±∞`/NaN. `v.normalize()` is
bit-identical to `v / v.norm()` wherever the largest component lies in
`(2⁻⁵¹¹, 2⁵¹¹)` (its own docs), so the mechanical rewrite moves no
in-range bit. The reciprocal spelling `v * (T::one() / v.norm())`
collapses the same way. Rewriting it to `.normalize()` is NOT
bit-identical, because it drops one rounding. Re-derive both lists
with:

```
grep -rnE '([A-Za-z_][A-Za-z0-9_.]*) (/ |\* \(T::one\(\) / )\1\.norm\(\)' crates/topo/src crates/geom-brep/src
```

As of this filing, outside test modules (52 sites):

- `crates/topo/src/ring_path.rs`, lines 166, 175, 183, 287, 299, 419, 973 and 1184
- `crates/topo/src/census/depth_gap.rs`, lines 359, 360, 389, 416, 436, 456, 463 and 494
- `crates/topo/src/boolean/torus_region.rs`, lines 262, 323, 593 and 947
- `crates/topo/src/boolean/fate.rs`, lines 266 and 1110
- `crates/topo/src/boolean/rim_wedge.rs`, line 811
- `crates/topo/src/boolean/sphere_region.rs`, lines 163, 173, 175, 274 and 341
- `crates/topo/src/boolean/solid_contain.rs`, lines 2529, 2826 and 4412
- `crates/topo/src/boolean/node.rs`, line 108 (the `unit` closure)
- `crates/topo/src/boolean/carrier_touch.rs`, lines 134 and 348
- `crates/topo/src/boolean/section_cert.rs`, line 470
- `crates/topo/src/boolean/section_cert/cone_search.rs`, line 349
- `crates/topo/src/boolean/section_cert/cone_pair_search.rs`, lines 555, 596, 701 and 1691
- `crates/geom-brep/src/implicit.rs`, lines 247 and 503
- `crates/geom-brep/src/curve_pair/pose.rs`, line 419 (reciprocal spelling)
- `crates/geom-brep/src/curve_pair/walk.rs`, line 278
- `crates/geom-brep/src/intersect.rs`, lines 1265, 1268, 1485 and 1488
- `crates/geom-brep/src/props/curved.rs`, lines 3420, 3612 and 4801 (4801 is the reciprocal spelling)
- `crates/geom-brep/src/tangent.rs`, line 90 (`d_hat`)

Many divide a stored unit axis or a carrier normal whose length is
bounded by construction, so for those the rewrite fixes nothing. It is
still the one spelling that cannot collapse. The grep's blind spot is a
length bound to a name first. Those sites are below, under shape 2,
because each of them also reads that length.

## Shape 2: the un-asked length is also READ

These are worse than shape 1, because the rewrite alone does not close
them. The length is a lever, a denominator or a span, and an `∞` there
turns a finite quantity into an exact zero or an infinite definite one.
`chart_region::proper_crossings` was this shape, and is closed by the
unit that filed this row. It asks each edge's length whether it is
bounded (`Real::is_bounded`) and not underflowed
(`is_underflowed_length`), and refuses `ChartRegionError::EdgeScale`.

- `topo` `census::edge_geo` and `census::segment` (the segment
  twin, for an op's segments before any body holds them) both store
  `len: chord.norm()` un-asked.
  `edge_vertex_at` reads it as `Margin::of(e.len - s)`
  (`pm_census_bound_end`). For an edge between ~1e154 and ~1.8e308 m,
  `len = ∞` while `s` is finite, so the far-end question answers
  definitely "not at v1" for the vertex that is v1. The wedge door
  (`edge_wedge_shape(own * arm, …)`) is asked downstream through
  `sector_shape`.
- `topo` `offset_axial`: the datum arm decides `Margin::of(len)`, then
  divides `v.x / len`. That is the parent's decide-then-normalize class,
  spelled in two steps.
- `topo` `boolean::fate`: `mid / len` after `sign(len, …)`, and
  `dir / dir.norm()` beside `len = (end - base).norm()`.
- `topo` `ring_path`: a line's `dir: d / length` and `span: (0, length)`.
- `topo` `chord_join`: `t = … / len.powi(2)` and `u_ref: dir / len`.
- `geom-brep` `offset_fit` (`m * (d / len)`) and `extent`
  (`… / len`, with `len` floored at `f64::MIN_POSITIVE` but not capped).
- `geom-brep` `conic_bend::conic_bend_side`:
  `bend.curvature.dot(n) / bend.curvature.norm()` is a two-step
  direction component. The curvature's length is never asked:
  overflow reads the component as 0, and a straight departure reads
  `0/0`.
- `geom-brep` `enters::enters_material_order2`: the margin divides by
  the caller's `speed_sq`, which is never asked to be finite or
  nonzero. An overflowed square reads the curvature comparison as an
  exact Zero. **Unsure:** every in-tree caller may already derive
  `speed_sq` from a jet whose length a door asked. That has not been
  traced.
- `topo` `boolean::section_cert::search_kit`,
  `ring_path::cone_islands` and
  `boolean::solid_contain::r1_generic_poses`: `v / n` after a local
  `n = v.norm()`.

## Ownership

From `python3 scripts/work.py territory --files -` over every file
above:

- **topo boolean:** CLEAVE and HONE (`fate`, `node`, `rim_wedge`,
  `torus_region`, `section_cert/*`, `solid_contain/r1_generic_poses`,
  `chord_join`, `carrier_touch`, `sphere_region`, `solid_contain`).
  Also ORBIT (`section_cert`, `sphere_region`), INSIDE
  (`solid_contain`), APEX (`carrier_touch`) and TANG (`chord_join`).
- **topo census:** ATREST and RESTREAD (`census`), ATREST
  (`census/depth_gap`).
- **topo, other:** GERMTAIL (`ring_path`); CURVED, OFFSET, SHELF and
  SHELL (`offset_axial`).
- **geom-brep:** CPAIR (`curve_pair/pose`, `curve_pair/walk`), ROOTS
  (`implicit`), GERM, GERMCERT, GERMTAIL, ROOTS and TANG
  (`intersect`), TANG (`tangent`), ENCL (`offset_fit`), FLUX and
  FLUXTAIL (`props/curved`).
- **Unowned:** `conic_bend`, `enters`, `extent` and
  `ring_path/cone_islands`.

That spread is why this is an issue rather than a row on one slate.
The shape-1 rewrite is mechanical and bit-identical in range, so one
lane could take all of it across those fences. Shape 2 wants each
owner's length question and the typed refusal that goes with it.
