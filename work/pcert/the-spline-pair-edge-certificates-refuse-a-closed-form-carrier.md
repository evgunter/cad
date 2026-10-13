---
id: the-spline-pair-edge-certificates-refuse-a-closed-form-carrier
kind: issue
title: the plane x NURBS and NURBS x NURBS edge certificates refuse any non-spline carrier, so the mint's General route for one is unreachable at rest
status: open
opened: 2026-10-12
priority: P1
cost: M
design: true
needs_ev: true
---


Found by `pcert/fitted-row-reads-carrier-and-chart`.

## What happens

That branch lets the fitted-grade row certify a line, circle, ellipse or
spiric on a spline chart (check 4 is the hull against the chart, the
carrier entering as its enclosing net, `crates/geom-brep/src/carrier_net.rs`).
It also widens the mint's `Intersection` arm
(`crates/topo/src/pcurves.rs`, `nurbs_iso_derive`) to give such a
carrier its `General` image. No body at rest reaches that arm, because
the edge certificate refuses the edge first. `run_checks`
(`crates/geom-brep/src/certify.rs`) resolves a plane × spline pair to
`Resolved::PlaneNurbs` and a spline × spline pair to
`Resolved::PairNurbs`. Each refuses a non-`Nurbs` carrier ("the
declared carrier of a … Intersection must be a spline: the
certificate's limbs are hull statements about a control net"). A
non-plane analytic × spline pair refuses at `resolve`
(`CertifyError::Unimplemented`).

So the mint's widened arm is covered by rows at its doors
(`pcurve_general::the_derived_route_certifies_a_line_and_refuses_a_circle_at_the_hull`,
`general_image` then `certify_general`), not through a body.

## The question

Whether the two lanes take a closed-form carrier, and how:

- limbs 1–2 can read the carrier as the fitted row now does: a line as
  its exact degree-1 net, a conic or spiric enclosed per span;
- limb 3 needs nothing new: on these two pairs the tube is the chart
  tube (`probe_tube_chart`) or the two-chart tube (`probe_tube_pair`),
  which read the pcurves (`edge_nurbs::chart_image`, a schedule over
  the carrier's values) and the carrier's end points. Only the
  analytic × analytic ℝ³ tube (`box_chain`) reads a control net, and no
  spline pair uses it. So limb 2's composite `S(P(t)) − C(t)` is the
  one place a net is owed, and `geom_brep::carrier_net` already
  supplies it to the fitted row.

D7 recognition's Line promotion lands on the same wall
(`work/exch/d7-promotes-an-off-iso-straight-spline-the-intersection-rung-then-refuses.md`).

A non-plane analytic surface paired with a spline refuses at `resolve`
for spline carriers too, so it is its own gap:
`analytic-by-spline-edges-refuse-as-unimplemented-at-resolve`.
