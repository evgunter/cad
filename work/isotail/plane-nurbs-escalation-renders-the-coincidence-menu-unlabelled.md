---
id: plane-nurbs-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: PlaneNurbsRefusal::Escalated forwards Indeterminate's coincidence menu, unlabelled, where the edge's own faces leave nothing to declare
status: closed
opened: 2026-09-28
priority: P3
cost: E
closed: 2026-10-26
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

## Closed (2026-10-26, tracker sweep)

**Fixed on main.** `PlaneNurbsRefusal::Escalated`'s `Display` (`crates/geom-brep/src/edge_nurbs.rs`, `impl Display for PlaneNurbsRefusal`, ~509) now renders `"{limb} escalated: {cause.payload()}"`, the margin payload with no recourse tail (`Indeterminate::payload`, `crates/geom-core/src/predicate.rs` ~2597), so the coincidence menu is no longer forwarded. Its ending is the limb's own decision, `PlaneNurbsRefusal::ending` (~371: `SsiDecision::Limb(*limb).ending(cause, at)`), which `CertifyError::ending` reads for the `PlaneNurbs` arm (`crates/geom-brep/src/certify.rs` ~966). Its sibling `work/encl/certify-escalation-renders-the-coincidence-menu-unlabelled.md` closed 2026-09-28 with the same shape. The commit is not findable in this shallow checkout; verified against the code as it stands.
