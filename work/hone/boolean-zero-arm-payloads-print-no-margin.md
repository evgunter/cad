---
id: boolean-zero-arm-payloads-print-no-margin
kind: issue
title: topo: CurvedSectorSideUnsupported's and SpheresMeet's Zero arms print no margin
status: open
opened: 2026-10-13
---

priority: P3
cost: S
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from its sweep for Zero/collapse payload Displays.)

Since that unit, geom-brep's Zero-arm payloads print their margin and claim no size: whether a reading shows none is the ending's to say, from `geom_core::MarginDiag`'s one classification (D4 ¶1 (i): the margin rides the payload as data). The Displays below are on this program's ground and still read the arm instead.

## What

`crates/topo/src/boolean/mod.rs`, the Boolean error's `Display`:
- `Self::CurvedSectorSideUnsupported { verdict }`, Zero arm: "a direction leaving the pierce point runs too close to tangent to the face, or along too short an edge, for this tolerance to tell". Its words hedge correctly, but it prints no margin.
- `Self::SpheresMeet { verdict, .. }`, Zero arm: "touch within the tolerance, the smaller inside the larger". No margin.

## Repair shape

Print `verdict.margin()` in each Zero arm, as geom-brep's `CertifyError` Zero arms do.
