---
id: a-derivative-nets-difference-step-is-spelled-five-ways
kind: issue
title: A derivative net's per-pair difference step (q₁ − q₀)·p/Δu is spelled five ways; give it one home in geom-core spline::derivative_pair
status: open
opened: 2026-11-14
priority: P2
cost: M
---


Filed by ONEHOME's sweep-walls-at-t lane (PR 6952), on its FULL review's ask. That PR gave the per-pair step one home, `geom_core::spline::derivative_pair` (`crates/geom-core/src/spline/derivative.rs`), and made `derivative_net` and `NurbsCurve2`/`NurbsCurve3::derivative_at` (`crates/geom/src/curves/nurbs.rs`) read it. The scale alone already has one home (`span_derivative_scale`, closed row `the-span-derivative-scale-is-spelled-in-six-places`, PR 4866); the step around it does not.

**The other spellings on main**, none touched by PR 6952:

| Site | Form |
|---|---|
| `crates/geom-brep/src/props/quad/net.rs:302` `raw_deriv` | `Interval` coefficients on loose knots, a degenerate span skipped |
| `crates/geom-core/src/spline/hull.rs:796` `derivative_coeffs` (through `deriv_coeff`) | `Interval` only, its own association |
| `crates/geom-brep/src/curve_face/composite.rs:501` `derivative` | `Interval` row, its own loop |
| `crates/geom-core/src/spline/basis.rs:188` `ders_rows` | the derivative basis triangle, the same differences taken on the basis side |

**Owed.** Each site reads `derivative_pair` (or `derivative_net`), or says at the site why it cannot (a skipped degenerate span, an association a certificate pins). Moving an association moves bits: the census of pins each one shifts is the first measurement.
