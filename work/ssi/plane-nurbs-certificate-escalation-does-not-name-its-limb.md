---
id: plane-nurbs-certificate-escalation-does-not-name-its-limb
kind: issue
title: geom-brep: the rung-3 certificate's escalation does not name its limb, so certification routes the uniqueness tube as a residual
status: open
opened: 2026-09-28
---


(ENCL implementer, from the D4 ¶1 reshape of
`certify-escalation-renders-the-coincidence-menu-unlabelled`.)

## What

`SsiError::Escalated(Indeterminate)` (`crates/geom-brep/src/ssi.rs`)
carries no `SsiLimb`, though `certify_branch`
(`crates/geom-brep/src/ssi/certify.rs`) escalates under three limbs
with different pass sets: the on-locus and hull limbs are residuals
(pass on Zero), and the uniqueness tube passes on a positive
transversality. `edge_nurbs::refusal` forwards it as
`PlaneNurbsRefusal::Escalated`, and certification reports it as ONE
decision, `CertCheck::PlaneNurbsCertificate`, routed as an
approximation's residual (the last resort). D4 ¶1 (i) routes by
decision, so the tube's undecided margin should instead name its own
lever and the tolerance below `m/K`, and a routing by predicate name
is not allowed.

## Repair shape

Carry the limb on the certificate's escalation
(`CertificateLimb` already does, on the definite side), and give
certification one `CertCheck` per limb so `recourse` routes each.
