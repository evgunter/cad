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

A second source lands on the same variant. The lane's poisoned-aggregate
guard (`plane_nurbs_transversality_reported`,
`geom_core::k_stats::gate_measured` in `plane_nurbs_limbs`,
`crates/geom-brep/src/edge_nurbs.rs` ~:387) maps a NaN minimum sine to
`PlaneNurbsRefusal::Escalated`, so certification reports it as
`PlaneNurbsCertificate` and ends it in the last resort, "loosen the
tolerance". Every per-sample transversality has already decided
Positive by then, so no geometry and no tolerance reaches it: a poison
that survives the fold is a kernel defect, and should end in the
kernel-defect ending. It needs its own arm (or the limb, as below) for
the routing to see that.

The tube's definite refusal is already routed by its own decision.
`PlaneNurbsRefusal::TubeStraddles` is the uniqueness tube's
transversality enclosure containing zero (`ssi_tube_transversality`
in `certify_rung3`), so PR 3351 routes it as
`CertCheck::Transversality`'s straddling arm
(`RefusedArm::Straddles`): the lever alone, at every reading. What
stays on the certificate's ending is `PlaneNurbsRefusal::Limb` (the
on-locus and hull limbs' definite miss) and the undecided
`Escalated`, until the limb rides the escalation.

## Repair shape

Carry the limb on the certificate's escalation
(`CertificateLimb` already does, on the definite side), and give
certification one `CertCheck` per limb so `recourse` routes each.
