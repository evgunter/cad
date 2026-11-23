## For Ev

**Where I am now.** A combination. The lattice keeping clear of the boundary does the work (B's rule and mine). The promise is scoped and checked the way A scopes and checks it. The mesher reads no ε, no band and no rounding count. Two questions are left for you, one against A and one against B; both are at the end of part 3.

**1. What moved me.**

- **A, on scope.** I withdraw "every triangle has area", `check_mesh` refusing a flat triangle, and a typed refusal for a flat triangle on three boundary points. The lane cannot remove a corner the body gave, and `planar`'s module docs allow a zero-area sliver inside a face to be kept. The promise is about triangles with a lattice corner.
  - A's evidence for it is overstated. I ran the three issue-111 bodies (plain, counter, stencil A×Z) on main at δ 1e-2: none has a triangle with least height under 1e-9 m, and the least in any of them is 0.06 m. So "sometimes required" is unmeasured. The scope stands on the principle alone.
- **A, on the scale of the promise.** My rounding-resolved census is withdrawn. Boundary points carry ε of slack against the surface, so a triangle 1e-12 high has an exactly computed normal of a shape that means nothing. The floor the promise can name is ε.
- **B, on provenance.** `git log --all -S'owes a guard' -- docs/DESIGN.md` ends at commit 1240686, under your name, 2026-08-21. It wrote "this paragraph keeps its promise unqualified, and the classification is guarded so the promise stays true" and "this project does not read intent into numerical coincidence". A's report says an agent wrote the sentence in 99cc678 with no ruling behind it; that commit only condensed yours. So "no `Band` in the mesher" is your ruling, not my reading.
- **My own lemma argument, which B also makes, is wrong as stated.** The cell bound is about a triangle's extent, and a node too near the boundary enlarges no extent. Clearance only costs deviation (worst certificate 1.06, then 1.22, then 2.21 cell bounds). It is not what the containment lemma needs, and I no longer argue from it.

**2. The arguments against my recommendation.**

- *A: clearance makes boundary triangles up to two cells wide and may refuse at today's headroom.*
  - Holds at half a cell. In my round-1 run over 486 faces, six cone and cylinder faces refuse `CertificateExceeded` (worst 2.21 against the 2 allowed). The sphere arm alone reads 1.49 and refuses nothing, which is all B measured.
  - Does not hold at 3/8 or below: nothing refuses. At 1/16 the worst is 1.22, the same as at 1/100. That is the cost of one missing node, which A's own drop pays too.
- *A: it does not cover ear or refinement points.* True as stated, and answered by the scope. Those points are centroids of a kept triangle, so their triangles are flat only if the parent is. Under clearance a parent with a lattice corner is not flat, and a parent on three boundary points is the body's. This is reasoned, not run.
- *A: one verdict at emission covers every source, so no filter is needed.* Holds as sufficiency. It fails on scale. At ε it changes your 2026-08-21 ruling; A asks for that openly, but on the wrong provenance. At rounding size A's own objections apply. It is also a fourth drop-and-rebuild beside the three the lane has.
- *A: it moves every trimmed face.* True (292 of 486 at 1/16). Not a ranking matter under `designer.md` §4.
- *B: any fraction but ½ is a number someone chose.* Holds. My answer is the measurement above.
- *A and B against a rounding-size threshold.* Both hold; withdrawn in part 1.

Every option removes every instance any of us measured. In my corpus no node lies between 1e-9 and 1e-6 of a step from its polygon, and A's 56 pairs are all under 1e-15 m apart. The options differ only in what scale the mesher reads and in what is promised.

**3. Recommendation now, as a final state.**

- **The lattice** lays a node only where the box `c` steps each way around it is clear of the trim polygon and of every crease line. Scale: the lattice step. This part does the work.
- **At emission** nothing new is decided. The winding census stays the fold check; every triangle with a lattice corner that it reads now has width. The exact-duplicate drop and the node-on-a-side retry become assertions, as B says.
- **The promise** (`FacePatch::triangles` and the mesh README): a triangle with a lattice corner is not flat. A flat triangle on three boundary points is the body's, and is kept. This is A's scope.
- **Where it is checked:** a geometric check beside `validate::check_volume` that takes ε and the mesh's boundary polylines; this is A's. `check_mesh` stays combinatorial. The suites use the new check in place of `MIN_SHAPE`. ε is read by the validator only, so the mesh stays a function of (body, δ).
- **The STL writer** keeps its rule. Its "non-degeneracy contract" sentence points at this promise.

**The question against A:** may the mesher read ε to decide which of its own points stay?
- If yes, A's emission verdict is the smaller change and needs no lattice rule, and your 2026-08-21 ruling changes.
- If no, nearness has to be decided at the lattice's own scale, which is clearance.
- I say no. `sure` on the provenance, `likely` on the answer.

**The question against B:** is the clearance the node's whole cell, or a named fraction?
- Whole cell: no number. These arms then size finer than today, and at today's sizing six faces in 486 refuse.
- Fraction: one number of the *quality* kind. Nothing refuses, and 1/16 costs what any single missing node costs.
- I lean 1/16. `unsure`.

**4. Ratified text.**

- `crates/mesh/README.md` gains the scoped promise sentence, the clearance clause and its constant. My round-1 line that the headroom "is derived from `c`" is withdrawn; that entry stays as it is.
- `docs/DESIGN.md` `[tolerance.chordal]` is unchanged. A's recommendation would change it.
- `work/tesslane/plan.md`, "a run on a chart iso line gets a lattice line": that line's nodes on the run are not laid. All three reports say this.

## For the orchestrator

- **Run this round.** `fork-scratch-C/needle/` holds a probe against the tree's own `crates/mesh`, unmodified, built into the shared `target/release` as `needle-probe-C`.
  - It builds the three A×Z bodies of `crates/mesh/tests/issue111_az_needle.rs` and meshes each at δ 1e-2.
  - Result: `check_mesh` passes on all three; triangles 84, 132 and 168; no exact-zero cross product; none with least height under 1e-9; least heights 0.20, 0.11 and 0.062 m.
- **Not rerun.** Every figure about clearance is from my round-1 logs in the old shared scratchpad (`base.log`, `clear_<c>.log`). The per-arm worst at half a cell is cone 1.103 δ, cylinder 1.042 δ, sphere 0.745 δ, torus 0.462 δ, at a target of δ/2.
- **Provenance commands.**
  - `git log --all -S'owes a guard' -- docs/DESIGN.md`: the earliest hit is 1240686 (2026-08-21, Ev's name, agent co-author).
  - `git show 1240686 -- docs/DESIGN.md` has the ruling text quoted above.
  - 99cc678 is the later editing pass that shortened it.
- **Corrections to my round-1 report.**
  - Part 3 is withdrawn as written: no flat-triangle refusal in `check_mesh`, no one definition shared with the STL writer, and no new refusal for a boundary-only flat triangle.
  - The "Coupling to unit 5" bullet is withdrawn: the lemma need not take `c` as a hypothesis. What survives is that unit 5 should measure with the rule in place, because the worst triangle is then 1.22 cell bounds, not 1.06.
- **What I found wrong or narrower in the other two reports.**
  - A's provenance of the ε sentence, and A's claim that the planar lane keeps the issue-111 needle; both are covered above.
  - B's "no new refusals" covers the sphere arm only. B says so in its own orchestrator section, but its Ev section does not.
  - B's contract wording ("no two corners of a triangle are one point of the body, and no corner lies on its opposite side") is mesh-wide. `planar`'s docs allow the opposite for boundary points, so it needs A's scope.
- **Open edge, no instance measured.** On the analytic arms the fold check still reads a flat triangle on three boundary points by the sign of its noise. It keeps the triangle or refuses `FoldedTriangle` on rounding alone, and refuses an exact zero. Under the scoped promise that triangle should be kept. Telling flat from folded there needs a scale, and I have no ε-free answer; it wants its own row.
- **Not checked.**
  - Whether any body on main forces a flat triangle on three boundary points. None of the three issue-111 bodies does.
  - A's argument that a flat triangle with an interior corner forms only against a constraint. I rely on it for "ear and refinement points inherit", and did not test it.
  - The spline arm under clearance. B's risk is real: that arm sizes its worst sliver at δ exactly, so larger boundary triangles may refuse. Measure before applying the rule there.
- **All three reports agree on:**
  - the STL premise correction;
  - the fold check's bare `> 0.0` being the line that let these through;
  - the plan sentence;
  - the stale cone-apex "known live case" in `crates/stl/src/lib.rs`;
  - `CREASE_MERGE` and `MIN_SHAPE` being the same class.
- **B's find, worth a row:** the `d9_mesh_goldens.rs` header says D9 pins "(body, δ, ε)" where DESIGN says (body, δ).