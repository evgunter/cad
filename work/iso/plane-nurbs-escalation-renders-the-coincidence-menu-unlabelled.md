---
id: plane-nurbs-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: PlaneNurbsRefusal::Escalated forwards Indeterminate's coincidence menu, unlabelled, where the edge's own faces leave nothing to declare
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of
`work/encl/offset-meter-escalation-renders-the-coincidence-menu-unlabelled.md`;
one of four siblings, filed on each owner's slate.)

## What

`geom_brep::PlaneNurbsRefusal::Escalated`'s `Display`
(`crates/geom-brep/src/edge_nurbs.rs`, `Self::Escalated(diag)`) renders
"a plane × NURBS limb margin escalated: {diag}" — the `Indeterminate`
whole, ending in `geom_core::COINCIDENCE_RECOURSE` unlabelled. So
`test_utils::refusal::recourse_markers` counts zero on it, and it offers
"declare the coincidence", while `topo::validate`'s `classify_certify`
ends the same arm (`CertifyError::PlaneNurbs(P::Escalated(cause))`)
through its decision's ending at rest (`CertifyError::ending`,
`CertCheck::PlaneNurbsCertificate`'s last resort) — an edge's own two
faces, where a declaration has no object.

## Repair shape

As `geom_brep::offset_meters::MeterError::Escalated` now does: render
`diag.payload()` and append one labelled `Recourse:` (the edge-local
lever), or `geom_core::KERNEL_DEFECT_ENDING` on `MarginDiag::Invalid`.
The lever is shared with `CertifyError::Escalated`
(`work/encl/certify-escalation-renders-the-coincidence-menu-unlabelled.md`),
so the two land together or one waits for the other's home.
