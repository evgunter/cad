---
id: an-analytic-edge-against-a-nurbs-face-has-no-crossing-lane
kind: issue
title: An analytic operand edge against the other operand's NURBS face refuses CurvedBooleanUnsupported at the crossing layer: the contact klein wall 3, lily wall 8 and teapot wall 3 reach once the edge gate is gone
status: closed
opened: 2026-10-26
closed: 2026-10-27
priority: P0
cost: H
refs: [delete-the-boolean-operand-edge-gate, nurbs-edge-crossing-rung-is-the-ring-composite, a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm, nurbs-plane-section-has-no-component-arm, 5723]
branch: splice/interior-arm
---


Filed by the SPLICE orchestrator, from the gate lane's measurement on
PR 5723 (`splice/gate`, the operand edge gate deleted).

## What

With `gate_operand_edges` gone, all three demo joins refuse **at their
contact**, and at the same site:
`BooleanError::CurvedBooleanUnsupported { operand: B, face, kind: Nurbs }`.
- **klein wall 3**: the bulb's outer neck-cylinder seam LINE (edge 2v1
  on face 4v1, r = 0.275, z ∈ [2.58, 3.0]), which ends on the z = 3 rim
  that the loop's top cap meets, is read against loop face 3v3. That
  face is one of the loop's four NURBS walls and rises from that rim.
- **lily wall 8**: the blade's base-section rim circle, against a
  sheath NURBS wall rising from the same rim.
- **teapot wall 3**: edge 9v1, a quarter MERIDIAN of the belly sphere
  (its revolve seam at azimuth π, in z = 0), against a spout NURBS
  wall. The spout's axis leaves the belly on that meridian. (Measured
  by PR 5723's review; the PR's first account called it a latitude.)

So the first thing each join needs is an analytic edge (line, circle)
of one operand read against a NURBS face of the other, and no crossing
layer reads a spline face. It is not
`nurbs-edge-crossing-rung-is-the-ring-composite`: that rung composes a
face's implicit residual along a NURBS EDGE (`spline::compose`, f ∘ C),
and a NURBS face has no implicit residual to compose. It is not
SECT's `a-nurbs-face-against-a-cone-or-sphere-has-no-section-arm`
either: that is the face × face section the klein and teapot pierces
need after the contact.

Lily wall 8 was this track's first demo witness on the premise that it
needs no curved section. It still needs this row.

## Open (designers first)

Where and how the crossing is found has more than one viable answer:
- a certified curve × NURBS-surface root lane in the crossing layer
  (the edge's points on the surface, by subdivision of the surface's
  net against the edge's carrier);
- the crossing read off the section of the edge's own faces with the
  NURBS face, as SECT's ratified section-first design reads chords
  (`work/sect/plan.md`, "The design"). C5's rung 3 has plane × NURBS,
  and has no cylinder × NURBS or sphere × NURBS arm;
- or something the designers find.

The contact here is also a coincidence: the edge lies on the rim the
NURBS wall rises from. Whether that coincidence is decided at the
crossing layer, or by the `[coincidence]` door before it, is part of
the question. One Opus and one Fable designer weigh it
(`memories/orchestration-model.md`) before a lane builds.

Reachable once PR 5723 merges.

## Weighed (2026-10-26)

Two designers, A and B, weighed it and converged (design-fork log row
198). The recommendation to Ev is to give C5 a curve × face clause:
- one exhaustive `(CurveKind, SurfaceKind)` meeting table in geom-brep,
  whose arms are the boolean's root doors;
- the converse spline-edge rung
  (`nurbs-edge-crossing-rung-is-the-ring-composite`) is another arm of
  that table;
- a span is read against the face's boundary edges first, so a shared
  rim is a coincidence glued at the coincidence door;
- the interior is decided by enclosure or a Krawczyk contraction;
- a NURBS face gets no signed residual, only a margined foot.

**Corrections to this row's premise**, both designers:
- The three walls are not one case. Lily 8's blade rim IS the sheath
  wall's boundary rim: a shared run, decided by the curve-pair table
  with no spline arithmetic. Klein 3's seam line ends on the wall's rim
  circle, with the wall tangent there (it leaves the rim straight up).
  Teapot 3 is a transverse pierce of the spout's wall by the belly's meridian seam,
  not a rim contact.
- Klein 3 is also held by the loop's 3.99e-4 rad cap tilt (the rims are
  up to 1.1e-4 m apart, not Zero at any ε the demos run), which goes
  with `carvetail/a-half-turn-spine-sweeps-only-off-its-exact-tangents`.

The pieces the design implies, each to be its own row once Ev rules:
the boundary-first span reading; the interior arm; point-in-face for a
NURBS chart (`contain::curved_interior_placement` answers `Trim(None)`
for `Nurbs`); a NURBS arm for the dihedral side reading (with the
SEAMRANK flush rows); and `CurvedBooleanUnsupported` carrying the edge.
Lily 8 needs only the first and the flush rows.

## Decided (2026-10-26)

Ev took the converged design on PR 5727 (fork log row 198). C5
(`crates/geom-brep/README.md`) now has the curve × face clause. This
row is now the table and its interior arm. An analytic span against a
spline face is settled by enclosure, or by a Krawczyk contraction on
the piece × cell. Whether that runs in (t, u, v) on C − S, or in
(u, v) on the curve's two implicits composed with S, is the lane's
choice. The pieces the design implies are their own rows:
- `a-span-is-read-against-a-faces-boundary-edges-first` (P0): the
  boundary-first reading. Lily wall 8 needs only it and the flush
  rows.
- `a-point-has-no-place-in-a-nurbs-faces-trim` (P1): point-in-face
  for a spline chart.
- `the-side-of-a-spline-face-is-read-by-no-dihedral` (P1): the
  dihedral side reading on a spline face.
- `curved-boolean-unsupported-names-no-edge` (P3): the refusal names
  its edge.

The converse arm (a spline span against an analytic face) is
`nurbs-edge-crossing-rung-is-the-ring-composite`, now an arm of the
same table. Build the two against one brief, or one after the other.

## Closed

Branch `splice/interior-arm`. The table's interior arm for an analytic
span (line, circle, ellipse) against a spline face (`Nurbs`, `Approx`)
is built: `geom_brep::curve_face::spline_face` (`isolate`, then
`settle`, composed as `NurbsLane::chart_meetings`). It runs on boxes of
`(t, u, v)` on `C(t) − S(u, v)`, clears a box only where `|F|` is decided
past the band, and proves one transverse meeting by the Krawczyk test
with `det J` definitely nonzero. The chart is cut at its creases, and a
box at the floor that no isolating box holds refuses. The boolean's
spline-face door reads it past the boundary pass
(`topo::boolean::reduce::spline_interior`). It places each meeting by
the trim at its own chart point
(`topo::boolean::spline_region::spline_chart_placement`), then records
the span's ends.

Witnesses: `geom-brep` `curve_face::spline_face::rows` (a line crossing
a bicubic patch once, a circle crossing it twice, a tangent line, an
in-band pass and a pass past the band, a meeting on a crease and one
beside it, a meeting at a span's end, a rational patch); `topo`
`spline_face_rows::a_line_pierces_a_spline_face_inside_its_trim_and_passes_it_outside`
and `a_chord_between_two_corners_of_a_spline_face_records_its_ends`;
`spline_region::rows::a_meetings_chart_point_is_placed_on_its_own_sheet`
(the meeting outside the trim, and the hairpin's other sheet); `sweep`
`m5_s11_concave_sense::a_lofted_operand_refuses_the_union_check_typed`.

The demo walls, measured:
- **Lily wall 8 does not move.** Its refusal is a NURBS tip seam of the
  blade against the sheath's ridge wall, `(Nurbs, Nurbs)`, which this
  arm does not read. Filed as
  `a-spline-span-against-a-spline-face-has-no-interior-arm`.
- **Klein wall 3 moves** to `SplineCrossingUnsettled { operand: A,
  edge: 4v1, face: 3v3, face_of: B, why: Crease }`. That is a meeting on
  the joint of the loop wall's two rational arcs. Filed as
  `a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease`.
- **Teapot wall 3 does not move.** It still refuses
  `PointSplitCarrierUnsupported` ahead of this door
  (`a-meeting-on-a-spline-boundary-edge-has-no-split-parameter`).

Also filed: `a-meeting-on-a-closed-charts-seam-is-not-isolated`.

