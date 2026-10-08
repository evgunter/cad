---
id: point-in-solid-escalation-carries-no-decision
kind: issue
title: topo: PointInSolidError::Escalated carries no decision, and its refusals end in the coincidence menu with no declaration door
status: open
opened: 2026-09-29
---


(CONTACT-10 implementer, from the §5 sweep of `contact/10-contain-endings`;
narrowed at its merge with `main` against what PR 3493 did.) The rule is
D4 ¶1 (i) in `docs/DESIGN.md`: the recourse follows from the decision
and its verdict, and the decision is a closed type at its site.

## What CONTACT-10 left here

CONTACT-10 carries the planar loop walk's decision (`LoopDecision`) and
contfp's own (`boolean::ContainDecision`) to every renderer:
- `validate::classify_contain` at rest;
- the census's `Undecided::WitnessTooClose(Some(_))`;
- the Boolean's `BooleanDecision::Containment { decision, escalation }`.

**What it does not carry is the point-in-solid door's own escalation.**
`PointInSolidError::Escalated { face, diag }`
(`crates/topo/src/boolean/solid_contain.rs`) is raised from some thirty
rows of that door: its chart trims, wall outlines, ray × surface roots
and point-in-window tests. It names none of them, and it does not say
how its reading stands (an in-band margin, a straddle of two bounds, or
a row decided and still refused).

Every chain spells "no decision named" one way: `decision: None`. Those
chains are:
- `contain::solid_err` (and `sphere_region`'s `RegionRefusal::Escalated`);
- `Undecided::WitnessTooClose(None)`;
- `classify_point_in_solid`;
- `BooleanDecision::CONTAINMENT_UNNAMED`.

At contfp and at rest these end in the one unnamed lever,
`boolean::placement_lever(None)` ("move the geometry clear of the
boundary"). At the Boolean they end in PR 3493's lever ("move the parts
so they meet clearly inside or clearly outside that face's boundary",
`LeverPass::ByRung`).

`impl Display for PointInSolidError` still ends `Escalated`,
`RayExhausted` and `Loop(RayExhausted)` in `COINCIDENCE_RECOURSE`
("declare the coincidence, move the geometry, or lower the tolerance").
The point-in-solid door takes no declaration, and D4 ¶1 (i) offers no
unvalued lowering. The Boolean shows these through
`BooleanError::Containment`. That is the refusal PR 3513's executed
offers meet later, the stories `topo::test_support::LATER_STORIES_OWNED`
logs under this row (moved here from
`contain-escalation-carries-no-decision`, whose notes record the poses).

## Repair shape

Give the door its own closed decision type on `Escalated`, the way
`LoopDecision` and `ContainDecision` name the walk's and contfp's, and
tag each site's reading with `splitting::Escalation`. Three things
follow from 3493's finding:
- each rung's pass set is the caller's;
- the torus trim's ring convention is `geom_brep::TorusConvention`, not a containment rung (PR 3506);
- a ray-cast rung is no size the user chose.

Then end the `Display` arms through the decision.
