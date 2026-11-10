---
id: predicate-3-reads-a-spine-past-the-axis-as-regular
kind: issue
title: blend: spine_regularity reads a negative spine radius as regular, so a frustum's base rim at r = 0.42–0.46 passes predicate 3 and refuses in the surgery's annulus host trim
status: open
opened: 2026-11-10
priority: P2
cost: E
---


## Finding

Found by BANDREACH's fix pass on PR 6779 (lane `bandreach-dome`), on
the reviewer's question whether predicate 3 is sign-blind.

`crates/sweep/src/blend/battery.rs:679` `spine_regularity` takes the
margin `r − r²·κ_spine`. The coaxial sheet arm stores the spine radius
`s` SIGNED (`crates/sweep/src/blend/arms.rs:1054`, `Meridian::blend`)
and `κ_spine = 1/s` (`arms.rs:1095`), so where the ball's centre has
crossed the axis (`s < 0`) the margin is `r + r²/|s| > 0` and the
predicate passes a spine that is not regular.

Witness: the frustum `(0,0) (1,0) (0.2,0.8) (0,0.8)` revolved fully
about y (a 45° cone wall on a full base disc). Its base rim's ball
centre sits `r/tan(22.5°) ≈ 2.414·r` in from the rim, past the axis
from `r ≈ 0.414`. `fillet_edges` on `rim_arcs_at(1.0, 0.0)` reads:

| r | outcome |
|---|---|
| 0.3 | `SpineIrregular`, margin −0.0264 |
| 0.4, 0.41 | `SpineIrregular`, margins −4.26, −16.1 |
| 0.42 – 0.46 | `Op { site: "annulus host trim mef", source: Certification(StatedInterval …) }`, margins −0.044 to −0.347 |
| 0.47, 0.5 | `FaceClearanceUncertified` |

So from `r = 0.42` the spine passes predicate 3 and the request is
refused deep in the surgery's assembly, typed but under the wrong
reason, with no predicate-3 recourse.

## What it needs

Predicate 3 reading the spine's radius as a magnitude the ball must
clear on its own side: refuse `SpineIrregular` wherever `s ≤ r`, `s`
signed (a centre past the axis included). Pin the frustum ladder above.
