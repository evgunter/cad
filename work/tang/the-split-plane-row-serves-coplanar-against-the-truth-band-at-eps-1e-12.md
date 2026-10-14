---
id: the-split-plane-row-serves-coplanar-against-the-truth-band-at-eps-1e-12
kind: issue
title: span_reach_differential's split rows serve coplanar where main escalated, against the truth band, at eps 1e-12 (seed 0x4a5a1477da7f8d8a)
status: closed
opened: 2026-10-08
closed: 2026-10-08
priority: P0
cost: M
---


Found on PR 4300's CI (2026-10-08), red in the ε 1e-12 step on a row
whose crate the PR does not touch (`crates/geom-brep` is identical to
main f4761f30).

## What

`geom-brep::all span_reach_differential::the_split_rows_serve_where_main_escalated_only_on_their_truth`
(PR 4292, the span-bounded conic reach) draws a random seed per run. At
ε 1e-12 a seed can draw a plane-sector case the head serves `coplanar`
where main escalated, against the truth band:

```
CAD_TOLERANCE_EPS=1e-12 CAD_FUZZ_SEED=0x4a5a1477da7f8d8a CAD_FUZZ_EFFORT=1 \
  cargo nextest run -p geom-brep --test all \
  -E 'test(/span_reach_differential::the_split_rows/)'
```

fails with "split #218: head serves coplanar where main escalated,
against the truth band". The same seed passes at ε 1e-6, and 1 in 3
unseeded runs at 1e-12 failed locally. So main's nightly row at 1e-12
is red on some nights, and any PR whose diff seeds geom-brep's eps rows
(or `all()`) can go red on it.

## Owed

Pin the seed's case as a deterministic row, then decide whether the
split plane row's served verdict is wrong at that scale (a wrong
answer, which makes this P0) or the truth band the row compares against
is mis-scaled at 1e-12.

## Closed

PR 4305: the row's truth was the test's own and could not resolve
ε 1e-12. The fix is an exact `sector_offset` and the Q6 room cap in the
test; there is no production change.
