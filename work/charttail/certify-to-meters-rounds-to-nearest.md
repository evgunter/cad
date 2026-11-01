---
id: certify-to-meters-rounds-to-nearest
kind: issue
title: ssi/certify.rs's analytic_limbs rounds sup_bound() * to_meters to nearest inside a certificate, so the upper bound can come out an ulp low
status: open
priority: P2
cost: E
opened: 2026-09-29
---

Found by a designer lane weighing the torus meters question for CURVED
(2026-09-30), by reading; confirmed by the second designer.

`crates/geom-brep/src/ssi/certify.rs`, `analytic_limbs`: the limb-2
bound is formed as `T::from_f64(composite.sup_bound() * to_meters)` with
`to_meters = 1.0 / (2.0 * radius)`. Both the reciprocal and the product
round to nearest, not upward, so the stated upper bound can be one ulp
LOW, and the comment's "converts to meters exactly" is false. A
certificate's bound must round outward. Fix: form the product in
`Interval` and read `.hi()`, or `next_up` both steps; a row with a
radius whose reciprocal is inexact and a sup at a rounding boundary.
