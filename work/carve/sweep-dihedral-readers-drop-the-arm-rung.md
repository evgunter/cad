---
id: sweep-dihedral-readers-drop-the-arm-rung
kind: issue
title: sweep: extrude and revolve upgrade drop classify_dihedral's rung, so an arm, undecided or collapsed, refuses as a sliver wedge
status: open
opened: 2026-09-29
---

## What

`sweep::extrude` (near lines 1332 and 1654) and
`sweep::revolve::upgrade` (near line 189) read only
`geom_brep::LeverEscalation::diag()`, so an escalation of
the dihedral's arm — undecided, or decided collapsed
(`LeverEscalation::collapsed_arm`) — refuses as `ExtrudeError::SliverJoin` /
`SliverRim` or the upgrade's sliver, the wedge's story, rather than as
the arm's decision (`geom_brep::DIHEDRAL_ARM`, a length). Found by
ENCL's sweep (PR 3431).

## Shape

Keep the rung and end the arm through `DIHEDRAL_ARM` at
`Reading::Build`; a collapsed arm as a definite refusal.

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

## Since (ENCL, `encl/sweep-must-carry-escalation`)

`ExtrudeError::SliverJoin`/`SliverRim` and `RevolveError::SliverJoin`/`SliverRim`
carry `reading: topo::DihedralReading`, `Lever(rung)` at both the witness
classification and the must-carry rule's first-order stations, and `Bend` for
the second-order one. The rung now reaches `Display`
(`sweep::swept::sliver_text`), which still words both rungs as the wedge's
sliver: the repair is that function's `Lever(LeverRung::Arm)` arm.

**One reading for two decisions (review of PR 4450).** At the extrude and
revolve must-carry first-order stations (arm or wedge after a witness that
read smooth) the generic `Indeterminate` `Display` the sliver sentence ends
in offers "tighten the tolerance below …", which is false there: a smaller
ε reads the wedge `Transverse` and the edge refuses `SmoothJoinRefuted`. At
the witness classification the same offer is true (either class builds).
`DihedralReading::Lever(rung)` cannot tell the two sites apart, so one
reading names two decisions with different pass sets. The type-level fix
is a closed decision per reading at each site, as `sweep::blend` has
(`BlendDecision::ContactArm`/`ContactWedge`/`ContactSecondOrder`).
Optional companion: a closed bit on `geom_brep::LeverEscalation` saying
whether its margin is the arm's own or the re-quoted wedge's (`at_wedge`)
would let `ContactArm` offer the tolerance truthfully where the arm's own
margin binds.
