## For Ev

**What moved.** I now back the third form: write the strip down by walking the two boundary rows, with no Delaunay step. My round-two form (no rows, plus one held edge per split side) and the edge-held-per-column form both steer a Delaunay triangulation toward a shape the walk simply writes. Mine also left the unsplit part of the face to "Delaunay makes the strip unaided", which I measured but did not prove. That contradicts the sentence all three reports want in `[mesh.sizing]`: a premise is never left to what Delaunay does.

**What did not move.** Scope (every cylinder and cone wall), home (the chart lane, as a fact on the chart's table row), the interim answer (route split-side faces to the chart lane now, delete the fallback lattice, keep PR 7043's refusal as backstop) and the list of what needs your word.

**The rule (likely).** On a cylinder or cone, take a face bounded by two rows (two rims, or one rim and the apex) and two straight sides.

- **No search.** Its triangles are written down, with no interior points and no triangulation search.
- **The walk.** Sort each row's boundary points by angle round the axis; the apex is a row of one point. Start at one side with the edge joining the two rows' first points. At each step, advance on whichever row's next point comes first in angle, and emit the triangle on the current cross edge and that point.
- **A vertex on a straight side** divides the one triangle that side is an edge of, fanned from that triangle's third corner. On a face through its apex that corner is a rim point, never the apex.
- **A face one column wide with vertices on both sides** walks its two sides the same way.

**Why it is within δ.**
- Every triangle spans at most one rim chord in angle, by a four-line proof in the orchestrator section. That is exactly what the existing cone and cylinder bounds need, so the face sizes at δ with no headroom (the chart lane's half-of-δ margin where it has no proof).
- Triangles made by a side vertex lie in the plane of the triangle they divide, so they add nothing to the bound.

**What it makes unrepresentable.**
- A triangle along one ruling (a straight line of the surface): every triangle has two corners at different angles on one row, or two on a side plus a corner off that side.
- A triangle spanning two columns.
- The two apex copies of the raw chart, and with them the non-manifold fan the lane's pole-column floor guards against.

**Why this form over the other two.**
- **It has a proof that holds in any plane.** It needs no development, no chart metric and no inside/outside count for these faces.
- **It covers a frustum whose rims carry different chord counts.** I measured 11 against 9 points at δ 0.03 on a half-turn frustum. "Each rim point's partner" does not exist there.
- **It cannot fail to place a held edge.** There is none to place.

**What is not measured.** Nobody has built the walk. My round-two prototype (held edges in the rectangle lane) emits the same triangle counts the walk would: the unsplit count plus one per vertex, on every split cone, frustum, cylinder and slender-cone cell, at every δ from 0.1 to 0.001. On the 18 cells of the T1 pose it passed the outward and true-distance checks. That is evidence for the shape, not for the walk's code, which is why this is likely and not sure.

**What would change my mind.** A ruled rectangle face whose row is not a single run sorted in angle, or a walk triangle that fails its certificate at the rectangle lane's step.

**One thing to notice before agreeing.** The chart lane then holds two ways to make triangles: the lattice and search for every other face, and the walk for a rectangle on a ruled chart. The second is chosen by the face's domain and the chart's row, not by what kind of curve bounds the face. I think it earns its place: it is the only form that gives δ with no headroom, the fewest triangles, and a proof. The plan's "one lane" survives as one reader of the domain, one chart table, one certificate and one join.

**For `[mesh.sizing]`** I would now take the other report's wording: *a lane's triangulation is built to what its theorem assumes; a premise is never left to what Delaunay does.*

## For the orchestrator

**The proof the reports lacked** (one of them rated it "likely, not yet written").
- Rows `a_0 ≤ … ≤ a_m` and `b_0 ≤ … ≤ b_n` in angle, with common ends. The front is the edge `a_i–b_j`.
- Invariant: `max(a_i, b_j) ≤ min(a_{i+1}, b_{j+1})`. It holds at the start, because the ends coincide.
- Advance `a` when `a_{i+1} ≤ b_{j+1}`. The new front `(a_{i+1}, b_j)` keeps the invariant, since `b_j ≤ a_{i+1} ≤ min(a_{i+2}, b_{j+1})`. Advancing `b` is symmetric.
- The emitted triangle `(a_i, a_{i+1}, b_j)` has angle extent `a_{i+1} − min(a_i, b_j)`. If `b_j ≥ a_i`, that is one chord of row `a`. If not, it is at most `b_{j+1} − b_j`, one chord of row `b`.
- So every triangle spans at most the larger rim chord, which the chord pass already holds to the column step. `cert::cone_chord_sag` and the cylinder sagitta read only that extent.
- The apex as a one-point row gives the fan directly.
- This closes the frustum with misaligned rims that one report still lists as open.

**I built nothing this round.** The target directory is already deleted, and the tree still holds my probe edits. The saved diff is `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork2/C/probe.diff`.

**A correction to how my prototype is cited.** One report says it "holds one edge per column". Its apex branch does: every interior rim point is linked to one apex copy, and the last one to the other copy. Its frustum and cylinder branch holds one diagonal per side, on bare sides too, and nothing else. Neither is the walk.

**What the walk changes for an implementer**
- **`face_target` must answer δ for these faces.** The chord pass reads it through `lane_of`, so the "ruled rectangle" test has to be readable before the chord pass, as the interim route's test is.
- **The chart lane's development, clearance, ear cure and retry loop do not run on these faces.** The ear and shared-side cures exist to stop two faces making the same chord. I did not check whether a strip's corner triangle can make a chord its neighbour also makes. My prototype's suite run showed no four-use edge.
- **A one-face full-turn wall** can be walked: its two sides are one seam edge, and ids repeat. The chart lane's search path refuses it `SeamTwice` today. The walk would take it, but I did not test that.
- **`curved::pole_columns` and the both-nappes refusal** are untouched for spheres and for trimmed cones.

**If the walk is judged too large a step,** my round-two form is the fallback I would still stand behind for the chart lane: no rows, one held edge per split side. Its unsplit half then needs its own proof per plane. It is provable (two parallel rows, or two concentric arcs, zip locally under Delaunay), but that is one argument per chart.

**Unchanged from round two:** the rows to file, the predicate caveat for the interim route, and the unchecked list.
