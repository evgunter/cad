---
id: pcurve-certify-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: PcurveCertifyError's two escalation arms render the coincidence menu unlabelled, where the boundary's own fit leaves nothing to declare
status: open
opened: 2026-09-28
cost: E
priority: P3
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
`classify_pcurve` ends the same arms through their decision's ending
at rest (`PcurveCertifyError::ending`, below) — the boundary's own fit,
where a declaration has no object. Both arms also open with the stage prefix
`pcurve certification:`.
`recourse-chain-stops-at-pcurve-certify-error` counts these two arms as
"delegating soundly" because they carry the menu; that reading is the
one this row disputes.

## Repair shape

As `geom_brep::offset_meters::MeterError::Escalated` now does: render
`cause.payload()` and append one labelled `Recourse:` (the lever
`classify_pcurve` names), or `geom_core::KERNEL_DEFECT_ENDING` on
`MarginDiag::Invalid`, with the lever in one home both surfaces read.

**The one home exists now (ENCL, 2026-09-29).** `PcurveCertifyError::ending(reading)`
and `PcurveCheck::recourse` (`crates/geom-brep/src/pcurve_cache.rs`)
route each check to its decision's ending: the span and azimuth checks
through the edge certifier's `ParamSpan` and `ParamWinding`, the chart
winding as an exact form selection, the map residual, envelope, trim box
and the fitted certificate as the last resort. `classify_pcurve` reads it
at rest; this `Display` can append it at the reading its door knows.

## The escalation's subject (CHROME, PR 3457)

`geom_core::IndeterminatePayload` no longer renders the predicate's
name, so the words before the margin carry the whole subject. Here they
are "the fitted lane's certificate escalated — enclosure […] …", which
names the lane, not what it was deciding.
`test_utils::refusal::subjectless_escalations` would read that clause as
no subject if a guarded row rendered it (none does today;
`topo/tests/review_ssiflat_r1_probes.rs` pins the current words). The
repair is a clause saying what the certificate could not decide, in
words.

## The fitted escalation drops its limb (SSI, `ssi-diag`)

The SSI certificate's escalations now carry the limb that escalated
(`geom_brep::SsiError::CertificateEscalated { limb, cause }`), and the
plane × NURBS lane routes each limb by its own decision: limbs 1 and 2
as residuals (the last resort), limb 3's in-band transversality as
`CertCheck::Transversality` (its lever and the tolerance below `m/K`).
`pcurve_cache.rs` `ssi_refusal` forwards it as
`PcurveCertifyError::FittedEscalated { cause }`, dropping the limb, and
`PcurveCertifyError::ending` ends every `FittedEscalated` in
`Unsized::LastResort`. So a fitted pcurve whose uniqueness tube's
transversality lands in band is told to loosen the tolerance, where the
edge certifier tells the same verdict to move the geometry. The subject
clause this row asks for is the limb (`SsiLimb::name`), and the ending
is `SsiLimb::recourse`.
