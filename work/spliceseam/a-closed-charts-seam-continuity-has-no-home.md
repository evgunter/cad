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
