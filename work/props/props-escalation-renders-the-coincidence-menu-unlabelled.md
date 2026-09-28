---
id: props-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: PropsError::Escalated forwards Indeterminate's coincidence menu, unlabelled, where one face's contribution leaves nothing to declare
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of
`work/encl/offset-meter-escalation-renders-the-coincidence-menu-unlabelled.md`;
one of four siblings, filed on each owner's slate.)

## What

`geom_brep::props::PropsError::Escalated`'s `Display`
(`crates/geom-brep/src/props/mod.rs`, `Self::Escalated { cause }`)
renders "integral properties: classification escalated: {cause}" —
the `Indeterminate` whole, ending in `geom_core::COINCIDENCE_RECOURSE`
unlabelled. So `test_utils::refusal::recourse_markers` counts zero on
it, and it offers "declare the coincidence", while `topo::validate`'s
`classify_mass_props` routes the same arm (`M::Face { P::Escalated }`)
through `own_close(&cause.margin, "Recourse: lower the tolerance")` —
one face's own contribution, where a declaration has no object — and a
poisoned margin to the defect ending. The arm also opens with the stage
prefix `integral properties:`.

## Repair shape

As `geom_brep::offset_meters::MeterError::Escalated` now does: render
`cause.payload()` and append one labelled `Recourse:` (the lever
`classify_mass_props` names), or `geom_core::KERNEL_DEFECT_ENDING` on
`MarginDiag::Invalid`, with the lever in one home both surfaces read.
