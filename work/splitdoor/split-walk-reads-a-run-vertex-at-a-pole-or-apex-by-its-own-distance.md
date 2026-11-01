---
id: split-walk-reads-a-run-vertex-at-a-pole-or-apex-by-its-own-distance
kind: issue
title: The split walk decides a run vertex at a sphere's pole or a cone's apex by two readings of its own, one of them quadratic near the pole
status: open
opened: 2026-10-30
---


Whether a vertex lies on its chart's singular point (a sphere's pole, a
cone's apex) has one reading,
`geom_brep::chart_singularity::vertex_at_singular_point`
(`chart_singularity_at_vertex`, the vertex's 3-D distance to the point).
The joint's reset (`topo::pcurves::singular_at`), `locate`'s at-vertex
placement and tier 3's wrap-tip rule read it. The split walk's chart
lift decides the same fact for a run's entry vertex with two readings of
its own (`crates/topo/src/chord_join.rs`, `run_azimuth_images`, near
line 3510):

- a cone: `bool_cone_apex_visit`, `|p − apex|` at the band. It is the
  same quantity, decided at a site of its own.
- a sphere: `split_sphere_window_pole`, `radius − |(p − c)·axis|`. Near
  a pole that is about `ρ²/2r` for a vertex `ρ` off the axis, so it is
  quadratic in the distance it stands for. On the unit sphere at
  ε = 1e-9, a vertex about 4.5e-5 from the pole decides Zero there and
  is lifted as a pole junction (`pole_branch`), while the one reading
  decides it clear of the pole. `docs/predicate-dimension-audit.md`
  rows it "m, OK", which reads the units and not the order.

**What it needs:** read the one incidence, at the walk's band. The
sphere's arm then decides `|p − pole|` against each pole and loses the
`ρ²` margin. `split_sphere_window_pole_side` (which pole it is) goes
away, because the reading answers per point.

Found by the RESTCURVE wrap-tip unit's sweep (branch
`restcurve/wrap-tip-tiers`); not fixed there.

Signed: (RESTCURVE implementer)
