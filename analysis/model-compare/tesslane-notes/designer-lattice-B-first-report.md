## For Ev

**Recommendation (likely).** The fault is in the lattice, and the fix goes there. Today a lattice point and a boundary point that are the same point of the surface stay two mesh vertices unless their floats agree bit for bit. In the final state the lattice gives way to the boundary in its own unit, the cell. A lattice node is not laid if the boundary (a chord point or a polygon side) passes through its cell, the box half a step either side of it in the chart. Those cells are covered by the boundary's own points.

- The mesh then depends on (body, δ) alone, as D9 requires.
- No ε is read and no constant is typed in.
- Neither the lattice nor the boundary ever asks whether two of their points are the same. Whether two boundary points are one is already settled by id.

**Premise check (measured on main by a scratch build that was not committed):**
1. Reproduced. At δ = 0.04, ball ∪ box gives 2·area/L² = 1.11e-15 and ball ∖ box gives 5.85e-15. Two such triangles appear per mesh. Their shortest edges are 1.9e-16 m and 8.1e-16 m, against ε = 1e-9 m. *(sure)*
2. **The STL writer does not refuse these triangles.** `stl::write_binary` succeeds on every one of these meshes, because it refuses only a cross product that is exactly zero. The file gets a facet whose normal is rounding noise. So no layer stops these triangles today: not `check_mesh`, not the certificates, not STL. *(sure)*
3. The same geometry mirrored (box at z ∈ (−2, 0)) meshes cleanly (worst shape 5.9e-3). I read that as the two computations of the one point happening to land on the same bits there. *(likely; I did not trace it.)* Either way, whether the mesh is right depends on the last bit of two computations.
4. This is not the thin-triangle problem of the needle cone. Over 6 bodies × 4 booleans × 6 values of δ (4e-2 down to 1e-3), every other triangle has shape ≥ 1.4e-3. The bad triangles are coincidences, not thin triangles. *(sure for this set)*
5. The winding check after triangulation (the check that drops triangles folded over a chord) already reads these triangles. It compares a rounding-noise normal with a raw `> 0.0` and passed them as outward by luck of the sign. *(sure)*

**Is a triangle with no area a defect, and whose?**
- **It breaks the mesh's own contract.** Each face's triangles are promised to wind outward (`FacePatch::triangles`). A triangle with two corners on one point of the surface has no winding. The STL writer cites "the mesh's non-degeneracy contract", which the mesh never states: two descriptions of one contract that disagree.
- **The cause is the lattice, not the triangulation.** spade correctly triangulates the distinct points it is given. The checker is not at fault either: `check_mesh` checks combinatorics by design, and any geometric threshold added to it would need ε or a typed constant.
- **The contract should say this outright** in the mesh crate's contract text (the `Mesh`/`FacePatch` doc comments): no two corners of a triangle are one point of the body, and no corner lies on its opposite side. That is what "outward" needs. Each lane guarantees it by construction; it is not detected afterwards.

**The options, as final states:**
- **A. Cell clearance (recommended).** A lattice node is laid only if its own cell is free of the boundary.
  - It removes exact coincidences, near-coincidences, and nodes an ulp off a boundary side, all by one rule.
  - Two drops in the trimmed lane become unreachable for lattice nodes: dropping a lattice node that equals a boundary point (`TrimmedFace::insert_points`), and the drop-and-rebuild when one lies on a boundary side (`constrain_boundary`). That retry arm can become a typed refusal: a node on a side would then be a bug. The points the lane adds later (ear centroids, refinement centroids) lie strictly inside a triangle, so they cannot be on a side either.
  - It is also the premise unit 5's containment lemma (a bound on the triangles beside an oblique boundary run) needs. With two corners almost one point, a triangle's circumradius can be arbitrarily large, so a circumradius bound is false unless the lattice keeps clear of the boundary.
  - Cost: triangles beside the boundary reach about 1.5 cells. The lemma has to be stated for that triangulation.
  - Measured on a scratch copy, sphere arm only: the bad triangles are gone, there are no new refusals, triangle counts fall 1–9%, and the worst shape is mostly unchanged or better.
  - It is easy to reverse: one filter.
- **B. Treat points within the run's band (ε) as one point.**
  - It contradicts Ev's ruling of 2026-08-21 (commit 1240686, Ev's own): the mesh is a function of (body, δ) alone, and an ε read that classifies owes a guard. No guard is possible here, because lattice nodes move continuously with δ, so some δ puts one exactly ε from a boundary point.
  - It still leaves triangles whose corners are about K·ε apart.
  - It adds an ε read to the mesh crate, which moves the inventory pin.
  - Rejected. *(sure)*
- **C. Merge within rounding (some number of ulps).**
  - It needs an evaluation-error bound for each kind of boundary image, and none exists. In practice it becomes a typed ulp count. That is the recurring defect class: `CREASE_MERGE` = 2⁻⁴⁰ is a live instance.
  - It only catches points that are equal in exact arithmetic.
  - Rejected. *(likely)*
- **D. Detect at the triangle and drop/rebuild** (read the winding at a band, or check each triangle's width).
  - It acts after the damage, needs a scale (so it falls back to B or C), and rebuilds meshes the lattice never needed to spoil.
  - It is worth keeping only as a test-level reading, not as the mechanism. *(likely)*

**Ratified text I would touch:**
- None of DESIGN.md's decisions changes; A is built to keep D9.
- `work/tesslane/plan.md` "The chart lane" says "a run on a chart iso line gets a lattice line". Under A that lattice line belongs to the boundary: its nodes are ceded, and the run's chord points stand in for them. That is a wording change in the tracker, not ratified text.
- The mesh README's "not a promise about triangle shape" stays true. Shape is still not promised; having a winding is.

**What would move (separate from the ranking):**
- Every trimmed-lane face with a lattice node within half a cell of its boundary, which is nearly every such face.
- The D9 golden digests for trimmed-lane bodies (`d9_mesh_goldens.rs`: `tilted_halves`, `loft_prism`).
- `docs/tess-budget-data/tess-budget-baseline.csv`, if A is applied to the spline lattice.
- The tour pins.
- All of these are regenerable.
- **A real risk:** the spline (NURBS) lane sizes so that its worst sliver lands exactly at δ, so larger triangles beside the boundary might exceed δ and refuse. Its refinement retry is the backstop. This is unmeasured and should be measured before A is applied there.

Confidence: recommendation **likely**. Every claim in the premise check is **sure** except the bitwise explanation in point 3, which is likely.

## For the orchestrator

- **Probe.** It is a scratch crate outside the tree, built against a copy of `crates/`:
  - probe crate: `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/latprobe-q7`
  - copied crates: `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/latws-q7`
  - The clearance is behind `CLEAR=1` in the copied `trimmed.rs`, analytic arms only. It tests whether any polygon side or point meets the half-cell box around each candidate.
  - Outputs: `base.txt` and `clear.txt` in the probe directory.
  - Bodies: the three box poses and three offset balls (radius 0.8, 0.6, 0.5) against the unit y-poled ball, every boolean, δ from 4e-2 to 1e-3. ball(0.5) refuses `RingOnCurvedFace` under ∪ and under ball ∖ ball(0.5), identically with and without the clearance.
- **Not measured:**
  - the torus case named in the brief;
  - cone, cylinder and torus arms under clearance;
  - the spline lane under clearance;
  - worst certificate against target (I only observed that nothing refused).
  - Unit 5 should measure these.
- **Scratchpad collision.** Our session scratchpad is shared with another lane. My first probe directory (`scratchpad/probe`) had its `src/main.rs` overwritten by another process mid-build. I saw that file's probe code (an STL and shape loop), not any findings, then moved to uniquely named directories. Other lanes running in parallel should get distinct scratch subdirectories.
- **Defects found off the question (not filed; I was told not to touch the tree):**
  - `crates/stl/src/lib.rs` module docs: "Known live case: … a cone's apex fan can emit triangles … exactly collinear" is probably stale since the cone moved to its development. It also cites a mesh "non-degeneracy contract" that nothing states. Its exact-zero refusal does not catch the triangles measured here.
  - `crates/mesh/tests/d9_mesh_goldens.rs` header says D9 promises bytes for "(body, δ, ε)", but DESIGN `[tolerance.chordal]`, as Ev ruled it, says (body, δ) alone. Two descriptions that disagree.
  - `trimmed.rs` `CREASE_MERGE` (2⁻⁴⁰) is a typed constant beside a comparison. Under A the lattice side of it goes away; the snapping of boundary points onto a crease would remain.
  - The winding check in `TrimmedFace::emit` decides a sign with a raw `> 0.0`. Under A every triangle it reads has width, so this becomes harmless, but it is still an unnamed sign decision.
- **Assumption:** half a cell (the box of chart points nearer this node than any other) is the natural, parameter-free clearance. Another fraction would be a choice Ev might want named. On the spline lattice the "cell" is the node's own band cell.
