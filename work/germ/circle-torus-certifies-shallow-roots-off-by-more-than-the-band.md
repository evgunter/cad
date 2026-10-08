---
id: circle-torus-certifies-shallow-roots-off-by-more-than-the-band
kind: issue
title: The circle x torus door certifies shallow-crossing roots up to 185 K-eps along the carrier from the truth at eps 1e-12: its subdivision passes no root-slack meter
status: open
opened: 2026-10-08
priority: P0
refs: [the-half-angle-ladder-certifies-in-band-configurations, circle-torus-meters-accept-an-unreadable-reading]
---


Found by GERM's circle × torus measurement lane (2026-10-08, branch
`germ/circle-torus-outside-band`), which asked whether the door on main
certifies any answer that is wrong OUTSIDE the band. It does, at
`ε = 1e-12`, by root placement only. A later escape of DR-16 (PR 3375).

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
it. This door has passed `None` since the ladder's own `root_slack`
was removed in `0f74dfdb7e`.

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

## What would close it

Pass this door a `RootSlack` meter, as `ellipse_torus` does
(`ellipse_torus.rs:120`, the running-bound residual
`geom_brep::conic_torus_residual`). A shallow crossing whose root the
`f64` residual cannot place then refuses. Pin the three poses above
with a row at `Band::new(1e-12, 1e-11)`. The probe's oracle shape for
that row: mpmath roots on the exact inputs, with arc distance compared
against `Kε`.

## A sibling, unmeasured

`conic_quadric::conic_quadric_roots` passes `None` to the same
`half_angle_roots` too
(`crates/topo/src/boolean/conic_quadric/mod.rs:270`; it handles a
circle tilted to a cylinder wall and an ellipse against a sphere or a
wall). The bisection is the same, so the same placement error is
expected at shallow crossings at `ε = 1e-12`. This lane did not
measure it. The cone arm (`:375`) passes a meter.
