---
id: plane-nurbs-ssi-does-not-certify-a-curved-dome
kind: issue
title: ssi: plane × NURBS does not certify a 3×3 quadratic dome of curvature ≳ 1/m at ε 1e-9 or 1e-12; the stepper does not collapse, the refusals are a seed refined off the chart, the fit budget and limb 2
status: open
opened: 2026-10-02
priority: P1
cost: H
---


(SSI investigation lane `ssi-stp`, 2026-10-02, on origin/main
06dbddaa7. Asked to verify a designer's probe: "plane × NURBS fails on
a genuinely curved wall because the step rule collapses —
`ssi_step_progress` escalates at a 7–10 nm step at ε 1e-9 — perhaps
because `h_quad`/`h_cub`/`h_fit` read κ and ‖d₃‖ in mixed units".)

## What

**The probe's mechanism does not reproduce, but its conclusion does.**
In about 110 certified runs and about 110 uncertified traces of a curved dome,
at ε 1e-6, 1e-9 and 1e-12, no march refused `ssi_step_progress`, and
the smallest step the realized stepper took was 22 µm (on a 1 mm wall),
1.1 mm on the 1 m dome. It is not the 7–10 nm the probe reported. Plane × curved NURBS
still fails broadly. Of the four plane families below, only the
axis-aligned cut `z = 0.2` certifies at ε 1e-9 once the dome's
centre curvature reaches 1/m, and none certifies at 1e-12. The
refusals come from four separate mechanisms, listed under *Causes*.
The step rule's units are mixed, as the probe suspected. The
mixing makes the march take *more* samples than it needs, not
fewer, and that costs the fit budget.

**No existing test certifies plane × NURBS on a wall with κ ≳ 1/m.**
Every plane × NURBS fixture is the cubic × linear extrusion in
`crates/geom-brep/tests/m5_pr7_ssi.rs:1029`
(`review_m5_pr7b_ssi.rs` copies it). The wall that certifies,
`certifiable_wall` (`m5_pr7_ssi.rs:1073`), has a section curvature
of about 0.22/m. The inflected `nurbs_wall` (`:1060`) reaches about
2.6/m locally, and it is pinned as a refusal (`HullSup`). So the
domain that fails here has never been inside the suite.

## Repro

The probe's wall is `W(d)`: a clamped quadratic 3×3 net, weights 1,
control `(i/2, 0, j/2)` with the centre point moved to `y = −d`. Its
surface is `x = s, z = t, y = −4d·s(1−s)·t(1−t)`, and its section
curvature at the centre is `2d` per metre. The probe's 3/m is d = 1.5.
The domain is `SsiDomain { center: (0.5, −d/8, 0.5), half_extent: 2,
extent: 1, floor_scale: 1 }`. The probe's planes miss this wall
(`x + z = 0` touches only its corner, and `z = −0.3` misses it), so
they were moved onto it:

- **oblique** `x + z = 1`: normal `(1,0,1)/√2`, through the dip
  direction.
- **tilt** `x + y = 0.5 − d/8`: normal `(1,1,0)/√2`, across the dip.
  At d = 4 this plane contains the wall's `s = 0` edge, which is the
  edge-flush case of
  `ssi-a-plane-through-a-faces-vertex-is-a-point-contact-not-a-refusal`.
- **level** `y = −d/8`: an interior loop. Its x–z shape does not
  depend on d, because `y = −d·f(s,t)`. The loop has radius about
  0.35 m and 3-D curvature 1.4–4.7/m.
- **zcut** `z = 0.2`: an open arc.

The scratch suite was not committed. It drove
`ssi::plane_nurbs_ssi` and
`ssi::trace_plane_nurbs_uncertified(…, march_tol = ε, …)` with
`CAD_TOLERANCE_EPS` set to each row.

## Measurements: `plane_nurbs_ssi`

The κ column is the dome's centre section curvature, `2d`. The
abbreviations are: OK (certifies), `OL v` (`CertificateLimb { OnLocus,
v }`), `OL~` (OnLocus escalated in band), `HS v` (`CertificateLimb
{ HullSup, v }`), `HS~` (HullSup escalated), `FSB n` (`FitSampleBudget
{ samples: n, budget: 1200 }`), `TS` (`TubeStraddles`), and `TU n`
(`TraceUnresolved { samples: n }`).

| d (κ) | ε | oblique | tilt | level loop | zcut |
|---|---|---|---|---|---|
| 0 (flat) | all | OK | OK | n/a | OK |
| 0.05–0.25 (0.1–0.5) | 1e-6 | OK | OK | TS | OK |
| 0.5 (1) | 1e-6 | OK | TS | TS | OK |
| 1 (2) | 1e-6 | OK | OL 1.57e-3 | TS | OK |
| 2 (4) | 1e-6 | OK | OL 3.36e-3 | TS | OK |
| 4 (8) | 1e-6 | OK | TU 3 (edge-flush) | TS | OK |
| 0.05–0.25 | 1e-9 | OL~ (1.3–1.7e-9) | OK | FSB 1335 | OK |
| 0.5 | 1e-9 | HS 1.61e-8 | TS | FSB 1335 | OK |
| 1 | 1e-9 | HS 1.33e-8 | OL 1.57e-3 | FSB 1335 | OK |
| 2 | 1e-9 | HS 1.99e-8 | TU 1 | FSB 1335 | OK |
| 4 | 1e-9 | HS 1.61e-8 | TU 3 (edge-flush) | FSB 1335 | HS~ (1.54e-9) |
| 0.05 | 1e-12 | OL 3.4e-11 | OK | FSB 7505 | OK |
| 0.1–0.25 | 1e-12 | OL~ | OK | FSB 7505 | OK |
| 0.5 | 1e-12 | FSB 1325 | FSB 1665 | FSB 7505 | OK |
| 1 | 1e-12 | FSB 2165 | FSB 2333 | FSB 7505 | FSB 1372 |
| 2 | 1e-12 | FSB 3289 | TU 1 | FSB 7505 | FSB 2052 |
| 4 | 1e-12 | FSB 4383 | TU 3 (edge-flush) | FSB 7505 | FSB 2685 |

The uncertified door, seeded on the locus, marches and fits every
oblique, tilt and zcut row at ε 1e-6 and 1e-9. At 1e-12 it refuses
`FitSampleBudget` from d = 0.5 (zcut from d = 1). It refuses the level
loop by budget at 1e-9 and 1e-12. Seeds placed off the locus at
`(0.3, 0.3)` and `(0.2, 0.7)` refine and march normally. A seed at
`(0.5, 0.5)`, the dome's floor, refuses `SeedRefinementFailed`.
Nothing reaches `ssi_step_progress` anywhere.

### The step rule, instrumented

`march.rs` was instrumented locally to print each step's rungs (in
metres) beside the 3-D curvature of the carrier itself, which was
measured by central differences of `sys.point` along the approximant.

| case | speed (m/state) | κ_rule = κ/speed² | κ_3D true | ratio | h_fit (binds) | h_quad | h_cub |
|---|---|---|---|---|---|---|---|
| level loop, d = 1, ε 1e-9 | 0.707 | 2.0–6.6 | 1.4–4.7 | **1.414** at every step | 1.1–2.8 mm | 43–141 mm | 150–550 mm |
| zcut, wall scaled ×1e-3 | 1.0e-3 | 989–1276 | 869–1274 | 1.00–1.14 | 22–27 µm | **157–178 mm** (wall is 1 mm) | 610–690 mm |
| zcut, ×1 | 0.71 | 1.05–1.28 | 1.0–1.28 | 1.00–1.04 | 3.9–4.5 mm | 221–260 mm | 854–872 mm |
| zcut, ×100 | 1.0 | 0.0121–0.0128 | same | 1.00 | 123–128 mm | 15.6–16.5 m | 59–61 m |

## Causes

1. **A seed that Newton refines off the wall's chart (tilt d = 1, 2 —
   `OL 1.6e-3`, `OL 3.4e-3`, `TU 1`; every ε).** The ℝ⁴ seeder
   (`crates/geom-brep/src/ssi.rs:1906`) hands
   `(s, t) = (0.369, 0.00195)` (d = 1) and `(0.243, 0.00098)` (d = 2),
   one seed cell from the `t = 0` edge. `march` refines the seed at
   `crates/geom-brep/src/ssi/march.rs:547` and never asks whether the
   refined state is inside `ctx.domain`. The min-norm Newton moves `t`
   to −1.15e-3 and −1.87e-3, and that state is `states[0]` of both
   halves. `push_boundary` (`march.rs:987`) bisects from it as the
   "inside" end, so no bisection finds an in-domain state
   (`march.rs:1006`) and the backward half stays a single
   out-of-chart state. The fit then interpolates a wall point
   evaluated off its knot domain, and limb 1 reads millimetres. When
   the forward march also leaves at once, the trace has length 0 and
   the run refuses `TraceUnresolved { samples: 1 }`. The fault is a
   defect, not a limit. Which of the two refusals a run shows depends
   on ε.

2. **The fit budget (level loop at 1e-9 and 1e-12; everything curved at
   1e-12).** `h_fit` (`march.rs:671`) gives samples ∝
   `ε^{-1/4}·∫κ^{3/4}`. The 2.2 m loop needs 1335 samples at 1e-9 and
   7505 at 1e-12, against `SSI_MAX_FIT_SAMPLES = 1200`
   (`ssi.rs:193`). This is the cubic-interpolant design meeting a
   fixed budget, and it is the same family as
   `plane-nurbs-certificate-bound-does-not-refine-with-eps`.

3. **`h_fit` over-counts curvature by the wall pcurve's bending (the
   units finding).** `march.rs:664-670` converts with `κ3d = κ/speed²`,
   where `κ = ‖d₂‖` is the curvature of the **ℝ⁴ state curve**. That
   curve includes `d₂`'s wall-chart components `(u₂, v₂)''`, which is
   the pcurve bending in parameter units. Only chart A (the plane,
   `system.rs:324`'s carrier-primary speed) generates the 3-D curve.
   For the plane, `C'' = J_A·d₂_A`, so the honest quantity is
   `‖(J_A·d₂_A)⊥T‖/speed²`. The rule overstates it by
   `√(1 + ‖d₂_B‖²/‖d₂_A‖²)`, which is exactly √2 on the level loop,
   so the loop takes (√2)^{3/4} = 1.30× the samples it needs: 1335
   instead of about 1030 at 1e-9. With the honest curvature the loop
   would fit the 1200 budget at 1e-9, and at 1e-12 it would still
   need about 5790. On a straight pcurve (zcut) the over-count is
   0–14%.

   `h_quad` and `h_cub` (`march.rs:653`, `:658`) are worse in kind.
   They are state-space relative bounds, turned into metres only by
   `h·speed`, so they read `2ρ·speed/κ_state` where the 3-D heuristic
   wants `2ρ/κ_3D`. With the plane isometric and `d₁` unit in ℝ⁴,
   `speed ≤ 1`, so they are too permissive by up to `1/speed`. On the
   1 mm wall `h_quad` allows 160 mm. `h_fit` binds on every row
   measured, so this changes no outcome today. These rungs could only
   *shrink* the step where `‖d₂_B‖` is very large in parameter units:
   a slow or degenerate wall axis, or a σ_min near the C7 band. That
   is the one route left to a 7–10 nm step, and this dome does not
   take it. The module docs (`march.rs:34-37`) still describe the
   rungs as `√(2δ/κ)`, `∛(6δ/‖d₃‖)` in ε, and the code at
   `:645-662` is the relative heuristic.

4. **Limb 2 refuses the oblique cut at 1e-9 (`HS` 13–20ε, d ≥ 0.5).**
   This one is inferred, not traced. `SSI_STEP_DEVIATION`'s design
   target assumes `‖C⁗‖ ≈ κ³` (`march.rs:366-379`). Along the
   diagonal the section is `y = −4d·(s(1−s))²`, which gives
   `‖C⁗‖/κ³ ≈ 3/d²` at the floor: 12× at d = 0.5 and 3× at d = 1.
   The docs name this caveat. On this curve it bites at every
   curvature from 1/m upward.

Two refusals are **not explained here**:

- Limb 1 escalates in band (1.3–1.7ε) on the gently curved oblique
  (d 0.05–0.25), at 1e-9 and at 1e-12.
- Once cause (1) is fixed, limb 1 also escalates in band on the tilt at
  d = 3 and ε 1e-9 (`CertificateEscalated { OnLocus }` at 6.6ε). Cause
  (1) masked this one until then.

A third, `TubeStraddles`, is explained and fixed in
`plane-nurbs-tube-straddles-a-curved-dome-at-coarse-eps`. It refused
the level loop at 1e-6 at every d, and the tilt at d = 0.5, because
limb 3 read the wall's derivative off its one span cell whole.
- At ε 1e-6, the level loop and the tilt now certify at d = 0.5, 1,
  1.5, 2 and 3.
- At 1e-9, the tilt at d = 0.5 certifies.
- d = 0.05–0.25 and d = 4 were not re-measured.

## Fix shape

- **(1), E–M:** after the seed refines, decide that it lies inside the
  march domain. If it does not, clip it back along the locus from the
  unrefined seed, the same way `push_boundary` clips an end. Failing
  that, treat the seed as no branch, as `SeedRefinementFailed` is
  treated (`ssi.rs:1932`). Pin the tilt d = 1 and d = 2 rows above.
- **(3), M:** state every step rung in the carrier's metres. Add a
  `LocalSystem` method that returns the carrier's 3-D acceleration,
  `J_A·d₂_A + D²P_A(d₁_A, d₁_A)` (the second term is zero on the
  plane), and the matching third derivative. Then use
  `κ_3D = ‖C''⊥T‖/speed²` in `h_fit`, and `2ρ/κ_3D` and `√(6ρ/‖C'''⊥‖)`
  in metres for the relative rungs. Sample counts move on every ℝ⁴
  fixture, so the fixtures are re-baselined. The ℝ³ implicit pair
  already marches in metres and does not change.
- **(2) and (4), H, design:** the budget and the κ³ proxy are limits
  of the cubic-interpolant certificate. Either the budget grows, or
  the fit's order or its between-sample estimate (a measured ‖C⁗‖
  from `d₃` differences) changes. Weigh them together with
  `plane-nurbs-certificate-bound-does-not-refine-with-eps`.
- **Coverage:** add a curved-dome fixture (κ ≥ 1/m, an interior loop
  and an open arc) to `m5_pr7_ssi.rs` once the arm can carry it.
  Until then, add it as a pinned refusal, so the domain is visible
  rather than absent.

Causes (1) and (3) are separable, and each could be its own row if the
orchestrator prefers. (1) is the cheapest and the only one that hands
a wrong carrier to the certificate.

**Cause (1) is split out** to `ssi-a-seed-refined-off-the-chart-is-marched`,
which carries its fix. With that fix in place, the tilt cut refuses as
follows:
- At ε 1e-6 it certifies at d = 0.5–3, as does the level loop, since
  `plane-nurbs-tube-straddles-a-curved-dome-at-coarse-eps`.
- At ε 1e-9, limb 2 (`HullSup`) at d = 1–2. Assigning that to cause 4
  is an inference, as it is for the oblique: no one has traced it. At
  d = 3 the cut instead escalates limb 1 at 6.6ε, which is unexplained
  (listed above).
- At ε 1e-12, `FitSampleBudget` (cause 2) at d = 1–3.

`a_seed_settled_off_the_walls_chart_is_no_branch` in
`crates/geom-brep/tests/m5_pr7_ssi.rs` pins the outcomes for d = 1 and
d = 2.

**Cause 3 is split out** to `ssi-step-rungs-mix-state-and-carrier-units`,
which carries its fix: every step rung reads the carrier in metres. With
it in place, the table above moves as follows (d = 0.05–3; d = 3–4 on
the oblique and zcut, and d = 4 on the tilt, refuse
`WindowShortOfWall` at this domain):
- The level loop takes 1030 samples at ε 1e-9 and certifies at every d;
  at 1e-12 it refuses `FitSampleBudget` at 5787 (was 7505). At 1e-6 it
  takes 184 (was 238).
- Every other curved cut takes fewer samples, by up to 1.2×; at 1e-12
  the tilt reads `FSB` 1413 / 1950 / 2616 / 3303 at d = 0.5 / 1 / 2 / 3
  (was 1664 / 2329 / 3186 / 4073), and the oblique and zcut 1–5% fewer.
- Two cuts that certified now escalate limb 2 in band: the tilt at
  d = 2 and ε 1e-6, and the zcut at d = 2 and ε 1e-9. The old rung's
  over-count was slack that hid cause 4 there; this is further evidence
  for cause 4, not a cause of its own.
