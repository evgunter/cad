## For Ev

**Recommendation.** A triangle of no area is a defect of the mesh, and it is the lattice's. Four parts; the fourth is a fork.

1. **The lattice keeps clear of the boundary.** A lattice node is laid only where the box `c` lattice steps each way around it (in the chart, per axis) meets no side of the trim polygon and no crease line. Nothing is moved or merged: a node near the boundary is not a candidate, and the boundary point already there serves. `sure` this is the missing piece; `likely` on the box form.
2. **Nothing decides "the same point".** The lane never asks whether a node and a boundary point coincide. It asks whether the node is `c` steps clear, in the lattice's own unit, where either answer at the margin gives a sound mesh. Not `geom_core::Band`: ε is the model's coincidence scale, and a mesh is a view at its own step. `sure`.
3. **The mesh states what it already assumes:** every triangle has area, meaning its outward winding is resolved above its own rounding. One definition in `mesh::validate`, read by the lane's winding census, `check_mesh` and the STL writer. `likely`.
4. **`c` is the design's one number.** I lean `1/ASPECT_CAP` (1/16); `1/2` is the defensible alternative. `unsure`; the table is below.

Terms: the *lattice* is the trimmed lane's regular grid of interior points, a *node* one of them, a *step* its spacing on one chart axis; the *trim polygon* is the face's boundary chord points joined in the chart; a *cell bound* is the deviation `[mesh.sizing]`'s per-chart theorem proves for a triangle inside one lattice cell; `q` is `2·area/L²`, `L` the longest side.

**Premise check.** The statement holds, with two corrections.

- Reproduced exactly on the checkout: `q` 1.111e-15 (union) and 5.852e-15 (ball less box), the node and loop vertex as stated; the two mesh vertices are 1.9e-16 apart in space.
- *Correction.* The STL writer refuses only an exactly zero cross product (`stl::facets`, `len == 0.0`). This triangle's is 2e-17, so it is written, with a normal 13° off the sphere's. The writer's doc cites "the mesh's non-degeneracy contract"; no mesh doc states one, and `check_mesh`'s doc says such slivers pass.
- *Correction.* Nodes are dropped in a third place. The winding census (`TrimmedFace::emit`) reads `normal·outward ≤ 0` as a fold, drops that triangle's nodes and rebuilds. So on analytic charts an exactly flat triangle is already refused; the measured ones survive because rounding left the product positive.
- How common: 38 bodies at six δ (the plane-across-ball and ball-pair poses of `tilted_sphere_pair`, the cone and torus bodies and tilted cylinders of `trimmed`'s tests), 486 analytic trimmed faces, 416,572 nodes. Eight nodes lie within 1e-9 step of their polygon, on eight faces, all plane-across-ball at δ 0.04; none lies between 1e-9 and 1e-6. They are exact coincidences split by rounding. Four of the eight faces emit the flat pair; on the other four the node happens to land on or outside the polygon. The torus instance is not reproduced here.

**What the issue is evidence of.** The lane holds two point sets and half a relation between them. Boundary to lattice exists: `chords::tighten` holds each boundary chord to one lattice step per axis. Lattice to boundary does not: nothing says how near a node may come. Four mechanisms each hold a sliver of the missing half, at exact zero or at a typed constant: the duplicate drop in `insert_points`, the node-on-a-side retry in `constrain_boundary`, the fold drop in `emit`, and `CREASE_MERGE` (2⁻⁴⁰ of the chart extent, for nodes beside a crease). The lemma `UNPROVED_HEADROOM` waits on (`one-containment-lemma-for-an-oblique-boundary`) needs the same half as its hypothesis: a triangle beside the boundary cannot be bounded without knowing where the nearest node may be.

Separately, "a triangle has area" is defined four ways: the STL writer (`len == 0.0`), `check_mesh` (a repeated index), the winding census (`> 0.0`), and the reach probes' `MIN_SHAPE` (a typed 1e-12).

**The answers as final states.**

**A. Clearance (recommended).**
- Makes true: no node within `c` steps of the boundary; a triangle with a lattice corner beside the boundary has height between `c` and `1 + c` steps; the lemma has its hypothesis, and the headroom becomes a number derived from `c`.
- The three reactive drops stop being how a flat triangle is avoided. The exact-zero pair become checks. The fold drop stays as it is (at coarse δ its strip can be wider than `c`); it fired twice in the corpus on main and not at all under any `c` tried.
- Leaves possible: thin triangles the boundary alone makes (needle cones; a cylinder seam ruling chorded as one side across 20 rows, `q` 2e-3 here). They stay certified and unpromised. A flat triangle on three boundary points is refused typed by part 3 instead of emitted.
- Reversible: one filter and one number.

**B. A tolerance decides sameness** (a `Band`, or a typed 2⁻ᵏ): a node within it of a boundary point is dropped.
- Makes true: the measured instance stops.
- Leaves possible: a triangle just past the threshold (height 1e-8 m at default ε, `q` near 1e-7).
- `[tolerance.chordal]` says the mesh layer never reads ε for sizing, pinned by `the_eps_inventory_is_pinned`; here emitted coordinates would move with ε.
- The band's indeterminate arm has nothing to refuse, since the node is optional. A typed constant is the 64ε class PR 6950's reviewers broke.
- The decision sits at the noise, so it is ill-posed however the threshold is spelled.

**C. Not a defect.** The contract stays distance-only in every sense, and the STL writer skips or writes flat facets.
- Leaves possible: two mesh vertices at one point in any mesh, a facet normal that is rounding noise, and each consumer coping on its own.
- `FacePatch::triangles` already promises outward winding, which a flat triangle cannot have. This is the layer below producing the wrong thing and every layer after compensating.

**D. No lattice** (interior points placed by refinement, or by an offset layer of the boundary). It could promise shape. It contradicts the chart lane's ratified final state ("interior points are a lattice"), the per-chart cell theorem and simple replay, and `trimmed`'s docs already record that insertion does not converge in an anisotropic strip. Not weighed further.

**The fork: `c`.** Measured with a prototype of A on the 486 faces, at today's sizing (target δ/2). "Worst" is the largest certificate, in cell bounds; the headroom allows 2.

| `c` | nodes not laid | worst | faces refused | ear rebuilds |
|---|---|---|---|---|
| main | 0 | 1.06 | 0 | 128 |
| 1/16 | 2,168 | 1.22 | 0 | 128 |
| 1/8 | 4,202 | 1.39 | 0 | 128 |
| 1/4 | 7,800 | 1.66 | 0 | 140 |
| 3/8 | 11,852 | 1.98 | 0 | 166 |
| 1/2 | 15,686 | 2.21 | 6 (`CertificateExceeded`) | 170 |

At every `c` tried no triangle is flat: the least `q` on any face is 2e-3, the seam fans above.

- **`1/2`: a node's own cell** (the points nearer to it than to any other node). Nothing to choose; cells divide cleanly into the lattice's and the boundary's; best shape (heights ½ to 1½ steps). But its worst triangle is past today's headroom, so it can land only with the lemma, and it then sizes these arms finer than today (near δ/2.25). That runs against "as coarse as possible".
- **`1/ASPECT_CAP`: the shape dial the page already ratifies** (a cell no thinner than 1:16), read once more: a triangle beside the boundary no flatter than 1/16 of a step. Nothing refuses at today's sizing. Once the lemma is proved these arms could size near δ/1.25 instead of δ/2, about a third fewer triangles than today (measured, not proved).
- I lean `1/16`: the page's rule is coarseness, shape is not promised, and it adds no constant. If you want shape beside the boundary to be a property of the lattice, choose `1/2`.

**Worked example.** The union's sphere face at δ 0.04: 38 boundary points, 121 nodes, step π/12 on both axes. The node `(−π/3, 0)` lies 3e-16 from a loop vertex. On main both are mesh vertices and two triangles have `q` near 1e-15. With `c = 1/16` three nodes are not laid, the face has 232 triangles for 238, its least `q` is 0.115 and its worst certificate is unchanged at 0.42 δ.

**Ratified text I would change.**

- `crates/mesh/README.md`. The promise paragraph gains one sentence: every triangle has area (its winding is resolved); shape beyond that is not promised. A new clause states the relation both ways: a boundary chord spans at most one lattice step, and a node keeps `c` steps from every constraint line. The constants list gains the clearance as *quality*, and the headroom entry says it is derived from `c` once the lemma lands. The sentence "not a promise about triangle shape" stays; it was written in the `[ev]` S29 rewrite of the page under your review, and I did not read that thread.
- `docs/DESIGN.md` `[tolerance.chordal]` needs no change: δ still bounds distance and nothing else, and part 3 is about winding, which the mesh already promises.
- `work/tesslane/plan.md`, "The chart lane": "a run on a chart iso line gets a lattice line" stands. Under A that line's nodes on the run are simply not laid, so the line is a sizing fact, and one computed an ulp off the run does no harm.
- `check_mesh`'s doc ("combinatorial only … zero-area slivers … pass") and the STL writer's "non-degeneracy contract" are code docs from M2, not design pages. They become one statement in `mesh::validate`.

**What would move.**

- A trimmed face re-triangulates beside its boundary wherever a node is within `c`: at 1/16, 292 of the 486 faces change triangle count.
- Goldens: `d9_mesh_goldens`' `tilted_above` and `tilted_below`, and `loft_prism` and `swept_elbow` once the spline arm takes the rule; `docs/tess-budget-data`; the tour's pins on trimmed faces. All regenerable.
- Rectangle faces on the curved lane and planar faces do not move. Sizing does not move until the lemma lands.
- `check_mesh` begins refusing a hand-built mesh with a flat triangle. A face whose boundary alone makes a flat triangle refuses typed where it is emitted today.

**Confidence.** The lattice-to-boundary half is absent and is what both this issue and the lemma lack: `sure`. A `Band` is the wrong instrument: `sure`. One definition of "has area", checked in `check_mesh`: `likely`. The value of `c`: `unsure`.

## For the orchestrator

- **Checkout.** HEAD is `tesslane/orchestrator` at b0f4b4f (the merge of PR 6950), an ancestor of `origin/main`; I treated it as main. Nothing tracked was touched; `git status` is clean.
- **Shared build directory.** I built a scratch workspace into `/home/user/cad/target` (release): a copy of `crates/mesh` with env-gated instrumentation and the clearance filter, and a probe binary. Another lane had a binary of the same name (`probe`) and it overwrote mine. I ran it once by accident and saw the first lines of its output (a pose and δ with the same kind of triangle). I deleted those logs, used nothing from them, renamed mine `b8c3-lattice-probe`, and confirmed my baseline log reproduces byte for byte. Both binaries are still in `target/release`.
- **Scratch.** Everything is under `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad`: `ws/` (the workspace), `base.log` and `clear_<c>.log` (per-face lines), `summ.py`. With no env var set the copy behaves as main's and reproduces the brief's two numbers.
- **What the prototype is and is not.**
  - Analytic arms only. The spline arm (`per_cell_candidates`) was not filtered; the rule there needs each band's own steps, and its crease half would replace `Creases::cut_candidates`' snap.
  - The box is in raw chart steps read back from the candidates, not from `lattice_steps`.
  - Nodes outside the polygon within `c` are dropped too, which is harmless.
  - Refinement and ear points are not filtered.
- **Not checked.**
  - The torus instance in the brief: my 18 torus faces show no node at rounding distance.
  - Whether the box should be in the plane the CDT runs in rather than the chart. For the cone's development I assumed the chart is right, because the decision is taken `c` steps out.
  - The `(1 + c)` height claim is the one-dimensional argument. The measured worst follows it loosely (1.22 at 1/16, 2.21 at 1/2) and is not a proof.
  - That the winding census can be given a rounding-resolved verdict from `cert::rounding_allowance` at scale `M·L` (`M` the coordinate magnitude, `L` the longest side). I worked this on paper only. The accepted needle-cone meshes (`q` 1.6e-8) sit far above it and the measured triangle (1e-15) below.
  - Why four of the eight coincident faces emit nothing flat: I did not separate "exactly on the polygon" from "an ulp outside".
- **Brief errors.** The two premise corrections in "For Ev". Also, the brief says a candidate is dropped "only" at exact coincidence; the fold drop and `Creases::cut_candidates`' snap are two more.
- **Coupling to unit 5.** The lemma should be stated with `c` as hypothesis, and the unit should start by re-measuring the worst certificate under the rule. One reading for it: main's worst is 1.06 cell bounds only because no node happened to be dropped; 1.22 is what one dropped node gives at any small `c`, and a node an ulp outside the polygon gives the same on main.
- **Off the question.**
  - The thinnest triangles left in my corpus (260 under `q` 1e-2, least 2.2e-3) are fans beside a cylinder seam ruling chorded as one side across about 20 lattice rows. This is the cylinder arm's stated exception to the chord rule (`trimmed`'s module docs, "except the cylinder arm's"). It belongs with unit 1's rows, or as a note on `a-needle-cone-face-meshes-thin-triangles-beside-its-rulings`. I filed nothing, since I may not commit.
  - The ear cure rebuilds 128 of 486 faces once on main: a second full CDT build on a quarter of the faces. Clearance raises the count at `c ≥ 1/4` (140, then 166 and 170).
  - `Creases::snap` on boundary points is a typed 2⁻⁴⁰ deciding nearness. Under A its candidate half retires; its boundary half stays, and is the same class.
  - `reach_probes`' `MIN_SHAPE` (1e-12) retires into the one definition under part 3.
  - `constrain_boundary`'s node-on-a-side retry never fired in the corpus, on main or under the rule.