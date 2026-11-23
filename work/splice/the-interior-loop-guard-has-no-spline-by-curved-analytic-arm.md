---
id: the-interior-loop-guard-has-no-spline-by-curved-analytic-arm
kind: issue
title: The interior-loop guard has no arm for a spline face against a cylinder, cone, sphere or torus: the pair falls to R-reach
status: open
opened: 2026-10-31
priority: P2
cost: H
---


## What

`topo/src/boolean/section_cert.rs` `classify_known` now reads a spline face
against a plane (`spline_plane`) and against another spline face
(`spline_pair`). A spline face against a cylinder, cone, sphere or torus
still falls to the catch-all `_ => Section::Intractable`, so the guard
refuses `CurvedPairUnsupported { site: InteriorLoopGuard }` (R-reach). In
`geom_brep::pair_section` those kind pairs route `SectionRead::Elsewhere`.
Lily's wall 8 (NURBS × torus) is one instance.

Found in the NURBS × NURBS arm's class sweep (`splice/nurbs-nurbs-guard-arm`):
the pattern swept was the classifier's kind dispatch reaching its
catch-all for a pair with a spline side. The three catch-alls in
`section_cert.rs` (`classify_known`, `torus_pair`, `cone_pair`) were each
read: only `classify_known`'s takes a spline face.

## What is open

A section reading for a spline wall against a quadric or a torus, which
C3 has no lane for (the march's ℝ⁴ state needs a chart on the analytic
side, or an implicit lane against the wall's chart), and its arm in the
guard.
