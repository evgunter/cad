---
id: props-escalation-renders-the-coincidence-menu-unlabelled
kind: issue
title: geom-brep: PropsError::Escalated forwards Indeterminate's coincidence menu, unlabelled, where one face's contribution leaves nothing to declare
status: review
opened: 2026-09-28
branch: props/recourse-grammar
---

(ENCL implementer, from the §5 sweep of
`work/encl/offset-meter-escalation-renders-the-coincidence-menu-unlabelled.md`;
one of four siblings, filed on each owner's slate.)

## What

`geom_brep::props::PropsError::Escalated`'s `Display`
(`crates/geom-brep/src/props/mod.rs`, `Self::Escalated { cause }`)
renders "integral properties: classification escalated: {cause}" —
the `Indeterminate` whole, ending in `geom_core::COINCIDENCE_RECOURSE`
unlabelled. So `test_utils::refusal::recourse_markers` counts zero on
it, and it offers "declare the coincidence", while `topo::validate`'s
`classify_mass_props` ends the same arm (`M::Face { P::Escalated }`)
with no lever, since its decision is not carried (below) — one face's
own contribution, where a declaration has no object. The arm also opens with the stage
prefix `integral properties:`.

## Repair shape

As `geom_brep::offset_meters::MeterError::Escalated` now does: render
`cause.payload()` and append one labelled `Recourse:` (the lever
`classify_mass_props` names), or `geom_core::KERNEL_DEFECT_ENDING` on
`MarginDiag::Invalid`, with the lever in one home both surfaces read.

## The decision is not carried (ENCL, 2026-09-29)

`PropsError::Escalated { cause }` is raised by some forty decisions
(`props_cone_apex`, `props_meridian_pole`, `props_face_extent`,
`props_quad_converged`, …) and keeps only the `Indeterminate`, so no
ending can follow its decision (D4 ¶1 (i)): `props_quad_converged`'s
in-band twin is `QuadratureBudget`, whose story is the last resort,
while the geometric ones are sized or residual, and no one lever
reaches them all. `classify_mass_props` (`crates/topo/src/validate.rs`,
through `unnamed`) therefore ends it in no lever: "There is no way
through yet", with `geom_brep::recourse::UNREADABLE_MARGIN_NOTE` on a
poisoned margin.
The repair above wants a closed check type on the variant first, with
its ending table beside it (`geom_brep::recourse::SizedDecision` /
`Unsized`), so both this `Display` and `classify_mass_props` read one
ending per decision.

## Resolved (props/recourse-grammar)

**The variant now carries its decision, as a closed type.**
`PropsError::Escalated { cause, check: PropsCheck }`, with `PropsCheck`
a four-row closed enum and one ending per row out of
`geom_brep::recourse`'s table:

| check | what it decides | ending |
|---|---|---|
| `OnSurface` | a premise an exact construction establishes | `Unsized::Defect` |
| `Inventory` | a premise of a lane's certified inventory | `recourse::not_yet` |
| `Extent` | the face's parameter extent, a size the user may intend | `props::FACE_EXTENT` (`SizedDecision`) |
| `Converged` | the quadrature's convergence meter | `Unsized::LastResort` |

So the arm no longer forwards the coincidence menu at all: nothing
offers "declare the coincidence" where one face's own contribution
leaves nothing to declare, the ending is labelled (or is a dead end that
says so plainly), and `recourse_markers` counts 1.

The stage prefix is gone, and the arm now opens with the question that
escalated (`PropsCheck::subject`), so
`test_utils::refusal::subjectless_escalations` reads a subject.

`topo::validate`'s `classify_mass_props` reads the SAME check for both
halves — its clause and its recourse — so TOPO's PR 3398 placeholder
("There is no way through yet" until props carries its decision) is
discharged: that ending is now what `PropsCheck::Inventory` composes, out
of one home (`geom_brep::recourse::not_yet`), and the other three checks
have their own.

`DegenerateFace`, the definite arm of the same face-extent decision,
ends in `FACE_EXTENT` too (D4 ¶1 (iv)), which drops the coincidence menu
from it as well.
