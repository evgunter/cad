# SELECT — the log

## 2026-10-17 — opened

Cut out of TANG by TANG's orchestrator on its priority seam (Ev, in
chat: split TANG into multiple programs). TANG measured 142.5 budget
points against 30 and handed its track back the same day; it keeps the
pinch machinery and the declared-tangency spine. Every row here moved
by `git mv` with its id, body and history unchanged. Rows: 11; 28.5
dispatchable points. — (TANG orchestrator)
- 2026-10-20 — **Seam (NURBS):** PR 5499 merged (`16b6b72f`, NURBS build C, TESS ground, FULL). **Lofted `Circle` sections now mesh.** The mesher's only door, `geom_brep::patch_bound::patch_cells_across_creases`, admits degree-≥2 creases on the rational arm; the integral arm, degree-1 creases and every other `patch_cells` caller (offset_fit, offset_meters) stay C¹-gated. A crease becomes a CDT constraint in `mesh::trimmed`. `geom_brep::sub_arc_joint(k, m)` is the one spelling of a sub-arc joint (shared by `sweep::skin::segment_curve`, `IsoArc` breaks and chord schedules; bits identical). An `IsoArc` rim with m ≥ 2 sub-arcs rounds its chord count to a multiple of m, so the joint is a chord point. A boundary crossing a crease off a chord point refuses `UnsupportedCurve`; creases in both directions refuse `UnsupportedNurbsFace`. No golden moved; the falsifier stays ≤ 0.4999 over 11 lofts × 20 δ. Closes `lofted-circle-sections-are-unmeshable-and-say-so-three-steps-late`. Still open: `work/tess/lofted-rational-walls-refuse-at-some-deltas` (P0, design), refusals away from the crease. (NURBS orchestrator)
- 2026-10-25 — **Band rule (work/README.md, Priority):** `edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short` re-banded P1→P0: OFFSET's conic unit 3 (`a-classified-pairs-section-states-the-bounds-that-retire-the-tube`, now P0 because the P0 `offset-lane-has-no-conic-carrier` waits on it) is parked on this row until `edge_extent` is a true bound on the arc. (band-rule triage)
