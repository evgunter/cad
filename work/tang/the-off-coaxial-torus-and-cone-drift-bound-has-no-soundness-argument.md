---
id: the-off-coaxial-torus-and-cone-drift-bound-has-no-soundness-argument
kind: issue
title: The off-coaxial torus and cone drift bound has no soundness argument
status: open
opened: 2026-11-04
priority: P3
cost: M
---


Filed by the band-meridian lane (2026-11-04), from a check made while measuring the torus-meridian certificate.

`crates/geom-brep/src/tangent.rs` (`circle_span_bounds`, the `Torus` and `Cone` arms) charges `kappa_drift` as `step · 8·dev/r²`. `dev` is how far the carrier's velocity departs from the surface's Killing field (`dev_rot`), and `r` is `r_eff = r·(R − r)/R` for the torus and the carrier's radial floor `g_lo` for the cone. The `8·motion/r²` accounting comes from the line arm (`tangent_span_bounds`). That arm's derivation assumes a **constant** Hessian: on a sphere or cylinder, `κ = d̂ᵀHd̂/|∇F|` moves only through the turning of `d̂` and the drift of `|∇F|`. The torus and cone Hessians vary with the point. So once a carrier is off coaxial (`dev > 0`), `dκ/dt` also carries a `∇³F[C′, d̂, d̂]/|∇F|` term, and the accounting does not charge it. A coaxial carrier is unaffected, because there `κ_rel` is constant by equivariance and `dev = 0`. What is missing is the argument for every other carrier: nothing shows that the factor 8 absorbs the third-derivative term.

No counterexample is known. The oracle `crates/geom-brep/tests/off_coaxial_drift_oracle.rs` (`off_coaxial_drift_stays_within_the_stated_bound`, ignored, run command in its module doc) walks every span of the 9-sample schedule at 64 points. It compares `|κ_rel(t) − κ_rel(tᵢ)|` against the stated `kappa_drift`, at `f64` and at `Interval`, over 1080 rows:
- tori with (R, r) = (1, 0.2), (0.3, 0.2), (0.21, 0.2) and (5, 0.1), each against its top plane, outer-equator cylinder and inner-equator cylinder;
- cones of half-angle 5°, 30° and 80°, each against its inscribed sphere;
- the carrier's centre offset, its axis tilted, or both, by δ ∈ {1e-3, 1e-4, 1e-7};
- the full circle and a sixteenth of it, in both surface orders.

The worst ratio of sampled to stated drift is **0.1168**: the R 5, r 0.1 torus against its outer cylinder, with a centre offset. The ratio does not change between δ = 1e-4 and 1e-7, so the sampled drift and the bound both scale linearly with the miss. No smaller-miss regime is in sight where the margin closes.

Owed: a derivation of `sup|dκ_rel/dt|` for the torus and cone arms off coaxial that includes the `∇³F` term. The bound then either stands with its constant justified or is re-derived. Once that argument exists, the oracle can come off `#[ignore]`.
