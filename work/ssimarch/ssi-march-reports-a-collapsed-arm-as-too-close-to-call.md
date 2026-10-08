---
id: ssi-march-reports-a-collapsed-arm-as-too-close-to-call
kind: issue
title: geom-brep: the SSI march reports a transversality arm decided collapsed as SsiError::Escalated, 'too close to call'
status: open
opened: 2026-10-08
---

(Filed by ENCL, PR 3431: the residue of the SSI half of
`work/encl/certify-collapsed-arm-gates-route-as-the-decision-they-guard.md`,
whose ending half landed there.)

## What

`geom_brep::ssi::march`'s `decide_transversality`
(`crates/geom-brep/src/ssi/march.rs:928-929`) gates the point arm with
`decide_positive("ssi_transversality_arm", ..)` and maps every refusal
to `TraceDecision::TransversalityArm.escalated(cause)`, so an arm the
gate DECIDED zero or negative (a point arm of no length) is an
`SsiError::Escalated`. Its `Display` (`crates/geom-brep/src/ssi.rs:1133-1138`)
reads "ssi: whether the crossing angle's lever arm … is a positive
length is too close to call", which is false for a decided arm: the
question was answered, and the answer is that there is no arm.

PR 3431 routed the ENDING: `TraceDecision::ending` now ends
`TransversalityArm` by `CertCheck::TransversalityArm` (a length, through
`geom_brep::DIHEDRAL_ARM`), and the gate's escalation already carries
the decided margin tagged with the refused sign. What is left is the
payload's story.

## Shape

In `decide_transversality`, read the gate's own verdict with
`geom_brep::recourse::Refused::rejected(&cause)` (the march's cause is
the gate's own escalation, never re-quoted, so the tag is the gate's
verdict) and give a decided arm a definite `SsiError` variant that ends
through `CertCheck::TransversalityArm` on `verdict.arm()`, as
certification's `CertifyError::ArmCollapsed` does. A reader that
re-quotes the arm's margin first must keep the gate's verdict instead,
as `geom_brep::LeverEscalation::with_diag` does.


## The question's shape (ENCL, `encl/lever-escalation-one-shape`)

The arm decision's other doors now tell it in one shape (D4 ¶1 (iv)):
`CertCheck::TransversalityArm`'s noun, `topo::validate`'s
`certify_undecided`, `classify_certify` and `WedgeCheck::Arm`'s lead,
`ValidationError::NoDihedralArm` and the boolean's `LeverArm::Seam`
subject all ask "whether the edge is long enough, for how its faces
curve, to measure their angle" (pinned by `topo::validate`'s
`the_dihedral_arm_is_told_in_one_shape`). `TraceDecision::question`'s
`TransversalityArm` arm still reads "the crossing angle's lever arm (the
surfaces' curvature radius, or the feature extent) is a positive
length", beside the shared `DIHEDRAL_ARM` ending. When the definite
variant above lands, tell its question in the shared shape too (the
march has a crossing curve rather than a stored edge, so the subject
noun may differ; the clause after it should not), and keep the
rendered refusal within the 75-word budget.
