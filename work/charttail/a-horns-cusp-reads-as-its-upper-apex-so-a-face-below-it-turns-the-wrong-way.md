---
id: a-horns-cusp-reads-as-its-upper-apex-so-a-face-below-it-turns-the-wrong-way
kind: issue
title: singular_at reads a horn's cusp as SheetApexUp whichever side the face reaches it from, so a face below the cusp stores the complement turn
status: open
opened: 2026-11-21
priority: P1
cost: M
refs: [singular-turn-spells-the-chart-winding-rule-by-hand, mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers]
---

Found by TESSLIE's lift-turns unit (2026-11-21), which instrumented
`mesh::walk`'s whole-turn reads beside `topo::Body::loop_lift` over the
mesh and sweep suites.

**The measurement.** On two rows of
`crates/sweep/tests/the_football_revolves.rs`, the stored lift of a
curved face's outer loop **winds the chart once**
(`Lift::winding() == Some(Winding { deck: Deck { u: 1, v: 0, twin: false } })`):
- `a_horns_cusp_revolved_part_way_is_its_lune`, "the lower needle"
  revolved through θ = 2: loop rim (u 0→2 at v = −π/2), meridian at
  u = 2 down to the horn's cusp at v = −π, meridian at u = 0 back up.
  The lift puts the second meridian at u = 2π, the sector `(2, 2π)`
  (4.28 rad), where the face is `(0, 2)`; the rim's lifted exit (8.28)
  does not meet the first meridian's entry (2.0).
- `the_wide_apples_and_the_lower_needle_mesh_and_measure`, "the lower
  needle" (full revolve split at π): the same shape, the sector π the
  wrong way.

`mesh::walk` meshes both faces right (both rows check volume), because the
curved lane walks its own columns. A reader of the lift does not:
`Body::face_lift` refuses `OuterOpen` on such a face, and any lane that
reads a column off the lift there takes the complement.

**The probable cause** (read from the code; not yet confirmed with a
probe): `pcurves::decision::singular_at` returns the FIRST singular point
a vertex lies on (`for (which, point) in singular_points(..) … Ok(true) =>
return Singular::On(which)`). On a horn, `chart_singularity::sheet_apexes`
states `SheetApexUp` and `SheetApexDown` at the same point, the one cusp,
so a cusp vertex always reads `SheetApexUp`. `singular_turn` then takes
`above = false` (`SheetApexUp => false`) and steps the azimuth as for a
face lying BELOW the cusp in `v`. A face reaching the cusp from below at
`v = −π` (the lower needle) lies above it, so its step has the wrong sign
and the distinct-edges arm decides the complement branch.

**What closing it takes.** Confirm with a probe: the joint element at the
cusp on the lower needle lune, against the face's sector. Then decide the
horn cusp's side from the face (the copy of `v` its edges reach the cusp
at, as `mesh::walk` does with `vp.copysign(chart.v_of(..))`), not from
the point. Pin both football rows' lifts at winding zero.


