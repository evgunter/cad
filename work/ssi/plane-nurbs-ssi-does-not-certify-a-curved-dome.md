---
id: plane-nurbs-ssi-does-not-certify-a-curved-dome
kind: issue
title: ssi: plane × NURBS certifies a curved 3×3 quadratic dome at ε 1e-6 and 1e-9 but not at 1e-12, where the march asks the cubic fit for more samples than its budget; what changes is a design fork
status: closed
opened: 2026-10-02
priority: P2
cost: H
pr: 4028
branch: ssi/retire-fit-budget
closed: 2026-10-04
---

(Filed 2026-10-02 by SSI lane `ssi-stp` from a designer's probe; causes
1 and 3 split out and fixed (`ssi-a-seed-refined-off-the-chart-is-marched`,
`ssi-step-rungs-mix-state-and-carrier-units`), cause 4 answered by
refinement by certification (PR 3998). Re-measured on main at b3af5e268
by `ssi/dome-remeasure`, 2026-10-04; what follows is that measurement.)

## What

Plane × NURBS certifies every curved-dome cut at ε 1e-6 and 1e-9, at
every curvature measured. At ε 1e-12 it refuses `FitSampleBudget`
from a centre curvature of 1/m (the zcut from 2/m, the level loop at
every d): the march asks the cubic fit for 1.1–4.8× its budget of
`SSI_MAX_FIT_SAMPLES = 1200` (`crates/geom-brep/src/ssi.rs:207`,
checked in `fit_branch`). With the budget lifted, every one of those
cuts certifies, so the refusal is the budget and nothing else. What to
change is a design fork, below. No cut refuses
`RefinementExhausted`, `TubeStraddles`, `TraceUnresolved` or a
certificate limb at any ε.

## Repro

The wall is `W(d)`: a clamped quadratic 3×3 net, weights 1, control
points `(i/2, 0, j/2)` with the centre one moved to `y = −d`. Its
surface is `x = s, z = t, y = −4d·s(1−s)·t(1−t)`, and its centre
section curvature is `2d` per metre. The domain is `SsiDomain {
center: (0.5, −d/8, 0.5), half_extent: 2, extent: 1, floor_scale: 1 }`.
The four planes:

- **oblique** `x + z = 1`, along the diagonal. Its section
  `y = −4d·(s(1−s))²` has curvature zero at `s = (3 ∓ √3)/6 ≈ 0.211,
  0.789`.
- **tilt** `x + y = 0.5 − d/8`, across the dip. Its curvature is zero
  at the `t = 0` end at d = 1 and at a parameter fraction of 0.134 at
  d = 3. At d = 4 it holds the wall's `s = 0` side.
- **level** `y = −d/8`: an interior loop, the same x–z shape at every
  d, of 3-D curvature 1.4–4.7/m.
- **zcut** `z = 0.2`: an open arc.

`m5_pr7_ssi.rs` builds the wall as `dome_wall` and the tilt as
`dome_tilt`. The scratch driver (`crates/geom-brep/examples/dome_scratch.rs`)
and the env-gated traces it reads (rounds, refused spans, the widest
sample gaps, the time per certify) are on branch
`analysis/ssi-dome/remeasure` (7b615edc5d, 72feee3d9f); release
profile.

## Measurements: `plane_nurbs_ssi`

`OK n (m+kr)` is certified on `n` samples, the march's `m` refined
over `k` rounds (no parenthesis: the march's samples certified as they
stood). `FSB n` is `FitSampleBudget { samples: n }`. `WSW` is
`WindowShortOfWall`, and the bracket after it is the same cut with a
window of half-extent 4 m.

| d (κ) | ε | oblique | tilt | level loop | zcut |
|---|---|---|---|---|---|
| 0 | all | OK 4 | OK 4 | `CellBudget` (the wall lies in the plane) | OK 4 |
| 0.05 (0.1) | 1e-6 | OK 21 (9+2r) | OK 10 | OK 184 | OK 6 |
| 0.25 (0.5) | 1e-6 | OK 39 (20+3r) | OK 30 | OK 184 | OK 17 |
| 0.5 (1) | 1e-6 | OK 49 (38+2r) | OK 46 | OK 184 | OK 28 |
| 1 (2) | 1e-6 | OK 81 (61+3r) | OK 68 (60+2r) | OK 184 | OK 43 |
| 2 (4) | 1e-6 | OK 120 (92+3r) | OK 99 (81+3r) | OK 184 | OK 62 |
| 3 (6) | 1e-6 | WSW [OK 136 (115+2r)] | OK 128 (102+3r) | OK 184 | WSW [OK 77 (72+1r)] |
| 4 (8) | 1e-6 | WSW [OK 147 (126+2r)] | WSW [`CellBudget`] | OK 184 | WSW [OK 83 (78+1r)] |
| 0.05 | 1e-9 | OK 114 (34+4r) | OK 50 | OK 1030 | OK 28 |
| 0.25 | 1e-9 | OK 177 (131+4r) | OK 161 | OK 1030 | OK 91 |
| 0.5 | 1e-9 | OK 270 (235+3r) | OK 252 | OK 1030 | OK 151 |
| 1 | 1e-9 | OK 414 (372+4r) | OK 352 (326+5r) | OK 1030 | OK 240 |
| 2 | 1e-9 | OK 589 (556+3r) | OK 500 (465+3r) | OK 1030 | OK 349 (346+1r) |
| 3 | 1e-9 | WSW [OK 699 (655+2r)] | OK 654 (574+5r) | OK 1030 | WSW [OK 408 (402+1r)] |
| 4 | 1e-9 | WSW [OK 783 (710+3r)] | WSW [`CellBudget`] | OK 1030 | WSW [OK 440 (434+1r)] |
| 0.05 | 1e-12 | OK 549 (238+6r) | OK 278 | FSB 5787 | OK 154 |
| 0.25 | 1e-12 | OK 932 (796+4r) | OK 902 | FSB 5787 | OK 509 |
| 0.5 | 1e-12 | FSB 1321 | FSB 1413 | FSB 5787 | OK 843 |
| 1 | 1e-12 | FSB 2127 | FSB 1816 | FSB 5787 | FSB 1343 |
| 2 | 1e-12 | FSB 3123 | FSB 2616 | FSB 5787 | FSB 1943 |
| 3 | 1e-12 | WSW [FSB 3675] | FSB 3266 | FSB 5787 | WSW [FSB 2256] |
| 4 | 1e-12 | WSW [FSB 3994] | WSW [`CellBudget`] | FSB 5787 | WSW [FSB 2435] |

The three refusals that are not the budget each have a row of their own:

- `WindowShortOfWall` reads the wall's control hull, which reaches
  3.5–7× past the surface here
  (`ssi-window-check-reads-the-walls-control-hull`, P3).
- `CellBudget` meets a wall wholly in the plane (level, d = 0), and a
  locus that crosses itself on a side lying in the plane (tilt, d = 4)
  (`ssi-a-coincident-wall-or-side-crossing-refuses-the-cell-budget`,
  P3).

## Causes

**The refusals the table listed as unexplained are cause 4 at an
inflection, and refinement now answers them.** These were limb 1 in
band on the gently curved oblique (d 0.05–0.25, 1.3–1.7ε) and limb 1
at 6.6ε on the tilt at d = 3. The fit rung (`h_fit`, at `kappa3d` in
`crates/geom-brep/src/ssi/march.rs:1043`) prices a step's
between-sample error by `‖C⁗‖ ≈ κ³` (`SSI_STEP_DEVIATION`'s docs,
`march.rs:387-409`). Where the carrier's curvature is zero, the rung
reads no limit, and neither does `h_quad`; near the zero both read a
long one. So one step spans the zero:
- at the cap (`StepCap::Crossing`, `march.rs:457`), a fifth of the
  distance between the branch's crossings, on the tilt at d = 1 and 3
  (0.20 m) and on the oblique at 1e-6 and at d ≤ 0.25 (0.21–0.28 m);
- elsewhere at two to five times its neighbours (5.5 cm beside 2.2 cm
  on the oblique at d = 0.5 and 1e-9).

`h_cub` does not hold it, because `‖C‴⊥‖` is small there as well.
Along the oblique it allows about `√(0.06/d)` m.

`‖C⁗‖` is not zero there. Along the oblique it is a constant 24d in
arc length, so `‖C⁗‖/κ³ = 3/(8d²|1−6s+6s²|³)`: 3/d² at the centre, and
unbounded at the inflections. The widest gap of every refused
oblique carrier starts at a parameter fraction of 0.204–0.225 or
0.78–0.80. The exception is d = 0.05 at 1e-6, where the march takes
only 9 samples. On the tilt the widest gap starts at the zeros of its
curvature: 0.000 at d = 1, and 0.13 at d = 3. Limbs 1 and 2 refuse there, and which of them reports
is only which one runs first.

`refine::refine_by_certificate` (`crates/geom-brep/src/ssi/refine.rs:71`)
halves that gap each round. A 0.2 m gap takes 4–8 rounds to come down
to its neighbours' spacing, which is where the rounds in the table go:
the tilt at d = 1 and 1e-12 takes 8. Disposition: no defect. It is a
limit of the step rule's between-sample estimate, which is a design
target and not a bound, and the certificate decides. It is the third
option of the fork below.

**Cause 2: the fit budget.** The march's samples go as `ε^{-1/4}`,
×1.778 a decade, on every cut (the march's count, read with the
budget at 1):

| cut, d | 1e-6 | 1e-7 | 1e-8 | 1e-9 | 1e-10 | 1e-11 | 1e-12 | 1e-13 |
|---|---|---|---|---|---|---|---|---|
| level, any d | 184 | 326 | 579 | 1030 | 1831 | 3255 | 5787 | 10291 |
| oblique 0.5 | 38 | 70 | 133 | 235 | 415 | 596 | 1321 | 2346 |
| oblique 1 | 61 | 115 | 193 | 372 | 673 | 1197 | 2127 | 3782 |
| oblique 2 | 92 | 163 | 301 | 556 | 972 | 1735 | 3123 | 5570 |
| tilt 1 | 60 | 105 | 184 | 326 | 576 | 1023 | 1816 | 3226 |
| tilt 3 | 102 | 181 | 329 | 574 | 1041 | 1857 | 3266 | 5866 |
| zcut 1 | 43 | 76 | 135 | 240 | 425 | 756 | 1343 | 2388 |
| zcut 3 | 72 | 128 | 227 | 402 | 714 | 1269 | 2256 | 4011 |

With the budget lifted (`CAD_SCRATCH_FIT_BUDGET`, on the analysis
branch), every cut at 1e-12 certifies:

| cut | d = 0.5 | 1 | 2 | 3 | 4 |
|---|---|---|---|---|---|
| oblique | 1432 (1321+4r) | 2222 (2127+3r) | 3268 (3123+5r) | 3927 (3675+5r) | 4371 (3994+4r) |
| tilt | 1413 | 1965 (1816+8r) | 2740 (2616+4r) | 3552 (3266+8r) | — |
| zcut | 843 | 1343 | 1943 | 2262 (2256+1r) | 2441 (2435+1r) |
| level | 5787 at every d (0 rounds) | | | | |

So at 1e-12 a dome of centre curvature up to 8/m needs up to 5787
samples per branch (the loop) and up to 4371 per open arc. From
κ = 1/m up, refinement adds 0–10% to the march's count. On the gently
curved oblique it adds more (238 → 549 at d = 0.05), because the step
across an inflection is longest there.

**What it costs.** Most of the time is the fit, the dense
O(n³) collocation solve that runs three times per round
(`work/flux/the-interpolating-fit-solves-a-banded-collocation-system-densely.md`).
Release profile, on a 4-vCPU box with 3–4 of these runs at once, so
the absolute times are pessimistic and the ratios are what to read:

| n | fit (s) | certificate (s) | per round (s) |
|---|---|---|---|
| 1030 | 1.3 | 0.45 | 1.8 |
| 1343 | 2.9 | 0.47 | 3.3 |
| 1943 | 12.3 | 1.0 | 13.3 |
| 2435 | — | — | 27 |
| 3123 | — | — | 78 |
| 3266 | — | — | 95–119 |
| 5787 | — | — | 1311 |

A round goes as n³, about 2.5 s at n = 1000 times `(n/1000)³`. With
`n ∝ ε^{-1/4}`, a branch's cost goes as `ε^{-3/4}` per round, and the
rounds multiply it. The loop at 1e-12 spends 22 minutes on one fit.
The certificate goes about as n. A banded solve would make the fit
O(n) too, which changes what any budget costs, though not what it
allows.

## The fork

The budget, the fit's order and the between-sample estimate are limits
of the cubic-interpolant certificate. They are weighed together with
`plane-nurbs-certificate-bound-does-not-refine-with-eps`, whose
limb-2 term is the same design question on the fixed-schedule
pcurve lane. The options the row names, and others seen while
measuring, unranked:

1. **The budget grows by a rule**: for example with ε, as `ε^{-1/4}`
   times a curvature integral, or set by what a fit may cost.
2. **The fit's order changes**: a quintic interpolant's
   between-sample error goes as `h⁶`, so its samples would go as
   `ε^{-1/6}`. From 1e-6 to 1e-12 the count would grow 10×, against
   the cubic's 31.6×.
3. **The between-sample estimate changes**: a measured `‖C⁗‖` (from
   differences of `d₃`), or a step accepted only when its far end
   agrees with its near end. Either one would also stop the march from
   stepping across an inflection at the cap.
4. **Approximation instead of interpolation.** `fit_branch` keeps one
   control point per sample. A least-squares fit with fewer control
   points on the same shared parameters (`approximate_with_params`,
   named in `fit_branch`'s comment) separates the samples the march
   takes from the size of the carrier and of the solve.
5. **A branch carried by several carriers**, each within the budget,
   joined at shared samples. The budget then bounds a piece, not a
   branch.
6. **The carrier composed exactly as `S(P(t))`** from the wall and a
   fitted pcurve, so that it lies on the wall by construction and only
   the plane residual of `P` is left to bound. That changes what limb 2
   reads and the carrier's degree.

Independent of the fork, the banded solve on flux's slate changes
every option's cost.

## Coverage

`crates/geom-brep/tests/m5_pr7_ssi.rs` pins the dome:

- `a_curved_domes_level_loop_certifies_or_refuses_the_fit_budget`:
  the interior loop at d = 1 and 3, at the run's ε. It certifies at
  1e-6 and 1e-9 (about 184 and 1030 samples) and refuses
  `FitSampleBudget` at 1e-12 (5787).
- `a_curved_domes_oblique_arc_is_refined_across_its_inflections`: the
  open arc at d = 0.5 (κ = 1/m), at the run's ε. It certifies at 1e-6
  and 1e-9 (about 49 and 270 samples) and refuses `FitSampleBudget` at
  1e-12 (1321).
- `a_curved_domes_open_arc_is_refined_where_the_hull_limb_refused`
  (the zcut, 1e-9), `a_seed_settled_off_the_walls_chart_is_no_branch`
  (the tilt, every ε), `a_curved_domes_cuts_prove_their_tube_at_the_widest_rung`
  and `a_loop_whose_seeds_newton_carries_into_its_tube_is_found_once`.

## Priority

What remains is the fork alone. Every refusal on the dome at 1e-6 and
1e-9 is answered, and at 1e-12 the only one left is the budget. Every
other refusal the dome shows is filed on its own row, and none of them
is a wrong carrier. Proposed: **P2**, `design: true`. The open question
is how the certificate's sample count scales with ε, which is
interval and error propagation, and no architecture is being built on
the present budget.

## Closed (2026-10-04, PR 4028)

The fork converged (`work/ssi/log.md`, 2026-10-04): the fit's
collocation solve is banded (PR 4023), `SSI_MAX_FIT_SAMPLES` and
`FitSampleBudget` are retired, and one resource wall bounds a branch's
samples, marched or inserted (`SSI_MAX_STEPS`, 20 000 steps). Every
curved-dome cut certifies at ε 1e-6, 1e-9 and 1e-12; the refusals left
are other rows' (`WindowShortOfWall`, `CellBudget`). Re-measured on
`ssi/retire-fit-budget`, release, on a 4-vCPU box shared with other
builds (times are pessimistic). `OK n (m+kr)`: certified on `n`
samples, the march's `m` refined over `k` rounds; the bracket after
`WSW` is the same cut in a window of half-extent 4 m.

**ε 1e-12**

| d (κ) | oblique | tilt | level loop | zcut |
|---|---|---|---|---|
| 0.05 (0.1) | OK 549 (238+6r), 1.0 s | OK 278, 0.2 s | OK 5787, 8.0 s | OK 154, 0.2 s |
| 0.25 (0.5) | OK 932 (796+4r), 1.5 s | OK 902, 0.8 s | OK 5787, 5.9 s | OK 509, 0.3 s |
| 0.5 (1) | OK 1432 (1321+4r), 3.5 s | OK 1413, 1.2 s | OK 5787, 7.4 s | OK 843, 0.6 s |
| 1 (2) | OK 2222 (2127+3r), 6.0 s | OK 1965 (1816+8r), 8.4 s | OK 5787, 6.2 s | OK 1343, 1.0 s |
| 2 (4) | OK 3268 (3123+5r), 15.2 s | OK 2740 (2616+4r), 9.7 s | OK 5787, 6.9 s | OK 1943, 1.9 s |
| 3 (6) | WSW [OK 3927 (3675+5r), 11.8 s] | OK 3552 (3266+8r), 21.1 s | OK 5787, 7.6 s | WSW [OK 2262 (2256+1r), 1.8 s] |
| 4 (8) | WSW [OK 4371 (3994+4r), 10.3 s] | WSW [`CellBudget`] | OK 5787, 8.2 s | WSW [OK 2441 (2435+1r), 2.4 s] |

**ε 1e-13**

| d (κ) | oblique | tilt | level loop | zcut |
|---|---|---|---|---|
| 0.05 (0.1) | OK 950 (426+4r), 1.5 s | OK 493, 0.3 s | OK 10291, 17.1 s | OK 273, 0.2 s |
| 0.25 (0.5) | OK 1665 (1393+6r), 5.4 s | OK 1603, 1.3 s | OK 10291, 13.7 s | OK 905, 0.5 s |
| 0.5 (1) | OK 2558 (2346+5r), 9.3 s | OK 2511, 2.5 s | OK 10291, 14.6 s | OK 1499, 0.8 s |
| 1 (2) | OK 3948 (3782+3r), 12.3 s | OK 3609 (3226+36r), 36.9 s | OK 10291, 13.8 s | OK 2388, 1.8 s |
| 2 (4) | OK 5792 (5570+4r), 29.3 s | OK 4832 (4653+4r), 19.3 s | OK 10291, 14.2 s | OK 3454, 2.8 s |
| 3 (6) | WSW [OK 6980 (6539+5r), 30.7 s] | OK 6310 (5866+6r), 33.2 s | OK 10291, 14.3 s | WSW [OK 4017 (4011+1r), 4.1 s] |
| 4 (8) | WSW [OK 7804 (7085+9r), 57.0 s] | WSW [`CellBudget`] | OK 10291, 15.7 s | WSW [OK 4335 (4329+1r), 4.7 s] |

At ε 1e-14 (d = 1) every cut meets the wall, typed
`RefinementExhausted { stop: StepBudget }`: the level loop marches
18 299 samples and the first round would take 23 395; the zcut's limb-2
margin stays at 1.6–1.8e-14 m in band while its samples grow 4 245 →
14 965 over 5 rounds; the oblique and tilt take 32 and 37 rounds. Each
run's peak RSS is 24–30 MB, and the longest (the oblique) takes 233 s.
Pinned by `a_loop_past_the_step_budget_refuses_typed_at_the_wall` and
`refinement_past_the_arithmetics_floor_meets_the_wall_typed`; the two
dome rows now pin certification at 1e-12
(`a_curved_domes_level_loop_certifies_at_every_eps`,
`a_curved_domes_oblique_arc_certifies_refined_across_its_inflections`).

