# CONTACT-12: the edge-on-face overlap lane cuts at boundary crossings

Row: `work/contact/overlap-lane-boundary-crossing-cuts` (issue 1500),
steps 1 and 3. Step 2, the `ef_bound_backed` region-confinement
migration, is dropped under the D10 hold: those arms are declared-pair
machinery that INTENT stage 4 retires, and that half is parked as
`ef-bound-backed-migrates-to-region-confinement`. No declared rung is
changed or re-baselined here.

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
- **A circle or ellipse arc** at the roots of its whole carrier against
  the plane through the edge's line normal to the face
  (`splitting::conic_plane_candidates`, the carrier half of the
  splitting lane's reading, extracted from `conic_plane_meet` with no
  change to it). Each root is then placed on the ARC by the arc's own
  boundary reading (`LoopEdge::contact`, metres along the carrier). `On`
  is a cut. `End`, within the band of an arc end, is that end's vertex,
  which the vertex cut takes on its own row. `Carrier` is off the arc.
  The carrier parameter is never read against the span: a parameter gap
  times the smaller semi-axis understates an ellipse's arc length by up
  to the axis ratio. That was the review's MAJOR, and the splitting
  lane's own use of it is filed below.
- **A spiric arc** has no certified crossing position. The lane runs
  only where the edge definitely clears the arc, read piece by piece
  (new `SpiricArc::clears_segment`). Otherwise it refuses the face
  typed: `CensusUnsupported` / `Containment(Uncrossable { carrier:
  Spiric })`.
- **A spline arc**: the same, through the ball its control hull lies
  in.

Nothing is skipped silently. A crossing that cannot be decided escalates
(`CensusEscalated`) and abandons the pair's lane; it never leaves a cell
straddling a crossing. The one exception is a crossing whose position is
in band of the edge's own end. It escalates, is not cut, and the lane
goes on, exactly as the vertex cuts' span row does: the end's own cut
bounds that cell to within the band. With a cut at every place the
boundary meets the edge, each cell lies inside or outside the face as a
whole on a decided reading.

`boundary_crossings` is split per carrier (`edge_crossings`,
`chord_crossing`, `conic_crossings`, `span_interior`), and the
`Uncrossable` refusal is spelled once. Coincident cuts are ordered by a
fixed rank (`CutAt::rank`: vertex, then straight crossings by edge key,
then a conic crossing), so the kind of bound on either side of a
coincident group does not depend on the order the cuts were found in. A
point-to-segment distance now lives once, in
`sector_shape::point_segment_distance`. The census, `SpiricArc` and
`carrier_touch` share it; `ray_walk`'s generic version decides a
degenerate segment and is left alone.

**A crossing bound takes the crossing's own backing.** A cell bound at
a straight-edge crossing is the pass-5 `EdgeEdgeCross` event, so
`ef_bound_backed` backs it as that lane does: with an op's edge-edge
record (`Declared::ee_recorded`) or with `ee_cross_backed`. Neither is
changed. A bound at a conic crossing has no rung,
because no census lane examines a line × conic crossing as an event. That
cell is an `UndeclaredContact`. The vertex arms of `ef_bound_backed` are
untouched and stay grandfathered.

**The touch analysis reads every cell.** `TouchSite::stars` returned the
stars at the overlap's FIRST cell only, so a site was read as resting
whenever that one cell rested. It now returns one star pair per cell.
`verdict` is a rest only when every cell reads as one. Otherwise it
returns a decided `Crossing` at any cell over a refusal at another, and
failing that the first refusal. `stars` returns a head and a tail, so
there is no empty case to default.

## Each decide and the verdict it feeds

Every new decide reads a point's signed distance in metres
(`Margin::of`). None of them is levered.

| Row | Quantity | Verdict fed | Why the bound is sound for it |
|---|---|---|---|
| `pm_census_ef_cross_side` | signed distance of a straight boundary edge's end from the plane through the edge's line, normal to the face: its in-plane offset from the line | both ends definitely across → a crossing, so a cut; Zero at an end → no crossing through that edge's interior; same side → nothing; in band → escalate | A cut needs both ends more than the band off the line on opposite sides; continuity then puts a crossing strictly inside the boundary edge. A Zero end is a boundary vertex within ε of the line, which the vertex cut decides on its own row (`pm_census_ef_cut_gap`). Between that vertex and the true crossing, the boundary edge stays within ε of the line, so any probe there reads `OnEdge` or escalates. It never reads a definite `In` over a stretch outside the face. |
| `pm_census_ef_cross_span` | the crossing's arc length from each end of the edge | both Positive → a cut; Zero → it is the end's own cut; Negative → off the edge; in band → escalate | This is the position of a real point along the edge. An end within ε is already a cut. |
| `pm_census_ef_cross_screen` | a straight boundary edge's ends read along the edge, past either end | definitely past the same end on both → skip the edge | The only verdict is a skip. It is taken only when the boundary edge lies more than the band beyond an end, so no crossing of it can be inside the span. Anything else, including in band, goes on to the side row. |
| `pm_census_ef_cross_reach` | distance from a spline ball's centre to the closed edge segment, less the radius (a spiric arc: the same per subdivided piece) | Positive → the lane runs; otherwise → refuse typed | Clearance of a ball that holds the whole arc (or the piece) is a lower bound on the arc's distance from the edge. |
| splitting carrier rows (`split_conic_plane_parallel`, `split_conic_belly_graze`, `split_conic_graze_side`) | the conic's amplitude against the plane, and its graze `R − |D|`, in metres | roots → candidates; miss → nothing; fault → escalate | They are the splitting lane's carrier rows, unchanged. Its span row (`split_conic_crossing_root`, metered at the smaller semi-axis) is NOT read here. |
| `pm_census_ef_cross_arc_{on,end,trim,straddle}` (`LoopEdge::contact`) | a carrier root's distance from each arc end, and its side of the arc's trim, in metres | On → a cut (through the span row); End → the end's vertex (vertex cut); Carrier → off the arc; in band → escalate | Each is the arc's boundary reading in metres along the carrier, both semi-axes bounded (`a_steep_ellipse_reads_the_band_in_metres`). |

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

**The MAJOR, red first.** Both review witnesses are rows. They were
committed before the fix (`89889c2ec`) and run there red, then run green
on the fix (`65dcbf4fb`):

- `crossing_cuts::a_steep_ellipse_crossing_near_an_arc_end_is_cut`: the
  half-ellipse `a = 20, b = 1`, edge on `x = −19ε`. Red: no cells
  (`[]`). Green: one cell from `y ≈ −1` to `y ≈ 1`, both bounds
  `ConicCrossing`.
- `crossing_cuts::a_tilted_ellipse_crossing_near_an_arc_end_is_cut`: the
  plane `z = 20x` against the cylinder `x² + y² = 0.01`, with the arc
  ending `8ε` in parameter past each crossing. Red: `y ∈ [−0.05, 0.3]`
  gave no cells (`[]`). Green: one cell bounded at the crossing
  `y = 0.1`, and `y ∈ [0, 0.15]` gives one cell `[0, 0.1]` with no
  straddle.

**Mutants.** Every pinned decide was mutated on the final code; each
mutant reds the rows named, and every row is green unmutated.

| Mutant | Rows it reds |
|---|---|
| side row levered: `Margin::of(da·0.05)`, `(db·0.05)` | `a_crossing_is_cut_where_both_ends_lie_definitely_across` (a corner `12ε` across now reads Zero), `a_crossing_within_the_band_escalates` |
| side in band read as "no crossing" | `a_crossing_within_the_band_escalates` |
| span Zero taken as a cut | `a_crossing_is_cut_only_strictly_inside_the_span` |
| screen under-skips (`\|\|` → `&&` between the two ends' tests) | `a_boundary_edge_wholly_past_an_end_is_not_read` |
| screen over-skips (`&&` → `\|\|` within the start-end test) | `a_boundary_edge_reaching_past_an_end_is_read` |
| conic arm skipped | `a_conic_boundary_is_cut_at_its_roots`, both ellipse witnesses |
| spiric arm always clears | `a_spiric_boundary_refuses_where_it_meets_the_edge_and_clears_elsewhere` |
| `SpiricArc::clears_segment` ball radius halved | `a_spiric_arc_never_clears_a_segment_through_it` (257 segments through the arc) |
| spline arm always clears | `a_spline_boundary_refuses_where_its_ball_meets_the_edge` |
| touch verdict reads the first cell only | `the_touch_analysis_reads_every_cell_of_an_edge_in_a_face` |
| crossing cuts dropped (previous pass) | `a_boundary_crossing_away_from_any_vertex_bounds_the_cell`, `the_bare_lap_seat_reports_each_stretch_inside_its_face`, the touch row, `the_bare_straddle_seat_is_untouched` |
| crossing bound without the op's edge-edge record (previous pass) | `sweep` `join1_r2_rand::r2_random_zprism_pairs`, `join_pierce_runs_sweep::the_sweep_subset_ships_no_bad_body` |

Not pinned by a row: the `Crossing`-over-refusal choice across cells,
and the coincident-cut rank. Both are deterministic orderings with no
fixture that separates them yet.

`contact12_crossing_cuts::the_declared_lap_seat_certifies` is a control,
green both ways.

## What moved

- `mate4a_ef_bound_rung::the_bare_straddle_seat_is_untouched`, re-baselined.
  Two findings were added: each cap side edge rests under the shelf
  from its lower corner up to the shelf edge, `EdgeFaceOverlap` at
  `(0.6, 0.25, 0.5)` and `(0.3, 0.25, 0.5)`. These overlaps are real.
  The old lane missed them because the edges' midpoints (`y = 0.31`)
  lie outside the shelf. The shelf edge's own witness moved from `0.45`
  to `0.44999999999999996`: it is now the midpoint of the cell between
  the two crossings rather than of the whole edge.
- No declared rung moved. The declared straddle, overhang and lap seats
  certify as before. The anomaly pin
  `r2_an_unrelated_declared_pair_backs_the_ef_bound` is unchanged and
  green.
- **Configurations that now refuse or report where they did not:**
  - A spiric or spline boundary arc refuses (`Uncrossable`) any edge
    lying in its face's plane that it cannot clear, including one that
    only touches the arc, even at a shared vertex.
  - A line within the graze band of a conic rim now escalates on
    `split_conic_belly_graze`.
  - A line tangent to a hole's rim gets a `ConicCrossing` cut at the
    tangent point, so the edge reads as two cells. Each cell bound at
    that point has no rung, which leaves a declared seat there unbacked
    (`UndeclaredContact`).
  - A bound that used to sit at the edge's far endpoint, backed by the
    grandfathered vertex arms, can now sit at a crossing. There it is
    backed only as the crossing itself is backed.

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

- `work/contact/census-edge-pass-reads-no-line-conic-crossing.md`: pass 5
  sees only line edges, so a line crossing a coplanar conic boundary
  arc is no event of its own, and the overlap cell's bound there has no
  rung.
- `work/cleave/conic-plane-root-at-an-arc-end-reads-the-minor-meter.md`:
  the MAJOR's twin. The splitting lane's `split_conic_crossing_root`
  reads a root at an arc end at the smaller semi-axis, so a steep
  ellipse's crossing within `(major/minor)·ε` of an end is probably
  neither swept nor inserted. The census fix does not touch that lane:
  `conic_plane_meet` is only split into its carrier and span halves, and
  every splitting row stays green.
- `work/contact/edge-face-crossing-cut-and-pass-five-decide-one-crossing-twice.md`
  (P3): a straight crossing is decided by the cut's side rows and by
  pass 5's `ee_gap`/`ee_span`, and nothing ties the two.
- `EF_CROSS_ROWS`: `BoundaryRows` has no optional fields, so its `line`
  and `spiric` rows, which this lane never reads, carry one name
  (`pm_census_ef_cross_unread`). The conic rows are all read.
- Not filed (pre-existing, tracked):
  `parallel_cylinder_join::a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings`
  is red at `1e-12`, as on main
  (`work/tint/tipped-rod-join-escalates-at-1e-12.md`).

## Battery

All rows were run locally on `0b78a0c35`, the commit before this
body's own (the body is the only change since), with
`CARGO_TARGET_DIR` set to this lane's own directory.

| Check | Result |
|---|---|
| `cargo fmt --all --check` | ok |
| topo + sweep, default ε (1e-9) | 5091 run, 5091 passed |
| topo + sweep, `CAD_TOLERANCE_EPS=1e-6` | 5091 run, 5091 passed |
| topo + sweep, `CAD_TOLERANCE_EPS=1e-12` | 5091 run, 5090 passed. 1 red: `parallel_cylinder_join::a_tipped_rod_whose_origin_is_stored_far_joins_along_its_rulings`, which is pre-existing and tracked (`work/tint/tipped-rod-join-escalates-at-1e-12.md`); its boolean refuses before the census runs |
| editor-core, all (slow set included) | 2910 run, 2910 passed |
| test-utils | 89 run, 89 passed |
| clippy `--workspace --exclude viewer --all-targets --all-features -D warnings` | ok |
| clippy `-p pncad-py --features python` | ok |
| `scripts/gates/*.sh`, payload-rung sweep, `work.py lint` | ok |
| python lint (`check-python-lint.py`) | SKIPPED locally: this box has ruff 0.15.8 and CI pins 0.16.1. The diff touches no Python file. |
| Python suite (maturin wheel, unittest) | 950 tests, OK |

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01HkgsMyrV52i5fDxhA2ojxL
