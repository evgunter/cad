---
id: contact-gate-readers-drop-the-arm-verdict-or-mint-invalid
kind: issue
title: topo: census's dihedral read drops the arm's rung and verdict, and contact_verify mints its gate refusals as Invalid
status: open
opened: 2026-09-29
---

## What

Found by ENCL's sweep of the collapse gates (PR 3431), re-read against
main on 2026-10-08:

- `topo::census` (near line 2180) reads only
  `LeverEscalation::diag()` from `classify_dihedral`, so an arm
  the gate decided collapsed, and an undecided one, both reach the
  census as an undecided candidate, with no rung to end them by.
- `topo::boolean::contact_verify` hand-mints `MarginDiag::INVALID`
  escalations at seven sites. Each wants reading to tell a gate's
  definite verdict, which the funnel now carries tagged, from a real
  poison.

## Shape

Where the verdict reaches a person, keep the rung and the arm's
verdict, and end each as its own decision.

## The door that exists

A gate's rejection carries the margin the classifier decided, tagged
with the sign it refused (`geom_core::MarginDiag::rejected_sign`;
`geom_brep::recourse::Refused::rejected` reads it as a verdict). A
dihedral or `enters_material` escalation is a `geom_brep::LeverEscalation`:
its `rung` names the arm or the reading, and
`LeverEscalation::collapsed_arm` hands back the arm's verdict where the
gate decided it (ENCL, `work/encl/certify-collapsed-arm-gates-route-as-the-decision-they-guard.md`,
PR 3431). Certification (`CertCheck::TransversalityArm`,
`CertifyError::ArmCollapsed`, `CertCheck::ParamSpanMeter`) and the
validator (`WedgeCheck::Arm`, `ValidationError::NoDihedralArm`) route
through them.
