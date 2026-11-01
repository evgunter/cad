---
id: onpair-on-two-analytic-surfaces-refuses-a-spline-carrier
kind: issue
title: OnPair on two analytic surfaces refuses a spline carrier, though incidence_limbs over two analytic operands bounds one
status: open
opened: 2026-10-15
priority: P3
cost: M
---

Found by PR 5194's review (m4, and style S2).

## What happens

An `OnPair` between two ANALYTIC surfaces with a spline (`Curve3::Nurbs`) carrier refuses `CertifyError::Unimplemented` (`crates/geom-brep/src/certify.rs`, the `OnPair` arm of `run_checks`). The comment there argues that two analytic surfaces' per-sample residuals do not bound a spline carrier between samples. But `ssi::certify::incidence_limbs` over two `IncidenceOperand::Analytic` operands is exactly that bound: `analytic_limbs`' limb 2 is the implicit-composite hull, the one the `Intersection` + spline carrier path already runs through its `rung3_lane`.

The same refusal sits at two sites:
- check 1's spline-carrier gate exempts `OnPair` only so that the arm can refuse it after `plane_nurbs_pair`;
- the arm's own `Unimplemented`.

## What closing it needs

Route an analytic-pair `OnPair` with a spline carrier through a lane function that calls `incidence_limbs` with two analytic operands, as `plane_nurbs_incidence` does with one. That needs a `NurbsLane` pointer, because certify's `T: Real` cannot call the `CertifiedEnclosure`-bounded limbs. Then retire the second `Unimplemented` site.
