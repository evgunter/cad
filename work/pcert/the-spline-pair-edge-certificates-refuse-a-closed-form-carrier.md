---
id: the-spline-pair-edge-certificates-refuse-a-closed-form-carrier
kind: issue
title: the plane x NURBS and NURBS x NURBS edge certificates refuse any non-spline carrier, so the mint's General route for one is unreachable at rest
status: closed
opened: 2026-10-12
priority: P1
cost: M
closed: 2026-10-14
branch: pcert/spline-pair-any-carrier
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

## A premise that moves with the answer

`topo::pcurves::nurbs_iso_derive`'s circle fall-through returns the
arc-rim pick without falling through to `General`, on the premise that
no door certifies an `Intersection` circle on a spline chart (PR 4988).
A closed-form carrier the spline-pair lanes take would end that premise:
`an_intersection_circle_on_a_spline_wall_is_refused_at_the_door`
(`crates/sweep/tests/an_arc_rim_picks_the_row_it_lies_on.rs`) goes red
then, and the fall-through has to reach `General` where the arc-rim
certification refuses.

## The answer

Ruled by Ev (PR 4974, 2026-10-14): as recommended. The two lanes take
a carrier of any kind on a rung-3 pair, checked by the C2 certificate's
three limbs (`crates/geom-brep/README.md` C1/C2): limbs 1–2 read a
closed-form carrier through `geom_brep::carrier_net`, limb 3 is the
chart or two-chart tube as today. Ev's framing: "it's a bit weird to
have something that's only reachable by import but i'm ok with this as
a temporary state while the within-kernel certification is unbuilt".
Today only import reaches it (a file's stated line or circle, or D7's
Line promotion); the kernel's own spline-pair intersections mint spline
carriers. When the lanes land, `nurbs_iso_derive`'s circle fall-through
(above) must reach `General` where the arc-rim certification refuses.

## Closed

Built on `pcert/spline-pair-any-carrier`, as ruled:

- **One carrier-enclosure door**, `geom_brep::carrier_net::carrier_net`:
  a spline is its own net, its round's breaks inserted exactly by the
  composite; a line its degree-1 net; a circle, ellipse or spiric its
  Taylor-enclosing net. A span it cannot enclose refuses typed,
  `CarrierEnclosureRefusal { kind, span }`, carried as
  `SsiError::CarrierEnclosure`, `PlaneNurbsRefusal::CarrierEnclosure`
  and `PcurveCertifyError::CarrierEnclosure`.
- The fitted row's round and the SSI certificate's limb 2 (both
  `analytic_limbs`, which runs on the plane side of every plane × NURBS
  edge, and `nurbs_limbs`) read the carrier only through it
  (`ssi::certify::carrier_chart_round`). `certify_branch` takes a
  `Curve3` and the edge's interval; `carrier_span` is the one rule for
  the span a certificate and the mint's image cover. The ℝ³ box chain
  stays spline-only (`BOX_CHAIN_NEEDS_SPLINE`).
- `run_checks`'s two "must be a spline" refusals are deleted; the
  resolver is unchanged.
- `nurbs_iso_derive`'s circle fall-through certifies the arc-rim pick
  and reaches `General` where that certification definitely refuses.

Evidence: `geom-brep`'s `spline_pair_any_carrier` rows (a line, a
circle and an ellipse on a plane × spline pair, and a line and a circle
between two spline charts, each certifying or refusing at limb 2 on its
measured bound, with the same verdict for its spline twin; a carrier
2ε off the wall between samples escalating at limb 2 on the 2ε it read;
the door's typed refusal by kind and span; the closed forms through
the certification door). `sweep`'s
`an_intersection_circle_on_a_spline_wall_is_refused_at_the_door` is
re-pinned: the rim's circle now reaches limb 2 and refuses on its
chord error (≈4.6e-4 m) rather than as `Unsupported`.

The non-plane analytic × spline gap stays its own item:
`analytic-by-spline-edges-refuse-as-unimplemented-at-resolve`.
