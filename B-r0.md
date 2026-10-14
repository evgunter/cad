## For Ev

**Recommendation (likely).** Make Boolean semantics the *definition* of a blend now, and build it in stages. A convex band's result is `S − R` and a concave band's is `S ∪ F`. `R` (or `F`) is the band's region: the material between the band surface and the two support faces' carriers, between the band's ends. The band and its supports stay in-place surgery, as today. Everything that meets the region without crossing the band surface gets built by surgery, and the cases that need a void door use `topo`'s. That covers features wholly inside, closed shells, and holes whose mouths a fill seals. A feature that crosses the band surface refuses *as unbuilt*: the result is defined, the kernel doesn't build it yet, and the refusal offers a reorder where one exists. When the crossing arm is built, it should be the hybrid (C3), never the literal Boolean (C2). The op-agnostic lint is built under every option.

**Premise corrections (first).**
1. *The A/B/C fork mixes up the rule with the mechanism.* "If we take C1, don't aim for Boolean semantics" only holds if crossings are refused **for good**. A refusal never contradicts a definition. It is a coverage gap, like the Boolean's own `RimCuspArmUnbuilt` or `TangentSlitArmUnbuilt`. With crossings unbuilt for now, A and "Boolean semantics" build the same set except for one case (S, the sealed mouth, below). So the real decision is (i) whether we commit to the definition, and (ii) whether the sealed mouth seals. Under C1-as-rule ("refuse whatever touches the region, except what we know how to carry"), the meaning is whatever the surgery manages to carry. That is the state to avoid. (sure)
2. *Boolean semantics is not "blend before drilling".* Subtraction commutes with subtraction, and union with union. So a band agrees with blending first only for features of its own sign:
   - convex band, hole: `(S−H)−R = (S−R)−H`. Equal.
   - convex band, boss P: `(S∪P)−R` trims the boss, while blending first keeps it whole. Different.
   - concave band, protrusion: `(S∪P)∪F = (S∪F)∪P`. Equal.
   - concave band, hole whose mouth the fill covers: `(S−H)∪F` seals the hole, while drilling after the fill drills through it. Different.

   So reordering is not a workaround that reaches the same answer. It is how the user picks the *other* answer in exactly the two cases where intent is ambiguous. That is a reason to want both: Boolean semantics as the rule, and reordering as the explicit alternative. (sure)
3. *The recourse: how clunky the reorder is depends on where it is written, and it does not exist at all for one common style.*
   - **In a script it is the one-line reorder you expected**: `fillet(subtract(body, hole))` becomes `subtract(fillet(body), hole)`. One caveat: **the edge selection has to be re-queried on `body`.** A selection is frozen name text (`Node.fillet`/`Node.chamfer` in `crates/pncad-py/pncad.pyi`). An edge on the subtract's output is named *by the subtract*: its node is the subtract, wrapping the original name in `From{read, of}` (`names/emit_topo.rs`; N1 in `names/README.md`). So the old text names nothing on `body`, **even for edges the hole never touched**. A selection written as a query (`ev.select_where(body, …)`) just works. Pasted names do not.
   - **When the hole split the selected edge**, `body` has one parent edge where the subtract's output had fragments. Re-querying selects the parent, and filleting it whole before drilling is almost always what was meant. Selecting only *some* fragments has no counterpart before the drill.
   - **How often the split case arises**: only when the feature cuts the filleted edge itself, such as a slot or a hole breaking out through the corner. There the band ends at the feature's walls, a different refusal (`CornerConfig`) from the meter refusals this fork is about. In the meters' crossing case (a hole near the edge whose wall pokes into the band's strip), every selected edge is carried whole, and only the re-query is needed. (likely; read from the code, not run)
   - **On an existing document in the editor it is clunky.** It takes 2 operand writes plus one per reader of the fillet (`DocEdit::SetParam`, DM6 in `crates/editor-core/REFERENCES.md`). Every selected name then strands with `Took::Reach`, because its minter (the subtract) is no longer upstream (`edit.rs`, `stranded_by_repoint`), and each needs a `Rebind`. A client action could do all of this as one step: re-point, then unwrap each `From{read: <subtract's A seat>, of}` to its `of`. That is mechanical and exact for carried edges, and DM6 permits it as a client's offer of explicit writes.
   - **No reorder exists when the obstacle and the filleted edge come from one node.** A hole loop in an extruded profile is one example. The rocker's keyhole is another: its creases and its slot end are one extrude (`demos/tour/src/rocker.rs`). There the user has to rewrite the profile so the hole becomes a downstream subtract, in a script or through `SetProgram` (which strands the hole's names). (likely)
4. *"Crossing" is wider than foreign features.* Under Boolean semantics, every face other than the two supports that meets the band surface is a crossing. That includes a neighbouring face the band's strip overruns, which today is C8's "r vs adjacent-face extent" validity predicate. Under the definition, that refusal becomes "unbuilt" rather than "invalid". The literal Boolean would build it as an overflow, where the band cuts the next face at a sharp edge. (likely)
5. *The definition settles an open question for free.* A feature wholly inside the **union** of a mitre's two bands goes with them: `S − (R₁ ∪ R₂)`. Today it reads as a crossing (`band_planar_mitre`; the step-1 findings in `work/band/ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver.md`).

**What the user sees**

| Case | Convex band (removes R) | Concave band (fills F) |
|---|---|---|
| Feature wholly inside, attached by cycles on the faces bounding the region | Builds. The feature goes (through-bore: merged; blind bore, dimple, boss: owed) | Builds. A protrusion is absorbed |
| Closed shell wholly inside | Builds. A void goes, and an island inside that void goes with it | Builds. An island is absorbed, and its voids become the solid's |
| Hole mouth sealed (the mouth lies wholly on the face F is glued to) | No such case | Builds sealed. A through-hole becomes blind; a blind hole becomes a void born through `topo::insert_void` |
| Anything crossing the band surface (a wall poking in, a boss partly in, a mouth straddling F's footprint, a strip overrunning its face) | Refuses as an unbuilt crossing, naming the face. Recourse: "move ⟨node⟩ after the blend" (in a script: reorder and re-query the edges on the earlier body) when the obstacle's node is downstream of the edge's minter; otherwise change the radius or move the feature | Same |

The **lint** is a fifth resident of the advisory-check registry (`crates/editor-core/src/checks.rs`, closed `CheckId`). It reports and never gates. It is op-agnostic and runs on every node. It reports (a) changes in solid count, void count and per-solid genus (through-holes), derived from the body's Euler–Poincaré counts, and (b) names that vanished because of the node's verdict (N5's `Vanished{PredicateFlip}`, the path step 2 already uses). Example: "fillet #12 made the through-hole ⟨name⟩ blind (genus 1→0)". A union that seals a hole, or a subtract that empties a void, reports the same way.

**Alternatives rejected, as final states.**
- **A (C1 as the rule, plus a reorder action).** Simplest kernel. All of steps 1–4 survive. The script recourse is cheap (premise 3). But the meaning is "what the surgery carries". The sealed mouth refuses (S1). The recourse is missing wherever the obstacle shares a node with the edge. For opposite-sign features the reorder gives a different body, not the refused one (premise 2). It is reversible into the recommendation, since it builds a subset of it. The cheap reorder is also what makes leaving crossings unbuilt tolerable under the recommendation.
- **C2 (the blend is a literal Boolean against a body for R).** The most uniform on paper. It deletes the feature meters and `consume.rs`, so none of steps 1–4's consumption survives. Its costs are real:
  - The tool has a **wedge-0 cusp along both trimlines**. `docs/DESIGN.md` tier 3 lists sector classification among the consumers that refuse a cusp, and the Boolean's cusp rim arm is unbuilt.
  - Tangency the surgery knows by construction would be re-proved by verification.
  - The Boolean's tangency side certificates exclude torus–cylinder and torus–torus (`crates/topo/README.md`, "Which tangencies count"). A torus band on a cylinder support is an ordinary concave fillet at a boss's foot, so that is a likely coverage regression. (likely)
  - `R` has to be built as a closed certified body at corners, turns and ends, which the surgery never closes.
- **C3 as the crossing arm (recommended for later).** Surgery keeps the band–support tangency. The Boolean's intersection machinery cuts only the band face against the faces that cross it. Ownership is clean: the blend owns what it constructs, and the Boolean owns what the blend meets but did not make. It is a new operation and needs its own design. A known hard spot is a hole mouth crossing a trimline, which puts a vertex on a smooth seam. Every approach meets that spot.
- **S1 (refuse a sealed mouth) under the definition.** It contradicts the rule for no soundness gain. The objection that it is "a fourth birthplace of cavities" goes away if the seal calls `insert_void`, the door that convention says is callable with caller-certified containment.

**Ratified text to touch.**
- `docs/DESIGN.md` "every CAVITY is born through the shared void-insertion door" lists three producers. Add the blend's seal as a fourth caller of the same door. The convention stands.
- C8 (`crates/geom-brep/README.md`): face clearance's *crossing* verdict becomes a coverage refusal, not a validity one.
- `crates/sweep/README.md` predicate 2: state the definition.

**Order of work.**
1. Record the definition: the item on PR 4963, C8, the sweep README.
2. The concave bar: step 3's carve with the side flipped.
3. One-cycle members: blind bore, pin, boss, dimple.
4. Whole shells, through a shell-removal door beside `insert_void` that shares its bookkeeping. Under C3 the region is never needed as a body, so there is no reason to build one for the containment fallback.
5. The seal.

The lint and the reorder client action run in parallel with any step. The reorder action is a client offer of the explicit slot writes plus the `From`-unwrap of the selection, which DM6 permits. The crossing arm (C3) stays an unscheduled frontier.

**Confidence.** Definition: likely. C3 over C2: likely. Recourse findings: likely (read from code, not run). Sealing now: likely.

## For the orchestrator

- **Errors in the PR thread and brief.**
  - The 2026-10-14 comment says edge selections "should still resolve, because those edges come from upstream of the hole". That is false for *every* edge selected on the subtract's output. The name's node is the subtract (`From` wrapper), so the re-point strands all of them with `Took::Reach`, and split edges have no one-to-one counterpart at all. Ev's fifth comment is answered in premise 3. Its upshot: cheap in a script with a re-query, clunky in the editor without an unwrap action, impossible for profile-born holes.
  - It also omits the per-reader re-points and the profile-born case.
  - The local checkout (`band/orchestrator`) holds the **old** DM6 text ("Splice is not added"). Main's DM6 is "No edit infers a re-point". Read from `origin/main`.
- The work item on main is 486 lines. The working tree's copy is the 80-line pre-step version.
- **Unchecked:**
  - Whether the Boolean's seam verification (as opposed to the one-sided cover's side certificate) covers torus–cylinder. I cite the side-certificate exclusion only.
  - Whether the Boolean would accept a cusp-edged tool at all. I inferred refusal from tier 3's consumer list and `RimCuspArmUnbuilt`.
  - Whether `topo` exposes per-shell genus directly.
- **Assumed:** "the region" is defined from the supports' carriers and the band's ends, independently of the features near it. Any rolling-ball reading on the featured faces is rejected as unpredictable.
- **Not filed, may deserve an item.** Reorder recourse does not exist for profile-born obstacles. A refusal's recourse sentence ("blend before drilling") is wrong for them today. Grep `FILLET3_*_RECOURSE` in `crates/sweep/src/blend/mod.rs` for the sentence. I did not open it.
