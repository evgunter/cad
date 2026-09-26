---
id: offset-fit-seed-grid-is-a-fourth-equal-split-spelling
kind: issue
title: offset_fit::seed_direction re-spells the equal-split interior grid and a third emptiness test
status: open
opened: 2026-09-26
priority: P1
cost: E
---


Filed by the fix pass on PR 3292 (`encl/equal-split-points-home`),
which homed "cut every nonempty span into `splits` equal pieces,
skipping float-collapsed slivers" in
`geom_core::spline::algebra::equal_split_points`.

## What

`offset_fit::seed_direction` (`crates/geom-brep/src/offset_fit.rs`,
~1390) still writes the whole per-span interior grid itself:
`lo + (hi − lo)·(k / N)` for `k in 1..N` with the `t > lo && t < hi`
sliver guard, `N = OFFSET_FIT_SEED_PER_SPAN`. It also spells the span
emptiness test a third way — `!(hi > lo)` under
`#[allow(clippy::neg_cmp_op_on_partial_ord)]`, after a
`let (Some(&lo), Some(&hi)) = (knots.get(span), knots.get(span + 1))
else { continue }` that `first_span()..=last_span()` already makes
unreachable — where the home uses `KnotVector::span_is_nonempty`.

## What a fix looks like

The interior points are `equal_split_points(kv, OFFSET_FIT_SEED_PER_SPAN)`;
what `seed_direction` adds on top is the span ends (every distinct knot
value from the first to the last) and its `OFFSET_FIT_DEGREE + 1`
top-up. Either merge `equal_split_points`' output with the distinct
knot values, or keep the per-span walk for the ends and take the
interior from the home. Output should be bit-identical: the expression
and guard are the same.

Not fixed in PR 3292 because another ENCL lane was editing
`offset_fit.rs` at the time.
