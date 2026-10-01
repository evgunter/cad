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

Two certificate arms carry no ending at all: `SsiError::CertificateLimb`
("… the cache is not within tolerance of the locus it claims") and
`SsiError::TubeStraddles`, whose undecided sibling
`SsiError::CertificateEscalated` now ends by `SsiLimb::recourse`.
`TubeStraddles` is already about 50 literal words, so appending the
ending means rewriting it shorter.

## Repair shape

Give the trace escalation the decision it escalated (a closed type, not
a lookup by predicate name), end the transversality and tangency ones
by the same table their verdicts use, and end the rest by their own
levers or, where none exists, the last resort. End `CertificateLimb`
and `TubeStraddles` by `SsiLimb::recourse` on their definite arm.
