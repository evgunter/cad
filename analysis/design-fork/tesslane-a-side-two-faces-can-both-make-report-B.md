## For Ev

**Recommendation (likely): a face should avoid a side only when another face actually makes it, and that should be decided where the patches join.** A side here means a triangle edge between two shared boundary points.
- Every lane meshes its face from the face's own inputs. Today those are the face's geometry, its chord points, δ and ε. The lanes then read nothing about a neighbour beyond the chord points they share.
- The census where `tessellate` joins the patches (`tessellate::joined`) stops being only a backstop and becomes the detector. When a side has more than two triangle uses per chord, every face that uses it as an **interior** side reruns with that side on a list of sides to avoid. A face that has the side as a boundary chord keeps it.
- Each lane avoids a listed side its own way:
  - the CDT arms, the planar lane and the rectangle lane insert a point and rebuild (all three triangulate with constraints);
  - the walk cuts the side, as it already does.
- This repeats until no side is overused. The avoid lists only grow and there are finitely many shared pairs, so it stops, and the result is deterministic.

This replaces all three of today's rules: `faces_holding_side` on the CDT arms, `may_make` on the walk, and nothing at all on the rectangle lane. `OverusedSide` stays only for the separate coincident-edges case. A face that cannot clear its list still refuses `UnclearedSharedSide`.

### Premise check (sure)
1. **The memo-key row is already closed.** PR 7241 made "the face across each edge" a declared input, and the memo key folds it as `EdgeInputs.across`. The purity gap that is still open is a different one. `may_make` reads the neighbour's **surface** (`face_surface(body, f)` in `walk_face`). The key does not fold that, and `lib.rs`'s invariant says a neighbour enters "never as the neighbour's own geometry". So the walk's rule breaks the ratified purity invariant today.
2. **"Where a lane has no triangulation to rebuild" covers only the walk.** The rectangle lane runs a constrained triangulation in UV (`curved.rs`) and can insert a point exactly as the CDT arms do.
3. **The question is a symptom.** The flaw underneath is that each lane predicts what its neighbour's mesh will do. One rule reads who holds the two points; another reads where the neighbour's surface lies. Neither prediction is the fact that matters: whether the neighbour makes the side. That fact exists only once both patches exist, which is at the join. So the asked-for choice of rule is the wrong thing to choose between. The other option is to stop predicting.

### The answers, as final states

**A: the side is avoided only where it is actually made, decided at the join (lean).**
- Each patch depends only on its own lane's inputs plus its avoid list, a declared input that the memo key folds as pairs of chord ids. Those ids are already folded as a relabeling.
- No lane reads a neighbour's identity or geometry. The `across` fold, `faces_across`, `faces_holding_side` and `may_make` all retire. The invariant then holds without the exception it has today.
- A patch is cut only where a conflict actually happened, so nothing is over-cut. That ends the 256 to 440 cells measured denser than main and the one-chord band case. It also ends the unshown premise at the smallest ε, and the CDT arms' uncounted extra insertions.
- The seven unsplit bored-ball cells mesh instead of refusing.
- Worked example, the 1.05 ball split, at δ 0.1: the sphere face lays a row between the rims and nothing conflicts, so there is no cut and the density matches main, where today the walk cuts. At δ 1: both faces join the rims, the census finds the shared sides, and both faces rerun with those sides on their lists. The bore's walk cuts them and the outer face inserts points, so the mesh is manifold.
- What is left possible: a face meshes twice where it conflicts, and the memo holds both versions. The mesh is still a function of the body and δ, and `tessellate_with` stays byte-identical, because the rounds run the same way on memo hits.
- Reversible: yes. The detector and the avoid list are one seam.

**B: one rule for every lane, "one neighbour holds both points".** This is today's CDT test, applied everywhere.
- The patch stays a function of inputs the key already folds.
- But it over-cuts much more than today: every walked or rectangle face whose neighbour holds both rims gets cut at every δ, which covers every bored body. It also leaves the CDT arms' insertions uncounted.
- It is the measured flaw made universal. Not recommended.

**C: one rule for every lane, "the neighbour could reach the side"** (`may_make` everywhere).
- The memo key would have to fold each neighbour's surface, which widens the key and makes the invariant state a neighbour-geometry exception.
- It still cuts beside a same-surface neighbour, because distance cannot tell two faces of one surface apart. It still over-cuts at finer δ, and it keeps the premise that is not shown at the smallest ε.
- Not recommended.

**Who avoids the side (a sub-choice inside A; unsure).**
- The rule I lean to: **any face that uses the side as an interior side**, never a face whose boundary carries it. That much is forced. In the unsplit bored ball the overused side is the bore's own seam, which the bore must keep.
- The open part is when both faces use it as an interior side. Both rerunning needs no rule and costs one extra cure per conflict. Only one rerunning saves that cure but needs a deterministic choice of face that means something, and I found none that is not arbitrary.
- I lean to both. Changing this later is local.

### Ratified text this changes
- `crates/mesh/src/lib.rs`, the invariant's neighbour sentence ("…and on the trimmed lane as the identity of the face across each edge"). It would become: a neighbour enters only through the shared chord points, and through the avoid list the join hands a face. That clause was written by an agent in commits `4e30668774` and `6113bc3d1b` (PR 7241, 2026-11-21). I could not see whether you approved it in your own words. The core "per-face purity" invariant (PR #32) is unchanged and gets stronger.
- `memo.rs`'s per-lane input lists: the trimmed lane's `across` entry goes, and an avoid list is added on every lane.
- `ruled_walk.rs` "No edge of four triangles" and `trimmed.rs`'s cure section are rewritten to match.

### Confidence on the load-bearing claims
- *Every lane that makes triangles can avoid a given side:* likely. The walk already cuts. Planar, rectangle and the trimmed arms insert a point and rebuild, and on the plane that is always certifiable. On curved arms the children are certified as every triangle is.
- *The rounds stop and are deterministic:* sure. The avoid lists only grow over a finite set of pairs, and each round is an indexed map plus an arena-order fold, as today.
- *A stops all over-cutting:* sure, by construction. It cuts only where the census sees more than two uses.

### Cost (separate from the ranking)
`tessellate_impl` needs a second round between the map and placement. Conflict detection can read the patches before placement, because shared ids are already the boundary points. Rows that pin cut counts (`split_rulings::holds_the_relation`, the `reach_probes` `…keeps_no_side…` rows) and `UnclearedSharedSide` rows re-baseline. Unit 6 (rectangle faces move to the chart lane) is independent of this choice: it removes one lane but does not decide the rule.

## For the orchestrator

- **Errors in the brief.**
  - It says the memo key "does not hold" the faces across. That row (`the-patch-memo-key-misses-the-faces-across-a-trimmed-face`) is closed: PR 7241 folds `across`.
  - It implies the rectangle lane has no triangulation. `curved.rs` runs a constrained triangulation in UV.
- **Defect found (P2 suggested, unmeasured): `may_make` makes the walked patch depend on the neighbour's surface, which the key does not fold.**
  - `trimmed::walk_face` calls `tessellate::face_surface(body, f)` for each face across. `memo::FaceInputs` folds `across` only as an identity relabeling. This contradicts `lib.rs`'s "never as the neighbour's own geometry".
  - Candidate witness: the split bored ball, swapping the profile sign (torus versus sphere through the same two rims) at a δ where the bore's chord points come out equal. A memo replay could then keep a cut a fresh run would not make, which breaks `tessellate_with`'s byte-identity, or miss one, which gives a wrong `OverusedSide`.
  - Not run. It should be filed under `work/tesslane/` if the recommendation is not taken.
- **Stale comment.** The comment in `tessellate.rs` (`tessellate_impl`, above `joined(...)`) says "No lane-produced body is known to reach the census… held by reading". The unsplit bored ball's seven cells do reach it (`OverusedSide`, per the strip row).
- **Not measured.** I ran nothing. Every density and cell figure is the brief's or the rows'. The CDT arms' over-insertion count is still unknown; option A makes it moot rather than measuring it.
- **What I assumed.** I assumed the outer face's lane in the unsplit bored ball does not matter. Under A any lane that uses the side as an interior side reruns. I did not check whether `curved`'s pole-fan census (`identified_ids`) tolerates an inserted interior point next to a pole. The point is strictly inside a triangle, so it should, but this is unchecked.
