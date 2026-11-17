## For Ev

**Recommendation (likely).** Mesh every cone face in one place, in the cone's *development*: the cone unrolled flat about its apex, where the apex is one point and each ruling (a straight line of the cone through the apex) is a straight ray. Delete the raw-chart cone handling in the *rectangle lane* (`mesh::curved`, the lane that triangulates rectangle faces directly in the chart `(u, v)`). One thing has to move with the cones. The rectangle lane knows a fact about cones that lets it skip interior rows: the *strip argument*. When neither side of a cone face has a point between the face's top and bottom edges, every triangle stays inside one column, so rows would add triangles and tighten no bound. That fact has to become a property of the cone's chart, read by the *chart lane* (`mesh::trimmed`, which already triangulates cones in the development). Moving the faces without it is correct, but it multiplies cone triangle counts by 14 to 160, measured below. Until this lands, the right answer for these faces is PR 7043's typed refusal, `UnresolvedWinding`, unchanged.

**Premise check (sure).** The problem statement is right about the symptom and needs one correction about the cause.
- I reproduced the bad triangles on `reach_probes`' T1 pose by logging the rectangle lane's emitted triangles. Each one has an apex copy at the face's side, `u = π` or `u = 2π`, plus two lattice nodes on the first interior column, for example `(3.2289, −0.2087)` and `(3.2289, −0.3130)`. Their cross products are about 1e-18 on sides of about 0.2. In space, all three corners lie on one ruling, so the triangle has no area.
- The cause is not the lattice and not the split. It is the plane the triangulation runs in. In the raw chart the apex is a whole row, entered once at each azimuth where the face meets it. The lane's only test for a collapsed triangle compares mesh ids ("two corners with the same id"). Three distinct ids on one line in space look like a real triangle in the chart.
- With bare sides the lane lays no interior lattice, so every triangle's other two corners sit on the rim at different `u`, and the shape cannot occur. A split side switches the lattice on, and with it the shape.
- The chart lane's own module notes (`trimmed.rs`, section "The development") describe this exact failure as the reason the development exists. The tree already knows the cure. Only one of the two cone triangulations has it.
- A second, smaller defect (sure). For a rectangle-lane face, the pass that sets how many points each edge gets (`chords::tighten`, via `FaceLattice`) skips every straight edge. It treats a ruling as needing no points ("an infinite step along a ruling"). But the split-side cone handling in `curved::grid_counts` lays rows. So the split side reaches the triangulation as one long unbroken edge, and the triangulation fans the apex all the way down the next column. That breaks `[mesh.sizing]`'s rule that edges are cut at the face's grid step. Fixing it would make the bad fan unlikely, not impossible: it would rest on how Delaunay triangulation behaves in a chart that mixes radians with metres.

**Whose defect (likely).** The split between the two lanes. The kernel has two triangulations of the cone, and only one of them runs in a plane where the apex is a single point. The routing rule (`tessellate::lane_of`) is not the fault in itself: it sends these faces to a lane whose plane cannot represent the apex.

**The options, each as a final state.**
- **A. Every cone face is meshed in the development, and the cone's chart rules when a face needs interior rows (recommended).**
  - Makes true: no triangle has zero area by construction. In the development, three points on one ruling are exactly collinear and the triangulation never makes a triangle from them. Three points of a cone on one line always lie on a ruling, so no other zero-area triangle exists.
  - Removes: the rectangle lane's cone code (its branch in `grid_counts`, `bare_sides`, the apex handling) and the open issue `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`. The split-side face becomes an ordinary chart-lane face whose sides run along chart lines (unit 4b's case).
  - Still open: the strip argument has to be restated in the development. It is proved today in the raw chart, where the bottom and top rims are cut at the same `u` spacing. In the development the inner rim's chords are shorter (confidence that it carries over: unsure). Very thin triangles beside a ruling on needle cones remain possible; that is already filed, and `[mesh.winding]` permits thin triangles as long as their winding is resolved.
  - Reversible: yes, it is a routing change plus one rule on the cone's chart.
- **B. Send every cone face to the chart lane as it stands.**
  - Correct: all the reported cells mesh. Measured with a temporary route: every cone mesh passes the winding check.
  - Too dense. The chart lane works at half of δ (its `UNPROVED_HEADROOM`, kept until a theorem covers its boundary triangles) and always lays graded rows. Cone triangles, rectangle lane → chart lane:

    | face | δ 0.1 | δ 0.01 | δ 0.001 |
    |---|---|---|---|
    | full cone | 8 → 112 | 26 → 1252 | 78 → 12710 |
    | frustum (cone cut flat at the top) | 16 → 36 | 46 → 398 | 140 → 3690 |
    | 80° cone | 14 → 42 | 40 → 364 | 122 → 3394 |

    The plan's "about twice the triangles" does not hold for cones. This breaks `[mesh.sizing]`'s "as coarse as possible without extra work". Not a final state; it is A without the rule about rows.
- **C. Give the rectangle lane its own development.** Two copies of the development and of the apex rule, in a lane the plan retires anyway. Rejected (sure).
- **D. Cut the split ruling at the row step (fix the second defect only).** Not impossible by construction (above). Worth doing only if the rectangle lane were to stay. Rejected as the cure (likely).
- **E. Drop the zero-area triangles, as the lane already drops collapsed ones at the apex.** The column nodes lie on the long edge from the apex that stays in the mesh, so the mesh would have points sitting on another triangle's edge (T-junctions) and would no longer be watertight. Rejected (sure).

**What to answer until A lands (likely).** PR 7043's `UnresolvedWinding` refusal, with no new code.
- Main returns a mesh whose normals are noise, which violates the ratified `[mesh.winding]`. It should not come back.
- A new refusal naming the cause ("cone through its apex with a split side") would be a special case that is deleted when A lands, so it earns no place.
- One optional step that is defensible: route only the cone faces with a point on a side (the ones `bare_sides` already singles out) to the chart lane now. The rectangle lane has no theorem for them anyway, all 84 cells then mesh, and their density moves about 2× rather than ×14–160. Its cost is that it routes by a property of the boundary, which the plan rejects in its final state. If it is done, it is a temporary bridge, not where things end.

**What needs your word.**
- A. The plan (`work/tesslane/plan.md`, unit 6) names moving rectangle faces to the chart lane as your call. That text was written by the agents in commit f31f71b, not in your words.
- A also rewords `[mesh.sizing]`'s chord bullet. It currently names "the curved lane" and "the trimmed lane's cone".
- The strip rule becomes a new theorem attached to the cone's chart.
- The plan's density figure ("about twice") is wrong for cones, so unit 6's measurement should be made per chart.
- The PR 7043 interim and the optional bridge change nothing ratified.

## For the orchestrator

- **Measured:** triangle corners on the T1 pose, through a temporary log line in `curved.rs`'s emit loop (36 hits across the test). Cone-only triangle counts on a full cone, half cone, frustum and 80° cone at δ 0.1, 0.01 and 0.001, rectangle lane against an env-gated route of cones to the chart lane. All reverted; target directory deleted.
- **Not reproduced:** the 84-cell sweep and item 7's "all mesh" over split cones. I measured plain cones only. The density table above is for bare cones, the case the statement did not cover.
- **Brief correction:** "about twice" (plan, unit 6) is badly wrong for cones. The chart lane lays a full graded lattice where the rectangle lane's strip argument lays none.
- **Item 5, faces spanning both nappes:** the chart lane refuses these with `SphereTrimShape { BothNappes }` (`trimmed::singular_row_shape`). If those faces are in the 84 cells, route-everything does not make them mesh. Unchecked. Whether such a face should exist at all, with the apex as a pinch vertex, is a modelling question for `DESIGN.md`'s rule that a chart singularity is a vertex of every face whose closure holds it.
- **Off-question defect to file (TESSLANE):** `chords::tighten` skips straight edges on rectangle-lane faces, while `grid_counts`' split-side cone handling lays rows every `ρ_max·hu`. The ruling is never cut at that step. Moot if A lands.
- The row `a-cone-face-met-on-its-axis-meshes-a-triangle-wound-against-it` lists "which triangle it is" as unmeasured. The corners above answer it.
- Unchecked risk: needle cones in the chart lane (`a-needle-cone-face-meshes-thin-triangles-beside-its-rulings`) may now refuse `UnresolvedWinding` on thin triangles.
