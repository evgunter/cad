---
id: the-arc-rim-pick-on-a-chart-closed-in-v-has-no-row
kind: issue
title: nothing exercises the arc rim's row-and-direction pick on a chart whose two boundary rows coincide in 3-D (closed in v), the one case where candidate order decides
status: closed
closed: 2026-10-19
opened: 2026-10-13
priority: P3
cost: E
---


Found by the review of PR 4988 (`pcert/arc-rim-row-pick`, its S6).

## What is missing

`nurbs_iso_derive`'s arc rim (`crates/topo/src/pcurves.rs`, the
`arc_rim` closure) offers four `IsoArc` candidates to `probe_pick`:
each boundary row `v ∈ {v₀, v₁}` × each `u` direction, `v₀` first. On a
chart closed in `v` the two rows are one 3-D curve, so both rows' images
hold the carrier at the probe and the pick returns the `v₀` one by
order alone. That is the one case where the candidate ORDER, not the
geometry, decides the stored row. Both answers lie on the same 3-D
locus, so the arc-rim certification accepts either; what is unpinned is
which row the loop walk then sees, and whether it closes.

## Why no row in 4988

A chart closed in `v` with the rim running along `u` is a torus-like
spline wall: the rim is a circle along `u`, and `v` closes on itself.
`sweep::loft_body`, the door 4988's rows use, stacks sections, so its
`v` is open; 4988 did not find another door that builds one, and did
not survey them all. Building it needs a spline chart closed in `v` set on a face whose
rim edge is a circle along `u`, through a door that keeps the rim's
description coherent.

## Fix

A row on such a body: the rim derives its image, the mint stores it,
and the loop walk closes over it. If the `v₀`-first order makes the
walk refuse where the `v₁` row would close, the pick wants the loop's
own context (the neighbouring rows), not a fixed order.

## The question for Ev

PR 5137 fixed the seam's pick by giving each half the row on its own side, which stores two images for one seam edge. Its review and both designers on the branch-offset fork judged this the wrong home for the branch. C4 says a row is a function of the edge and the chart alone, and that a seam has one image and two joint elements; the kill-sum argument rests on that. The converged answer gives every closed channel its own period, so a spline net decided closed in v gets a v deck (`chart_v_period`, `Deck.v` in knot lengths, `kv` decided at the joint). The seam then keeps one image, and the arc-rim and `iso_schedule` picks take the first row that holds. That extends C4, so it waits for Ev on the same `[ev]` PR. PR 5137 is held until it is answered.

Ev then asked (on that PR) whether an angle type that compares modulo
the period would fix this class. Two more designers converged in three
rounds: two small types, a `Period` per periodic channel that the chart
owns (nets included) and an `Azimuth` exit from `atan2` read through a
stated reference, radians kept, stored coordinates never wrapped. They
also settled the open sub-point (a lifted image is the row plus its deck
element, never re-certified) and dropped the walk's estimate (every
decision's gap is within about two periods once images are placed).

## The answer

Ev took the recommendation on `[ev]` PR 5142 (2026-10-15, "i think this sounds good"): no cap; images placed over the edge's span in the principal window; a row is its placed image and a lifted copy is the (row, deck element) pair, never re-certified; the walk reads the half-period marks from zero; a `Period` per closed channel (a net's `v` included) owned by the chart, and an `Azimuth` exit from `atan2` through a stated reference. C4 carries it.

## Fixed (PCERT unit C, `pcert/net-v-period`)

The answer carried out, in place of PR 5137's per-side pick:
- A spline net decided closed in `v` has a `v` period, the knot
  domain's length (`chart_v_period`, one closure rule with `u`'s,
  `pcurve_chart_v_closed`), and the walk decides `Deck.v` on it by the
  half-period marks from zero. `Deck::apply` shifts a net's `v` by that
  length.
- The seam keeps one image. The arc rim (`geom_brep::pick_arc_rim`) and
  `iso_schedule` take the first row that holds, `v₀`; both halves of a
  seam circle derive it (`a_seam_circle_on_a_chart_closed_in_v_has_one_image`,
  on PR 5137's `seam_tube` fixture).
- The walk crossing a `v` seam by one period, the chart boundary's lift
  by it and validation are pinned on a net whose walk closes
  (`a_net_closed_in_v_lifts_its_seam_crossing_by_a_v_period`).

The candidate order still decides which row is the image; with one image
and the joint elements it no longer decides whether the walk closes.

`seam_tube` itself does not mint: its cap arcs' fitted images fold back
across the chart at the seam vertex, which is
`work/isotail/a-general-image-ending-on-a-closed-charts-seam-takes-its-foot-on-the-first-branch`.
