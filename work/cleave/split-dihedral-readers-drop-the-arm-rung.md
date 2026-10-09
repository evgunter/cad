---
id: split-dihedral-readers-drop-the-arm-rung
kind: issue
title: topo: the split's finish and reduce rules drop classify_dihedral's rung, so an arm, undecided or collapsed, refuses as a sliver wedge
status: open
opened: 2026-09-29
---

## What

`topo::splitting::finish` (near line 874,
`SplitFinishError::DescribeEscalated`), `splitting::neighborhood`
(near line 264) and `splitting::rules` (near lines 247, 325, 440, 454
and 494, `SplitReduceError::SliverSector`) read only
`geom_brep::LeverEscalation::diag()`, so the dihedral's arm —
undecided, or decided collapsed (`LeverEscalation::collapsed_arm`) —
refuses as a sliver of the wedge rather than as the arm's decision
(`geom_brep::DIHEDRAL_ARM`). The boolean's seam
(`boolean::ops::seam_refusal`) keeps the rung, and does not yet read
the arm's verdict. Found by ENCL's sweep (PR 3431); filed on REACH,
which closed, and carried here with its ground.

## Shape

Keep the rung, end the arm through `DIHEDRAL_ARM` at `Reading::Build`,
and a collapsed arm as a definite refusal.

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

## The boolean seam and merge doors drop the verdict too (ENCL, PR 4366)

`boolean::ops::seam_reading` and `must_carry_reading` hand on
`(DihedralReading::Lever(escalation.rung()), escalation.diag())`, and
`merge_faces::DihedralReading::Lever` carries a bare `LeverRung` too, so
an arm the gate decided collapsed reaches `seam_refusal` and
`MergeCoplanarError::KeptBoundaryUndecided` as an undecided escalation.
`BooleanError::of_lever_rung(gate, read, rung, diag)` takes parts, so its
signature cannot carry the gate's verdict either. The design point for
the repair: carry the escalation whole, as
`DihedralReading::Lever(geom_brep::LeverEscalation)`, so
`LeverEscalation::collapsed_arm` stays reachable at every door that ends
the reading, and route a collapsed arm as the definite refusal it is
(`BooleanError::of_lever` already takes the whole escalation).
