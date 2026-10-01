---
id: coordinates-lifted-into-a-point-are-spelled-per-component
kind: issue
title: Whether geom_core should carry a public literal constructor (a point or vector at T from f64 coordinates) below pncad::authoring's p2/p3/v2/v3 — a question for the owner of geom-core's public surface
status: open
opened: 2026-09-24
priority: P3
cost: E
design: true
---


## What is left

One question, and it is a design question, not a fold: **is a public
door in `geom_core` wanted that builds a `Point2`/`Point3`/`Vec2`/`Vec3`
at a scalar `T` from `f64` coordinates?** Today the only public door of
that shape is `pncad::authoring`'s `p2`/`p3`/`v2`/`v3`, which is uphill
of every crate below `pncad`. Adding one to `geom_core` would be new
public API on the kernel's leaf crate, so it is the call of whoever
owns that surface, not a duplication lane's.

## Why this row is on this slate

It was filed on S-DUP's slate by the lane that folded the neighbouring
class, and S-DUP's PR #3304 folded everything that did not need this
call. What is left is a question about `crates/geom-core/src/linalg/`'s
public surface, and `work.py territory --files -` over that directory
names two programs: this one, whose `paths` are exactly
`crates/geom-core/src/linalg/*`, and `props`, which reaches it only
through its crate-wide `crates/geom-core/src/*`. The specific claim is
this program's, so the row moved here (2026-09-26). Whether to escalate
it is this program's call.

## The honest cost of a downhill door

What it would buy is small, and that belongs in front of the owner:

- **The spelling it would replace is already one expression at a door
  that exists.** `Point3::new(x, y, z).map(T::from_f64)` is the
  componentwise lift through `map`, public and downhill. Every home
  PR #3304 left is written that way, so a constructor would save only
  the `.map(...)`.
- **The copies are at one home per binary where a binary has many**:
  `sweep`'s suites at `tests/common/interval.rs` (`iv`, `p2`, `v2`,
  `p3`, `v3` at `Interval`), `geom-brep`'s at `tests/shared/point.rs`
  (generic `p3`/`v3`), and `topo`'s at `topo::test_support::identity_map`
  (generic, `Point3` only).
- **A generic door does not serve the densest home.** `sweep`'s
  interval suites call these where nothing fixes the scalar (a vertex
  inside a `vec!`), so a `T: Real` door would need a turbofish at the
  sites it exists to shorten — which is why that home is monomorphic.
  `geom-brep`'s generic `p3` needed one type annotation where it
  replaced a monomorphic copy (`review_m2_pr3_certify.rs`,
  `survives_interval_line_certification`).
- **What it would still collapse**: the three homes above,
  `pncad::authoring`'s four doors (which could delegate), and the
  sites listed under "Left" below. Nothing drifts at any of them,
  because every one routes through `Real::from_f64`, the one lift
  door; that is why this stays P3, and why the answer may well be "no
  door".

## Every member of the base census, and where it went

The base census (44 groups at `71f8ce204`) and PR #3304's re-take by
construction (every `fn` or closure taking two or three `f64` and
returning a `Point`/`Vec`, `git grep` with no path argument, each read
by its body). **Folded** means the copy is gone and its sites reach a
home; **through `map`** means the site now spells the lift as
`<Type>::new(…).map(lift)`, the one door, because no home in its binary
serves it (a generic `T`, or a lone copy).

Production:
- The five `SCHEDULE` lifts (`boolean/solid_contain.rs`,
  `chart_bound.rs`, `chart_region.rs`, `splitting/containment.rs`,
  `splitting/order.rs`) — the two tables typed `[Vec3<f64>; 16]` and
  `[Vec2<f64>; 16]`, every lift through `map`; bit-identity shown in
  PR #3304.

`sweep` (one `all` binary; home `tests/common/interval.rs`):
- Folded: 42 helper definitions in 29 suites (22 `iv`, 14 `p2`, 5
  `p3`, 1 `v3`), `m5_pr11_quad_interval`'s `i`, `m5_pr6_pcurves`'s
  `ip2` closure, the five `let iv = Interval::from_f64` bindings
  (`m5_pr5_tilted_cut` ×2, `r1_lane0_e2e` ×2, `verbs_offc_consumer`),
  and 86 inline per-component constructions in 27 suites.
- `sweep/src/test_support.rs`'s three (`corners`, and the `v` closures
  of `waisted_at` and `bowl_at`): generic `T`, so the home cannot serve
  them; the two closures now call `corners`, and `corners` is through
  `map`.
- `cert_m2r1_passes.rs`'s `v`: a bulge vertex `(Point2<T>, T)` over a
  generic `T` — the base row counted it for its point half. The point
  half is through `map`; the vertex stays the suite's own.
- `issue93_az_intersect.rs`'s polygon lift: generic `T`, through `map`.
- Left: `spiric_rim.rs`'s `vessel_loop`/`vessel_at` construct through
  a lift passed in as a parameter (one spelling for both scalars), not
  through `Interval`; `cert_m2r1_passes.rs` (8) and
  `issue93_az_intersect.rs` (2) inline constructions at a generic `T`;
  `common/operands.rs`'s computed coordinate; `k_report`,
  `must_carry_rule` and `review_must_carry_rule_r1_probes` construct at
  the `Probe` scalar.

`topo` (lib test modules and the `all` binary; home
`topo::test_support::identity_map`, re-exported so both can reach it):
- Folded onto `identity_map`: `test_support_fixtures.rs`'s three
  `prism_ops` closures, `boolean/join.rs`'s `frame_dispatch_interval_tests`
  `p3`, `review_m2_pr3.rs`'s `wp` and the two inline points in its
  `fixed_interval_lane_certifies_self_loop_scaffolding`.
- Through `map`: `review_m2_pr3.rs`'s `sp` and `w`,
  `trim_3_chart_bound.rs`'s `p2`, `boolean/join.rs`'s test `v3`,
  `review_m3_pr55.rs`'s mapped corner (a transform, so not the
  identity), and `tests/fixture/mod.rs`'s `CYL_ORIGIN`, now typed
  `Point3<f64>` and lifted as `CYL_ORIGIN.map(T::from_f64)`.
- Left: `cube_doors_agree.rs`'s interval-row `ident`. It IS the
  identity map, and routing it through `identity_map` blinds the row:
  with `identity_map` planted to shift `x` by `0.5`, the row reds as it
  stands and passes when routed (PR #3304). It carries a
  ``Deliberately NOT `common::identity_map` `` note. The remaining
  inline generic-`T` plane origins and normals in `topo/tests`
  (`m3_pr2_reduce`, `m3_pr3_split`, `review_m3_pr2`,
  `review_m3_pr3_order`, `review_m3_pr3_rings`, `m6_3_chart_completion`)
  and `topo/src` (`boolean/r1_probes.rs`, `chart_region.rs`,
  `props.rs`, `chord_join.rs`) are construction sites at a generic `T`
  or with computed coordinates: this row's population if a door is
  wanted.

`geom-brep` (home `tests/shared/point.rs`, now through `map`):
- Folded: `review_m2_pr3_certify.rs`'s `ipt`/`ivec`,
  `onb_c_payoff_interval.rs`'s `p`, both importing the doors under
  their own names.
- Left: `arc_eval_anchor.rs`'s `p2` (a 2-D lone copy),
  `interior_iso_review.rs`'s `f` over a tuple.

`geom-core`, `geom`, `profile`:
- Through `map`: `geom-core/tests/onb_signed_zero_evidence.rs`'s ring
  and `r2_cert3_probes.rs`'s axis — lone copies, one binary.
- Left: `geom-core/src/real.rs`'s test `iv` closure (a lone copy);
  `geom/tests/dual_foot_tangent.rs`'s `p`, which adds a shift and a
  computed `z` after the lift, so it is not the lift; `profile`'s pairs
  (`interval_lane.rs`'s `ip2` with `cert4r2_e2e.rs`'s closure, and the
  `Probe` `pp` closures in `review_s2_probe.rs` and
  `scalar_channels_probe.rs`) — four local sites, for which a
  crate-local home would be new vocabulary (method item 6).

`pncad::authoring`'s `p2`/`p3`/`v2`/`v3` are the public doors and stay.

Out of this row: the `f64` constructors that lift nothing (`fn p2(x,
y) -> Point2<f64> { Point2::new(x, y) }` and its kin), which are S-DUP's
`f64-point-aliases-are-copied-beside-their-binarys-home`.

## Why the first census missed most of `sweep`

The base instrument grouped calls to `from_f64`/`constant` written at
the coordinate. Twenty-two of `sweep`'s suites lift through a local
`fn iv`, so their `p2`/`p3`/`v3` never spell `from_f64` at a coordinate
and were invisible to it — the blind spot the row itself disclosed ("a
coordinate lifted into a named local"), at the scale of a whole binary.
