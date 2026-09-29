---
id: offset-meter-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: MeterError::Escalated forwards Indeterminate's coincidence menu, unlabelled and offering a declaration a face's meter has no object for
status: closed
opened: 2026-09-28
priority: P3
cost: E
closed: 2026-09-28
---


(ENCL implementer, from the sweep of
`kernel-defect-endings-and-repair-labels-have-no-shared-home`.)

## What

`geom_brep::offset_meters::MeterError::Escalated`'s `Display`
(`crates/geom-brep/src/offset_meters.rs:242`) renders its `Indeterminate`
whole: "whether the face can be offset is too close to call:
{source}". `Indeterminate`'s own `Display`
(`crates/geom-core/src/predicate.rs`) ends in
`COINCIDENCE_RECOURSE`, unlabelled ("— a near-coincidence; declare
the coincidence, move the geometry, or lower the tolerance"). Two
things follow on the offset fit's refusal surface:

- The repair is unlabelled, so `test_utils::refusal::recourse_markers`
  counts zero on it, the one offset-fit arm that does after the
  labelling ruling (every other carrier the fit forwards labels its
  repair `Recourse:`). `editor-core/tests/refusal_concision_chains.rs`
  `every_offset_fit_refusal_ends_exactly_once` holds `Meter/Escalated`
  at zero markers, so the repair has to update that row.
- The menu offers "declare the coincidence", which a meter about one
  face's own normal or curvature has no object for. `topo::validate`'s
  `classify_offset_fit` already routes this arm per predicate name
  ("use an offset distance of smaller magnitude, or lower the
  tolerance" for `offset_curvature_headroom`; "split the face clear of
  any pole, cusp or pinch, or lower the tolerance" otherwise), and a
  poisoned margin to the kernel-defect ending.

## Repair shape

Render `source.payload()` (`IndeterminatePayload`, no shared tail) and
append one `Recourse:` routed the way `classify_offset_fit` routes it,
or `geom_core::KERNEL_DEFECT_ENDING` on `MarginDiag::Invalid`. Then
flip the chains row's exception to one marker like every other arm.
