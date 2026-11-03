---
id: a-closed-charts-seam-continuity-has-no-home
kind: issue
title: No statement of a closed chart's seam continuity exists beside crease_runs, so a tangent-continuous seam reads as a crease
status: open
opened: 2026-10-26
priority: P3
cost: E
refs: [the-side-of-a-spline-face-is-read-by-no-dihedral]
---



Filed by the SPLICE lane that built `geom_brep::dihedral_side` (PR
5764), on its re-review.

`KnotStructure::crease_runs` (`crates/geom-core/src/spline/knots.rs`,
~575) is the one statement of where a spline has no derivative inside
its domain. Nothing states whether a CLOSED chart (its two domain ends
meeting) is tangent continuous across the seam. So `dihedral_side`'s
`on_a_crease` (`crates/geom-brep/src/dihedral_side.rs`) escalates a foot
on any closed seam as `SideEscalation::Crease`, C¹ or not. That is
fail-safe, but it is an over-escalation, pinned as such by
`a_c1_seam_escalates_for_want_of_a_seam_continuity_reading`. The fixture
is a degree-2 chart closed in `v` whose end tangent runs along its start
tangent.

A revolved NURBS's seam edge is the realistic case: every point of a
rim on it would escalate. The fix is a seam-continuity reading beside
`crease_runs` (the end tangents' alignment, decided, per direction). The
door then reads a C¹ seam's one normal, and that row's assertion turns
into a side.

## The seam reading inherits it (SPLICE, 2026-10-27)

`geom_brep::classify_seam` reads a spline face's normal through the same
`chart_normal_at`, so a seam read on a closed chart's closure line (a wrap
edge's own reading, where both domain ends meet) escalates
`SeamEscalation::Crease`, which `topo`'s describer words as
`EdgeUndescribed { failure: Crease }`, C¹ closure or not. A rim arc's own
witness does not land there: its image (`Pcurve::IsoArc`) lies within one
period of the chart, so the middle of its interval is on the closure line
only where the edge is the closure row itself.

Moved from SPLICE to SPLICESEAM on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).

Moved from SPLICESEAM to SPLICE on 2026-11-02, by `git mv`, when SPLICESEAM merged back into SPLICE: the track budget now counts only the P0, P1 and unbanded rows a sitting dispatches (Ev, in chat, 2026-11-02), so the four tracks of SPLICE's 2026-10-28 split came in under one budget.

## A rim arc between a cylinder and a two-arc tube wall (SPLICE, 2026-11-03)

Filed by the SPLICE lane `splice/rim-arc-on-cylinder`. Its kernel row
(`crates/sweep/tests/a_rim_arc_on_a_cylinder_beside_a_spline_wall.rs`,
`a_rim_arc_beside_a_bent_spline_wall_is_taken_on_the_cylinder`) unions a
cylinder `r = 0.5`, `z ∈ [0, 1]` with a circle swept from its top along a
spine leaving along `+z`. Once the boolean takes the cap's rim arc on the
cylinder, the union stops at `EdgeUndescribed { edge: 10v1, failure:
Crease, cause: None, fitted: false }` at the default ε and at 1e-12. Edge
10v1 is the result's rim arc from `(−0.5, 0, 1)` to `(0.5, 0, 1)`, between
the cylinder wall and the tube's spline wall. The tube wall's `u` knots are
`[0, 0, 0, 0.5, 0.5, 1, 1, 1]`: two rational arcs, closed. The arc's ends
lie on the closure line and on the `u = 0.5` arc joint, and both are
tangent continuous. The describer reads either as a crease. That is the
seam-continuity reading this row asks for, and the interior-joint half
of `a-meeting-on-a-g1-arc-joint-of-a-spline-face-refuses-as-a-crease`. The
two walls meet tangentially along the arc, so the dihedral reading there
is also second order.
