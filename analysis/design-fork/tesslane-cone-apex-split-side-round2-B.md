## For Ev

**Where I moved.** My first report recommended sending every cone face to the lane that meshes cones in their unrolled (flat) form, and carrying one rule over with them. Since then I have moved on three points. First, the defect is wider than cones: a cylinder wall with a split straight side fails as well. I re-ran this myself. Second, the final state is a rule both other reports reached: the mesher builds the shape its own size bound assumes instead of hoping for it. I state it a little more generally than they do (option A). Third, until that is built, I now favour routing exactly the affected faces to the other lane over relying on PR 7043's refusal alone. **Where I held.** Main's mesh with noise for normals must not come back, and the `UnresolvedWinding` refusal stays as the backstop. Sending every cone wholesale is not a final state, because the triangle counts I measured rule it out.

**Terms.**
- **Ruled face**: a face on a cylinder or cone. Its lines `u = const` are straight lines on the surface, called **rulings**. On a cone they all pass through the apex.
- **Rectangle lane** (`mesh::curved`): the mesher that triangulates a face whose domain is a rectangle of its chart, directly in the raw chart `(u, v)`. On a cone `u` is the angle round the axis and `v` the distance from the apex along the surface.
- **Chart lane** (`mesh::trimmed`): the mesher for every other curved face. It triangulates a cone in its **development**, the cone unrolled flat, where the apex is one point.
- **Split side**: a straight side of the face that the body carries as two or more edges, with a vertex part-way along. Booleans make these.
- **Column**: the strip of face between two neighbouring rulings that pass through boundary points.
- **Strip mesh**: a mesh of a ruled face with no interior points, in which no triangle spans more than one column.
- **Headroom**: the chart lane sizes at half of δ where it has no proof for its triangles beside the boundary (`UNPROVED_HEADROOM`).

**Recommendation (likely).** In the final state, a ruled face bounded by two rims (or a rim and the apex) and two rulings is meshed as a strip mesh, built by construction, and sized at δ by the bound that already exists. Until then, faces whose straight side is split go to the chart lane, and the rectangle lane's fallback lattice is deleted.

**What is missing (sure).** The rectangle lane's size bound for a ruled face is the strip argument: a triangle whose corners lie on two neighbouring rulings deviates from the surface by no more than one rim chord's sagitta. But a Delaunay triangulation of the raw chart produces the triangles, and nothing makes it respect that premise. It holds only while every point is on the rims.
- A split vertex is the one point off the rims, and two things go wrong from it.
  - **Cylinder:** Delaunay fans that vertex across several columns, and the face refuses `CertificateExceeded`.
  - **Cone:** the lane's fallback lattice, added to paper over the cylinder-style failure, lets an apex copy join two nodes of one column. All three lie on one ruling, so the triangle has no area and the face refuses `UnresolvedWinding`.
- The chart lane's own header describes the cone triangle and cures it with the development. The cylinder half has no cure anywhere.
- So the defect is the triangulation, not the lattice, the boundary or the routing.

**Measured this round (PR 7043 head, one rectangle-lane edge split at its midpoint, placed obliquely):**

| face | δ 0.1 | δ 0.01 | δ 0.001 |
|---|---|---|---|
| unit cylinder, full turn, split ruling | meshes | `CertificateExceeded` | `CertificateExceeded` |
| unit cylinder, half turn, split ruling | meshes | `CertificateExceeded` | `CertificateExceeded` |
| 45° cone through its apex, split ruling | meshes | `UnresolvedWinding` | `UnresolvedWinding` |

Splitting a rim instead meshes at every δ.

Density from my first round, cone triangles only, rectangle lane against chart lane:

| face | δ 0.01 | δ 0.001 |
|---|---|---|
| full cone | 26 against 1252 | 78 against 12710 |
| frustum | 46 against 398 | 140 against 3690 |

Both other designers measured the same order of magnitude independently.

**Options, each as a final state.**
- **A. The strip mesh is built by construction, on every ruled chart (recommended, likely).**
  - The rule: on a cylinder or cone face whose domain is two boundary rows (rims, or one rim and the apex) and two rulings, no interior points are laid, and no triangle spans more than one column. The columns are those of every boundary point's azimuth (its angle round the axis), on both rows.
  - Building it does not need Delaunay. Walk the two sorted rows together and join them, with the apex as a row of one point. A split vertex sits on an end ruling, so it divides the end triangle it lies on: one more triangle, same bound.
  - Stating it per column rather than as "each rim point is joined to its partner at the same angle on the other rim" also covers a frustum whose two rims carry different chord counts. Partners do not exist there, and that was the gap one of the other reports left open (confidence that the bound holds over a column of both rows' azimuths: likely, not yet written as a proof).
  - What it makes true: splitting a straight edge can never make a face refuse or change its density. The zero-area triangle cannot be built. Also retired: the open issue `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem`, the fallback lattice, `bare_sides`, and the apex special case. These faces need no headroom.
  - One of the other reports prototyped the constrained form in the rectangle lane: every cell meshes at the unsplit triangle count plus one per vertex. I take that unverified.
  - **Where it lives:** in the chart's table row ("ruled"). The chart lane uses it for a face whose domain is that rectangle, so the rule outlives the rectangle lane if unit 6 retires it. Writing it first in the rectangle lane is defensible; that choice is only about order.
  - Reversible: yes. The certificate is unchanged.
- **B. Every cone face goes to the chart lane as it is today.** Correct, but cones become 15 to 160 times denser, and cylinders still refuse. Rejected (sure).
- **C. Give the rectangle lane its own development, or cut split rulings into points at the row step, or drop the zero-area triangles.**
  - The first two leave the cylinder failing. Cutting rulings also relies on how Delaunay behaves in a chart that mixes radians with metres.
  - Dropping the triangles leaves points sitting on another triangle's edge (T-junctions), so the mesh is no longer watertight.
  - Rejected (sure).

**What the kernel answers until A lands (likely).**
- Route a cylinder or cone face whose straight side is carried by more than one edge to the chart lane. That is a fact about the face's own loop (its domain), not about what kind of curve bounds it.
- Delete the rectangle lane's fallback lattice, and make that lane assert its sides are bare.
- One other report measured every cell meshing under this route, cylinders included. I take that unverified, and I verified all cones of my first round under a broader route.
- The cost is unneeded density on those faces only, until A lands.
- `UnresolvedWinding` stays as the backstop on every lane.
- Two shapes then refuse by name rather than by accident: a face spanning both nappes of the cone (`BothNappes`) and a one-face full-turn wall with a split seam. No construction is known to build either.
- If the route cannot land first, PR 7043's refusal is still better than main.

**What needs your word.**
1. A sentence for `[mesh.sizing]`. Its "each lane has a theorem" wording dates from the S29 review you did. The proposed sentence: *a lane's triangulation is built to what its theorem assumes; a premise is never left to what Delaunay does.* A's ruled-strip bound would then be written there as the cylinder's and cone's theorem.
2. Unit 6 of the plan (moving every rectangle face to the chart lane) stays yours, but it should be asked after A. The plan's "about twice the triangles" figure is wrong for ruled faces and would answer "stop" for a reason A removes.
3. The interim route changes no ratified text. It removes a fallback that an agent's fix pass added (PR 5717), not one you asked for. My view is that it does not need your word; say if you count it as part of unit 6.

## For the orchestrator

- **Re-ran:** the split-ruling sweep on a full cylinder (radius 1, height 2), a half cylinder and a full 45° cone, splitting each edge at 0.5 through `curved::tests::split_each_edge_then_place`. Results are as tabled above: split rims mesh, split rulings refuse. This confirms the cylinder half of the class both other reports found.
- **Taken unverified:**
  - The strip prototype's counts (unsplit plus one per vertex) and its 18 T1 cells.
  - The route's "every cell meshes" for cylinders.
  - That the four refusals in `curved::tests::TOTAL_REFUSED` are the mirror-nappe fixture's cylinder wall, not a cone. The test's comment says otherwise and is wrong if so.
- **Disagreements:**
  - With one report, on scope: it routes only cones that hold their apex and have a split side. That leaves cylinders refusing. Its own cylinder finding argues for the wider route.
  - With both, on the strip's statement: "partner at the same azimuth" fails on rims carrying different chord counts. Per-column extent over both rows' azimuths does not. A written proof of the bound for a triangle whose azimuth extent is within one column on a cone would settle it either way.
  - What would change my mind on A: a ruled-rectangle face A cannot mesh within δ at the rectangle lane's step.
- **Errors in the brief:**
  - Scope: cylinders are in the class.
  - "About twice" holds only at coarse δ.
  - Unit 4 (PR 7041) is not in this tree.
- **Rows to file:**
  - A P1 issue for a cylinder wall with a split ruling refusing `CertificateExceeded`.
  - Fix `TOTAL_REFUSED`'s comment.
  - Reopen or cite `curved-lane-triangulates-in-raw-chart-coordinates`: it was closed on the premise "no point off the lattice reaches this lane", and a split vertex is such a point.
  - The chord pass skips straight edges on rectangle-lane faces while the fallback arm lays rows. This becomes moot once that arm is deleted.
- **Housekeeping:** tree reverted, target directory deleted.
