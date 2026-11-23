## For Ev

**First, a correction to how the other two read me.** Both round-1 replies set the fork on `c` (the clearance, in lattice steps) as "½ for B against a small value for C". That is no longer my position. In round 1 I moved to a small `c`, after confirming C's measurement that ½ refuses `CertificateExceeded` on cone and cylinder faces at today's sizing. So there is no fork on the value of `c`.

**Agreed by all three:**
- The lattice does not lay a node whose box of `c` steps each way meets the trim polygon or a crease line. This is what does the work.
- The mesher reads no ε, as your ruling of 2026-08-21 (`1240686`) requires.
- The exact-duplicate drop and the node-on-a-side retry become assertions.
- The plan's sentence that a run on a chart iso line gets a lattice line stands; that line's nodes on the run are simply not laid.

**The four points:**

1. **A flat triangle on three boundary points: kept.** C keeps it; A now refuses it typed.
   - Measured: none exists in 50 meshes (the cone, torus and tilted-cylinder test bodies of `trimmed` and the tilted sphere pair, 5 values of δ each). The least height of a triangle on three boundary points is 5.7e-4 m, the same with and without the clearance.
   - A's measurement and C's agree that the issue-111 bodies emit no such needle either.
   - This is a real difference of final state (a mesh against a typed refusal), but about a state no fixture reaches.
   - I keep it because:
     - the lane cannot remove a corner the body gave it;
     - `planar`'s docs allow such a sliver;
     - refusing would need the mesher to decide "flat" at some scale and refuse a valid body on that decision.
   - Unmoved: A's reason is that no live needle exists, which shows a refusal would rarely fire, not that the body is wrong.

2. **The check outside the mesher reads "flat" at ε, and only the validator reads it.** I moved to C here.
   - C's argument holds. Boundary points carry ε of slack against the surface. A triangle 1e-12 m high has a normal that rounding resolves, but a winding the kernel cannot stand behind. So the promise's floor is ε, not rounding.
   - A validator reading ε moves no emitted coordinate, so D9 is untouched.
   - It is a public `mesh::validate` check beside `check_volume`, taking ε and the mesh's boundary polylines (A's construction):
     - a caller who needs the promise checked runs it;
     - the suites run it in place of `reach_probes`' `MIN_SHAPE`;
     - `check_mesh` stays combinatorial.
   - "Test-level or caller-run" is only wording: it is public, and the tests use it.

3. **The winding check at emission (`TrimmedFace::emit`) reads its sign as an enclosure sign: resolved above the triangle's existing rounding allowance (`cert::rounding_allowance`), or unresolved.** This answers C's open edge without ε.
   - A resolved negative is a fold, handled as today: drop the lattice corners and rebuild, or `FoldedTriangle` if there are none.
   - An unresolved sign with a lattice corner cannot occur under the clearance, so it is an assertion.
   - An unresolved sign on three boundary points means flat, and the triangle is kept (point 1). Today the check refuses or keeps that triangle on the sign of rounding noise.
   - This is a sign decision, not a "same point" decision. It is the same instrument as `geom_core::Band::enclosure_sign`. It keeps the mesh a function of (body, δ).
   - Versus A: we name the same scale, and differ only in the outcome for a flat boundary-only triangle (point 1).
   - Versus C (who called it its own row): it fits in this change.

4. **The clearance is its own named quality constant (1/16 is fine), not a read of `ASPECT_CAP`.**
   - The page says `ASPECT_CAP` is the NURBS split's heuristic and "can be replaced by the criterion it stands in for". If the clearance read it, retiring the cap would silently change the clearance, and one name would mean two things.
   - The value is not in dispute: C and I measured no refusals at 1/16 and 1/4, and the worst certificate is the same at 1/16 as at 1/100.
   - This is a real but small difference of final state (one constant against two).

**The final state, for the implementer and the design page:**
- **Lattice.** On every arm (the spline lattice per band, once measured), a node is laid only where the box `c`·step per chart axis around it is clear of the trim polygon and of every crease line.
  - `c` is a named quality constant.
  - The exact drop in `insert_points`, the on-a-side retry in `constrain_boundary`, and the lattice half of `CREASE_MERGE` retire into this rule. The first two become assertions.
- **Emission.** The winding check decides by enclosure sign at the rounding allowance: resolved negative is a fold; unresolved with a lattice corner is an assertion; unresolved on boundary points only is kept.
- **Promise** (README and `FacePatch::triangles`): a triangle with a corner the mesher placed is not flat. Its least height is not within ε, so its outward winding is meaningful. Boundary-only triangles are the body's. Shape is not otherwise promised.
- **Checked** by the public ε-reading validator above. The STL writer's "non-degeneracy contract" sentence points to this promise.
- **Ratified text.**
  - `crates/mesh/README.md` gains the promise sentence, the clearance clause and the constant.
  - DESIGN.md is unchanged.
  - The plan's wording on iso-run lattice lines is as agreed above.

**Still apart, each a question for Ev:**
- **(with A)** A flat triangle on three boundary points, which no fixture produces: is it kept as the body's, or refused typed? I say kept (likely).
- **(with A, if A still holds it)** Is the clearance its own quality constant, or a read of `ASPECT_CAP`? I say its own (likely).
- Points 2 and 3: I hold nothing apart, provided A accepts ε in the validator only. A's "rounding" was for the emission check, where I agree.

## For the orchestrator

- **Ran this round.** I extended my scratch unit test `b_clearance_sweep` (in my copied workspace `…/scratchpad/latws-q7`, built into `…/fork-scratch-B/target-B`) to count triangles whose three corners are all on boundary polylines, with their least height and least 2·area/L².
  - Binary: `…/fork-scratch-B/mesh-sweep2-B`. Outputs: `out2-none.txt` and `out2-c16.txt` in the same directory.
  - Least boundary-only height: 5.7e-4 m (the klein elbow at δ 1e-3), identical at `c` = 1/16.
  - The boundary set includes planar faces, so all their triangles count; the result is still that nothing is flat.
- **Correction to the round-2 framing.** The coordinator and both peers list me at `c` = ½. My round-1 reply recommended a small `c` as its own constant.
- **Not checked.**
  - The spline arm under clearance. This is still the open sizing risk, since that arm sizes its worst sliver at δ exactly.
  - Whether `cert::rounding_allowance` at the triangle's scale is the right enclosure for a cross product. That is C's paper argument; I adopt it without running it.
