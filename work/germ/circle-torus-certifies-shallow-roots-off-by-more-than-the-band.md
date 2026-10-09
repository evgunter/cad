---
id: circle-torus-certifies-shallow-roots-off-by-more-than-the-band
kind: issue
title: The circle x torus door certifies shallow-crossing roots up to 166 K-eps along the carrier from the truth at eps 1e-12: its subdivision passes no root-slack meter
status: closed
opened: 2026-10-08
closed: 2026-10-08
branch: germ/circle-torus-outside-band
pr: 4357
priority: P0
refs: [the-half-angle-ladder-certifies-in-band-configurations, circle-torus-meters-accept-an-unreadable-reading, degree-2-subdivision-doors-carry-no-root-slack-meter]
---


Found by GERM's circle × torus measurement lane (2026-10-08, branch
`germ/circle-torus-outside-band`), which asked whether the door on main
certifies any answer that is wrong OUTSIDE the band. It does, at
`ε = 1e-12`, by root placement only. A later escape of DR-54 (PR 3805).

## What

`circle_torus_roots` (`crates/topo/src/boolean/circle_torus.rs:274`)
hands the general pose to `half_angle_roots` with `slack: None`
(`circle_torus.rs:293`). Its answer is `certified_subdivision`'s
(`crates/topo/src/boolean/circle_roots.rs:594`), which bisects a
monotone piece's root on the `f64` residual and requires only that the
root read ON the surface (`circle_roots.rs:738`). At a shallow
crossing, the residual's slope along the carrier is small, and the
`f64` residual's sign change sits `≈ u·|p| / slope` of arc from the
true root: about 1e-15 m of residual noise over a slope of 1e-8 to
1e-6. The root's point is on the torus to about 1e-15 m. But it is
placed up to 1.85e-9 m along the carrier from the true root, which is
185 escalation thresholds at `ε = 1e-12` (`K = 10`). Nothing refuses
it. The `RootSlack` meter (`circle_roots.rs:277`) exists for exactly
this case ("or the span and trim decisions the caller makes on the root
are made on the wrong point"). The ellipse × torus and cone doors pass
it. This door has certified the subdivision's roots unmetered since
`31116eeaee` (PR 3805's first fix pass), which made the subdivision
answer in the ladder's place and kept only the ladder's escalations:
the ladder's own `root_slack` was still decided but no longer read,
and `0f74dfdb7e` later removed it as dead code.

Every wrong answer is from the `f64` lane, on the general arm, at
`ε = 1e-12`. No pose on the parallel arm was wrong (the arm is read as
`ρ·|n̂ × â| ≤ ε`). The `Interval` lane's enclosures held the truth in
every case.

## The measurement

The oracle is mpmath at 70 digits on the exact `f64` inputs, with
`v̂ = n̂ × û` taken exactly. Every critical point of `F` comes from the
octic of `F′` in the tangent half-angle. Roots are bracketed between
consecutive critical points and bisected to 1e-45. A pose's depth is
`min |d|` over the signed distance at `F`'s real critical points, and
a pose is outside the band when that depth is at least `Kε`. The
worst cases were re-checked by bisecting the true distance `d` itself,
and they agree to 4 digits.

"Wrong" means any of these:
- a certified count that differs from the oracle's count over the turn;
- `Miss` on a pose with a root;
- a certified root farther than `Kε` of arc from every true root, or a
  true root farther than `Kε` from every certified one.

There are 7,500 poses in five families, plus 1,500 pole variants. Each
was run at three ε values on both lanes.

| family | ε | poses (outside / in band) | f64: answered / refused | f64: WRONG outside / in band | f64 worst root error | Interval: answered / refused / wrong |
|---|---|---|---|---|---|---|
| unit | 1e-09 | 1500 (939 / 561) | 935 / 565 | **0** / 0 | 5.2e-12 m | 870 / 630 / 0 |
| unit | 1e-06 | 1500 (343 / 1157) | 351 / 1149 | **0** / 0 | 6.6e-14 m | 351 / 1149 / 0 |
| unit | 1e-12 | 1500 (1500 / 0) | 1481 / 19 | **35** / 0 | 3.5e-10 m | 831 / 669 / 0 |
| radii | 1e-09 | 1500 (1082 / 418) | 1050 / 450 | **0** / 0 | 3.1e-11 m | 977 / 523 / 0 |
| radii | 1e-06 | 1500 (666 / 834) | 653 / 847 | **0** / 0 | 1.2e-11 m | 645 / 855 / 0 |
| radii | 1e-12 | 1500 (1500 / 0) | 1387 / 113 | **27** / 0 | 1.9e-10 m | 976 / 524 / 0 |
| fillet | 1e-09 | 1500 (1044 / 456) | 1044 / 456 | **0** / 0 | 1.6e-13 m | 1020 / 480 / 0 |
| fillet | 1e-06 | 1500 (641 / 859) | 620 / 880 | **0** / 0 | 6.9e-15 m | 620 / 880 / 0 |
| fillet | 1e-12 | 1500 (1500 / 0) | 1480 / 20 | **0** / 0 | 1.9e-12 m | 1182 / 318 / 0 |
| pole | 1e-09 | 1500 (925 / 575) | 925 / 575 | **0** / 0 | 1.6e-11 m | 652 / 848 / 0 |
| pole | 1e-06 | 1500 (366 / 1134) | 369 / 1131 | **0** / 0 | 5e-12 m | 325 / 1175 / 0 |
| pole | 1e-12 | 1500 (1500 / 0) | 1463 / 37 | **93** / 0 | 1.9e-09 m | 335 / 1165 / 0 |
| parallel | 1e-09 | 1500 (951 / 549) | 822 / 678 | **0** / 0 | 4.1e-10 m | 799 / 701 / 0 |
| parallel | 1e-06 | 1500 (337 / 1163) | 311 / 1189 | **0** / 0 | 1.7e-07 m | 311 / 1189 / 0 |
| parallel | 1e-12 | 1500 (1411 / 89) | 1215 / 285 | **84** / 1 | 9e-10 m | 860 / 640 / 0 |
| tenm | 1e-09 | 1500 (926 / 574) | 922 / 578 | **0** / 0 | 9.6e-12 m | 812 / 688 / 0 |
| tenm | 1e-06 | 1500 (397 / 1103) | 403 / 1097 | **0** / 0 | 1.1e-12 m | 403 / 1097 / 0 |
| tenm | 1e-12 | 1500 (1500 / 0) | 1471 / 29 | **143** / 0 | 4.1e-10 m | 620 / 880 / 0 |

Across all 54,000 runs, there was no wrong count, no `Miss` on a
crossing, no phantom pair and no `OnSurface`, at any ε. All 382 wrong
answers outside the band (and the 1 inside it) are root placements at
1e-12. By depth:

| depth | wrong | worst error |
|---|---|---|
| 1e-11 to 1e-10 m | 177 | 1.85e-9 m |
| 1e-10 to 1e-9 m | 142 | 9.0e-10 m |
| 1e-9 to 1e-7 m | 62 | 7.7e-11 m |
| deeper | 1 | 1.2e-11 m |

## Exact poses (`ε = 1e-12`, band `(1e-12, 1e-11)`, `f64`)

Each row lists `t0 t1 | centre | axis | ρ | u_ref | torus centre |
torus axis | R r`. Torus `u_ref` does not enter.

- **Depth 12 Kε, root off by 90 Kε** (a near-parallel circle at the
  tube's top; tilt `1.2e-12` m lands in the gap, so it takes the
  general arm):
  `-3.9305973654122734 -1.6094454257892792 | 0.01918532248589285 -0.0495052766449747 -0.2499999998297648 | 3.0452648081370403e-13 -1.1362543548330235e-12 1.0 | 1.0530977176863732 | 0.7262695369226678 -0.6874100375599222 -1.0022409549346523e-12 | 0 0 0 | 0 0 1 | 1.0 0.25`.
  The door certifies 2 roots, θ = −3.5972628959633646 and
  −3.5722416128227470. The true roots are 2.685922410923398 − 2π and
  2.710943695213426 − 2π. The second door root is 9.02e-10 m of arc
  from its true root (the first is 3.08e-10 m off), and the residual
  there is −2.2e-17 m.
- **Root off by 185 Kε** (a 10 m-scale torus):
  `8.49296977448893 10.356489392302237 | 5.463512492317101 -9.746523357584595 -8.536346470744599 | -0.37684407561235617 0.7088647754925449 -0.5962375975580313 | 0.9989021230059661 | 0.23491985431202583 -0.5495055589517243 -0.8017832018265096 | 5.89035735624101 -9.360284482369583 -6.886263665719765 | 0.2667503859403426 -0.8732284204487586 0.4078190252075601 | 2.4284544765665266 0.3025175616444298`.
  Its depth is 1.50e-11 m. The door certifies 4 roots, among them
  θ = 6.2832337003843044, where the truth is 2π + 4.83913536895222e-5,
  1.849e-9 m off.
- **Unit torus, everyday pose**:
  `2.133241833704526 2.721386152157228 | -0.26974356112659936 0.18011020087962995 1.5720155407573382 | -0.822678302897906 -0.05270002445951232 -0.5660592878515361 | 2.209443825872272 | 0.5616114314743157 0.07927539338398445 -0.8235945677584237 | 0 0 0 | 0 0 1 | 1.0 0.25`.
  Its depth is 1.72e-11 m. The door's θ = 3.126831593869284e-5 is
  3.49e-10 m from the true 3.12681581559929e-5.

## The measurement, corrected

The mpmath oracle above read each input from its decimal string
(`mp.mpf("0.0191…")`), not from the `f64` it rounds to, so its truths
belong to inputs up to half an ulp away. At a shallow crossing that
moves the root as far as the defect does. Re-measured against the
exact oracle (`boolean::conic_oracle::exact`: double-double on the
inputs' binary values, which agrees with mpmath on those values to
the bit on the three poses below), main certifies **321** wrong
answers outside the band (and 1 inside), all `f64`, all at
`ε = 1e-12`, all root placements; the worst is 1.66e-9 m (166 `Kε`,
the pole family). The three poses' errors are 2.2e-10 and 3.8e-10 m
(near-parallel), 1.66e-9 m (ten-metre) and 2.0e-10 m (unit torus).
The defect stands as filed; the table's counts and worst errors above
are the decimal oracle's.

## Fixed (PR 4357)

The general arm hands the subdivision a `RootSlack` meter
(`bool_circle_torus_sub_root_slack`). Its reading is `F` itself, the
torus quartic in its factored form `((ρ − R)² + h² − r²)((ρ + R)² +
h² − r²)` at the root with a running bound on its rounding
(`geom_brep::conic_torus_implicit`), so no ceiling on `|F|` per metre
of residual enters (the residual-and-ceiling form of
`ellipse_torus` refused 183 more correct `f64` answers at `1e-12` on
this set). The true root lies within (reading + bound) over `F`'s
least slope near the root of the located one, the piece holding
exactly one true root (definite end signs, certified monotone); at the
top speed `ρ` that is an arc, and a root whose arc the band does not
read as zero refuses. The subdivision's derivative charges now cover
the dropped third and fourth harmonics (`16·dropped`, Bernstein at
degree four against the walk's degree two), which the meter's slope
reads; that alone refuses 6 answers on this set.

After, on the same 9,000 poses × 3 ε × 2 lanes: **0 wrong**, inside
or outside the band. What newly refuses (correct answers before):

| ε | `f64` | `Interval` |
|---|---|---|
| 1e-12 | 1,780 of 8,175 | 1,193 of 4,804 |
| 1e-9 | 4 of 5,698 | 1,134 of 5,130 |
| 1e-6 | 0 | 11 of 2,655 |

The `Interval` lane's cost is the meter reading `F` over the root's
whole enclosure, where the dependency problem widens it by `|∇F|·ρ`
rather than the slope along the carrier; filed as
`work/germ/root-slack-meter-reads-an-interval-root-at-its-whole-width.md`.

Rows: `circle_torus::tests::a_shallow_crossings_root_is_within_the_band_or_refused`
(the three poses below, pinned truths), the gated sweep
`circle_torus::shallow_sweep::no_certified_root_of_a_shallow_crossing_is_placed_past_the_band`
(three families, three ε, both lanes, the exact oracle) and
`the_exact_oracle_agrees_with_mpmath_on_the_pinned_poses`.

## The sibling, measured and fixed

`conic_quadric::conic_quadric_roots`'s ladder arm (a circle tilted to a
wall, an ellipse against a sphere or a wall) passed `None` too.
Measured with the exact oracle on 3,000 shallow poses (graze depth
`1e-11`–`1e-3` m) × 3 ε × 2 lanes, it certified **123** wrong answers outside the band
(35 circle × wall, 39 ellipse × sphere, 49 ellipse × wall), all `f64`,
all at `ε = 1e-12`, all root placements, the worst 7.4e-10 m (74
`Kε`); `1e-9` and `1e-6` were clean (worst 2.7e-11 m). It now meters each
root (`bool_conic_quadric_sub_root_slack`, the residual read by
`geom_brep::conic_quadric_residual`, ceiling `1` since `F` is the
residual): **0 wrong**. It newly refuses 747 of 2,831 correct `f64`
answers at `1e-12` (none at `1e-9` or `1e-6`), and on the `Interval`
lane 342 of 1,678 at `1e-12`, 432 of 1,708 at `1e-9` and 1 at `1e-6`.
Pinned by the gated sweep
`conic_quadric::shallow_sweep::no_certified_root_of_a_shallow_ladder_crossing_is_placed_past_the_band`. The open HONE row
`degree-2-subdivision-doors-carry-no-root-slack-meter` covered both
doors and closes with this fix.
