---
id: line-torus-roots-may-certify-noise-when-the-line-origin-is-far
kind: issue
title: The line × torus and ray × torus quartic ladders lever by R+r and may certify f64 noise when the line's parameter origin sits far from the torus
status: closed
opened: 2026-09-29
priority: P1
cost: M
closed: 2026-09-29
branch: germ/line-torus-noise
refs: [circle-crosses-a-torus-face-with-no-root-lane]
---


## What

PR 3375's delta review measured the circle × torus half-angle quartic certifying f64 rounding noise at large circle radius, with the lever at R+r. The coefficients scale as ρ⁴, so the harmonics' rounding noise, about ρ⁴·u, outgrows the band: 96 of 1500 dip/bump poses were wrong at ρ=100 m for R=1, r=0.25. The fix was a noise meter in `half_angle_roots`.

The line × torus quartic (`line_torus_roots`, used by `wall_crossing` since PR 3265) and the ray × torus quartic (`point_in_solid`'s torus arm, PR 3255) share the Ferrari ladder (`depressed_quartic_roots`). If a line's parameter origin, or its direction scale, sits far from the torus, the same coefficient growth may apply. That is unmeasured.

## Measure

Take lines and rays whose parameter origin is 10–1000 m from a torus with R=1, r=0.25, passing within ±[1e-8, 1e-5] m of the tube, and compare against an exact-ish oracle. Include `point_in_solid`, whose ray origin is the query point. If any answer is certified wrong, raise this item to P0 and give both lanes the same noise meter, or recenter the parameter at the foot nearest the torus.

## Home

GERM, beside `circle-crosses-a-torus-face-with-no-root-lane` (PR 3375).

## Closed — measured, not live (germ/line-torus-noise)

**No wrong certified answer at any origin distance in the kernel's
scale.** Neither lane has the circle lane's lever, because
`line_torus_roots` already recentres: the depression `y = t + b`
puts the ladder's variable at the foot of the perpendicular from the
torus centre, and `p`, `q̂`, `s` are built from `m = |perp|²`,
`n = perp·â` and `e = d·â` alone. For any line that reaches the tube,
`|perp| ≤ R + r`, so the coefficients are bounded by the torus's own
extent whatever the parameter origin is. The origin `L` enters only
through `perp = w0 − d·b` and `t = y − b`, each an absolute error of
order `L·u` — linear in `L`, never `L⁴`. `Curve3::Line`'s `dir` is
unit by convention and every non-test mint normalises it, so there is
no direction scale to lever by either; the ray lane normalises its
schedule directions itself.

**Measurement.** Torus `R = 1`, `r = 0.25`, random centre and axis.
3000 lines per origin distance, each tangent to the tube at a random
point (any `u`, `v`, including the inner saddle), offset along the
normal by `±10^U[log 1.6e-8, −5]` metres (a dip or a bump), a third of
them tilted through the normal by `10^U[−7, −3]` (near-tangent
crossings); the parameter origin is then moved `L` along the line.
f64 at the default band. Oracle: the exact quartic of the represented
line (the f64 `q`, `d` as given), solved by mpmath `polyroots` at 80
digits. Wrong = certified count differs, or a certified root is more
than 1e-8 m from the oracle's nearest real root.

| origin `L` (m) | answered | refused | wrong count | root > 1e-8 m | max root error |
|---|---|---|---|---|---|
| 0    | 2652 | 348 | 0 | 0 | 3.5e-12 |
| 1    | 2652 | 348 | 0 | 0 | 3.6e-12 |
| 10   | 2652 | 348 | 0 | 0 | 7.3e-12 |
| 100  | 2652 | 348 | 0 | 0 | 7.3e-11 |
| 1000 | 2652 | 348 | 0 | 0 | 5.0e-10 |
| 1e4  | 2652 | 348 | 0 | 0 | 6.0e-9  |
| 1e5  | 2479 | 348 | 0 | 173 | 4.9e-8 |
| 1e6  | 1728 | 348 | 0 | 924 | 5.2e-7 |

The refusal set is the same 348 poses at every `L`: the ladder sees
the same coefficients. No count is wrong even at 1e6 m. The root
error grows linearly, about `5e-13·L`, which is `L·u` magnified by
the near-tangent incidence (a normal displacement `η` moves a root by
`η / sin θ`). It stays under the escalation threshold through 10 km,
which is past D4's kilometre coverage; beyond 100 km a root
drifts past 1e-8 m. That is outside the kernel's stated scale,
and the consumers re-check a landing point on the tube
(`torus_landing_rows`) before acting on it.

`point_in_solid` on a full donut (`R = 1`, `r = 0.25`, axis `y`):
1000 queries per `L ∈ {1, 10, 100, 1000}`, half at random directions
`R + r + L` from the centre, half placed so the first schedule ray
(`+x`) passes `±[1.6e-8, 1e-5]` m off the tube (outer or inner
equator tangents, and the top and bottom circles). Against the closed-form
signed distance: 0 wrong at every `L`; 44–54 refusals per `L`
(an `Err` from `point_in_solid`; the cause was not broken down). A far query
point is outside by construction, and a miscounted grazing pair
cannot flip a closest-hit verdict from there, so this lane is the
weaker probe. The line measurement above is the one that bears.

The harness (a scratch `#[ignore]` dump plus the mpmath oracle) was
not committed. It asserts nothing a row would keep, and the
recentring that makes the lane safe is structural.
