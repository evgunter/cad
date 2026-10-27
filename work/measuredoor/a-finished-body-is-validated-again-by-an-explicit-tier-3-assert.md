---
id: a-finished-body-is-validated-again-by-an-explicit-tier-3-assert
kind: issue
title: about 380 test functions finish a body and also assert validate_geometric, so tier 3 runs twice where they read one body; the asserts right after finished cannot fail
status: open
opened: 2026-10-24
priority: P3
cost: M
---



Filed by FLUX (unit F2, review of PR 5668). F2 moved sweep's and
editor-core's measurement sites onto the typed door, so most measured
bodies now go through `topo::test_support::finished` (or
`AtRestBody::validate`), which runs tier 3. Many of the same functions
also assert tier 3 themselves, so the body pays the gate twice.

## Measured

A crude scan of `crates/sweep/tests` and `crates/editor-core/tests` at
F2's head finds 383 test functions, in 183 files, that both finish a body
and call `validate_geometric`. Not every pair reads the same body. In 44
functions the `validate_geometric` argument is the very binding `finished`
returned.

An `assert_eq!(validate_geometric(&body, tol), Ok(()))` placed right after
`let body = finished(...)` cannot fail: `finished` already panicked if
tier 3 refused. For example:

- `crates/sweep/tests/review_m3_pr1_sweep.rs:276` and `:329`;
- `crates/sweep/tests/m6_surgery.rs:268`, in the tier ladder after
  `finished("body", …)` at `:266`.

`crates/sweep/tests/m7_skin_integral.rs` shows the fold, done in F2 for
the fast-set bar. Tiers 1 and 2 are asserted on the plain body, and the
tier-3 rung *is* the finish:
`let body = AtRestBody::validate(Body::clone(&x.body), tol).expect("tier 3 at rest")`.

## Also here

- **Local copies of `finished`.** These tests and examples each define
  their own instead of using `topo::test_support::finished`:
  - in `crates/sweep/tests`: `a_mirrored_body_is_its_image.rs:30` (this
    one returns a `Body`), `a_vertex_on_a_grazing_plane.rs:28`,
    `join_pierce_runs_sweep.rs:262`, `join_pierce_strut_facing.rs:186`,
    `pinch_faces_tessellate.rs:109`, `review_r2_vv_probes.rs:106` and
    `section_end_poses.rs:37`;
  - in `crates/sweep/examples`: five more.
- **A measure-a-finished-body helper.** Many sites spell
  `topo::mass_properties(&finished("…", Body::clone(&x), tol), tol)` in
  full. A `test_support` helper that takes a `&Body` and a label, finishes
  it and measures it would shorten them, and would put the one gate in
  one place.

## What closes it

Each pair on one body is folded to a single tier-3 run:
- a ladder's tier-3 rung becomes the finish;
- an assert right after `finished` goes.

The local copies give way to the shared helper.
