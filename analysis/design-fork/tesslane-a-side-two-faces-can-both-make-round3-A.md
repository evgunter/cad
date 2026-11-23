## For Ev

**What a mesh promises at such a side, as the code stands (sure on what I read):**
- **To a consumer with vertex ids:** `check_mesh` checks only that every edge, by vertex id, belongs to exactly two triangles.
- **To a consumer without ids, an STL closed after joining vertices by position.**
  - The STL writer copies the mesh as it is ("no snapping, no deduplication"). The crate's docs and tests call the result a "watertight STL" (`issue111_az_export`: "a leaky STL").
  - The nightly run (`.github/workflows/nightly.yml` → `scripts/check_admesh.sh`) uses an external checker, admesh, which joins vertices by position. It fails a file on any disconnected facet, fixed edge, or more than one part.
  - So the tree does promise that an exported body, joined by position, is closed and has no edge of four triangles.
  - **Two separate vertices at one position are allowed when no edge is shared:** `m3_pr6_exports` accepts two shells that touch at a single point.
  - **Two separate points at one position that are both joined to the same two corners are not allowed:** joined by position, that is an edge of four triangles again.
- **No promise covers sheets that touch.**
  - **Two faces' triangles lying in one plane over each other (a double layer):** nothing promises against this.
  - **A vertex sitting on another face's edge:** nothing promises against this either.
  - `[mesh.winding]` is checked one triangle at a time. `check_volume` reads signed volume, so a double layer cancels out. The admesh gate never tests whether the surface crosses itself.
  - Parting the sheets is something a designer would like. The tree does not promise it.
- **`[determinism.equivariance]` as written** asks that rules commute with rigid motions and reflections.
  - Arena order (the order faces are stored in the body) does not change when the body is moved, so a tie-break by arena order breaks nothing in the clause's letter.
  - But the clause prefers intrinsic quantities over enumeration or construction order. It allows an order-based fallback only where symmetry makes a symmetric rule impossible or costly, and then it must be documented.
  - Here a symmetric rule exists, so a tie-break by arena order is allowed only as a documented, disfavoured fallback. The clause is also marked as never audited.

**What follows from that:**
- **Splitting at the straight midpoint in every patch** breaks the STL promise. Both new points sit at one position, joined to the same two corners. Ruled out (sure).
- **Splitting at the straight midpoint in all patches but one** breaks no promise. It needs the arena-order tie-break.
- **Splitting at a point of each face's own surface, in every patch that has the side inside it,** breaks no promise and needs no tie-break. The one risk is that the halves could fail their certificate or fold. I measured that risk.

**Measured this round, all at the default ε:**
- **What I ran:** the surface-point split in every patch, with the lane rules off and the chord-pass and fold fixes on, over the mesh crate's default suite.
- **Splits:** 1,416 patch splits across 59 meshes.
  - Pairs of face kinds: cone against plane 26, cylinder against plane 7, sphere 8, torus 7, cylinder 8, plane against sphere 1.
  - Plus a lens of two spline faces, which my stand-in split could not handle (see the next section).
- **Halves:** no half folded against its parent. No half lay farther than δ from its surface. The worst half reached 0.905 δ, and no half was worse than its parent triangle.
- **Coincident points:** none of the new points shared a position.
- **Leftovers:** no side was left used more than twice.
- **Tests:** the suite passes except six rows. Each of those pins either today's cut counts or the name of a refusal. That is the same set designer C's straight-midpoint run had.

**Recommendation (likely):** split at a point of the face's own surface, in every patch that has the side inside it. A patch whose own boundary carries the side never gives way. The chord-pass fix and the fold fix go with it.

The straight midpoint in all but one patch is defensible: it breaks no promise, and it needs no certificate and no chart position. I would put it to you as a named alternative with a lean, not as an even split. The choice is between "no certificate on the cut" and "no tie-break by storage order", and the measurement says the certificate costs nothing observed.

## For the orchestrator

- **How I measured.** I applied C's `probe.diff` to my worktree and added a `JOIN_PROBE=surf` mode. Before `joined`, it splits every patch that uses an overused side.
  - **The split point** is the surface foot of the 3-D midpoint (`Surface::foot`). That stands in for "the midpoint in the face's own chart".
  - **Deviation** is sampled on a 13×13 barycentric grid against the foot distance. **Fold** means a half whose normal points against its parent's.
  - **Run:** `NO_LANE_RULE=1 CHORD_FLOOR=1 FOLD_INSERT=1 JOIN_PROBE=surf`, `--test all`, shared target, three runs. I reverted the worktree afterwards and it is clean.
  - **Files:** the diff is at `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/A/scratch/surf_probe.diff`. Outputs are `surf_all.txt`, `surf_all2.txt` and `surf_all3.txt` in the same directory.
- **Limits of the measurement:**
  - The deviation is the true distance, not each lane's certificate. The cone and torus certificates are conservative, so a half at 0.905 δ there could still fail its certificate. That is unchecked.
  - Inside `tessellate`, the per-triangle winding check runs before the probe, so the halves were never given the winding check. The rows' `check_mesh` and geometry checks did pass.
  - The run covered ε 10⁻⁶ and 10⁻¹², the slow set and other crates not at all.
  - All of these can wait for the build unit.
- **What else I found:**
  - **A spline witness.** With the lane rules off, `faces_that_bound_no_region::a_lens_of_two_short_arcs_meshes_at_a_coarse_delta` has two spline faces that both make one side. My probe had no foot on a spline surface, so it fell back to the straight midpoint in both patches, and the two new points coincided. That is the straight-midpoint failure in miniature.
  - With the chart midpoint (the design), the two points sit apart by the lens's thickness. This is unverified.
  - It may also be the witness `the-shared-side-rule-on-the-spline-arm-has-no-witness` asks for, if today's rule fires there with the lane rules on. Check before closing that row.
- **Wrong or overstated in the round-two reports:**
  - **B** says the arena tie-break is "harmless". Under the clause it is a disfavoured fallback wherever a symmetric rule exists, and one does.
  - **B and C** both rest the STL case on "STL has no ids". The binding fact is narrower and stronger: the nightly admesh gate joins by position and fails any disconnected facet. And a touching point is already accepted, so the coincidence that matters is of edges, not of points.
- **What should happen before this goes to Ev:** nothing further. The measurement above answers B's request, approximately.
- **What can wait for the build unit:**
  - the lanes' own certificates on the halves;
  - the chart midpoint, including the cone's apex `u`;
  - the lens;
  - the other ε values and the slow set.
- **Moved this round:** the admesh gate and the touching-assembly precedent. They show that the straight midpoint in every patch breaks a promise the tree makes, so the straight midpoint can only work with a tie-break. **Not moved:** splitting every patch that has the side inside it, which I held in rounds one and two and hold again.
