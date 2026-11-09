---
id: main-red-the-circle-lane-coefficient-charge-row-fails-on-some-seeds
kind: issue
title: geom-brep's circle_lane_rows coefficient-charge fuzz row fails on some seeds, on main and every PR's ci
status: open
opened: 2026-11-09
---


## What

`geom-brep pcurve_cache::circle_lane_rows::the_other_charts_circle_coefficients_stay_within_their_charge` (`crates/geom-brep/src/pcurve_cache/circle_lane_rows.rs:1716`) is in the fast (`ci`) set and draws a fresh seed each run. Some seeds fail, which reds whichever PR's hosted `test (kernel)` job draws one.

- Seed `0xbb6048d2e37fd735`: "draw 367: the torus's coefficients' exact excess is over their charge". Found by PR 6624's delta review, which replayed it with `CAD_FUZZ_SEED` and got the same failure on base.
- Hosted run 42653548115 (job 129287125502), PR 6632 head `733b0579`: the same assertion at :1716. PR 6632 does not touch `pcurve_cache`.

## Owed

1. Replay seed `0xbb6048d2e37fd735` and decide what it shows.
   - The torus chart's coefficient charge may undercount: a soundness defect in the charge, which is the fix.
   - Or the exact excess may be computed with a lost bit: a defect in the oracle.
2. Pin the failing seed as a fixed row.
3. Until it is fixed, the fuzz keeps reddening unrelated PRs. Re-running a job is the stopgap, cited to this row.
