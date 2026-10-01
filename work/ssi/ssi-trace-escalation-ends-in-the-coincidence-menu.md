---
id: ssi-trace-escalation-ends-in-the-coincidence-menu
kind: issue
title: geom-brep: SsiError's trace escalation ends in the coincidence menu, and two certificate arms carry no ending, where the SSI doors take no declaration
status: open
opened: 2026-10-01
---


(SSI implementer `ssi-diag`, from the §5 sweep of PR "SSI: refusals
name their operand and limb, and end by the recourse table".)

## What

`geom_brep::SsiError::Escalated(Indeterminate)`'s `Display`
(`crates/geom-brep/src/ssi.rs`, `impl Display for SsiError`) renders
"ssi: a trace trilean escalated — an ill-conditioned operand pair at
this tolerance: {diag}", which is `Indeterminate`'s whole Display and
so ends in `geom_core::COINCIDENCE_RECOURSE` ("declare the coincidence
…"). No SSI door takes a declaration (`cylinder_sphere_ssi`,
`plane_nurbs_ssi`, `trace_plane_nurbs_uncertified`,
`idealized_trace_r3` all take two surfaces, a domain and a band), so
the menu offers a lever the reader does not hold, and the arm names no
decision: after the certificate's limbs moved to
`SsiError::CertificateEscalated`, the producers left on this variant
are the march's and the door's own trileans
(`crates/geom-brep/src/ssi/march.rs` `march`: `ssi_transversality_arm`,
`ssi_transversality`, `ssi_step_progress`, `ssi_branch_open_end`,
`ssi_closure_return`, `ssi_closure_tangent`; `ssi.rs`
`cylinder_sphere_ssi`: `ssi_cs_tangency`).

Two of them are the undecided arms of decisions whose decided arms now
end by the table: `ssi_transversality` is the transversality decision
(`SsiError::TransversalityBand` ends through
`CertCheck::Transversality`), and `ssi_cs_tangency` is the pair's
tangency gap (`SsiError::PairTangent`). D4 ¶1 (iv) wants one story per
decision, so their escalation should end as their verdict does.

`SsiError`'s `Display` is the payload alone and `SsiError::ending`
gives the ending, as `CertifyError`'s pair does; `Escalated` is the
one arm whose `Display` still carries a recourse (the menu above), and
its `ending` is `None`.

Two certificate arms have no ending (`SsiError::ending` gives `None`):
`SsiError::CertificateLimb` ("… the cache is not within tolerance of
the locus it claims") and `SsiError::TubeStraddles`, whose undecided
sibling `SsiError::CertificateEscalated` ends by its limb's check
(`SsiLimb::check`, read through `geom_brep::certify::recourse`).
`TubeStraddles` is already about 50 literal words, so its rendered text
needs rewriting shorter as it gains the ending.

Limb 3's check is `CertCheck::Transversality`, whose lever is the edge
certifier's ("move the geometry so the faces cross at a clearer
angle"). At an SSI door the operands are surfaces, not an edge's faces,
and separating them passes, so a tube escalation read there names the
edge's lever while `TransversalityBand`, the same question on the
march, names the SSI's own (`ssi.rs` `TRANSVERSALITY`). One story per
decision wants one lever in words true at both doors.

## Repair shape

Give the trace escalation the decision it escalated (a closed type, not
a lookup by predicate name), end the transversality and tangency ones
by the same table their verdicts use, and end the rest by their own
levers or, where none exists, the last resort. End `CertificateLimb`
and `TubeStraddles` through `SsiLimb::check` on their definite arm.
