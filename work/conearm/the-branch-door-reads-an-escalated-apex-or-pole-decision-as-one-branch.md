---
id: the-branch-door-reads-an-escalated-apex-or-pole-decision-as-one-branch
kind: issue
title: require_one_chart_branch reads an escalated apex or pole decision as one branch, so a door that cannot say answers rectangle
status: closed
opened: 2026-11-17
closed: 2026-11-17
---



## What

`geom_brep::props::require_one_chart_branch` (`crates/geom-brep/src/props/curved.rs`)
reads its two singular-point decisions with `matches!(decide(..), Ok(Sign::Positive))`:
the cone arm's `props_cone_apex` and the sphere arm's `props_meridian_pole`
(inside the loop over `sphere_meridian_pole_margins`). Only a definite
`Positive` refuses `NotOneChartBranch`; an `Err` (the decision escalated,
the singular point within band of the span's interior) falls through to
"one branch". So a door that cannot say answers "rectangle", and
`mesh`'s curved lane walks an arc it may read a half-turn away
(`mesh::curved::iso_rectangle_shape` routes on this door's `Ok`).

The same holds on main before `tesslane/route-by-domain` (found by that
unit's review, 2026-11-17): the reading is the door's, not the route's.

## The shape of a fix

Map the escalation to `PropsError::Escalated` (check `Exact` for the
apex/pole's incidence, as the parse's own names do), so the door's
`Ok` means "decided on one branch". A row: a meridian span whose pole
margin lands in the band, and a generator whose apex margin does,
each answering `Escalated` at the door and `UnsupportedCurvedShape {
source: Escalated }` through `mesh::tessellate`.

## Closed (2026-11-17, PR 7041's second fix pass)

Both decisions now read through `classify_as` under `PropsCheck::Inventory`:
a definite crossing anywhere in the loop refuses `NotOneChartBranch`, and
an apex or pole the band cannot place answers `Escalated` with its cause
only where no crossing is definite, so the door's `Ok` means decided on one
branch. Rows: `geom-brep` `mesh11_arc_branch::an_apex_the_band_cannot_place_escalates_at_the_branch_door`
(the cone) and `mesh11r1_probes::r1_arcs_ending_past_a_pole_across_the_whole_band`
(the sphere's indeterminate rung, which pinned the old admission and now
pins the escalation). No body in the `mesh`, `sweep` or tour suites reaches
an undecided apex or pole at this door.
