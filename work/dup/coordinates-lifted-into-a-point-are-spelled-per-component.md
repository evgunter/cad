---
id: coordinates-lifted-into-a-point-are-spelled-per-component
kind: issue
title: A point or vector built at T from f64 coordinates that are not already a Point/Vec is spelled per component at 45 sites; pncad::authoring's p2/p3/v2/v3 are the only door and sit uphill
status: open
opened: 2026-09-24
priority: P3
cost: D
---


## Finding

Filed by the `dup/scalar-lift-home` lane (the PR that closed
`the-componentwise-scalar-lift-has-no-shared-home`). That unit folded
every lift whose SOURCE is already a `Point2`/`Point3`/`Vec2`/`Vec3`
onto the leaf door `map` (`p.map(T::from_f64)`). Its census also found
the neighbouring class it could not fold: a point or vector **built at
`T` from `f64` coordinates that are not a point value** — bare
arguments (`|x, y, z| Point3::new(T::from_f64(x), …)`), a `[f64; 3]`
row (`Vec3::new(T::from_f64(r[0]), …)`), a tuple (`CYL_ORIGIN.0`).
There is no `Point3<f64>` to call `map` on, so `map` does not serve
them without first building one (`Point3::new(x, y, z).map(T::from_f64)`),
which no site does.

**44 groups**, re-taken at the lane's head after merging `main` at
`4968e6862` with the unit's
denominator-first instrument (every `<path>::from_f64(`/`::constant(`
call whose argument is a component read or a bare `x`/`y`/`z`, over
`git ls-files` with no path argument, grouped by file, callee and
receiver) plus its callee-agnostic second pass (any `F(R.x), F(R.y)`):

- **Bare coordinates, 35**: `crates/pncad/src/authoring.rs` ×5 (the
  public `p2`/`p3`/`v2`/`v3` doors and `polygon`'s `at`),
  `crates/sweep/src/test_support.rs` ×3 (`corners`, `waisted_at`,
  `bowl_at`), `crates/topo/src/test_support_fixtures.rs` ×2 (the prism
  builders' vertex maps), `crates/geom-brep/tests/shared/point.rs` ×2
  (`p3`, `v3`), `review_m2_pr3_certify.rs` ×2 (`ipt`, `ivec`),
  `onb_c_payoff_interval.rs`, `crates/geom/tests/dual_foot_tangent.rs`,
  `crates/profile/tests/{cert4r2_e2e,interval_lane,review_s2_probe,scalar_channels_probe}.rs`,
  `crates/sweep/tests/{cert_m2r1_passes,extrude_interval,issue93_az_intersect,m5_pr6_pcurves,mass_props_interval,review_m2_pr4_interval,review_m2_pr5_interval,review_m2_pr7_interval,revolve_interval,sf2a_r2_interval_probe}.rs`
  (eight of those ten are the same `fn p2(x, y) -> Point2<Interval>`),
  `crates/topo/tests/{cube_doors_agree,review_m3_pr3_rings}.rs`, and
  an `iv` closure in `crates/geom-core/src/real.rs`'s test module.
- **A `[f64; N]` row, 8**: `crates/topo/src/boolean/solid_contain.rs`,
  `chart_bound.rs`, `chart_region.rs` (`SCHEDULE_2D`),
  `splitting/containment.rs`, `splitting/order.rs` — five PRODUCTION
  sites lifting the ray-direction `SCHEDULE` tables, which are typed
  `[[f64; N]; M]` rather than as vectors — and
  `crates/geom-core/tests/{onb_signed_zero_evidence,r2_cert3_probes}.rs`,
  `crates/topo/tests/review_m3_pr55.rs`.
- **A tuple, 1**: `crates/topo/tests/fixture/mod.rs` (`CYL_ORIGIN`).
- **Through a local alias, 2**: `crates/geom-brep/tests/interior_iso_review.rs`
  (`let f = T::from_f64;` then `Point2::new(f(p0.0), f(p0.1))` and the
  `Vec2` beside it).

Not members, and why: `geom-core/src/dual.rs` (`atan2` arguments, no
point), `topo/src/chart_region.rs`'s `decomposition_witness` closure
(two scalar arguments), `topo/tests/review_ssiflat_r2_probes.rs` (`arc`
lifted to two scalars), `demos/tour/src/klein.rs` (two scalar arguments;
and demos are never converted).

**Blind spots.** The instrument needs the lift to be written at the
coordinate; a coordinate lifted into a named local several statements
before the constructor, further than the grouping window, is not
grouped. A macro-assembled constructor is not seen at all.

## The shape of a home

Two different questions, and a unit takes them apart:

1. The **`SCHEDULE` tables** are a typing question on `topo`'s ground:
   typed as `Vec3<f64>`/`Vec2<f64>` constants (both `new`s are
   `const fn`), each of the five production sites becomes
   `r.map(T::from_f64)` on the existing door, and no new API is needed.
2. The **literal constructors** (`p2`/`p3`/`v2`/`v3` and their private
   copies) have exactly one public door, `pncad::authoring`, and it is
   uphill of every crate that copies it. Whether a downhill home in
   `geom_core` is wanted, or whether the copies are fine as the suites'
   own vocabulary (most are one-line and read well), is the design
   question; method item 6 applies site by site.

## Why this row is on this slate

The finding is one thing spelled many times, which is this program's
charter, and it was measured by the unit that folded its neighbour. The
`SCHEDULE` half lands on `topo`'s ground and the `pncad` half on
`lib`'s; either owner may claim the row by `git mv`.
