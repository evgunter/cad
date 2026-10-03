---
id: the-cylinder-chart-ellipse-pcurve-samples-a-fixed-schedule
kind: issue
title: pcurve: the cylinder-chart ellipse pcurve is fitted through a fixed 129-sample schedule, a density nothing asks the certificate about; whether it meets small eps on metre-scale cuts is unmeasured
status: open
opened: 2026-10-03
priority: P3
cost: M
refs: [ssi-step-max-is-a-sampling-heuristic, plane-nurbs-certificate-bound-does-not-refine-with-eps]
---

(SSI implementer on `ssi/step-max-certify`, from the class sweep of
retiring `SSI_STEP_MAX`, 2026-10-03.)

## What

`ellipse_pcurve_on_cylinder` (`crates/geom-brep/src/pcurve.rs`) samples
the tilted-section ellipse's graph `(u, v(u))` on the fixed
`PCURVE_FIT_SAMPLES = 129` schedule and fits it with
`NurbsCurve2::approximate` at the caller's tolerance. Nothing between
the samples is asked of the certificate when the density is chosen: it
is the shape `SSI_STEP_MAX` had, a constant standing in for what the
certificate downstream needs.

## Not measured

On a metre-scale cut the graph is `v ≈ m·r·cos(u − φ) + c`, and a cubic
through samples `h ≤ 2π/128` apart misses it by about
`(5/384)·h⁴·m·r ≈ 7.5e-8·m·r` m between them. That is above ε = 1e-9 at
`m·r ≈ 1 m`, so either this pcurve refuses at small ε on such cuts, or
something downstream absorbs it. Which one holds is not measured.

## What would settle it

Certify the pcurve this door returns (the PR 6 cache door) for a tilted
plane × unit cylinder at slopes 0.1–1, at ε 1e-6, 1e-9 and 1e-12. If it
refuses where a denser schedule certifies, the door should refine by the
certificate, as the SSI march now does
(`march::refine_by_certificate`), or double its schedule until the
bound is met, as `spiric_export_spline` does.

