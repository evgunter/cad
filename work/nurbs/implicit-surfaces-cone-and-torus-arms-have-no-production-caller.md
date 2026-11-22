---
id: implicit-surfaces-cone-and-torus-arms-have-no-production-caller
kind: issue
title: geom_core's ImplicitSurface keeps its Cone and Torus arms with no production caller
status: open
opened: 2026-11-21
priority: P4
cost: C
---

Filed by `sectcone/one-limb-two-reader`, which moved the search's
analytic limb 2 onto the edge certificate's reader and deleted
`ssi::certify::composite_form`, the one production caller of
`compose::ImplicitSurface`'s `Torus` arm.

`geom_core::spline::compose::ImplicitSurface` keeps a production caller
in `crates/step-import/src/recognize_curve.rs` (`composite_sup`,
`composite_sup_within`), which builds `Plane`, `Sphere` and `Cylinder`
only. Its `Cone` arm (`tan_half_angle: f64`, a transcendental of the
stored angle taken in `f64`) and its `Torus` arm now have callers only
in `compose.rs`'s own tests and in `crates/geom/tests/curves/`
(`review_m5_pr4_adversarial.rs`). The canonical form
(`compose::CanonicalSurface`, enclosure-valued coefficients) is the one
every certificate reads. Retire the two arms, or say what they are
kept for.
