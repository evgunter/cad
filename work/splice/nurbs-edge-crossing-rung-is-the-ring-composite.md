---
id: nurbs-edge-crossing-rung-is-the-ring-composite
kind: unit
title: a NURBS operand edge gets one crossing rung keyed on the carrier: the spline::compose ring composite
status: open
opened: 2026-10-03
priority: P0
cost: H
refs: [3984]
---


Decided by the REACH orchestrator on the NURBS/spiric operand fork,
adopting both designers' reconciled reports
(`analysis/design-fork/nurbs-spiric-operands-d1` and `-d2`,
`design.md`, round 1). The sweep's two crossing arms refuse a NURBS
edge typed (`BooleanError::CrossingCarrierUnsupported`, PR 3984), and
`delete-the-boolean-operand-edge-gate` retires the body-scoped gate in
front of them; this rung is what then lets such an edge through.

## What to build

One crossing rung for a NURBS edge, keyed on the carrier, not one root
lane per face kind. `geom_core::spline::compose` already builds the
rational Bernstein composite `f ∘ C` of a plane, sphere, cylinder, cone
or torus residual along a NURBS curve, in certification arithmetic,
with certified coefficient hulls per span (it serves `ssi::certify`
today).

- **Clearance**: the numerator hull one-signed over the span (the
  denominator is positive by the weight invariant).
- **Roots**: knot insertion in certification arithmetic
  (`insert_knot_plan`, `apply_certified`) until a piece is one-signed
  (clear), or changes sign with a one-signed derivative hull
  (`derivative_coeffs`: one root, bisected on the residual and
  confirmed ON the surface), or shrinks into the band (a tangency,
  refused typed). It is the clear / monotone / split ladder
  `circle_roots::certified_subdivision` runs, with hulls for Taylor
  bounds.
- The plane case is the linear composite and replaces the planar
  lane's `Unlaned` refusal; the curved arm's the same.
- The edge's box becomes its control hull (`nurbs_curve_aabb`), the
  obligation `boxes::EdgeBoxRule::NoSoundBox`'s docs defer; the
  continuation scan's unreadable face extent retires with it.
- The join's germ frame and ring run and the vertex sector walk each
  need their NURBS arm too
  (`join-and-continuation-sites-blame-the-edge-gate-for-a-spline-edge`).

## Why it waits

Its first consumer is frontier (d)'s cylinder×sphere join window
(`join/cylinder-sphere-germ-pair-has-no-join-lane`; the pair's frame
landed in PR 4025, and the window is the lane), which will
mint fitted NURBS seams on analytic faces. From then on DESIGN's
"every boolean output is a legal boolean operand" (the maximal-faces
paragraph) is owed, and the gate must be gone by then. Before (d)
there is no caller: the corpus's STEP
files carry no NURBS edge, and every loft or sweep operand refuses on
a NURBS face first (d1's measured table). Build it with (d), as (d)'s
consumer-side half.

## Moved to SPLICE (2026-10-25)

From REACHHOLD. Moved by `git mv` in the chat-approved refold that opened SPLICE (`work/splice/program.md`), the track for a body built by loft or sweep entering every boolean. Re-banded P1 → P0 and UN-PARKED (was `blocked_on: [cylinder-sphere-germ-pair-has-no-join-lane]`). That park waited for a first PRODUCER of spline edges between faces, and every loft and sweep already is one: the klein loop's four longitudinal seams are `Curve3::Nurbs` edges between two NURBS walls (measured on main, 2026-10-25, `work/splice/delete-the-boolean-operand-edge-gate.md`), and once the gate is gone they cross the bulb's cone. Klein wall 3 and teapot wall 3 reach this rung.

## The end-to-end row this rung owes (2026-10-26)

From `delete-the-boolean-operand-edge-gate` (closed), whose emit note
(2026-10-07) reads: *"Retiring this gate, or the spline crossing-root
arm it waits on, owes an end-to-end row: a face crossing one NURBS edge
twice with one sense, named through the boolean."* The gate's deletion
could not write it: a spline edge claims no box, so the boolean refuses
the first spline edge it reads against the first face it reads,
wherever the face stands, and never reaches the crossings. This rung is
the first that can. The fixture stands ready:

- the wavy loft of `editor-core` `emit_topo`'s
  `nurbs_crossings_rank_by_parameter` (`a = 0.3`), united through the
  recipe with a box over `x ∈ [1, 3]`, `z ∈ [−½, 3½]`;
- its face `x = 1` crosses each corner edge at `x = dx + 1` three times,
  the first and third with one sense;
- the row asserts those crossings reach `emit_topo::rank_crossings`
  through the boolean, ranked by parameter.

`…::a_union_across_a_nurbs_edge_refuses_until_a_crossing_rung_builds_it`
is the tripwire that reds when this rung lands; replace it with that row.

## A far spline edge refuses naming a pair that does not meet (2026-10-26)

Measured on the gate's deletion (PR 5723's review). With the gate gone,
`EdgeBoxRule::NoSoundBox` is the de facto gate: a spline edge's box is
the whole space, so the sweep meets it with every plane face of the
other operand, and the planar arm refuses it there. The loft prism
against a brick at `[5, 6]³` refuses
`EdgeCarrierUnsupported { site: PlanarCrossing }` on face `1v1`, a pair
that does not meet. The recourse it gives, rebuilding the solid with
conic edges, cannot be acted on for any loft or sweep. The control-hull
box in "What to build" (`nurbs_curve_aabb`) is the fix: with it, a far
spline edge is pruned before any arm and a disjoint operand builds.
