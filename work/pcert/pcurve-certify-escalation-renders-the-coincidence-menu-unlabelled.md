---
id: pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: PcurveCertifyError's two escalation arms render the coincidence menu unlabelled, where the boundary's own fit leaves nothing to declare
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of
`work/encl/offset-meter-escalation-renders-the-coincidence-menu-unlabelled.md`;
one of four siblings, filed on each owner's slate.)

## What

`geom_brep::PcurveCertifyError` (`crates/geom-brep/src/pcurve_cache.rs`)
has two escalation arms, and both end in
`geom_core::COINCIDENCE_RECOURSE` unlabelled:

- `Escalated { check, sample, cause }` renders "pcurve certification:
  {check:?} at sample {sample} escalated: {cause}" — the
  `Indeterminate` whole, and `{check:?}` is a `Debug` rendering;
- `FittedEscalated { cause }` renders `cause.payload()` followed by
  `COINCIDENCE_RECOURSE` in parentheses.

`test_utils::refusal::recourse_markers` counts zero on both, and both
offer "declare the coincidence", while `topo::validate`'s
`classify_pcurve` routes the same arms (`C::FittedEscalated { cause } |
C::Escalated { cause, .. }`) through
`own_close(&cause.margin, "Recourse: lower the tolerance")` — the
boundary's own fit, where a declaration has no object — and a poisoned
margin to the defect ending.
`recourse-chain-stops-at-pcurve-certify-error` counts these two arms as
"delegating soundly" because they carry the menu; that reading is the
one this row disputes.

## Repair shape

As `geom_brep::offset_meters::MeterError::Escalated` now does: render
`cause.payload()` and append one labelled `Recourse:` (the lever
`classify_pcurve` names), or `geom_core::KERNEL_DEFECT_ENDING` on
`MarginDiag::Invalid`, with the lever in one home both surfaces read.
