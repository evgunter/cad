---
id: test-support-has-no-exact-dyadic-decoder-so-seven-tests-carry-their-own
kind: issue
title: Test support has no exact f64-to-dyadic decoder, so seven test modules each carry a private copy
status: open
opened: 2026-10-09
priority: P3
cost: E
---


Found by ENCL's review of PR 4386 (Q1/Q6), which added one more copy.

## What

Exact-arithmetic oracles in the test suites decode an `f64` into an
exact dyadic or rational (`mantissa · 2^exponent` over `num_bigint`),
then add, multiply and compare. No shared helper exists, so each suite
writes its own, private to its module or test crate:

- `topo::props::quad_lane::tests::polygon_tests::Dyadic`
  (`crates/topo/src/props/quad_lane.rs`);
- `geom_brep::implicit` tests' `Dyadic` (`crates/geom-brep/src/implicit.rs`);
- `Dyadic` in `crates/bvh/tests/ray.rs`;
- `Q` in `crates/bvh/tests/ray_r2.rs`;
- `Q` in `crates/topo/src/boolean/sectors/cone_fuzz.rs`;
- `Q` in `geom_core::spline::compose`'s tests
  (`crates/geom-core/src/spline/compose.rs`);
- `big` in `crates/editor-core/tests/review_pick3_r2_probes.rs`.

Each repeats the same IEEE-754 bit decode, with its own handling of
subnormals and zero. A slip in one copy is a wrong oracle that no other
copy catches.

## The shape to give

One exact decoder with add, multiply, negate and compare, in
`test-utils`, behind the dev-only edge every suite already has. The
copies then move to it one by one. `num-bigint` would become a
`test-utils` dependency; `test-utils` is dependency-free today
(`crates/topo/Cargo.toml`'s note on it), so that cost belongs in the
ruling on where the helper lives.
