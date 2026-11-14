---
id: a-curved-partners-arc-on-a-spline-wall-has-no-at-rest-certificate
kind: issue
title: A sphere, cylinder or torus arc on a spline wall has no at-rest edge certificate: the join's carried chord refuses CertifyError::Unimplemented
status: closed
opened: 2026-11-12
priority: P0
cost: H
closed: 2026-11-12
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

## Closed (2026-11-12, `sectarms/spline-curved-at-rest`)

The arm is `edge_nurbs::curved_nurbs_limbs`, behind
`certify::residual_nurbs_pair`: the plane lane's certificate with the
partner's gradient at the wall's foot as its normal and either
surface's curvature in its arm, limbs 1 and 2 against the partner's
residual in metres and the wall's chart, limb 3 the chart tube against
the partner's form. Its image of the carrier on the wall is the pcurve
mint's climb (`general_chart_image`); the plane lane's degree-1 image
misses a sphere's arc by 4.6e-4 m.

The union this row expected does not build yet. Two stops remain, each
filed:

- past the join, the inside/outside test cannot cross a spline face
  (`work/inside/point-containments-ray-test-has-no-spline-arm.md`): the
  ball row (now
  `a_ball_across_a_lofted_wall_is_joined_and_its_chord_certifies_at_rest`)
  is pinned there at ε = 1e-9 and 1e-6;
- at ε = 1e-12, and on a ten-metre wall at 1e-9, the chord refuses on
  the lane's own image
  (`a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`).

The cost is `a-curved-partners-chord-certifies-at-rest-in-seconds`.
