---
id: invalid-margin-recourse-cannot-tell-an-unimplemented-kind-from-bad-inputs
kind: issue
title: An invalid-margin Indeterminate renders one recourse for poisoned inputs and for a surface kind with no implicit form (Nurbs/Approx), and the coincidence levers do not reach the second
status: open
opened: 2026-09-28
priority: P3
cost: M
design: true
---


## Finding

`geom_brep::implicit_gradient` and `geom_brep::curvature_lever_arm`
(`crates/geom-brep/src/implicit.rs`) return poison for
`Surface::Nurbs | Surface::Approx`: neither kind has an implicit form.
So `geom_brep::classify_dihedral` escalates with
`MarginDiag::Invalid` at any point on such a surface, and
`geom_brep::must_carry_over_edge` answers `MustCarryVerdict::InBand`
at its first station for every Nurbs/Approx pair — a genuinely smooth
join included (stated in the rule's doc since ENCL's PR 3324).

That refusal means "this kind is not implemented here". The one
rendering an invalid margin gets does not say so:
`geom_core::Indeterminate`'s `Display` (`crates/geom-core/src/predicate.rs`,
the `MarginDiag::Invalid` arm) renders "check the operation's inputs
upstream, then declare the coincidence, move the geometry, or lower the
tolerance", and `sweep::blend::BlendError::Escalated`'s `Display`
(`crates/sweep/src/blend/mod.rs`) composes the same full `Display` on
its fall-through arm for `dihedral_arm`/`dihedral_wedge`. None of those
levers reaches a surface kind the predicate has no arm for, and the
inputs were not bad. `MarginDiag::Invalid` carries no way to tell the
two causes apart, so no site downstream can render the right recourse.

## Reach

No public path hands a Nurbs/Approx pair to the must-carry rule today:
extrude and revolve mint analytic surfaces only, and the blend
surgery's battery admits planes, cylinders, cones and spheres. Other
`classify_dihedral` callers were not swept for a Nurbs-reaching path
here; that sweep is the first step.

Seam from SYM-15 (#3804, 2026-10-02): on the certify path a readable
gradient product that reaches zero is now told apart
(`CertCheck::TangentPlanes`, `dihedral::WedgeEscalation::NoTangentPlane`),
but `classify_dihedral` maps it back to the same `LeverEscalation::reading`,
so its other callers (`topo::census` (`census.rs:2043`),
`topo::boolean::ops` (`ops.rs:1886`), `sweep::extrude` (`extrude.rs:1403`)
and the rest of its ten non-test callers) still render the old
invalid-margin recourse for that case. A fix here should carry the cause
through.

## Shape of a fix

Either the kind gate answers before the predicate (a typed
"unsupported surface kind" refusal where a caller can reach a spline
kind), or the escalation carries its cause so the invalid-margin
recourse can name the unimplemented kind. Which is a design choice.
