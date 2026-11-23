## For Ev

**Recommendation.** A triangle that has a corner the mesher placed itself (a lattice point, an ear point or a refinement point) must never be flat. Here, flat means its least height (2·area ÷ longest side) is within the kernel's ε. The current code makes the opposite possible: the trimmed lane's winding check decides each triangle's winding against a bare `0.0`, so a triangle whose normal is rounding noise passes when the noise happens to come out positive. Instead, the check should decide winding at the band. A triangle the band calls flat would be handled the way the check already handles a fold: its mesher-placed corners are dropped and the face is rebuilt. A flat triangle made only of boundary points belongs to the body (a near-T junction, a thin face) and stays in the mesh. The mesh's design page should state this as a promise next to the δ promise. Confidence: likely. The one real choice is which band decides "flat" (A vs A′ below).

**Premise check** (measured on main with a scratch harness; nothing in the tree was changed):
- **STL does not refuse these triangles.** It refuses only a cross product that is exactly zero. Their cross product is about 1e-17, so the union at δ 0.04 writes to binary STL without error. The normals it writes point at random: their cosine against the sphere's radial direction is 0.71, 0.96 or 0.98 where it should be ±1. (sure)
- **There is a third way a lattice point gets dropped, and the defect is there.** Besides the exact-duplicate drop and the on-a-side drop, the winding check in `TrimmedFace::emit`/`attempt` drops the lattice corners of a folded triangle. It tests `normal·outward > 0.0`, and these triangles passed on the sign of their noise. (sure)
- **It is common, and the near-coincident pair is always one lattice point and one boundary point.** I swept the box cut at x ∈ {0.5, 0, 0.25, −0.5, √2/2}, three z ranges, all four booleans, and 40 values of δ from 0.0045 to 0.1. 56 of the 2400 meshes have a triangle with 2·area/L² < 1e-9. In every one, a lattice point lies 0.9–8e-16 (in 3-D) from a boundary point. For x = √2/2 it happens at 12 of the 40 δ. The reported numbers reproduce: 1.11e-15 for the union, and 5.9e-15 for the ball less the box. The cause in each case is a boundary vertex computed at v = −1e-16 where the exact value is 0, next to the lattice row at exactly 0. (sure)
- **So "of no area" really means flat at every band:** the area is rounding-sized, not zero. `check_mesh` passes the mesh, as its own doc says it will (it checks only indices and counts).

**Why this is a defect, and whose.** The mesh's contract is `FacePatch` ("triangles with outward winding"). A triangle within ε of flat has no winding the kernel can stand behind, because the promise already allows ε in every position. So these triangles break the outward-winding promise, which is a promise about each triangle. They do not break the shape promise: the design page says, in text you reviewed, that there is none, and that stays true. Thin triangles are fine; flat ones are not. The STL crate already cites a "non-degeneracy contract" that no document states; this would state it.

Flat triangles made only of boundary points are a different case and are sometimes required. The planar lane keeps a needle about 3e-18 thick on purpose (#111): it is how a face stays watertight across a vertex that lies 1 ulp off its neighbour's edge. So the promise is scoped to triangles with a mesher-placed corner. Any such corner can always be removed, and removing one only coarsens the mesh, which the per-triangle certificates still cover.

**What "the same point" should mean.** Today two points count as one only when their bits are equal, which is the right test for the triangulation library's own vertex identity. The question that matters here is different: is a triangle's winding decided? That is a 3-D fact about the emitted triangle, so it is decided where triangles are emitted. In a constrained Delaunay triangulation, a flat triangle with a mesher-placed corner can only form against a boundary point or a boundary side. So a single per-triangle verdict at emission covers every source: lattice points, ear and refinement points, and every chart, including the spline lane, whose winding check does not run today because it has no outward normal. The flatness test needs no outward normal. The lattice then needs to know nothing more about the boundary.

**The answers, as final states**
- **A (recommended): decide flatness at ε.** The check uses the crate's `Eps` band (`separates`): a triangle with a mesher-placed corner whose least height ε does not separate counts as flat.
  - Pro: ε is the kernel's one answer to "these are one point", and the promise is already "δ + ε".
  - Con: the result now depends on ε. Changing ε moves the mesh, but only on faces where a lattice point lies within the old or new ε of the boundary, and both outcomes are correct certified meshes.
  - Reversible: one predicate.
  - Ratified text it changes: `[tolerance.chordal]`'s sentence "the one ε read that is a classification — pole identification — owes a guard so an ε flip can never move emitted coordinates". An agent wrote it in an editing pass (`99cc678`), restating #896, and I found no ruling of yours behind it. The principle I read under it is narrower: ε never sizes the mesh and never moves a vertex the body declares. That principle should be stated in place of the current sentence, and it is your call.
- **A′: the same check at rounding size.** The threshold would be the certificate's existing rounding allowance (`cert::rounding_allowance`, 8 ulps of the coordinates).
  - Pro: does not depend on ε, so the sentence stands.
  - Con: it adds a second, private notion of "the same point" inside the mesher. It also keeps triangles 1e-15 to 1e-9 high, whose normals are geometric but which an f32 STL flattens anyway.
- **B: a lattice that keeps clear of the boundary.** Drop every lattice node whose cell the boundary crosses. This needs no constant, and together with the chord pass's step bound it means no node lies on a side.
  - Con: it makes the triangles next to the boundary up to about two cells wide, which ties it to the containment lemma not yet proved (unit 5) and could add refusals at the current headroom. It moves every trimmed face's mesh. It also does not cover ear or refinement points.
  - When to pick it: if you want thin triangles next to the boundary bounded, which is a separate decision. It would not help the needle-cone triangles, which have a different cause.
- **C: check only (in the checker or the exporter).** This turns a silent defect into a refusal while the mesher still produces it. Not recommended as the fix, but see the next item.

**What goes with A.**
- A geometric check placed next to `validate::check_volume`. It takes ε and the mesh's boundary polylines and re-derives "no flat triangle with a non-boundary corner"; `check_mesh` stays index-and-count only.
- The test suites use that check in place of `reach_probes`' typed-in `MIN_SHAPE = 1e-12`.
- The chart-lane plan's "a run on a chart iso line gets a lattice line" must say that this line's nodes are not candidates: the run's chord points stand in for them. Otherwise the plan creates this exact coincidence by construction.

**What would move.** Only meshes that today carry a flat triangle with a mesher-placed corner: there the two flat triangles become one. That is 56 of the 2400 in my sweep. Whether any byte-pinned golden (D9, `docs/tess-budget-data`, the tour) carries one is unmeasured. One pass computing least heights over the golden bodies would settle it; anything that moves is regenerable.

## For the orchestrator

- **Measurement harness.** `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/probe` is a standalone crate with path dependencies, built into the shared `target/`. The pairs it found:
  - union, x = 0.5, δ 0.04: face 8v3, grid vertex 152 against boundary vertex 39;
  - x = √2/2: face 9v3, grid vertex 105 against boundary vertex 17;
  - A∖B, x = −0.5: grid vertex 98 against boundary vertex 34.
- **Not rechecked:** the torus wall case named in the brief.
- **Off-question defects to file:**
  1. `crates/stl/src/lib.rs` module docs say "Known live case: … a cone's apex fan … collinear along a generator". This is stale since the development landed (`trimmed.rs`, "The development"). Its "non-degeneracy contract" names a promise that nothing states.
  2. `trimmed.rs` `emit`: the winding check `normal.dot(n.vec()) > 0.0` decides a sign that is pure noise on near-flat triangles. Today it can drop or keep a lattice point on rounding alone; this is the defect's own line.
  3. The spline lane runs no per-triangle winding or flatness check (`outward` is `None`), so a flat triangle there is unseen.
  4. `trimmed.rs` `CREASE_MERGE = 2⁻⁴⁰` is a typed merge band next to a comparison, the same defect class. Its retirement may come anyway through `the-crease-arms-retire-once-no-carrier-carries-a-c0-knot`.
  5. `crates/mesh/tests/reach_probes.rs` `MIN_SHAPE = 1e-12` compares a dimensionless shape against a typed constant. Under A it gives way to the length-against-ε check.
- **For the implementer, under A:**
  - The exact-bits dedup in `insert_points` and the on-a-side retry in `constrain_boundary` stay. They are facts about the triangulation's own input and plane, and a candidate exactly on a side in the chart (u, v) can be the sagitta's distance from the 3-D chord, so the 3-D verdict does not replace them.
  - A flat triangle with no mesher-placed corner is kept, not refused.
  - The ε inventory pin (`the_eps_inventory_is_pinned`) will move by one read.
- **Argument I rely on:** in a constrained Delaunay triangulation, a flat triangle with an interior corner forms only against a constraint (a boundary side) or a near-coincident point. That is why a pre-triangulation filter is not needed. Spade's bulk loader is Delaunay only in its own scaled plane, which does not change the argument. The emission verdict does not depend on it in any case.
- **Asking Ev:** the fork is A vs A′, and if A, the `[tolerance.chordal]` rewording. The new clause in `crates/mesh/README.md` binds future work, so it waits for Ev's sign-off.
