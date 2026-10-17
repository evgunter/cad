---
id: zero-arm-payloads-overclaim-and-print-no-margin
kind: issue
title: geom-brep: the Zero-arm payload Displays say 'no length' or 'not long enough' whatever the reading shows, and most print no margin
status: closed
pr: 4976
opened: 2026-10-13
priority: P3
cost: M
branch: encl/zero-arm-payloads
closed: 2026-10-13
---


(Filed by the ENCL implementer of `import-door-appends-a-defect-note-to-a-sub-eps-in-zero-span`, from its Answered section's "Filed separately" list.)

## What

Since that unit, a sized decision's ending says "no size" only where the reading shows none (`geom_core::MarginDiag`'s one classification, read by `MarginDiag::sized_recourse`). The payload Displays of the Zero arms still claim it on every Zero verdict, and most print no margin:

- `CertifyError::IntervalNotForward { verdict: Refused::Zero(_) }` (`crates/geom-brep/src/certify.rs`, `impl Display for CertifyError`): "the stored parameter interval spans no length at this tolerance". A straddling enclosure or a positive sub-floor span does not establish that. No margin is printed.
- `CertifyError::ArmCollapsed` (same impl): "at sample {sample} the edge is not long enough, for how its faces curve, to measure their angle". No margin.
- `CertifyError::SpanMeterCollapsed` (same impl): "… is not above zero at this tolerance …". No margin.
- `MeterError::NormalFloor` (`crates/geom-brep/src/offset_meters.rs`, `impl Display for MeterError`): "cannot be proved non-zero (a chart {m} m thin …)". It prints the margin; listed for the "no size" audit only.

## Repair shape

Each should print its margin, and any "no length" claim should come from the reading's class (NoSize), not from the arm.
