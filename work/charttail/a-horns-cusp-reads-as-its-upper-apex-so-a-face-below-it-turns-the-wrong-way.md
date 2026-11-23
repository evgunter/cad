---
id: a-horns-cusp-reads-as-its-upper-apex-so-a-face-below-it-turns-the-wrong-way
kind: issue
title: singular_at reads a horn's cusp as SheetApexUp whichever side the face reaches it from, so a face below the cusp stores the complement turn
status: closed
opened: 2026-11-21
priority: P1
cost: M
refs: [singular-turn-spells-the-chart-winding-rule-by-hand, mesh-pole-junction-unwrap-picks-a-whole-turn-off-numbers]
branch: charttail/horn-cusp
pr: 7511
closed: 2026-11-23
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



## Confirmed and fixed (`charttail/horn-cusp`, 2026-11-23)

**The probe** (a throwaway row over `revolve`, read through
`Body::loop_lift` and `Body::joint`) confirmed the cause. On the lower
needle's lune (θ = 2) the joint into the meridian leaving the cusp stored
`Deck { u: 1 }`: the rim runs `u` 0 → 2 at `v = −π/2`, the meridian at
`u = 2` reaches the cusp at `v = −π`, and the lift put the leaving meridian
at `u = 2π`, the sector `(2, 2π)` where the face is `(0, 2)`. The full
lower needle's two faces each stored the same `u: 1` at the cusp. The
probe found a third face the row did not list: in "the horn's circle in
quarters" (part way), the quarter that reaches the cusp from below stored
`u: −1` there, the complement the other way. The quarter from above, and
the needle's lune from above, stored `0`. Every reading was `SheetApexUp`,
the first of the two apexes `singular_at` matched.

**The fix.** `singular_at` reads a vertex on either apex of a spindle
sheet as `Singular::SheetApex`, with no label, because the point does not
say which apex it is on a horn. The joint reads which apex off its own
chart ends (`apex_reached`): the sign of `v` at the arriving image's exit
and at the leaving image's entry, metred at the polar radius
(`pcurve_sheet_apex_copy`). A sheet's images lie in `[−v*, v*]`, so the
margin is `r·v*`. Two ends at different copies refuse `TwoNappes`, the
cone's refusal for a face that crosses its point. On a lemon or an apple
the ends agree with the point, so one rule serves every sheet. After the
fix every face above winds zero.

The mesh's own read of the copy, and `Chart::poles`' one listing of the
cusp, are filed on TESSLIE as
`the-mesh-names-a-horns-cusp-copy-off-a-chord-point-beside-the-stored-lift`.

## Review fixes (PR 7511, 2026-11-23)

The fix section above names `apex_reached` and `Singular::SheetApex`.
After review, the incidence reads `Singular::On(AnalyticPoint)`, whose
`SpindleApexes` holds no Up/Down label. One function, `face_above`,
reads the face's side for every analytic point. For a cone's apex it
reads the nappe each edge leaves on (`apex_nappe`) and refuses
`TwoNappes`. For a sheet's apex it reads the copy of `v` each chart end
holds (`apex_side`) and refuses the new `CuspCrossed`, or `CuspUnsided`
where an end's `v` decides zero.

A kill across a closed meridian through a horn's cusp
(`kev_describing`'s `turn_element`) is reachable through the doors. On
main it wrote a body tier 3 refuses (`LoopNotClosed`). It now refuses
the kill (`KillTurnUndecided`). The row is
`crates/sweep/tests/the_football_revolves.rs`
`a_kill_across_a_closed_meridian_through_a_horns_cusp_has_no_turn`.
