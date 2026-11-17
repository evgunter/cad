## For Ev

**What moved.**
- I take one fact from the third analysis: its measured frustum rim counts. A cut-off cone's two rims carry different numbers of points from δ 0.03 down, so a rule that joins each rim point to "its partner at the same angle" on the other rim has nothing to join. A rule built on partners is out.
- I take one fact from the second analysis: a geometric argument that gives the strip rule its proof. I extend that argument below so that it also covers rims with unequal counts.
- I accept that the interim route goes by the rectangle lane's own admission test, and that faces with a split side are 20 to 500 times denser while they wait (slender cones are the worst case).
- I correct one claim of mine. The pass that places points along edges treats a straight side as needing none on both lanes, not only on the rectangle lane, and once no interior rows are laid that question is moot.

**What did not move.** The scope (every cylinder and cone wall), the home (the chart lane), the interim answer (send split-side faces there now), the backstop (`UnresolvedWinding`), and what needs your word all stand as in my second report.

**Where the three still differ, and where I stand.** The three analyses agree on what makes the mesh correct. They differ only on what holds the triangulation to it.
- The third analysis's form is: lay no rows, and hold one edge per split side. It meshed about 300 cells. But on an unsplit face it leaves the strip shape to what Delaunay happens to do in the development. That shape is measured there, not proved.
- That conflicts with the principle all three of us now propose for `[mesh.sizing]`: a premise is never left to what Delaunay does.
- So I keep a construction that makes the premise true. I also give the bound in a form that covers every variant, so the choice between them becomes a matter of how to build it, not of design.

**Terms.**
- **Ruled face**: a face on a cylinder or a cone. Its lines of constant angle are straight **rulings**, and on a cone they all pass through the apex.
- **Azimuth**: a point's angle round the axis.
- **Rim**: a circle edge of the face. A rim's chord points are the points it is cut into.
- **Split side**: a straight side carried by two or more edges.
- **Held edge**: an edge the triangulation is forced to keep (a constraint).
- **ρ_max**: the face's largest radius.

**The rule (likely).** This applies to a cylinder or cone face whose boundary is rims, rulings and possibly the apex.

**Bound (sure).** A triangle whose corners' azimuths all lie within a range Δ lies in the convex hull of the piece of surface between the two rulings at the ends of that range. So it deviates from the surface by at most the sagitta of an arc of radius ρ_max over the angle Δ (on a cone, the slant projection of that sagitta). That is the bound `sizing::cone_grid_step` and `sizing::sagitta_step` already hold to δ at the face's column step `h_u`. Corners at the apex, which has no azimuth, and split points, which sit on a side ruling, add nothing to the range. So the theorem is: **every triangle's azimuth range is at most `h_u`.**

**Construction (likely).** The face lays no interior rows. Its boundary rows are its two rims, or one rim and the apex as a row of one point.
- Merge the two rows in order of azimuth, the way two sorted lists are merged. Each step joins the current point of one row to the next point of the other: these joins are the **rungs**.
- Every rung is a held edge in the chart lane's constrained triangulation. A split point on a side lies on the first or last rung and divides that triangle in its own plane.
- Each triangle has two consecutive points of one row and one point of the other. So its azimuth range is at most the larger gap between consecutive points on either rim. Each rim is cut at the face's own `h_u` or finer, because a neighbouring face only ever asks for more points. So the range is at most `h_u`, whatever the two rims' counts.
- Consequences: splitting a straight edge adds exactly one triangle per vertex and can never make a face refuse. No triangle can lie along a single ruling. The issue `a-cone-face-with-a-split-side-has-no-worst-triangle-theorem` closes on the bound above.

**Relation to the other two forms.** On a face through its apex, and on a frustum whose rims have equal counts, holding the rungs gives the same mesh as the other two forms. Where the forms differ, this one alone is proved rather than measured. The cost is one held edge per column instead of one per split side, inside a triangulation the lane already runs.

**What would change my mind.** A proof that Delaunay in the development of two concentric arcs always keeps every triangle within one column. Then the third analysis's one held edge per split side is enough, and it is smaller.

## For the orchestrator

- **No build this round.** My answer rests on the bound's proof, which is a convex-hull argument, not on a measurement. The target directory is already deleted, and the tree is clean.
- **Taken unverified:** the third analysis's frustum rim counts, its run of the chart lane with no rows, and its slender-cone cells. Its full-cone densities are about half of mine because it used a different cone: mine was the T1 cone, radius 1.2 and slant 2.4; it used a unit 45° cone. Nothing rests on the difference.
- **Still unproved:** the bound on a cone triangle with two corners on one rim and the apex should be checked beside `cone_grid_step`. It is its existing fan triangle, so I expect no gap.
- **Unchecked for the build:** whether holding every rung interacts with the chart lane's ear cure and its inside/outside count (the third analysis flagged the same unknown for its single held edge).
- **Rows to file:** unchanged. The cylinder with a split ruling, the wrong comment on `TOTAL_REFUSED`, and reopening `curved-lane-triangulates-in-raw-chart-coordinates`.
