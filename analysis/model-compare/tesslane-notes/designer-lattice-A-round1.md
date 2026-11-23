## For Ev

**What moved me.** Designer B found that the sentence I proposed rewording is yours. Commit `1240686` (2026-08-21, "D9 keeps its promise unqualified; the pole classification is guarded (#896)") rules that the mesh is a function of (body, δ) alone, and that a classification that reads ε owes a guard rather than a qualifier on that promise. I had read the sentence as an agent's editing-pass text. It is your ruling, so my option A (decide "flat" at ε) contradicts it. B's further point also holds: no guard is possible here. Lattice nodes move continuously with δ, so some δ puts a node exactly ε from a boundary point, and an ε read would then flip the mesh. I withdraw A.

**Is the rest of my first argument answered?**
- *"A per-triangle check at emission is the single source of truth; the lattice need not know the boundary."* Answered. The others show that the relation from lattice to boundary is the part that is missing. The only relation today runs the other way: `chords::tighten` keeps each boundary chord within one lattice step. Four mechanisms each cover a sliver of the missing half, at exact zero or at a typed constant: the duplicate drop, the on-a-side retry, the bare `> 0.0` fold test, and `CREASE_MERGE`. A check at emission needs a scale. Once ε is ruled out, the only scale left is rounding, and deciding which point to drop at rounding scale is ill-posed, as C argues. A rule set in lattice units decides far from the noise, where either outcome gives a sound mesh. The containment lemma (unit 5) also needs this rule as its hypothesis. I agree.
- *"Flat triangles made only of boundary points are the body's own and sometimes required (the #111 needle), so the promise must exempt them."* Weakened by measurement. I meshed the three A×Z bodies of `issue111_az_needle.rs` at δ 0.1, 0.01 and 0.001: the least triangle height is 0.062 m or more, so no needle is emitted today. No live case carries a boundary-only flat triangle. C would refuse one typed, and I now agree that is the fail-loud reading of the outward-winding promise. Confidence: likely.
- *A′ (merge within rounding).* B says no bound on the evaluation error exists for each boundary image, so a ulp count would be a typed constant. That holds for a decision that drops points. It does not hold for a check that only verifies.

**My recommendation now** (the joint design of B and C; I add the scale of the check):
1. **The lattice keeps clear of the boundary. This part does the work.** A lattice node is not laid when the box `c` steps each way around it, per chart axis, meets a side or a point of the trim polygon, or a crease line. Nothing is merged. A node near the boundary is simply not laid, and the boundary point already there serves.
   - Scale: the lattice step. No ε is read and no point-sameness is decided, so D9 holds as you ruled it.
   - It applies on every arm, the spline lattice included, using each band's own steps.
2. **Emission keeps its winding check, now with the bare zero named.** The check (`TrimmedFace::emit`) decides fold or not as "resolved above its own rounding", using the existing `cert::rounding_allowance` at the triangle's coordinate scale.
   - Under part 1 it never fires on a triangle with a lattice corner, so it is a guard (a re-derivation of what part 1 makes true).
   - A flat triangle made only of boundary points refuses typed.
   - Rounding is the right scale for a check that verifies. It would be the wrong scale for one that decides what to drop.
3. **The mesh promises it.** Every triangle has area, meaning its winding is resolved above rounding. That is what `FacePatch`'s "outward" already needs, and shape beyond it is still not promised.
   - One definition lives in `mesh::validate`, read by the emission check, by a geometric check beside `check_volume`, and by the STL writer's documentation.
   - `reach_probes`' `MIN_SHAPE` retires into that definition.
   - Whether `check_mesh` itself calls it, or stays combinatorial as its doc says, matters less. I lean a separate check, as C half-proposes.
4. **Side effects of part 1:**
   - A node exactly on a side or exactly on a point becomes impossible. The two exact drops (`insert_points`, `constrain_boundary`) become checks that refuse typed if they ever fire.
   - Ear and refinement points lie strictly inside a triangle, so neither drop applies to them.
   - The lattice-node half of `CREASE_MERGE` retires; the half that snaps boundary points onto a crease stays.

**The one remaining disagreement (B vs C), for you to rule: what is `c`?** It comes down to one question: *beside the boundary, does the lattice promise shape (`c` = 1/2), or only a resolved winding (a small `c`)?*
- **`c` = 1/2**, a node's own cell (B). It is the only value nobody chooses, and it splits the cells cleanly between lattice and boundary.
  - C measured, at today's sizing on 486 analytic faces: worst certificate 2.21 cell bounds against the headroom's allowance of 2, and 6 faces refuse with `CertificateExceeded`.
  - So it lands only together with the lemma, and then sizes these arms finer than today.
  - B's sphere-only probe saw no refusals.
- **A small `c`**, 1/16 = `1/ASPECT_CAP` (C). It costs almost no coarseness: worst 1.22 against 1.06 on main, and no refusals.
  - Any `c` > 0 removes flat triangles: C measured a least `q` (2·area/L²) of 2e-3.
  - Below about 1/8, the sizing rule barely depends on `c`, so the value is a quality choice. Tying it to `ASPECT_CAP` reuses the only shape dial the page already names, rather than adding a constant.
- **I lean small `c`** (confidence: unsure). `[mesh.sizing]` says "as coarse as possible", shape is not promised, and C's measurements show coarseness flat in `c` at small values. If you want bounded shape beside the boundary, choose 1/2 and accept finer sizing once the lemma lands.

**Ratified text this changes**
- `[tolerance.chordal]` / D9: nothing. My first round's proposed rewording is withdrawn.
- `crates/mesh/README.md` (your sign-off), three additions:
  - one sentence in the promise: every triangle has area (its winding resolved above rounding), and no shape beyond that;
  - a clause stating the lattice/boundary relation in both directions (each chord within one step; each node at least `c` steps from every constraint);
  - the clearance in the constants list, as *quality*.
  - "not a promise about triangle shape" stays.
- `work/tesslane/plan.md`: "a run on a chart iso line gets a lattice line" stands, and that line's nodes on the run are not laid. This is tracker wording.

**What would move:**
- every trimmed face with a node within `c` of its boundary (C: 292 of 486 faces at 1/16);
- the D9 goldens `tilted_above`, `tilted_below` and `tilted_halves`, plus `loft_prism` and `swept_elbow` once the spline arm takes the rule;
- `docs/tess-budget-data`;
- the tour's pins on trimmed faces.

All of these are regenerable. The spline lane sizes its worst sliver to land at δ, so it needs a measurement under the rule before it lands (B's flagged risk).

## For the orchestrator

- **Ran this round.** A scratch crate at `…/scratchpad/fork-scratch-A` (binary `needle-a`, built into the shared `target/`). It reproduces the three A×Z bodies of `crates/mesh/tests/issue111_az_needle.rs` and reports the least triangle height and any triangle under 1e-9 m. Result: none; least heights are 0.110 (counter), 0.062 (stencil) and 0.201 (plain) at every δ.
- **Checked:** `git show 1240686` confirms B's ruling citation. My first report misattributed the `[tolerance.chordal]` sentence to the editing pass `99cc678`, which only reworded it. That error drove my option A.
- **Not rechecked:** C's `c` table and B's sphere-only refusal-free result. They are the evidence for the fork, and they differ at 1/2 (C counts 6 refusals over more arms; B counts none on the sphere arm). Unit 5 should re-measure on every arm before the ruling is applied.
- **Scratch collisions.** In round 1 my probe crate was named `probe` in the shared directory. B reports its `src/main.rs` was overwritten mid-build, and C reports a `probe` binary that overwrote theirs. The overwriter was probably me or B; my results were re-derived from my own runs and stand.
- **Off the question:** unchanged from round 1 (stale STL module docs; `CREASE_MERGE`; `MIN_SHAPE`; the spline arm has no winding check). Add C's finding that the cylinder seam ruling is chorded as one side across about 20 rows, giving `q` around 2e-3 fans. It belongs with unit 1.
