---
id: certify-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: CertifyError::Escalated forwards Indeterminate's coincidence menu, unlabelled, where the edge's own faces leave nothing to declare
status: closed
opened: 2026-09-28
priority: P3
cost: E
pr: 3351
closed: 2026-09-28
---

(ENCL implementer, from the §5 sweep of
`offset-meter-escalation-renders-the-coincidence-menu-unlabelled`; one
of four siblings, filed on each owner's slate.)

## What

`geom_brep::CertifyError::Escalated`'s `Display`
(`crates/geom-brep/src/certify.rs`, both `Self::Escalated` arms)
renders its `Indeterminate` whole: "certification: {check} … escalated:
{cause}". `Indeterminate`'s own `Display` ends in
`geom_core::COINCIDENCE_RECOURSE` unlabelled ("declare the coincidence,
move the geometry, or lower the tolerance"), so:

- `test_utils::refusal::recourse_markers` counts zero on it;
- it offers a declaration, while `topo::validate`'s `classify_certify`
  routes the same arm through `own_close(&cause.margin, EDGE_CLOSE)` —
  an edge's own two faces, where "declare the coincidence" has no
  object — and a poisoned margin to the defect ending.

The arm also opens with the stage prefix `certification:`, which the
CHROME standard lists as developer detail.

## Repair shape

The one `MeterError::Escalated` took: render `cause.payload()` (no
shared tail) and append one labelled `Recourse:` — the edge-local lever
`classify_certify` already names (`EDGE_CLOSE` in `topo::validate`) —
or `geom_core::KERNEL_DEFECT_ENDING` on `MarginDiag::Invalid`. Give the
lever one home both surfaces read (the meter's is
`geom_brep::offset_meters::escalation_recourse`); `EDGE_CLOSE` lives in
`topo`, which `geom-brep` cannot reach, so it moves down.
