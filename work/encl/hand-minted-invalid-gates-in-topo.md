---
id: hand-minted-invalid-gates-in-topo
kind: issue
title: topo: a few dozen hand-minted MarginDiag::INVALID escalations may be gate verdicts the funnel now carries tagged
status: open
opened: 2026-09-29
---

(ENCL implementer, residue of
`certify-collapsed-arm-gates-route-as-the-decision-they-guard`.)

## What

The second pass of ENCL's collapse-gate sweep (PR 3431) looked for
gates minted by hand rather than through the funnel: production
`MarginDiag::INVALID` in `topo`. As of 2026-10-08 that is a few dozen
sites, in `validate`, `boolean::{mod, rim_wedge, solid_contain,
carrier_eq, plane_eq, refusal_routes, vtxfac}`, `props`,
`chart_region`, `flush`, `merge_faces` and `invalid_margin`
(`contact_verify`'s seven are on CONTACT's
`contact-gate-readers-drop-the-arm-verdict-or-mint-invalid`). None is
read yet. Each is either a real poison or a definite verdict reported
as one; the second kind should come from the funnel, which now carries
the decided margin tagged with the refused sign
(`MarginDiag::rejected_sign`).

`dihedral::classify_material_pairing_as`'s `decide_nonzero` gate is
the same class through the funnel: its definite zero reaches the
validator as `WedgeCheck::MaterialSide`, whose ending reads no margin.

## Shape

Read each site; route a definite verdict through the funnel's gate and
end it as its own decision.

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
