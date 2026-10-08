# CONTACT-12: the edge-on-face overlap lane cuts at boundary crossings

Row: `work/contact/overlap-lane-boundary-crossing-cuts` (issue 1500),
steps 1 and 3. Step 2, the `ef_bound_backed` region-confinement
migration, is dropped under the D10 hold: those arms are declared-pair
machinery that INTENT stage 4 retires, and the row parks that half on
`intent-stage4-is-built`. No declared rung is changed or re-baselined
here.

## What changed

**`ef_overlap_cells` cuts where the face's boundary crosses the edge.**
It used to cut an edge lying in a face's plane only at the face's
boundary vertices on the edge's line. Where the boundary crossed the
edge away from a vertex, one cell spanned the crossing and its single
midpoint answered for both sides of it. Now `boundary_crossings` reads
every boundary edge of the face on its own carrier
(`splitting::containment::carrier_loop`) and cuts:

- **A straight boundary edge** where its two ends lie definitely on
  opposite sides of the edge's line.
- **A circle or ellipse arc** at the roots of its carrier against the
  plane through the edge's line normal to the face. These come from
  `splitting::plane_crossing_lane`, the splitting lane's root-based
  reading.
- **A spiric arc** has no certified crossing position. The lane runs
  only where the edge definitely clears the arc, read piece by piece
  (new `SpiricArc::clears_segment`). Otherwise it refuses the face
  typed: `CensusUnsupported` / `Containment(Uncrossable { carrier:
  Spiric })`.
- **A spline arc**: the same, through the ball its control hull lies
  in.

Nothing is skipped silently. A crossing that cannot be decided escalates
(`CensusEscalated`) and abandons the pair's lane; it never leaves a cell
straddling a crossing. With a cut at every place the boundary meets the
edge, each cell lies inside or outside the face as a whole.

**A crossing bound takes the crossing's own rung.** A cell bound at a
straight-edge crossing is the pass-5 `EdgeEdgeCross` event, so
`ef_bound_backed` asks `ee_cross_backed` for it. That is the rung that
lane already reads, unchanged. A bound at a conic crossing has no rung,
because no census lane examines a line × conic crossing as an event. That
cell is an `UndeclaredContact`. The vertex arms of `ef_bound_backed` are
untouched and stay grandfathered.

**The touch analysis reads every cell.** `TouchSite::stars` returned the
stars at the overlap's FIRST cell only, so a site was read as resting
whenever that one cell rested. It now returns one star pair per cell.
`verdict` is a rest only when every cell reads as one; otherwise it
returns the first refusal.

## Each decide and the verdict it feeds

Every new decide reads a point's signed distance in metres
(`Margin::of`). None of them is levered.

| Row | Quantity | Verdict fed | Why the bound is sound for it |
|---|---|---|---|
| `pm_census_ef_cross_side` | signed distance of a straight boundary edge's end from the plane through the edge's line, normal to the face: its in-plane offset from the line | both ends definitely across → a crossing, so a cut; Zero at an end → no crossing through that edge's interior; same side → nothing; in band → escalate | A cut needs both ends more than the band off the line on opposite sides; continuity then puts a crossing strictly inside the boundary edge. A Zero end is a boundary vertex within ε of the line, which the vertex cut decides on its own row (`pm_census_ef_cut_gap`). Between that vertex and the true crossing, the boundary edge stays within ε of the line, so any probe there reads `OnEdge` or escalates. It never reads a definite `In` over a stretch outside the face. |
| `pm_census_ef_cross_span` | the crossing's arc length from each end of the edge | both Positive → a cut; Zero → it is the end's own cut; Negative → off the edge; in band → escalate | This is the position of a real point along the edge. An end within ε is already a cut. |
| `pm_census_ef_cross_screen` | a straight boundary edge's ends read along the edge, past either end | definitely past the same end on both → skip the edge | The only verdict is a skip. It is taken only when the boundary edge lies more than the band beyond an end, so no crossing of it can be inside the span. Anything else, including in band, goes on to the side row. |
| `pm_census_ef_cross_reach` | distance from a spline ball's centre to the closed edge segment, less the radius (a spiric arc: the same per subdivided piece) | Positive → the lane runs; otherwise → refuse typed | Clearance of a ball that holds the whole arc (or the piece) is a lower bound on the arc's distance from the edge. |
| splitting rows (`split_conic_*`) | conic graze `R − |D|`, root interiority metered in metres | roots → cuts (through the span row); miss → nothing; fault → escalate | They are the splitting lane's rows, reused as they are. |

## The every-cell finding

The touch analysis read an edge-in-face site at the first cell only.
That was a gap: the cells are separate pieces of the face, and no cell
answers for another. The row
`the_touch_analysis_reads_every_cell_of_an_edge_in_a_face` is an L-plate
whose edge rests on a slotted floor in two cells. The plate is tilted so
that the far end of its upright dips 12ε below the floor. The cell that
sees the upright reads the dip; the other cell, where the reflex corner
hides it, reads a rest. With the rest cell first, the old read called
the site a rest.

## Rows

ROWS_PLACEHOLDER

## Rows that moved

MOVED_PLACEHOLDER

## Sweep

Shape: a span cut at a set of points and judged at each cell's midpoint
through a containment door. Grep: `contfp(` / `contain(body` call sites,
then `midpoint` / `mid_param` / `* half` in `crates/topo/src`.

- `census.rs` `ef_overlap_cells`: fixed here.
- `census.rs` `pair_holds_point`, `pair_vertex_face`, the pierce point,
  and the v-on-f confirm: single points, not cells.
- `census.rs` `ee_collinear_lane` / `ee_overlap_midpoint`: collinear
  overlaps, where a crossing cannot fall inside a span.
- `chart_region.rs` decomposition-cell centres: hints that are
  certified at use.
- `boolean/reduce.rs` (three `contfp` sites): the boolean lane splits
  the edge at each crossing (`split_other_at_point`) before it probes.
- `boolean/ops.rs` `place_witness` and the sphere-section witness: single
  points. The latter is guarded by a near-boundary refusal.

Blind spot: a cell probe through a door other than `contfp` (a curved
face's chart placement, the point-in-solid door). A second pass over
`curved_face_placement` / `point_in_solid` callers found single-point
placements only.

## Findings outside the fence

FILED_PLACEHOLDER

## Battery

BATTERY_PLACEHOLDER

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL
