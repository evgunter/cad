---
id: an-at-rest-certify-lead-calls-a-gates-definite-rejection-undecided
kind: issue
title: validate: an at-rest certification refusal routed as Escalated leads with 'undecided' (certify_undecided) on a gate's definite rejection, keyed on the variant rather than the reading's class
status: open
opened: 2026-10-14
---


(Filed by the ENCL implementer of `a-lever-decisions-definite-arm-at-rest-drops-its-note`, from its sweep.)

## What

`topo::validate::classify_certify` picks an at-rest certification finding's lead by the `CertifyError` variant: every `CertifyError::Escalated { check, .. }` leads with `certify_undecided(check)` (`crates/topo/src/validate.rs`). A gate's rejection of a reading past the band (`geom_core::k_stats::decide_positive` on a definitely negative margin, `MarginDiag::rejected_sign` = `Negative`) reaches a reader as an `Indeterminate`, so a certifier that carries it as `Escalated` gets the "undecided" lead although the reading is definite.

The ending already reads the class: since `encl/at-rest-one-story`, `SizedDecision::recourse` ends that reading as the sign-certain arm does (a stored contradiction's defect ending at rest; the lever and the note where the lever reaches it). So the lead and the ending now tell two stories on one finding. The new `validation_error_samples` rows (`CertifyError::Escalated` with `rejected_negative(..)`, `crates/topo/src/test_support_samples.rs`) render, at rest:

> an edge's stored curve does not certify against its faces: whether it is long enough, for how its faces curve, to measure their angle is undecided. There is no way through: this is a kernel defect or a damaged file; report it

## Question

Should the lead read the cause's class (as `Indeterminate::reading` already does for its word, "a decided sign this decision cannot use") rather than the variant, so a definite reading carried as `Escalated` leads as its sign-certain sibling (`CertifyError::ArmCollapsed`, `IntervalNotForward`, `SpanMeterCollapsed`) does? First check which certifiers can carry a gate's rejection as `Escalated` in production; if none can, the fix may be a debug assertion at the raise sites instead.

- 2026-10-20 (ENCL implementer, `encl/unsized-gate-word`): the payload now says a decided sign's word itself. `geom_core::IndeterminatePayload` appends " — a decided sign this decision cannot use" wherever a gate's tag carries a rejected nonzero sign (`MarginDiag::rejected_sign`), so every door that composes `payload()` with an ending tells it. This row's sample renders no payload, so it is unchanged and still tells the gate's rejection only as "undecided"; so do `topo::validate`'s other payload-free at-rest leads over an `Indeterminate` (`DegenerateTorusEscalated`, `PlanarFaceEscalated`, `PlanarBoundaryEscalated`, the props findings' `undecided!` leads), for any gate's rejection past the band. The lead's verb is wider than `validate`, too: every `Indeterminate::undecided` door (`geom_core::UndecidedRefusal`, about 40 callers across `geom-core`, `geom-brep`, `profile`, `sweep`, `topo` and `editor-core`) and every "{check} … escalated:" lead (`geom_brep::PcurveCertifyError::Escalated`, `CertifyError::Escalated`, `edge_nurbs`'s limbs) leads a gate's rejected sign with "undecided" or "escalated", now followed by the word. `MarginDiag::rejected_sign` is what a lead could read; a reading past the band with no tag (a declared pair's reported bound, `CarrierEqError::Unsettled`) is undecided, and its lead is right.
