---
id: plane-nurbs-not-transverse-says-coincident-on-a-zero-arm
kind: issue
title: geom-brep: PlaneNurbsRefusal::NotTransverse says the tangent planes coincide on a Zero verdict, and prints no margin
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from its sweep for Zero/collapse payload Displays.)

Since that unit, geom-brep's Zero-arm payloads print their margin and claim no size: whether a reading shows none is the ending's to say, from `geom_core::MarginDiag`'s one classification (D4 ¶1 (i): the margin rides the payload as data). The Displays below are on this program's ground and still read the arm instead.

## What

- `crates/geom-brep/src/edge_nurbs.rs`, `impl Display for PlaneNurbsRefusal`, `Self::NotTransverse`: "the plane and the NURBS wall have coincident tangent planes at interior sample {sample}, …". The verdict is a Zero (or Negative) classification of the levered angle, not a proof the planes coincide, and the margin is not printed. The analytic lane's `CertifyError::NotTransverse` now reads "the faces meet tangentially at sample {sample}, within the tolerance (margin … m), …".
- Same file, `impl Display for AnalyticRung3Refusal`, `Self::TubeStraddles`: "— it straddles zero, from a near-tangency …". It prints the margin, but asserts a straddle whatever the reading is; a positive sub-band point margin is not a straddle.

## Repair shape

Say "within the tolerance" (or "not shown") of the reading and print `verdict.margin()`; describe the reading the margin shows rather than the arm.
