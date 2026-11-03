---
id: the-implicit-chart-march-does-not-read-its-pcurves-timing
kind: issue
title: The 1×2 march's step test does not bound the wall pcurve's timing against the carrier
status: open
opened: 2026-11-03
---


Found when stage 4 of the NURBS × implicit arm moved plane × NURBS onto
the 1×2 shape (`ImplicitChartR2`). On the inflected wall of
`crates/geom-brep/tests/review_m5_pr7b_ssi.rs` (row
`deviation2a_the_inflected_walls_fit_deviation_is_measured_without_the_ring`),
the uncertified door `trace_plane_nurbs_uncertified`
(`crates/geom-brep/src/ssi.rs:3621`), marching at 1e-9, now reads a fitted
pair deviation `‖S(pb(t)) − C(t)‖` of **2.41e-9 m at u = 0.533**,
unrefined. The retired ℝ⁴ trace of the same plane read **1.31e-10 m**, so
the 1×2 march is about 18× coarser there, and its pair deviation is above
the march tolerance itself.

**Why.** The deviation sits at the section's curvature zero, where the
carrier's fit rung `h_fit ∝ (ε/κ³)^¼` unbinds
(`crates/geom-brep/src/ssi/march.rs:1236`) and only the residual test
holds the step (`march.rs:1270`). On the ℝ⁴ lane that test reads
`S(u, v) − P(s, t)`, three components, so a predicted state whose wall
pcurve runs ahead of or behind its plane point along the track fails it.
On the 1×2 lane it reads the one scalar `f(S(u, v))`, the predicted
point's distance from the plane. A step whose pcurve timing drifts along
the track passes that test, and the pair's fits (carrier and pcurve, on
one parameter) then disagree between samples by the drift.

**What it costs today.**
- On a certifying door the march tolerance is the band's, and the
  certificate refines a pair it refuses (`crates/geom-brep/src/ssi/refine.rs:191`).
  The differential across the plane fixtures found no decided reading
  that this moves.
- On the decoupled door (march at 1e-9, band elsewhere):
  - at ε 1e-6 the pair is handed back unrefined at 2.41e-9;
  - at ε 1e-9 one round refines it to 2.13e-10;
  - at ε 1e-12 refinement runs 11 rounds to 18 719 samples, and then the
    20 000-step budget stops it, on limb 1 (margins 4e-12 to 9e-12 in the
    (1e-12, 1e-11) band). The door returns its last refused triple, and
    its scan still reads 2.41e-9: the halving never reached the span at
    the curvature zero. The ℝ⁴ trace refined this case to 2.27e-11.

**A candidate.** A rung that bounds the pcurve's fit in metres: the
state curve's `h = (24·δ·ε / (speed·κ_state³))^¼`, in state units. On
the ℝ³ lane this is `h_fit` itself, since the state is the carrier at
unit speed. On the ℝ⁴ pair lane it would add a binding rung. It changes
sample counts on every 1×2 row, so it is a march design change and not a
fix pass.
