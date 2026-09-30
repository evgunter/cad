---
id: contain-escalation-carries-no-decision
kind: issue
title: topo: ContainError::Escalated and the point-in-solid refusals collapse their decisions, so their endings name the lever alone
status: open
opened: 2026-09-29
---


(ENCL implementer, `work/encl/validate-own-close-levers-follow-the-d4-recourse-ruling.md`.)
The rule is D4 ¶1 (i) in `docs/DESIGN.md`: the recourse follows from the
decision and its verdict, and the decision is a closed type at its site.

## What

- `ContainError::Escalated(Indeterminate)` (`crates/topo/src/boolean/contain.rs`)
  is the point-in-loop walk's escalation from any of its decisions
  (`PointInLoopError::Escalated`), so the decision is gone by the time
  `topo::validate::classify_contain` renders it. It ends in
  `OFF_BOUNDARY` alone ("Recourse: move the geometry clear of the
  boundary"), with no tolerance, since no one decision's margin gives
  one.
- `census::Undecided::of_point_in_solid` (`crates/topo/src/census.rs`)
  folds `PointInSolidError::Escalated`, `RayExhausted` and `Loop(_)` into
  `WitnessTooClose`, whose sentence now ends in the lever alone
  ("move the parts until their bounding boxes no longer overlap").

`ContainError::RayExhausted`'s own `Display`
(`crates/topo/src/boolean/contain.rs`, `impl Display for ContainError`)
still ends "move the point off the boundary or lower the tolerance",
where D4 ¶1 (i) offers no unvalued lowering; validate renders its own
ending over it, but a caller printing the refusal reads the old one.

A point's side of a boundary passes on either nonzero sign, so once the
decision is carried its ending wants a two-sided `SizedPass` (the one
PR 3390 adds as `SizedPass::NonZero`) to quote the tolerance below
`|m|/K` on either side.

## Repair shape

Carry the decision as a closed type on the escalation (and through
`Undecided`), end it through `geom_brep::recourse::SizedDecision` at
`Reading::AtRest` in `classify_contain`, and pin the valued ending.

## At the Boolean's door (TOPO, the §5 second pass of PR 3493)

`PointInSolidError`'s own `Display`
(`crates/topo/src/boolean/solid_contain.rs`, `impl Display for
PointInSolidError`) ends its `Escalated`, `RayExhausted` and
`Loop(RayExhausted)` arms in `COINCIDENCE_RECOURSE`. The Boolean shows
them through `BooleanError::Containment`. "Declare the coincidence" is
advice a face-pair declaration cannot follow there: a ray cast's
graze, or a point near a face of the other solid, is not a pair of
faces. PR 3493 gives the Boolean's face-containment escalations
(`ContainError::Escalated`, through `BooleanError::Escalated`) their
own closed decision, `boolean::BooleanDecision::Containment`, ending in
a `SizedDecision` (`SizedPass::NonZero`) in `boolean::refusal_routes`.
These three arms want the same change when the decision is carried.

## The Boolean's containment ending, and what carrying the rung needs (TOPO, PR 3493's fix pass)

PR 3493 first ended `boolean::BooleanDecision::Containment` in a
`SizedDecision` passing on either nonzero sign. That offered a valued
tolerance on rungs where D4 ¶1 (i) forbids one: at
`boolean::reduce::wall_crossing` the point is a certified root on the
wall, so `bool_curved_contain_carrier` (`contain.rs`,
`curved_face_placement`) is a residual there (it passes only at Zero),
and `bool_curved_contain_period` refuses a negative margin. The fix
pass ends `Containment` on its lever alone at every wrap site
(`reduce.rs` ×2, `ops.rs` ×1), as `validate::classify_contain` does;
`refusal_routes::tests::a_containment_escalation_on_a_residual_rung_names_its_lever_alone`
pins it on a real raise.

Carrying the rung is more than a closed type on
`ContainError::Escalated`. The walk's rungs are `bool_face_disc_carrier`,
`bool_contact_vertex`, `bool_contact_arc` and its end-vertex row, the
carrier and period rungs, the ray cast's rungs through `solid_err`, and
every `PointInLoopError` rung. Their pass sets depend on the caller:
the carrier rung passes on any definite sign at `curved_face_containment`
(off the carrier is a definite `Out`), and only at Zero where the caller
placed the point on the surface. So the carry has to say which caller's
question the rung answered, or each caller has to map the rung to its
own decision, as `BooleanDecision::of_normal` does for the face normal.
Then the Boolean's `Containment` arm splits into those decisions, each
ending as its pass set gives.

## The torus trim's ring convention (TOPO, the §5 second pass of PR 3506)

`boolean::solid_contain::point_on_torus_in_face`
(`crates/topo/src/boolean/solid_contain.rs`) reads the ring convention
through `geom::require_ring_torus`, the collapsed-arm gate: a torus
definitely outside the convention rides `PointInSolidError::Escalated`
with an `INVALID` margin, and an in-band one with its margin, and both
end in `COINCIDENCE_RECOURSE`. PR 3506 gave the convention its closed
type (`topo::TorusConvention`, whose `sized()` and `refused()` are the
pierce's and tier 3's one story). When this row carries the decision,
the torus trim's arm is `TorusConvention` with its verdict
(`geom::ring_torus`/`geom::torus_tube` keep the reporting margin), not a
containment rung.
