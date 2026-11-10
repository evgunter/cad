---
id: circle-lane-rows-torus-coefficients-exceed-their-charge-on-one-seed
kind: issue
title: geom-brep pcurve_cache::circle_lane_rows: the torus's coefficients' exact excess is over their charge at draw 367 of seed 0xbb6048d2e37fd735
status: closed
opened: 2026-11-09
priority: P3
cost: E
closed: 2026-11-10
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

**Diagnosed (ONECOPY, 2026-11-09):** the torus chart's charge undercounts. `implicit::conic_torus_harmonics` charged `p` at `|S + R² − r²|`, a cancelled difference on a fat torus; it now charges `|S| + R² + r²`. The fix and the pinned draw are on `onecopy/torus-harmonics-charge`, which also adds ONECOPY's row for the same red (`main-red-the-circle-lane-coefficient-charge-row-fails-on-some-seeds`, which reds random PRs' hosted kernel jobs) and closes this row into it.

## Closed (ONECOPY, `onecopy/torus-harmonics-charge`, 2026-11-10)

Closed into `work/onecopy/main-red-the-circle-lane-coefficient-charge-row-fails-on-some-seeds.md`, the same red, which records the fix (the torus harmonics charge `R² − r²` at `R² + r²`) and the pinned draw.
