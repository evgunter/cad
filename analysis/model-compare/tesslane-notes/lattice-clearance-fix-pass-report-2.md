FIX PASS REPORT, message 2 of 4 — THE WHOLE PR BODY, part 1 of 3 (from the ===== line here to the ===== line closing message 4).

=====
# The trimmed lattice keeps clear of the boundary, and `tessellate` refuses a face whose triangle f64 cannot wind

Builds the two rules of `crates/mesh/README.md`: `[mesh.winding]` and, in `[mesh.sizing]`, "The lattice keeps clear of the boundary" with `trimmed::BOUNDARY_CLEARANCE` (1/16). Closes `a-lattice-point-within-rounding-of-a-trim-chord-point-cuts-a-triangle-of-no-area` and `the-stl-writers-doc-names-a-contract-nothing-states`.

Terms. A **trimmed face** is a curved face meshed in its chart: its boundary is the chord points it shares with its neighbours (the **trim polygon**, every loop), its interior vertices are a **lattice** (a rectangular grid of chart points sized from δ), and a constrained Delaunay triangulation joins them. A **cell bound** is the deviation the face's lattice was sized at, `UNPROVED_HEADROOM`·δ = δ/2 on an analytic chart, so a certificate may reach 2 cell bounds before the face refuses.

## Bodies that mesh on main and refuse here

**Nine cells of one body return `Ok` on main and refuse `UnresolvedWinding` at this head.** T1's cone (half-angle π/6, radius 1.2) joined to a box whose edge crosses the cone's axis (`xf = 0`): `s = 0.6` at δ 5·10⁻³ and 10⁻³, `s = 0.3` at δ 10⁻³, in A ∪ B, B ∪ A and A ∖ B. The refusal is the page's (`[mesh.winding]`: a face with an unresolved triangle refuses typed). What the triangles are, as both reviewers measured them on main's meshes: 36 triangles, each on the apex and two lattice nodes of one column, 8·10⁻¹⁸ to 2·10⁻¹⁷ high over sides of 0.2 to 0.5, which is 0.002 to 0.005 of an allowance. On main `reach_probes::geometry` read one of them as wound against its face: a noise normal reads either way. Pinned: `reach_probes::a_cone_face_met_on_its_axis_refuses_unresolved_at_fine_delta`.

**The class is wider than that body, and it is on the rectangle lane.** A later unit measured on this head: every cone face through its apex with a split straight side refuses `UnresolvedWinding` from δ 0.01 down (84 cells of plain cones; cones of half-angle 5° and 1° at every δ). These are rectangle-lane faces (`curved::grid_counts`' fallback lattice for a side that is not bare), which this PR does not change and did not census. So the claim this PR can make is scoped: **in the chart (trimmed) lane, no face that meshes on main refuses** (my A/B over 548 analytic and 703 spline faces at 1/16 on the base before PR 6965, and both reviewers' base/head differentials on the final base); **on the rectangle lane, faces that meshed with unresolved triangles now refuse**, the nine cells above among them. The cure is not here: the unit on `tesslane/cone-axis-winding` routes those faces to the chart lane, and `[ev]` PR 7046 carries the final state.

A reviewer's census of 1,349 body×δ cells, base against head: 21 answers differ, 12 `NoArea` → `Meshed` and those 9.

One cell refused under any clearance on the base before PR 6965: the funnel cut `h = 2·10⁻²` above its apex, the half above, δ 5·10⁻⁵ (1.31 cell bounds on main; 2.33 at 1/16 with 15 of 15,252 nodes not laid). On PR 6965's certificate and graded rows, merged here, it meshes under the clearance (`reach_probes::the_unit_funnel_cut_near_its_apex_meshes_at_every_delta`). The table below is the earlier base's.

## Measured first

Every trimmed face the `mesh` suite builds, and 42 bodies more (frusta and a funnel cut by a tilted plane, the hollowed vessel and elbow, C1, the donut and B2, the tilted pair, a rod and a turned bar, a tube on a ball, rods cut at tilt 0.5 and 1.1; six δ each), meshed twice in one process: with main's lattice and with the clearance, same polygon and retry loop. 548 distinct analytic trimmed faces with 826,099 lattice nodes; 703 distinct spline faces with 3,611,422.

| clearance | nodes not laid | faces touched | triangle count changes | worst certificate, cell bounds | faces main meshes that refuse |
|---|---|---|---|---|---|
| none | 0 | 0 | 0 | 1.81 | 0 |
| 1/16 | 2,123 | 304 | 236 | 1.81 | 2 (the cell above, two faces) |
| 1/8 | 4,074 | 387 | 289 | 1.93 | 2 (the same) |
| 1/4 | 7,984 | 400 | 312 | 1.98 | 7 |

At 1/4 the five more are plain cylinders cut by a steep plane, each a few percent past the headroom: `m5_pr11_trimmed::a_wide_tilted_cylinder_meshes_at_every_delta` at r = 5, δ 1.5·10⁻³ (1.52 → 2.04), and a rod of radius 0.5 cut through the origin by the plane of normal (sin 1.1, 0, cos 1.1), both halves, at δ 2·10⁻² (1.11 → 2.04) and 5·10⁻⁴ (1.37 → 2.14). A suite-only run at 3/8 and 1/2, each test stopping at its first red, adds refusals on five and six more `reach_probes` rows.

Where this differs from the design fork's table (`work/tesslane/log.md`, 2026-11-14): that table read 1.06 cell bounds with no clearance over the designers' 486 faces and no refusal at 1/4. The suite's own faces reach 1.81 on main (the funnel near its apex), 1.75 (T1's ringed cone), 1.52 (the r = 5 cylinder), 1.30 (the holed pipe).

**The spline arm takes the rule and nothing moves, because the rule is barely exercised there.** On all 703 spline faces no node is within 1/16, 1/8, 1/4 or 3/8 of its own step of a side or a crease line (2,007 are at 1/2). A reviewer measured why: 962 of 964 spline face×δ records have iso-line sides only, so their lattice sits whole steps from the boundary; neither reviewer could build an oblique-sided spline face. No mesh, digest or budget row of a spline face changes, and that is not evidence about an oblique spline boundary.

**The worst certificate, by three measures.** Mine above, over faces meshing both ways on the earlier base: 1.81 → 1.81 cell bounds at 1/16. The reviewers', on the final base: 1.52 → 1.63 over the mesh suite, and 1.49 → 1.68 over a 1,349-cell census.

**The two frusta** (`revolved[(0,0), (1,0), (r,1), (0,1)]`, r = 20ε), on main:
- Cut by the tilted plane, δ 10⁻⁴: 226 certified triangles with a lattice corner stand at or under ε (least 5.56·10⁻¹⁰ m, 4.4·10³ rounding allowances). As the log has it.
- Whole, δ 10⁻⁸: 81,792 triangles, 6,780 with an unresolved winding, all in the plane cap; none at 2·10⁻⁸, 10⁻⁷ or 10⁻⁶. As the log has it.
- Whole, δ 3·10⁻⁹: main meshes it (149,324 triangles, 22,756 unresolved in the cap). The log says the cone face refuses `CertificateExceeded` there.
- Whole, moved a metre off its axis, δ 10⁻⁸: 9,330 to 9,732 unresolved by direction, and 4,465 to 4,734 at 2·10⁻⁸. The log's 6,610 is what a scale of the largest |coordinate| counts (6,183 to 6,810 here). This PR reads the scale as `cert` already spells a position's (the corner's norm).
