---
id: nurbs-edge-crossing-rung-is-the-ring-composite
kind: unit
title: a NURBS operand edge gets one crossing rung keyed on the carrier: the spline::compose ring composite
status: closed
opened: 2026-10-03
priority: P0
cost: H
refs: [3984]
closed: 2026-10-26
branch: splice/converse-arm
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

## An arm of the curve × face table (2026-10-26)

Ev took C5's curve × face clause on PR 5727 (fork log row 198). This
rung is that table's spline-span × analytic-face arm. The ring
composite is the arm's clearance test, and
`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane` is the
other column. Build the two against one brief, or one after the other.

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
  `wavy_loft` (`a = 0.3`), united through the
  recipe with a box over `x ∈ [1, 3]`, `z ∈ [−½, 3½]`;
- its face `x = 1` crosses each corner edge at `x = dx + 1` three times,
  the first and third with one sense;
- the row asserts those crossings are named through the boolean: no
  crossing is ranked (N2, *Vertices*, PR 5758), so the first and third,
  of one sense, tie.

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

## Lily wall 8 reaches this arm (2026-10-26)

Measured on `splice/boundary-first`: with the boundary pass reading the
shared rim, lily wall 8 refuses
`EdgeCarrierUnsupported { operand: A, edge: 3v1, face: 1v1, face_of: B, site: PlanarCrossing }`:
a NURBS tip seam of the blade, rising from the shared section, against
the sheath's base cap in that plane. The plane case of this arm is what
it needs next. `sweep`'s
`a_sheath_grafted_on_its_blade::a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`
pins the refusal in miniature and reds when this arm lands.

## Closed

The spline span × analytic face arm is
`geom_brep::curve_face::composite::crossings`, reached through
`NurbsLane::surface_crossings`. Its unit rows are in
`curve_face/composite/rows.rs`: clear, a pass within the zero band
inside the span refusing (an in-band pass is not clear: C5's "an in-band
decision escalating and never pruning"), one crossing, three crossings with the first and
third of one sense, a tangency refusing, a crossing on a knot, a meeting
at a crease refusing next to an ordinary crossing off it, an in-band
apex and an in-band dip at a crease and a crossing within the band of
one refusing as the crease's, an end on the
plane, a rational arc against a plane and a cylinder, an arc lying on
its cylinder refusing, and a non-rational cubic against a sphere
through the door. `curve_face::route` reads `(Nurbs, Plane)` as built.
The boolean's planar lane dispatches through it
(`reduce::spline_plane_crossings`). An unsettled span refuses
`BooleanError::SplineCrossingUnsettled`. The edge's box is its control
net's hull (`boxes::EdgeBoxRule::ControlHull`).

Witness rows:
- lily wall 8 now refuses `CurvedBooleanUnsupported { operand: B, kind: Nurbs }`.
  That is a blade edge against a sheath wall at the spline-face door
  (`an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane`). The
  miniature, `sweep`'s
  `a_sheath_grafted_on_its_blade::a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`,
  pins the same refusal in both operand orders, and pins the arm
  reading every blade seam against every sheath cap without a crossing.
- The end-to-end row: the boolean's sweep splits each wavy corner at
  its three crossings with the box's face (`sweep`'s
  `a_face_across_a_wavy_corner`), and the union stops one step on, at
  the sector walk. The naming row moves with that refusal to
  `the-sector-walk-reads-no-nurbs-face-at-a-crossing-vertex`, and
  `emit_topo`'s `a_union_across_a_nurbs_edge_refuses_at_the_sector_walk`
  is its tripwire.
- The far spline edge is pruned by its hull and no longer refuses at a
  pair that does not meet. The containment fallback's NURBS re-gate
  reads reach first (`s16_box_soundness::a_lofted_operand_reaches_the_fallbacks_re_gate`
  pins both halves). The vertex probe reads a point that stands
  definitely past the hull of every face's certified reach as being at
  infinity (`a_point_past_a_spline_solids_reach`, which also pins a box
  solid's centre as still refusing). The census `spline_spiric_operands`
  builds every op for the loft prism and the lofted cylinder against
  the far brick and the brick around each, at their closed-form
  volumes. A brick inside either refuses typed, at the probe or the
  spline-face door
  (`inside/point-containments-ray-test-has-no-spline-arm`).
- The plane rows in `topo`'s `planar_lane_carrier_rows` that refused an
  arc now split it at its closed-form crossings.

Left as their own items: the curved lane's dispatch
(`a-spline-span-against-a-curved-face-is-not-dispatched-to-the-composite`),
and the plane split's crossing reading
(`cleave/the-plane-split-reads-no-crossing-of-a-spline-edge`).
