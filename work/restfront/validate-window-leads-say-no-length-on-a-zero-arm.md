---
id: validate-window-leads-say-no-length-on-a-zero-arm
kind: issue
title: topo: the at-rest window's leads and NoDihedralArm say 'not long enough' or 'no measurable length' on a Zero verdict, and print no margin
status: open
opened: 2026-10-13
priority: P3
cost: E
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from its sweep for Zero/collapse payload Displays.)

Since that unit, geom-brep's Zero-arm payloads print their margin and claim no size: whether a reading shows none is the ending's to say, from `geom_core::MarginDiag`'s one classification (D4 ¶1 (i): the margin rides the payload as data). The Displays below are on this program's ground and still read the arm instead.

## What

- `crates/topo/src/validate.rs` `classify_certify`, the at-rest window's lead for a certify refusal (its `let why = match e` over `CertifyError`). These leads state unhedged claims on every Zero verdict, a straddling enclosure or a positive sub-floor size included, which do not establish them:
  - `ArmCollapsed`: "it is not long enough, for how its faces curve, to measure their angle";
  - `SpanMeterCollapsed`: "its spline's certified speed floor gives it no measurable length";
  - `NotTransverse` (and `PlaneNurbs(NotTransverse)`): "its faces are tangent where its description says they cross";
  - `NotSecondOrderSeparated`: "its faces agree to second order, so they do not fix where it runs, which its description says they do".

  `IntervalNotForward { Zero }`'s "its length is within the tolerance of zero" is true of a band-decided zero, though not literally of a symbolic-tier proved zero whose rounded number lies past the zero band.
- Same file, `impl Display for ValidationError`, `Self::NoDihedralArm`: "an edge is not {DIHEDRAL_ARM_CLAUSE}", with no margin. The certify payload it mirrors now reads "the edge is not shown {DIHEDRAL_ARM_CLAUSE} (margin … m)" (`CertifyError::ArmCollapsed`).
- Same file, the offset-meter lead (`M::NormalFloor { verdict: R::Negative { .. } }`): "the face's surface normal degenerates, so it has no offset". The thinness is never negative (`MeterError::NormalFloor`'s field doc), so this arm is unreachable or says more than its verdict. The meter's own payload now reads "the face's normal is not shown non-zero (margin … m: …)" and leaves the offset's refusal to the fit's lead.

## Repair shape

Say "not shown …" of the reading and print `(margin … m)`, as `CertifyError`'s and `MeterError`'s Displays do; leave any no-size claim to the ending. Pins: the `validate.rs` tests asserting `text.starts_with("an edge is not long enough")`, and the row at "an edge is not long enough, for how its faces curve".
