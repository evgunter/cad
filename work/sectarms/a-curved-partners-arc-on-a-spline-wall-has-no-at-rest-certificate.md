---
id: a-curved-partners-arc-on-a-spline-wall-has-no-at-rest-certificate
kind: issue
title: A sphere, cylinder or torus arc on a spline wall has no at-rest edge certificate: the join's carried chord refuses CertifyError::Unimplemented
status: open
opened: 2026-11-12
priority: P0
cost: H
---



Filed by the `sectarms/join-reads-spline-implicit` lane (stage A of
`the-join-reads-a-spline-implicit-section`). This is stage B: the stop
the unit's done-when (a sphere × lofted-wall union builds) now reaches.

## What

The join reads a sphere × spline wall section through the value and
mints its chords as arcs of the whole reading's certified carrier
(`chord_join::carried_chord_spec`). The chord's edge is described as
`Intersection { wall, sphere, witness }`, and the at-rest certificate
has no arm for it: `geom_brep::certify` (`resolve`, beside
`plane_nurbs_pair` and `nurbs_pair`) admits an `Intersection` only where
both sides are analytic, a plane meets a spline, or two splines meet, so
a spline beside a sphere, cylinder or torus refuses
`CertifyError::Unimplemented`.

Witness: `crates/sweep/tests/a_partner_across_a_lofted_wall_is_joined_on_its_section_value.rs`
`a_ball_across_a_lofted_wall_is_joined_on_its_carried_arc` pins the
refusal for a ball across the loft prism's flat and flared walls.

## What it needs

An at-rest arm for a curved implicit partner against a spline chart:
limb 2 against the partner's residual in metres (`ImplicitForm`) and
against the wall's chart, and limb 3 the chart tube read against the
partner's form (`ChartPhi`), as the trace certifies the branch (C3's
residual × NURBS lane) and as `edge_nurbs::plane_nurbs_limbs` does for a
plane. C2 says rest re-derives the door's certificate; no clause names
this arm's shape, so the plan should check whether it is a fork.

When it lands, the row above goes red; re-pin it to the union building,
with its volume held, which is the join unit's done-when.
