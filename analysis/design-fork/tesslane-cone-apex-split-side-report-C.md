## For Ev

**Recommendation.** The defect is wider than a cone's apex, and its cure is one rule.

- **Final state:** a ruled face (a cylinder or cone wall) with a vertex on a straight side meshes exactly as the same face without it, plus one triangle per vertex. Confidence: likely.
- **Until that is built:** send exactly these faces to the chart lane, where they all mesh today, and delete the rectangle lane's fallback lattice. Confidence: sure that they mesh, likely on the route.
- **PR 7043's refusal stays** as the backstop. Confidence: sure.

**Terms.**
- **Ruled face**: a face on a cylinder or cone; its lines `u = const` are straight rulings.
- **Column**: the strip of face between two neighbouring rim chord points' rulings.
- **Strip**: the mesh of a ruled face with no interior points, every triangle inside one column.
- **Rectangle lane / chart lane**: the two meshers of curved faces (`mesh::curved`, `mesh::trimmed`).
- **Development**: the cone unrolled flat about its apex; the chart lane triangulates cones there.
- **Headroom**: the chart lane sizes at half of δ where it has no theorem (`UNPROVED_HEADROOM`).
- **Split side**: a straight side of the face that is two or more edges.

### Premise check: four corrections

1. **The class is every ruled face with a split side, not cone faces through an apex.** Measured on PR 7043's head, rulings split at the midpoint, obliquely placed, δ from 0.1 to 0.001:

   | face | today |
   |---|---|
   | cylinder wall (unit radius, full and half turn) | refuses `CertificateExceeded` from δ 0.01 down |
   | cylinder wall, radius 1, height 2 | refuses at every δ |
   | cone through its apex, 45° | refuses `UnresolvedWinding` from δ 0.01 down |
   | cone through its apex, 80° | also `CertificateExceeded` at δ 0.03 and 0.003 |
   | frustum (no apex) | meshes, on a lattice no theorem sizes: 1662 triangles against 184 unsplit at δ 0.001 |
   | sphere face, split meridian | meshes at every δ, two triangles more |

   The sphere escapes because a meridian is an arc and is chorded on the lattice rows. A straight edge gets no chord points. So the class is exactly the charts with straight sides.

2. **The zero-area triangles also appear on the face's unsplit side.** Once the fallback lattice is laid, the apex copy on the bare side fans down its neighbouring column too. The lattice is the trigger, not the split vertex.

3. **"About twice the triangles" for a face moved to the chart lane holds only at coarse δ.** On a ruled face the chart lane lays rows the face does not need, and the ratio grows as δ shrinks. Cone-face triangles, unit 45° cone, full turn:

   | δ | plain, rectangle lane | plain, chart lane | split, today | split, chart lane | split, strip held |
   |---|---|---|---|---|---|
   | 0.1 | 8 | 58 | 26 | 60 | 10 |
   | 0.01 | 24 | 604 | refuses | 606 | 26 |
   | 0.001 | 72 | 6168 | refuses | 6170 | 74 |

   The unit cylinder behaves the same (whole body, δ 0.001): 284 plain, 3338 split in the chart lane, 286 with the strip held.

4. **This mechanism was filed and closed.** `curved-lane-triangulates-in-raw-chart-coordinates` (closed 2026-10-21) named it and was closed on "no point off the lattice reaches this lane". A split vertex is such a point.

### What the code is missing

The rectangle lane's theorem for a ruled face is the strip argument: a triangle whose corners lie on two neighbouring rulings deviates no more than one rim chord's sagitta. Nothing holds triangles to that shape.

- **What makes the triangles** is a Delaunay triangulation of the raw chart. It knows no columns, compares radians with metres, and sees the apex as a row of distinct points.
- **The two agree only while every point is on the grid.** A vertex on a straight side is the one boundary point that is not.
- **Then Delaunay is free.** It fans that vertex across several columns (the cylinder: past δ, refused). With the fallback lattice it joins an apex copy to two nodes of one column (the cone: three points of one ruling, no area).

So the defect is the lane's triangulation: its theorem's premise is hoped for, not held. Confidence: sure.

- **Not the boundary.** The vertex is the body's, and a line needs no chords.
- **Not the lattice as such.** The fallback lattice is a workaround for the same missing premise.
- **The route**, only in that it sends the lane faces its theorem does not cover.
- **The division into two lanes is why it lasted.** The chart lane's header describes this exact triangle and cures it with the development. The rectangle lane triangulates the same chart and never got the cure.

### The final state

**A. The strip is held, not hoped for.** On a ruled chart, a face whose rims are iso runs lays no rows. Its triangulation is constrained so that no triangle crosses a column. The cone runs in its development.

- **What it makes true.** A straight edge is its own chord, so a vertex on it lies on an edge of the unsplit face's mesh. It divides that one triangle in its own plane: same surface, same bound, one more triangle. Splitting a straight edge can then never make a face refuse or change its density.
- **What it retires.** The row `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` closes on the theorem that already exists. The fallback lattice, `bare_sides` and the apex special case go.
- **Unrepresentable.** In the development three points of one ruling are exactly collinear, so no triangle lies along one; a held column edge stops any triangle spanning two.
- **Measured by a prototype** in the rectangle lane (one held edge per split side, no rows, about 40 lines). Every cell above meshes at every δ, at the unsplit count plus one per vertex, and passes `check_mesh` and the winding check. All 18 cells of the T1 row mesh. The four refusals pinned since issue 653's sweep mesh.
- **Where it lives: the chart lane**, as the ruled-chart half of unit 4b (a face bounded by iso runs sizes at δ). The raw chart cannot state the rule at an apex without hand cases; the development can. Confidence: likely. I did not prototype it there.
- **Reversible**: yes. It is a choice of triangulation under an unchanged certificate.

**B. Until A is in the chart lane: the route.** The rectangle lane takes a ruled face only when each straight side is one edge, read from the face's own loop. Every other ruled face goes to the chart lane.

- **What it makes true.** Each lane meshes only what its theorem or its named headroom covers. The rectangle lane's fallback branch is deleted and the lane asserts bare sides.
- **Measured.** Every cell above meshes and passes both checks, including the cylinder walls and all 18 T1 cells.
- **What it leaves.** These faces carry 20 to 80 times the triangles they need until A lands. Two shapes then refuse by name rather than by accident: a one-face full-turn wall with a split seam (`SeamTwice`), and a face on both nappes (`BothNappes`). I did not find a door that builds either.
- **Reversible**: yes.

**Not recommended.**
- **Every cone face to the chart lane.** Plain cones become 25 to 86 times denser, and cylinder walls still refuse.
- **Only faces through an apex.** This leaves the cylinder refusing and the frustum on its unproved lattice.

**A written first in the rectangle lane** (for faces without an apex) gives the final meshes sooner and is defensible. Choosing between it and B is sequencing, not design.

### What the kernel answers meanwhile

`UnresolvedWinding` is the right answer for a triangle f64 cannot wind, on any lane. With B beside it, no plain cone or cylinder refuses. If only one can land first, the typed refusal is better than main's mesh with noise for normals. Confidence: sure.

### What needs your word

1. **B, as I read it, does not.** It changes no ratified sentence. It moves only faces that refuse today or mesh on an arm with neither theorem nor headroom. That arm came in an agent's fix pass on PR 5717, not from you, and is out of line with `[mesh.sizing]`'s "a lane without its theorem keeps a named headroom". Say so if you count B as part of unit 6. Confidence: likely.
2. **One sentence for `[mesh.sizing]`**, which you reviewed in the S29 PR: *a lane's triangulation is constrained to what its theorem assumes; a premise is never left to what Delaunay does on a grid.* Its consequence could sit beside it: a vertex on a straight edge adds one triangle to each face beside it.
3. **Unit 6 stays yours, but ask it after A.** On today's numbers the chart lane cannot match the rectangle lane on any ruled face, so the unit's own rule would answer "stop at unit 4" for a reason A removes.

## For the orchestrator

**Errors in the brief**
- **Scope.** Fact 4 is true but understates the class: cylinder walls with a split ruling refuse `CertificateExceeded` on this head.
- **Fact 2.** The same triangles are also made on the unsplit side of the same face (dumped: apex copy at the bare side's `u`, two nodes of the last interior column).
- **Density.** "About twice" (also in the unit 4b and unit 6 rows) is a coarse-δ figure. Cylinder half body: 20 vs 32 at δ 0.1, 148 vs 1674 at 0.001.
- **Unit 4** (PR 7041) is not in the tree I was given; `lane_of` there still routes by carrier kinds.

**What I measured, and how**
- Debug profile, default ε. A probe test in `curved.rs` reused `split_each_edge_then_place`.
- Bodies: 45° cone (full, quarter, three-quarter), 80° cone (full, half), frustum (full, half), unit cylinder (full, half), the mirror-nappe fixture, ball, sphere band, and 0.2 rad cone and cylinder wedges with both rulings split.
- Scratch diff: `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork2/C/probe.diff`. Suite summaries are the `suite-*.txt` files beside it. The build directory is deleted.

**The route probe over-routes**
- Its predicate was "more than two line-carrier traversals in the outer loop of a cylinder or cone face". That also catches keyways, notches and slits.
- Seven pins moved for that reason alone: both keyway doors, the slit, the notched pipe wall, the one-vertex outer loop, the rim-free pole loop, and the D9 goldens.
- B's real predicate is "an iso-rectangle whose straight side is more than one edge", composed with unit 4's domain reading.

**Pins that move**
- **Under the route**: the T1 row; the split-side sweep's totals (246 meshed and 4 refused become 250 and 0); `a_split_then_placed_swept_face_is_not_refused` (a triangle-count pin).
- **Under the strip prototype**: the T1 row; the sweep totals; `a_cone_patch_takes_no_rows_at_any_column_count` (it pins the fallback's rows); eight D9 digests on wedge, axis_wedge, rounded_prism and washer. The digests move only because the prototype holds a diagonal on bare sides too; a real one would not.

**Defects off the question**
1. **Cylinder wall with a split ruling refuses `CertificateExceeded`.** No row names it that I found (grep of `work/` for split ruling and split side). It wants a P1 issue row under tesslane, same cause as the two cone rows.
2. **`curved::tests::TOTAL_REFUSED`'s comment is wrong.** It blames the four refusals on "the mirror nappe, whose split geometry exceeds the chord certificate". They are that fixture's cylinder wall (edges 0 and 6, both split patterns), refusing at every δ I ran.
3. **The closed row** `curved-lane-triangulates-in-raw-chart-coordinates` rests on a premise a split vertex falsifies. Reopen it, or cite it from the new row.

**Not checked**
- Main itself (only PR 7043's head); release profile; other ε.
- The both-nappes half of fact 5. The apex-crossing bowtie stores no lift, so under B it would refuse `DomainUnread` before `BothNappes`.
- Whether any door builds a one-face full-turn ruled wall (revolve's full turn builds two half faces).
- A inside the chart lane. Its ear and shared-side cures may add points to a strip's corner triangles.
- The raw-chart prototype at an apex face one column wide with both sides split: one pose passes, and I have no argument that it must.
- On the unit cylinders I read today's refusals for the midpoint split only; the two-point pattern was run under the route and the prototype, where it meshes.

**Provenance**
- `bare_sides` and the fallback arm: commit `04d634073ca`, the fix pass on PR 5717.
- `[mesh.sizing]`'s wording: commit `ff1f4265bb4`, the `[ev] S29` rewrite under Ev's review.
- I read no PR thread.
