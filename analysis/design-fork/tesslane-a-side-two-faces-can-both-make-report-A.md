## For Ev

**Recommendation (likely): split only the sides that two faces actually both make, once, after every face has meshed.** Each face meshes without reading anything about the faces across its edges. When the patches are joined, the existing count of which sides each face uses finds every side that two patches both use as an inside edge. Each such patch splits that side at one new point of its own surface. Its two triangles on the side become four, and those four are certified like any other triangle of that face. Today's three answers all retire: the insert-and-rebuild on the triangulating arms, the walk's distance test, and the rectangle lane's refusal. The census stays, as a check on the kernel.

### Premise check
- **The question names the wrong fact (sure).** Two of today's answers decide whether a side is safe from facts about the neighbour: which face holds its two ends, or where the neighbour's surface lies. Both stand in for the fact that matters, whether the neighbour *does* make the side. That fact exists only after both faces have meshed. The two stand-ins therefore err in opposite directions:
  - the ends-held test over-cuts. On a bored body every strip triangle joins rim to rim, so on its own it would split every column.
  - the reach test still over-cuts (the measured 256 and 440 cells), cannot tell two faces of one surface apart (the 24 band-face cells), and rests on a premise not shown at the smallest ε.
  The flaw is that the decision sits before the fact exists. It is not which stand-in to pick.
- **The walk already reads a neighbour's geometry that nothing records (sure).** The walk's distance test (`trimmed::may_make`) reads the neighbour's surface. The memo key does not fold that surface: `memo.rs` lists the same inputs for the walked lane as for the trimmed lane. The crate's purity invariant (`lib.rs`) says a neighbour never enters "as the neighbour's own geometry". So any reach-based answer starts from a purity hole that already exists.
- **"No triangulation to rebuild" is not a constraint (sure).** Splitting one side locally (one point, two triangles become four) works the same on every lane: a Delaunay mesh, the walk, the rectangle strip or a plane. The walk already does exactly this.

### The answers, as final states

**C. Split at the join (recommended).**
- What a face reads: its patch reads nothing about other faces except the shared chord points. It no longer reads who is across an edge (`faces_across`, `ChordPass::holders`) or the neighbour's surface (`may_make`). Retired along with them: `faces_holding_side`, the triangulating arms' shared-side rebuild, `UnclearedSharedSide`, and the walk's cut and its pinned cut counts.
- At the join: tessellation runs in two passes.
  - Pass 1 builds every patch, from the lane or from the memo.
  - The existing ids-only census (`side_uses`) finds each side between shared points that is used more than twice per chord. For each such side, it lists the patches that use it as an inside edge: both triangles on the side are in that patch, and the side is not a chord of that face's own loop.
  - Pass 2 hands each of those faces the set S of its sides to split. Call S the face's split set. The lane triangulates exactly as in pass 1, because S does not enter the triangulation. It then splits each side in S at the surface point over the side's midpoint, measured in the plane the lane triangulated in, and certifies the new triangles.
- Why one pass ends it (sure): the new point is private to that face. Every new side has a private end, so no other face can make it, and the split cannot create a new conflict.
- Why the halves stay within δ (likely): each half's footprint in the face's chart lies inside the original triangle's footprint, and every lane's theorem bounds deviation by that footprint. The certificate and the winding check still run on each half. A failure refuses under the lane's existing variants.
- Which faces split: every patch that uses the side as an inside edge, not just one of them. This needs no tie rule with no meaning, such as arena order, and costs at most two triangles per face per side. Splitting only one face is a defensible variant.
- Purity: a face's patch is a pure function of its own inputs plus S. S is a fact about the mesh, computed from the other patches by ids only.
  - The memo key folds S, relabelled. S is empty on almost every face.
  - The neighbour-identity fold added on 2026-11-21 (`EdgeInputs::across`) retires.
  - `StoredPatchId` names one placed patch per entry, as it does today, because S is in the key.
- What meshes:
  - the 7 rectangle-lane cells of the bored ball, where the bore is one strip;
  - the 14 cells the walk needs its cut for, where only the face that makes the side as an inside edge splits it.
  Every cell the walk over-cuts goes back to the unsplit-body count, because nothing is split where nothing is shared.
  The rule also covers cases nothing covers today:
  - a third face that meets a face only at two corners (the gap read in the cone-apex row);
  - two coplanar plane faces;
  - the spline arm, whose rule has no witness today. It now runs the same code as every other lane.
- What refuses: only a split triangle that fails its certificate or folds. Neither case is known to be reachable.
- `OverusedSide` would be possible after the split only through a lane bug: two coincident edges are already excluded because the count is per chord. So it becomes a panic, per `[failure.taxonomy]` (likely).
- Reversible: yes. The split is one function in one place.
- Objection to weigh: is this the "hidden healing" `[loud-failure]` forbids? I read it as no (likely). Nothing failed and nothing is absorbed. Each patch is valid on its own, the split is a deterministic and certified step of building the mesh, and the triangulation is left as free as you asked on PR 7046 ("any division into triangles…").

**B. The reach test on every lane.** The walk's rule (the ends-held test, then `may_make`) runs on the triangulating arms, the rectangle lane and the plane lane.
- Gains: the 7 cells mesh.
- Keeps: the measured over-cut, which grows to every lane; the 24 same-surface cells, where distance cannot decide; the premise not shown at the smallest ε.
- Requires: the memo key must fold every neighbour's surface, which widens purity to neighbour geometry.
- Worse than C on every axis but one: no second pass.

**A. The ends-held test on every lane** (the trimmed arms' rule today).
- It needs no neighbour geometry.
- It splits every rim-to-rim triangle of a strip whose neighbour holds both rims, so a bored body's bore at least doubles at every δ.
- A plane cap would gain inside points it never needs.
- Rejected (sure).

**D. No rule; the census refuses.** The simplest option, but it refuses valid bodies: the bored ball, and the 52 cone-apex cells that today's rule meshes. Rejected (sure).

### Text that would change
- `crates/mesh/src/lib.rs`, the purity invariant. The clause "on the trimmed lane as the identity of the face across each edge" would be replaced by "and the set of its inside sides another patch also makes". An agent wrote this clause, in commit `6113bc3d1bd` for PR 7241, not you. Its root "ratified via PR #32" stays.
- `memo.rs`'s per-lane input lists, to match.
- No README design page changes. `[mesh.sizing]` is untouched.

## For the orchestrator

- **I measured nothing.** I read the code: `tessellate.rs` (`joined`, `side_uses`, `overused_side`, the place-and-join loop), `trimmed.rs` (`attempt`, `emit`, `faces_holding_side`, `faces_across`, `may_make`, `walk_face`), `memo.rs`'s docs and `EdgeInputs::across`, `lib.rs`, the mesh README, DESIGN's `[failure.taxonomy]` and `[loud-failure]`, and the rows the brief names. The claim that C meshes the 7 and the 14 cells is reasoned, not run. The nearest evidence: the walk's own cut is the same local split, and it meshes the split bored-ball cells.
- **To verify before building:**
  - that the curved lane's certificate passes the neighbour-side split at coarse δ on the 0.05 shell. These are halves within one column, so I expect yes;
  - the cone development's midpoint. The split point must be the midpoint in the development, apex `u` handled as `walk_face` does;
  - a grid like the 7198 review's, comparing C against main and the branch for triangle counts and refusals.
- **Defect found (file it):** the walked lane reads the neighbour's surface through `may_make` → `tessellate::face_surface`. The memo key does not fold that surface. `memo.rs` lists the walked lane with the trimmed lane's inputs, and `lib.rs` says a neighbour never enters as its geometry. A plausible stale hit: change a torus neighbour's bulge while keeping both rims. That may leave the bore's key unchanged and replay a patch cut or uncut for the old neighbour, so the census refuses where a fresh run meshes. This is a wrong refusal, not a wrong `Ok`. Unmeasured. Under C the read goes away.
- **Stale comment:** the comment before `joined()` in `tessellate.rs` says "No lane-produced body is known to reach the census", but the unsplit bored ball reaches it on 7 cells (row `a-strip-against-…`).
- **Implementation shape I assumed:**
  - `Patch` is in `PatchVertex::{Shared, Local}` terms, so the census can run on unplaced patches before placement. Split points then mint as the face's own `Local` points at its arena turn, which keeps the mint order D9 requires.
  - Pass 2 reruns only the faces with a non-empty S. The triangulating arms' rebuild must not be used for the split: a rebuild can flip sides elsewhere and create new conflicts, so it gives no one-pass guarantee.
- **Rows that move:**
  - `split_rulings::holds_the_relation`: cut counts go to zero where nothing is shared;
  - `a_side_whose_reach_the_band_cannot_decide_is_cut`: retires;
  - `reach_probes` `…keeps_no_side…`: restate them as "the joined mesh has no four-use side";
  - `side_census` rows: become assertion rows;
  - the triangulating arms' rebuild retires, so trimmed faces' bytes move wherever it fired.
- **Rows C would close:**
  - `a-walked-sides-reach-exclusion-is-read-only-by-the-walk` (moot);
  - `the-shared-side-rule-on-the-spline-arm-has-no-witness` (one code path for every lane; the witness is any lane);
  - `a-strip-against-a-neighbour-holding-both-rims-…`;
  - and this question's row.
