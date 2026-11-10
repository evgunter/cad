---
id: main-red-the-circle-lane-coefficient-charge-row-fails-on-some-seeds
kind: issue
title: geom-brep's circle_lane_rows coefficient-charge fuzz row fails on some seeds, on main and every PR's ci
status: closed
opened: 2026-11-09
closed: 2026-11-10
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

## Closed (ONECOPY, `onecopy/torus-harmonics-charge`, 2026-11-09)

**The charge undercounts; the oracle is right.** Seed `0xbb6048d2e37fd735`'s draw 367 is a circle of radius 0.055 near the axis of a fat torus (`R` 605.5, `r` 589.1). All the excess is in `c0`, about 1.5 times the charge, and it is unchanged when the oracle reads the f64 `v_ref` the implementation reads, so the oracle loses no bit.

`implicit::conic_torus_harmonics` charged `terms` with `p` at `|S + R² − r²|`. `R² − r²` (about 19613) cancels about 18 to 1 there, so it carries the rounding of `R²` and `r²` (about 1e-10) whole, and `p₀²` doubles it at `p₀`: about 4e-6 against a charge of 1.07e-6. `p` is now charged at `|S| + R² + r²`, each radius whole, the rule the sphere and cylinder already keep (`+ r²`).

The draw is pinned as `pcurve_cache::circle_lane_rows::a_fat_toruss_coefficients_charge_its_radii_whole`, red without the fix and green with it. The fuzz row and the pin share one torus check, `torus_coefficients_within`.

**The class, swept.** Every `rounding_charge` and `terms` site in geom-brep, topo and geom, and every difference of squared radii in their non-test source: `conic_torus_harmonics` was the only charge that bounds a term by a cancelled difference. `slit::torus_between` divides by `R² − r²` as a multiplicative floor (its relative error stays about 18ε). The ray–torus quartic (`solid_contain::line_torus_roots` → `quartic::depressed_quartic_roots`) builds `R² − r²` into coefficients that no decision charges at all, a different defect already filed as `work/inside/ray-torus-count-certifies-on-an-uncharged-discriminant.md`.

**Figures, named (follow-up, 2026-11-10).** The measured excess on draw 367 lies between 1.1 and 1.5 times the old charge (it passes at ×1.5, not at ×1.1). The "about 4e-6 against 1.07e-6" above is the analytic worst case, `2·p₀·δk` with `δk` the rounding `R² − r²` can carry, not the measured excess.
