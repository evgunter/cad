---
id: refusal-rows-that-count-instead-of-snapshot
kind: issue
title: Two in-lib refusal rows check the body unchanged by one arena count, and null.rs keeps a pcurve-rows check the deep snapshot now subsumes
status: open
opened: 2026-09-30
priority: P3
---


## What

Found by the receipt sweep of
`deep-snapshot-does-not-walk-the-body-side-tables`, which makes
`fixtures::deep_snapshot` (`crates/topo/src/fixtures.rs`) walk every
table on `Body`. Three in-lib rows observe "the body is unchanged
after a refusal" through something narrower, or alongside it:

- `instance.rs`, `a_source_that_is_not_a_single_solid_refuses_typed`
  (~`:441`). `graft_disjoint` refuses an empty source and a two-solid
  source. The first case checks `dst.solids().count() == 1` ("and
  nothing was written"); the second checks nothing about `dst`. A
  refusal that wrote any row of any other table, or a solid's
  payload, passes both.
- `review_m1_pr2/release_corruption.rs`, `empty_body_error_paths`
  (~`:340`). Six refusals on an empty body, then
  `body.vertices().count() == 0`. A refusal that minted a point, a
  curve, a surface or any side-table row passes.
- `null.rs`, `a_refusal_on_the_second_face_leaves_the_body_untouched`
  (~`:764`). Beside `deep_snapshot` it compares a `rows` closure over
  `body.pcurves()`. The snapshot now walks the pcurve table, so the
  second comparison is subsumed and can go.

## Shape to consider

Compare the first two by `fixtures::deep_snapshot` before and after
each refusal, and drop `null.rs`'s `rows` companion. `null.rs` is under
a live TOPO lane's diff, so that edit waits for it to land.
