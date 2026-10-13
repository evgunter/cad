---
id: validate-window-leads-say-no-length-on-a-zero-arm
kind: issue
title: topo: the at-rest window's leads and NoDihedralArm say 'not long enough' or 'no measurable length' on a Zero verdict, and print no margin
status: open
opened: 2026-10-13
---

priority: P3
cost: S
---

(Filed by the ENCL implementer of `zero-arm-payloads-overclaim-and-print-no-margin`, from its sweep for Zero/collapse payload Displays.)

Since that unit, geom-brep's Zero-arm payloads print their margin and claim no size: whether a reading shows none is the ending's to say, from `geom_core::MarginDiag`'s one classification (D4 ¶1 (i): the margin rides the payload as data). The Displays below are on this program's ground and still read the arm instead.

## What

- `crates/topo/src/validate.rs`, the at-rest window's lead for a certify refusal (the `let why = match e` over `CertifyError`, near `CertifyError::ArmCollapsed`): `ArmCollapsed` reads "it is not long enough, for how its faces curve, to measure their angle", and `SpanMeterCollapsed` "its spline's certified speed floor gives it no measurable length". Both are said on every Zero verdict, a straddling enclosure or a positive sub-floor size included, which do not establish them. `IntervalNotForward { Zero }`'s "its length is within the tolerance of zero" is accurate.
- Same file, `impl Display for ValidationError`, `Self::NoDihedralArm`: "an edge is not {DIHEDRAL_ARM_CLAUSE}", with no margin. The certify payload it mirrors now reads "the edge is not shown {DIHEDRAL_ARM_CLAUSE} (margin … m)" (`CertifyError::ArmCollapsed`).
- Same file, the offset-meter lead (`M::NormalFloor { verdict: R::Negative { .. } }`): "the face's surface normal degenerates, so it has no offset". The thinness is never negative (`MeterError::NormalFloor`'s field doc), so this arm is unreachable or says more than its verdict; the meter's own payload now reads "so no offset of it can be certified".

## Repair shape

Print the margin and say "not shown" of the reading, as `CertifyError`'s Displays do; leave any no-size claim to the ending. Pins: `validate.rs` tests asserting `text.starts_with("an edge is not long enough")` and the row at "an edge is not long enough, for how its faces curve".
