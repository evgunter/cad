---
id: the-graph-separator-accepts-a-side-past-its-edges-piece-by-membership
kind: issue
title: Under the graph separator, a corner presentation's side along a cylinder's parallel may run past the edge's piece, where the graph does not vanish by construction, and membership accepts it
status: closed
opened: 2026-11-13
priority: P0
cost: M
closed: 2026-11-18
pr: 7080
branch: embed/graph-piece
---



## Finding

Found by CLEAR's lane B (`clear/own-separators`, PR 6905), and not
fixed there.

The graph separator vanishes on its one shared edge's piece by
construction, and only there. Under it, a `Zero::Side` from a corner
presentation on a cylinder's parallel can run past that piece. Past
the piece, the graph does not vanish by construction.

PR 6905 kept the existing "as now" handling, as its brief asked:
membership accepts that side. If the side really runs past the piece,
`sign_on` treats it as a zero side where `ψ` is not zero, and the sign
argument across it would be unproven.

No instance has been shown.

## What is needed

1. **Probe.** Find whether any presentation hands the graph separator
   a side that runs past its edge's piece:
   - build a fixture: a cylinder's parallel meeting a corner;
   - count such sides over the injective test suite and the tour walk.
2. **If none can occur,** argue it in `sign_on`'s doc and close.
3. **If one occurs,** re-band to P0, since a hole in the certificate's
   soundness takes the gate's band. Then make membership require the
   side's parameters to lie in the graph's piece, as the curve arm
   already does.

## Measured and landed (embed/graph-piece, PR 7080, 2026-11-17)

**It occurs.** A presentation does hand the graph separator a side past
its edge's piece, and membership accepted it.

- **Where.** `present_line_corner` reads a corner by its coordinate line,
  and its zero side runs over the cell's range widened to the vertex, so
  past the vertex onto the line's continuation. `present_plane_corner`
  does the same on a plane's line edge. The graph's piece is clipped to
  the edge's span.
- **The fixture.** `rim_corner`'s cylinder (the rim `v = ½` at `u ≤ 1`,
  the other edge leaving `(1, ½)` down and on toward `u > 1`) against a
  torus whose crest is the rim. The torus is tangent to the rim's
  plane, so the plane arm fails, and the carrier arm fails once the
  torus's cell passes `v = 0`. On 36 of 36 pairs that reached the
  graph, the cylinder's side ran 0.01–0.1 rad past the piece, and
  `sign_on` read it `Some(Positive)`. No pair went on to `Certified`:
  the torus's sign under the graph was `None`.
- **In the suites.** In topo's suite, 0 sides reached the graph (all 365
  readings were curves). In sweep's injectivity, cavity and embedding
  suites (77 rows), 876 corner sides reached it. A projection probe
  found those past the piece all on line edges. There the graph's
  extension, affine with the line's own slope, vanishes on all of the
  line, so main was sound on them, though nothing said so. No circle
  corner side was seen past its piece outside the fixture.
- **And a second, ulp-level instance.** `edge_piece` reads the piece off
  the presentations' image boxes, which can miss the stored carrier by
  the chart's rounding. In the filleted drum, the box at `u = π` has
  `y ≥ sin π ≈ 1.2e-16`, and the ruling is stored at `y = 0`. So a
  side over `(0, 1.7)` met a piece of `(0, 0.85)`. Filed as
  `edge-piece-misses-a-chart-sides-stored-carrier-by-the-charts-rounding`.

**What landed.**
- Every side presenter records the side's parameters on its one edge
  (`Along::params`): an affine map of the chart side through the use
  image's and span's ends (`iso_params`), or of a plane segment's
  coordinate (`segment_params`). Parameters are kept only for a whole
  side, whose chart line is its edge's own line or circle.
- `graph_of` folds a whole side's parameters into the piece, as it
  already folded a whole curve's, a circle's moved by whole turns. The
  graph is then the carrier's own graph over the side, and the advance
  check covers it.
- `sign_on` under the graph reads a side zero where its parameters lie
  in the piece. A spline side, with no parameters, is read zero where
  it lies on its edge (`Along::on_edges`, present_chart's coverage and
  the plane segment's span), as main read it; the filed row is what
  that leaves.

**Pins.** `a_corner_side_past_its_vertex_is_zero_under_the_graph_only_within_its_piece`,
`the_graphs_piece_holds_a_whole_sides_parameters_past_its_edge`,
`a_plane_side_is_zero_under_the_graph_only_within_its_piece` (topo), and
the drum and the lofts for the fold's turns and the spline reading.

**Mutants.** Membership alone reds the corner and plane pins. Without
the fold, the piece pin goes red. The chart map read off the wrong end
reds the corner and piece pins; the segment map read off the wrong end
reds the plane pin. Without the whole-turn shift, the filleted drum
refuses (`Uncertified`, Departure). Without the spline reading, the
triangle loft and editor-core's loft refuse.

**Is a wrong `Embedded` reachable?** I ran a bounded search with main's
reading restored as a local mutant. The cylinder corner was paired with
the faces of revolution sharing the rim as a parallel, varied over
radius or angle, cell width, corner lean, sense and side. The faces were
tori (36 poses), spheres (9 latitudes), cones (5 angles, both nappes)
and the coaxial cylinder; 23,544 pairs in all.
- **The graph certified 84 pairs through a corner side past its piece**
  (torus 12, sphere 3, cone 69). So the unproven reading did feed
  certificates, not just readings.
- **None was wrong.** These faces meet the cylinder only where their
  radius is 1. Inside each certified cell that is the rim alone: the
  torus's second crossing at `v = −v0`, and the sphere's at latitude
  `−φ`, lie outside every certified cell. And the cylinder's face
  excludes the rim's continuation past the vertex (`present_corner`'s
  step 3).
- **Why a wrong one needs more than this family.** The other face would
  have to meet the cylinder's face at a point whose column runs past the
  vertex. There ψ on the side is the extension's second-order deviation
  from the circle, against a first-order departure. I found no such
  configuration, but that is no proof.
- **With the fix,** the same search certifies 93 pairs through such
  sides: the fold makes ψ the circle's own graph over the side.

**The class, every separator.** A corner presentation's side runs past
its vertex under every separator, not only the graph.
- Under a plane it was read by membership alone. That was sound only
  because a plane listing the edge holds the edge's whole line or
  circle, and the corner's chart line is that line or circle.
- `sign_on` now requires it: a side that does not lie on its edges is
  zero under a plane only where the plane and the side are both whole.
- Pinned in the corner pin. The mutant (`None => true`) reds it.
- No verdict moved in sweep's 77 rows or editor-core's loft.

**The piece is stated in one place.** Membership reads `Psi::Graph`'s
`piece`, which `graph_of` alone builds, the folds included. A change
there (embed/conic-arcs) changes what every side and curve is measured
against.

**Fix pass after review (2026-11-18).**
- **One graph rule.** `Along::under_graph` is the one statement sides
  and curves are read by under the graph. It reads `Params` (the turn
  shift included), which curves now carry too; `Curved` lost its copy.
- **A side reaches its edges only without a run-on.** `on_edges` is now
  true only where the edges cover the side without `run_on`'s carry past
  a vertex. A spline side run on past its edge's end answers nothing
  under the graph. Nothing measured moved: sweep's 77 rows,
  editor-core's 24 `m10_5_clearance` rows and the tour are unchanged.
- **Affine channels checked.** `iso_params` reads parameters only off a
  moving channel with no cosine or sine term.
- **The image shift stays.** The review read `iso_params`' image shift
  as redundant with `Params::nearest`. Removing it refused the tour's
  `diecomposed` (`Uncertified`, Departure). Its side lies two turns off
  the use image's sheet, and the piece's one turn either way does not
  reach it. The shifts cover different gaps, and both are pinned.
- **Smaller fixes.** The plane segment presenters carry each point's
  parameter with it. `Along::over` asks for parameters only for a whole
  side. A collapsed side keeps its vertex. `sign_on`'s doc is a list,
  with the sliver bound.
- **Gate-run pins (topo):**
  - `a_corner_side_past_its_vertex_is_zero_under_the_graph_only_within_its_piece`:
    a non-whole corner side; the turn shift; the plane half;
  - `a_chart_side_run_on_past_its_edges_end_answers_nothing_under_the_graph`;
  - `a_sides_parameters_are_read_only_off_an_affine_moving_channel`:
    the affinity check; the image shift;
  - `a_plane_side_is_zero_under_the_graph_only_within_its_piece`, now
    with a two-segment corner.
- **Every mutant goes red on a row the gate runs:**
  - a corner presenter setting `on_edges`;
  - a non-whole side without `on_edges`;
  - no affinity check;
  - no image shift;
  - no turns in `Params::nearest`;
  - `run_on` counted as on the edge;
  - a segment's orientation swapped, in each presenter.
- **Tour:** identical to main on `da555737c`, all 85 bodies.
