## For Ev

**Recommendation (likely): no face reads anything about the faces across its edges. Where the patches are joined, a side that two patches both made is divided at its midpoint in all but one of them.** One answer, in one place, for every lane. A fork remains on where the new point goes (answer A against answer E below); I lean A.

Terms. A *patch* is one face's triangles. A *shared point* is a vertex or edge chord point that two faces hold under one id. A *side* is a triangle edge. The *join* is where `tessellate` puts the patches into one mesh (`tessellate::joined`); the *census* is its count of how many triangles use each side between two shared points. A *lane* is one way of meshing a face: planar, rectangle (`curved.rs`), the trimmed lane's triangulating arms, its *walk* (a ruled face written row against row), the spline arm.

### What the issue is evidence of

An edge of four triangles is a fact about two patches together. Each face alone made a valid patch. Today each lane tries to predict the collision from its own side, and nothing a face can read alone decides it:

- "one neighbour holds both points" (the triangulating arms) is true of every rim-to-rim side of a bore, almost none of which collide;
- "the neighbour's surface is within reach" (the walk's `may_make`) is closer and still wrong, and it costs three things the page forbids or the code has not paid for (next section).

The exact fact, "the other face did make this side", exists only at the join. The census there already computes it and then refuses. So the join should act on it.

### Corrections to the premise (each measured or read on the tree)

1. **The seven refusing bored-ball cells hold two defects, and no answer to the side question alone makes them mesh.** In all seven, the sphere or torus face's arc seam and the bore's line seam join the same two vertices and each takes one chord, so both are the one segment (0, 1), used four times. The census passes that (two uses per chord) and `check_mesh` refuses it. Measured: with every shared side cured, all seven return `Ok` and fail `check_mesh` on (0, 1). (sure)
2. **The walk's reading makes the mesh depend on ε.** `may_make` decides on the run's ε band. The tree's own row pins the first cut at δ 0.036955750 at the default ε, 0.036947758 at 10⁻⁶, 0.036955758 at 10⁻¹². `[tolerance.chordal]` says an ε flip can never move the emitted mesh with δ fixed, and `[mesh.winding]` says "No ε is read". (sure)
3. **The walk's reading reads the neighbour's surface, and the memo key does not hold it.** The key folds which face lies across each edge, not what surface it has; `lib.rs` says a neighbour enters "never as the neighbour's own geometry". Read in the code; I have no witness body. (sure of the read, unsure of a reachable wrong replay)
4. **The triangulating arms' rule is now counted.** Over the mesh crate's default suite it inserts 997 points in 103 rebuilds of a face. The sides two faces actually both make there number about 502, in 44 meshes. (sure of the counts; they are not like for like, since one insert can clear several sides)

### Answer A (recommended): the join divides the side

Every lane meshes its face freely. At the join, for each side between two shared points that more patches make as an interior side than the mesh can hold, every such patch but one has its two triangles on that side split at the side's 3-D midpoint, a new point private to that patch. The side stays whole in one patch (in none, where an edge's chord already carries it).

What it makes true:
- **Exact.** A cut happens only where two faces did make the side. Measured with every lane rule switched off and this cure on, over the default suite: every row meshes and passes `check_mesh`, winding and volume, except rows that pin today's cut counts and two needle-cone cells (below). The walk rows: 406 cuts where today's rule makes 1,211. Split bored ball at δ 0.1: 134 triangles, against 220 today.
- **The δ promise is untouched without a certificate.** The four new triangles are pieces of two triangles already certified; the face's sheet is the same set of points.
- **Every lane is covered alike**: planar, rectangle, walk, triangulating arms, spline (which has no witness today), and a third face meeting at corners only, which today's rules miss.
- **Per-face purity is whole again.** A patch is a function of its own face and the shared chord points. The face-across entry leaves the memo key; ε leaves the mesh.
- **Less code and two fewer refusals.** `faces_across`, `faces_holding_side`, `may_make`, the rebuild-on-shared-side loop, `UnclearedSharedSide` and `OverusedSide` go.

What it leaves possible that should not be:
- The two sheets still lie on one segment there: the cut patch has a vertex on the other patch's edge. The mesh is a closed 2-manifold by its indices, not a surface free of self-contact. That is already so today: the walk's cut of a ruling puts its point on the same line, and nothing promises that two sheets of a body thinner than δ do not touch or cross.
- A child triangle can be too thin for f64 to wind where its parent just passed; it refuses typed, as any triangle does.
- A face the join cut reports no stored-patch id, since its triangles are no longer its key's alone.

Reversible: yes. It is one function at the join; answer E reuses its detection.

### Answer E: the join detects, the face's own lane cures

As A, but the face chosen to give way is meshed again with the side forbidden, and puts a new point on its own surface. The sheets then part at that side. Costs: every lane needs a way to be re-entered and to cut; the new triangles need the face's certificate and can refuse; a face's patch becomes a function of its neighbours' patches, so the memo cannot key it alone. For a planar face and for a ruling the on-surface point is the midpoint anyway, so E differs from A only on curved sides, in a regime (δ past the local thickness) where the sheets can cross regardless. I would not build it unless you want the sheets parted geometrically.

### Answers I would not take

- **Every lane predicts by reach** (the walk's rule everywhere): keeps corrections 2 and 3, over-cuts (it cannot exclude a neighbour on the same surface, and a spline neighbour has no closed-form distance), and still needs the census behind it.
- **Every lane predicts by holding** (the arms' rule everywhere): cuts every rim-to-rim side of a strip beside a neighbour holding both rims.
- **Refuse only**: valid bodies refuse at ordinary δ. 59 meshes in the default suite need a cut, Boolean slivers among them.

### Two things the answer needs beside it

- **Two edges joining one vertex pair never chord to one segment: the curved one takes a second chord.** A floor in the chord pass, beside its existing short-loop floor. Measured: with it and answer A, the seven bored-ball cells mesh and pass `check_mesh`, winding and volume, with 10 to 21 cuts each. (sure) Two straight edges on one segment, a body that really is pinched, stay the open question they are.
- **A folded triangle with no lattice corner gets its own inserted point.** On a 1° needle cone cut near its apex, two cells mesh today only because the shared-side insert happens to land in a folded triangle. With the lane rule off they refuse `FoldedTriangle`; with the fold given the same insert they mesh again. (sure, on those cells)

### Ratified text

- `crates/mesh/README.md`: no clause changes. A lays no rule on how a lane triangulates, as you asked on PR 7046. If you want what the join does stated, it is one sentence of what a mesh is: every edge is a side of exactly two triangles, and a side two faces both make is divided in one of them.
- `lib.rs`'s purity sentence and `memo.rs`'s per-lane list (written by the memo-key unit's commit `6113bc3d1b`, not in your words as far as the log shows) lose the face across each edge.
- `[tolerance.chordal]` and `[mesh.winding]`'s "No ε is read" (written in an `[ev]` change, `5241293306`) stand; A brings the code back under them.

## For the orchestrator

**How measured.** Throwaway env-gated edits in my worktree, since reverted (`git status` clean); the diff is `/tmp/claude-0/-home-user-cad/b8c34796-b049-500f-a1b1-501f0fcd1db9/scratchpad/fork3/C/scratch/probe.diff`, outputs beside it. Switches: `NO_LANE_RULE` (`faces_holding_side` answers none, which disables both the arms' rule and the walk's cut), `COUNT_RULE` (counts today's inserts and cuts), `JOIN_PROBE=cure` (the cut in `joined`), `FOLD_INSERT`, `CHORD_FLOOR`. Run with `cargo test -p mesh --test all`, default profile, default ε only, shared target (about 3 GB added).

**Numbers.**
- Baseline, whole default suite: 103 rebuild rounds inserting 997 points; walk cuts 1,297 on 66 faces; the only overused sides are the unsplit bored ball's 21 runs (7 cells, 3 readings).
- Lane rules off, join cure on, whole suite: 302 of 307 pass. Failures: `route_rows::a_split_side_body_records_the_walks_decisions` and three `split_rulings` rows (pins of today's cut and decision counts), and `reach_probes::a_needle_funnel_cut_at_its_apex_refuses_typed` (h 10⁻³ above, δ 10⁻³ and 2·10⁻⁴: meshed, now `FoldedTriangle`; h 10⁻⁵, δ 2·10⁻⁴: `UnclearedSharedSide` becomes `FoldedTriangle`).
- With `FOLD_INSERT` as well: no cell changes between meshed and refused. Two rows fail only on the refusal's name, because the probe reuses the shared-side channel for the fold.
- `split_rulings` with `PRINT_CUTS=1` (645 cells, `check_mesh` and geometry still read): 1,211 walk cuts today, 406 join cuts in 28 meshes.
- Rest of the suite: 502 join cuts in 44 meshes (slight undercount: two tests stopped early).
- The D9 golden rows passed in every mode, so no golden body meets the rule.

**What the probe does not show.**
- It keeps the side in the first patch in arena order and cuts the rest. The branch for an overused side that a chord carries never fired in the suite (0 cases) and is unbuilt in the probe; a real build needs each patch's own boundary uses, from the chord pass's holders.
- Unit tests in the lib, the slow set, ε 10⁻⁶ and 10⁻¹², and other crates (sweep's snowman row, the viewer's pick memo and tour pins) were not run.
- The cut point is the chord midpoint only; answer E was not prototyped.
- The key's face-across field was left in place during the probe; the memo rows passed with it.

**Errors in the brief.**
- "The rectangle lane refuses a valid body" on seven cells: true, but curing the sides alone leaves `Ok` with (0, 1) used four times. The second defect is the chord pass's and the census's two-per-chord allowance.
- The memo row cited (`the-patch-memo-key-misses-the-faces-across-a-trimmed-face`) is closed: the key folds the neighbour's identity. The live gap is the neighbour's surface, which only the walk reads.

**Defects off the question, to file if A is not taken (A removes the first two).**
- `Lane::Walked`'s key omits the surface of each face across, which `walk_face` reads through `tessellate::face_surface` for `may_make`. A plausible witness, not built: one bore between the same two rims, against two different outer surfaces through them (a sphere and a flatter torus arc), at a δ where the rims take the 8-chord cap and `1.25·δ` falls between the two distances.
- `may_make` moves the mesh with ε. `lib.rs` says mesh structure is a function of (body, δ) "for every body in the tree"; `split_rulings::a_side_whose_reach_the_band_cannot_decide_is_cut` is a body in the tree where it is not. `SizingTols::band`'s doc says the read is not counted by the ε inventory pin.
- A line and an arc between one vertex pair, each of one chord: belongs with `work/tesslie/two-coincident-edges-between-one-vertex-pair-mesh-non-manifold.md`, but it is a different case (the body is not pinched; only its coarse chords are) and has the chord-floor cure.
- A boundary fold has no cure of its own; relates to `work/tesslane/a-needle-funnel-cut-at-its-apex-refuses-folded.md`.

**Assumed.** That `check_mesh`'s closed 2-manifold by indices is the definition of watertight the project wants (the coincident-edges row calls coincident positions under distinct ids "a combinatorial distinction `check_mesh` already admits"). If Ev wants a mesh free of self-contact, neither A nor today's walk gives it, and E gives it only partly.

**Provenance.** The checkout is shallow, so `git log -S` found only the writing commits named above; I read no PR pages.
