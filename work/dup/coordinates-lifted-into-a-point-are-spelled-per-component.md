---
id: coordinates-lifted-into-a-point-are-spelled-per-component
kind: issue
title: Whether geom_core should carry a public literal constructor (a point or vector at T from f64 coordinates) below pncad::authoring's p2/p3/v2/v3 — a question for the owner of geom-core's public surface
status: open
opened: 2026-09-24
priority: P3
cost: D
---


## What is left

One question, and it is a design question, not a fold: **is a public
door in `geom_core` wanted that builds a `Point2`/`Point3`/`Vec2`/`Vec3`
at a scalar `T` from `f64` coordinates?** Today the only public door of
that shape is `pncad::authoring`'s `p2`/`p3`/`v2`/`v3`, which is uphill
of every crate below `pncad`, so each crate that wants one spells its
own. Adding one to `geom_core` would be new public API on the kernel's
leaf crate, so it is the call of whoever owns that surface, not a
duplication lane's.

Everything that could be folded without that call was folded in the PR
recorded below; what remains are the per-crate homes and the scattered
single copies such a door would serve.

## The honest cost of a downhill door

What it would buy is small, and that belongs in front of the owner:

- **The spelling it would replace is already one expression at a door
  that exists.** `Point3::new(x, y, z).map(T::from_f64)` is the
  componentwise lift through `map`, which is public and downhill
  (`geom_core::linalg`). A constructor would save the `.map(...)`.
- **The copies are already at one home per crate where a crate has
  many**: `sweep`'s suites at `tests/common/interval.rs` (`iv`, `p2`,
  `p3`, `v3` at `Interval`), `geom-brep`'s at `tests/shared/point.rs`
  (generic `p3`/`v3`) and `tests/shared/interval.rs`, and `topo`'s
  prism fixtures at `test_support_fixtures.rs`'s `identity_map`.
- **A generic door does not serve the densest home.** `sweep`'s
  interval suites call these where nothing fixes the scalar (a vertex
  inside a `vec!`), so a `T: Real` door would need a turbofish at the
  sites it exists to shorten — which is why that home is monomorphic
  at `Interval`. `geom-brep`'s generic `p3` needed one type annotation
  where it replaced `review_m2_pr3_certify.rs`'s monomorphic `ipt`
  (`survives_interval_line_certification`).
- **What it would collapse**, at `360eb7320` plus the fold: the three
  homes above, `pncad::authoring`'s four doors (which could delegate),
  and these single copies — `topo/src/boolean/join.rs`'s test module
  (`iv`/`p3`/`v3`), `topo/tests/trim_3_chart_bound.rs` (`iv`/`p2`),
  `topo/tests/review_m2_pr3.rs` (`sp`/`wp` over a local `f`),
  `topo/tests/cube_doors_agree.rs` (two inline lifts),
  `geom-brep/tests/arc_eval_anchor.rs` (`p2`),
  `geom-brep/tests/interior_iso_review.rs` (`f` over a tuple),
  `geom-core/src/real.rs`'s test `iv` closure,
  `profile/tests/interval_lane.rs` (`ip2`) with its closure twin in
  `cert4r2_e2e.rs`, and the `Probe` pair in `review_s2_probe.rs` /
  `scalar_channels_probe.rs` (`pp`). The `profile` pairs are two local
  closures and one suite helper: a crate-local home for them would be
  new vocabulary for four sites, so they were left to this question
  (method item 6).

Nothing drifts at any of these, because every one routes through
`Real::from_f64`, the one lift door; that is why this stays P3 and
why the answer may well be "no door".

Out of this row: the `f64` constructors that lift nothing
(`fn p2(x, y) -> Point2<f64> { Point2::new(x, y) }` and its kin),
which are `f64-point-aliases-are-copied-beside-their-binarys-home`.

## What was folded (PR #PRNUM, 2026-09-26)

- **The `SCHEDULE` tables.** There were never five tables: the five
  production lifts read two consts, `splitting::containment::SCHEDULE`
  (3-D; read by `containment`, `order` and `boolean::solid_contain`)
  and `chart_region::SCHEDULE_2D` (read by `chart_region` and
  `chart_bound`), distinct by dimension. Both are now typed
  `[Vec3<f64>; 16]` / `[Vec2<f64>; 16]`, and all five lifts are
  `r.map(T::from_f64)`. Bit-identity was shown by a throwaway in-crate
  test holding the old `[[f64; N]; 16]` literals verbatim from
  `360eb7320`: every component's `to_bits` equal, and every lifted
  component's `lo`/`hi` bits equal between the old per-component
  spelling and `map`, at `f64` and at `Interval` (160 lifted
  components). Its divergent control, one entry moved by one ulp, red.
- **`sweep`'s interval literals**: 42 helper definitions across 29
  suites of the one `all` binary (22 `iv`, 14 `p2`, 5 `p3`, 1 `v3`;
  nine of the files nest them inside a module), plus
  `m5_pr11_quad_interval`'s `i` and `m5_pr6_pcurves`'s `ip2` closure,
  onto one home, `tests/common/interval.rs`, registered in
  `common/mod.rs`'s routing list.
- **`geom-brep`**: `review_m2_pr3_certify.rs`'s `ipt`/`ivec` and
  `onb_c_payoff_interval.rs`'s `p` closure onto `shared::point`.
- **`topo`**: `test_support_fixtures.rs`'s three identical point-lift
  closures handed to `prism_ops` onto one private `identity_map`.

## Why the first census missed most of `sweep`

The row's instrument grouped calls to `from_f64`/`constant` written at
the coordinate. Twenty-two of `sweep`'s suites lift through a local
`fn iv`, so their `p2`/`p3`/`v3` never spell `from_f64` at a coordinate
and were invisible to it — the blind spot the row itself disclosed ("a
coordinate lifted into a named local"), at the scale of a whole
binary. The re-take used the construction instead: every `fn` whose
arguments are two or three `f64` and whose return type is a
`Point`/`Vec`, over `git grep` with no path argument, and the same for
closures, each read by its body.

## Why this row is on this slate

The finding was one thing spelled many times, which is this program's
charter. What is left is `geom-core`'s surface; its owner may claim the
row by `git mv`.
