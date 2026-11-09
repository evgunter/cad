---
id: circle-lane-rows-torus-coefficients-exceed-their-charge-on-one-seed
kind: issue
title: geom-brep pcurve_cache::circle_lane_rows: the torus's coefficients' exact excess is over their charge at draw 367 of seed 0xbb6048d2e37fd735
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Reported by a ONECOPY lane (2026-11-09) as replaying identically on its base; re-run on main (merged 2026-11-09) by `onecopy/main-reds-k-and-m10`, still red.

`crates/geom-brep/src/pcurve_cache/circle_lane_rows.rs`,
`the_other_charts_circle_coefficients_stay_within_their_charge` (a
varying-seed sweep), at `CAD_FUZZ_SEED=0xbb6048d2e37fd735`, ε 1e-9: "draw
367: the torus's coefficients' exact excess is over their charge". The row
checks that each chart's circle coefficients, read exactly, lie within
the rounding charge the lane assigns them; on this draw the torus chart's
do not, so the charge undercounts some evaluation of a torus circle.

Not bisected (a seed found today; the row's generator may have changed with
the draw). Pin the draw as a deterministic row beside the fix.

Reproduce: `CAD_FUZZ_SEED=0xbb6048d2e37fd735 cargo nextest run -p geom-brep
--profile ci --ignore-default-filter -E
'test(/circle_lane_rows::the_other_charts_circle_coefficients/)'`.
