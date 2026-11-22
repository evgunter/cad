## For Ev

**Recommendation now (likely):** no lane reads anything about the faces across its edges. Where the patches are joined, a side that two patches both make inside their triangulations is cut at its straight midpoint (the 3-D midpoint, not a point on the surface), at the join itself. No certificate runs on the cut and no lane is run again. Two more fixes have to land with it:
- **The seam fix in the chord pass.** When a straight edge and a curved edge join the same two corners, the curved edge must never be drawn as one straight segment; it gets at least two.
- **The fold fix.** A folded triangle that has no grid corner to drop gets its own inserted point.

This is designer C's cure, plus the chord-pass fix and the fold fix C found. I have switched to it from my own cure.

**What changed my mind, point by point**
- **The cut point (likely).** I had proposed a point on the face's surface, at the side's midpoint in the face's chart, with each new triangle certified. C's straight midpoint is better, and simply so. The four new triangles lie inside the two triangles they replace, which were already certified, so they are within δ with nothing to check.
  - Mine needed things C's does not: each patch carrying the chart position of its corners, or the lane run again; a certificate that could refuse; and the memo keyed by the set of sides to split.
  - My argument for a point on the surface was that the two faces would then pull apart at that side. That only matters where δ is larger than the body is thick, and there the two sheets can cross anyway.
  - C's cut leaves one cost: a new triangle can be too thin for floating point to settle its winding. It then refuses with a typed error, like any triangle.
- **Running the lane again, in rounds (B's cure): not recommended (likely).** B has the face re-meshed by its own lane with a list of sides to avoid, repeating until nothing conflicts. Inserting a point and rebuilding can change the triangulation elsewhere and start new conflicts, which is why B needs rounds.
  - B's rounds do stop, as B says.
  - But a face's patch then depends on its neighbours' patches, the memo key must hold the avoid list, and a face that cannot clear its list refuses a valid body (`UnclearedSharedSide`).
  - The straight-midpoint cut makes no side another face could share, so one pass always ends it, and nothing can refuse except the winding case above.
- **My seven-cell claim was wrong (sure, from C's measurement, which I reproduced by reading its outputs).** I took the defect on the seven refusing bored-ball cells from the issue file: the outer face's triangulation joining the bore's two rims.
  - That part is real, and the join's cut cures it.
  - But the cells also have a second defect. The bore's straight seam and the outer face's arc seam join the same two corners, each is drawn as one straight segment, so both edges are the same segment (0, 1), used by four triangles.
  - The join's count allows two uses per edge drawn on a segment, and two edges are drawn on this one, so the count passes and `check_mesh` refuses.
  - With C's chord-pass fix the arc gets a second chord point, and all seven cells mesh and pass `check_mesh`, the winding check and the volume check.
- **The fold fix (sure on the cells C measured).** Two needle-cone cells mesh today only because the shared-side point happens to land inside a folded triangle. They need a cure of their own.
- **The walk's distance test reads ε (sure; I missed this).** That test (`may_make`) decides on the run's ε band, so the first cut moves with ε at a fixed δ. That breaks "No ε is read" (`[mesh.winding]`) and the rule that ε never moves the mesh (`[tolerance.chordal]`), both of which you wrote. This is one more reason to retire the test, not to repair it.

**What did not move me**
- **The diagnosis is unchanged (sure).** Every rule that lets one face predict what its neighbour will do is the flaw. The fact is acted on at the join. All three reports agree.
- **Which patches give way: every patch that has the side inside its triangulation (unsure; it is close).** C keeps the side in one patch, chosen by face order. That order is an arbitrary fact about the body's storage and means nothing about the geometry; cutting every such patch needs no ordering rule. The cost is at most two triangles per face per side. With straight-midpoint cuts the two new points sit at one position under two ids, which is harmless to `check_mesh` and to the δ promise. Either choice is easy to change later. A face that carries the side as its own boundary edge always keeps it; that much is forced.
- **What retires.** `faces_across`, `faces_holding_side`, `may_make`, the rebuild for shared sides, `UnclearedSharedSide`, and the memo key's record of the face across each edge. After the cut, a side used more than twice per segment edge can only be a mesher bug, so `OverusedSide` becomes a panic, as `[failure.taxonomy]` requires. The case of two straight edges between one pair of corners stays its own open question.

**Text that would change.** No README design clause. Two other texts lose the neighbour-identity clause, which was written by the memo-key fix's commit `6113bc3d1b` and not in your words: the purity sentence in `lib.rs` ("…on the trimmed lane as the identity of the face across each edge") and `memo.rs`'s list of each lane's inputs.

## For the orchestrator

- **Corrections to my round-one report:**
  - The seven cells do not mesh without the chord-pass fix.
  - The stored-patch claim was wrong. In my report the memo key held the split set, so `StoredPatchId` stayed valid. Under the straight-midpoint cut, a face the join cuts is no longer its key's alone, so either it reports no stored-patch id (as C says) or the id is paired with the cut set. Ask which one the pick memo in the viewer tolerates; nobody has checked.
- **Things I think are wrong in the other reports:**
  - **B:** "the rectangle, planar and CDT lanes insert a point and rebuild" is a cure that can create new shared sides elsewhere in the face. B's termination argument holds (the avoid lists only grow, over a finite set), but B claims the rounds always end in a mesh, and they do not: a face can end on `UnclearedSharedSide`.
  - **B** assumes the seven cells mesh once the sides are cured. C's measurement refutes that (`C/scratch/cure.txt`: `(0, 1)` is used four times on unsplit sign +1 at δ 3, 1 and 0.3; `cure_floor.txt` passes `check_mesh`).
  - **C:** "`OverusedSide` goes" should be "becomes a panic": a patch that uses its own side more than twice is a mesher bug and should not pass silently.
  - **C:** the branch for an overused side that a segment edge also carries is unbuilt and never fired (0 cases). The real build needs each patch's own boundary uses, which can be read from the edge's two faces.
  - **C's chord-pass fix as probed** reads edges with exactly one chord whose ends match. A rule stated as "two edges between one vertex pair never draw the same segment" is the general form; that general rule is what should be built.
- **Still unmeasured:**
  - the other ε values (10⁻⁶ and 10⁻¹²), the slow set and other crates (the snowman row in `sweep`, the viewer's pick memo, the tour pins);
  - how often a new triangle at the join fails the winding check;
  - the both-cut variant: C's probe cut all but one patch, so cutting every patch is unrun.
- **The defects all three of us found** (`Lane::Walked`'s key leaves out the neighbour's surface; the ε dependence) disappear with the recommendation. File them only if it is not taken.
