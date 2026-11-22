## For Ev

**Recommendation.** Lever the cell's reading at an extent of the two faces, not at check 4's lever, and change which extent: use how far the narrower of the two faces runs from the edge's line. Today's recipe uses how far the farther face runs from the edge's middle. The narrower face's extent answers both worked cases correctly and does not depend on check 4's meter, which is still open. Confidence: **likely**.

What all three reports now agree on:
- Tier 3 decides each edge's class once: a crease and its turn, a smooth seam, or a cusp end. The finished body keeps it, with each face's pole or apex vertex. The embedding question reads these and never classes an edge.
- The class decides which arguments may run. The other face's own surface is a separator only across a crease, a cusp end goes to the second-order argument, and a smooth seam is proved through a separator both faces leave squarely.
- In every cell beside a shared edge or vertex, the question makes one banded reading, and this is the end state:
  - Add the two faces' least rates of leaving the separator over the cell.
  - Multiply the sum by one lever, kept on the verdict per edge or vertex and never recomputed per cell.
  - Compare the result with K·ε.
- Where tier 3 holds no fact (a vertex shared alone, a face's own turning), the lever is the same kind of extent, measured from that entity. Tier 3 gains no new check.
- Whether check 4's lever is the right meter is a separate tier 3 question.

Terms:
- **Separator:** a surface through the shared edge that the two faces lie on opposite sides of.
- **Rate:** how fast a face leaves the separator, per unit of distance across the edge, enclosed over a cell.
- **Lever:** the length the summed rate is multiplied by before it is compared with K·ε.
- **Run:** how far a face's boundary stands from the edge's line, measured across the edge and not along it.

**The lever.** It is the shorter of the two faces' runs.
- Two faces are compared only where both exist. Past the narrower face's far side, the wider face has nothing of this pair left to part from; it meets other faces, and the ordinary distance test reads those.
- So a pass means: had the slowest rate in this cell held across the narrower face, the two faces would be more than the band apart at its far side. That is your PR 7095 test, "a wedge that opens by less than about 2ε at the faces' far side could really open the other way", with "far side" made the far side of the pair.
- `[topology.census]` already reads a meeting pair "over the shorter cell".

### The two worked cases

**A curling sliver of cylinder.** A cylinder face of radius 1 mm leaves a 1 m plane at a sine of 1e-4, curls back toward it across its 1e-7 m width, and ends leaving at a rate of 2e-8, with its far edge 5e-12 m from the plane.
- *Today's reach* is the farther face's, the plane's 1 m. The reading is 2e-8 × 1 m = 2e-8, above K·ε = 1e-8, so the first cell certifies the whole sliver. That is a face lying wholly inside the band, which geometry ε off could make cross the plane or lose its edge. **Wrong**, and the case is fair against the reach as built.
- *The shorter run* is the sliver's own 1e-7 m. The reading is 2e-8 × 1e-7 = 2e-15, so the cell refuses and splits. Its cells away from the edge then stop touching it, meet the distance test at 5e-12 m from the plane, and refuse. **Right.**
- *Check 4's lever* (1e-3) also refuses, at 2e-11. It is right here, but only because the fold happens to be smaller than the plane.

**A cone that owns its apex, cut by a plane through its axis.**
- *Check 4's lever* is the least of the reach and the curvature radii at each point. Along the ruling, the cone's curvature radius shrinks to 0 at the apex.
  - Read from below, one lever per edge is 0, so every cell beside that edge refuses for good, on a crease tier 3 passed.
  - Kept per stretch, it is check 4 read again cell by cell, which all three reports reject.
  - The fix would be a concession inside the apex's band, which is the sliver's shape again.
- *The shorter run* is one number per edge. The half-cone stands from its ruling's line up to about its rim diameter, and the plane is the shorter of the two, so it stays positive at the apex. The plane leaves the cone squarely along the whole ruling, at a rate of about 1. Cells by the apex read through the pole reading (the sine's limit) the PR already has. **Right.**

So each side's case answers the other side's lever, and the shorter run answers both.

**Ratified text to change.** `[topology.tiers]`, "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach of the faces it governs…". It is agent wording of your "presumably unified with the other case". Proposed:

> The question reads from the verdict what tier 3 decided at a shared edge or singular vertex, and decides none of it. Over each cell it reads the rate at which the two faces part, levered at how far the narrower of them runs from what they share ([tolerance.epsilon]), never at a cell of its own subdivision, passing by more than the band. Where tier 3 holds no fact, that reading is the only one.

"Never re-deciding one over a cell" stays, and becomes true.

**The other lever, at its strongest:** one meter should govern both the angle at the edge and the nearness beside it, so that when tier 3's meter is fixed both move together and the two cannot disagree on the same curvature. An extent of the faces knows nothing of curvature. A face that curls toward the other well inside its own width is caught only by the cells being split and the distance test, not by the reading itself.

**The one sentence, if you rule between them:** is the cell's lever an extent of the pair of faces (the narrower face's run from the edge), or the lever tier 3 decided that edge's class at?

## For the orchestrator

**(a) The case against my side, read concretely.**
- On the sliver, the reach as built (`wedge_reach.rs`: "the reach is the farther face's … one large face beside a small one reads the gap the large one opens") certifies at depth 0. A is right. I had argued the distance tests carry that case, but they never run, because nothing splits.
- The defect is which face's run the recipe takes, not that the lever is an extent. For tier 3's wedge the farther face is defensible: an angle robust over the large face. For the pair's separation it is not, since both faces must exist at the distance read.
- B flagged the same over-read as "the reach is the farther face's" against census's "over the shorter cell" (review 2's 20ε slab) and filed it under ONEROUND's site fork. It is a separate point from the site: from the line rather than the middle is ONEROUND's question; the shorter face rather than the farther is not.
- Under the shorter run, review 1's strip probe (0.05 wide, far edges 0.9ε apart) also refuses: 18ε × 0.05 = 0.9ε.

**(b) What moved me.**
- A's sliver moved me off "the faces' reach" as the recipe defines it, to the pair's shorter run across the edge's line.
- Not moved to check 4's lever. B's apex case stands against it, and A's report does not answer it. A's own caveat, a lever per stretch where the curvature varies, is the per-cell re-reading.
- A's "tube" argument (at L = reach the guarantee is the whole face): at L = the shorter run the reading is a real far-side statement about the pair. It extrapolates within a touching cell at either lever, and every point outside the touching cells is distance-tested.

**(d)** Given in `For Ev` above.

**For the measuring lane.** Run both worked cases a third time under the shorter run across the edge's line. The predictions are: the sliver refuses (2e-15), and the apex cone holds. On the census, list the bodies whose verdict moves between today's reach and the shorter run, with each one's narrower face. I expect thin faces beside large ones, and nothing else.

**Caveats.**
- Faces with no edge use the same rule. A vertex shared alone takes the shorter of the two faces' runs from the vertex, and this changes `reach_from_vertex`, which today is the farther face's.
- "Run across the edge's line" assumes ONEROUND's site answer. If ONEROUND keeps the middle, the along-edge over-read remains on the narrower face too: half the edge length can still exceed the narrower face's width. Say so on ONEROUND's row.
- The summed bound needs both faces measured at a common distance, which is why the lever is the min and not each face's own run. It is exact for a plane separator, close to exact for a face's own surface near the edge, and unchecked for the graph separator.
- I ran nothing. The sliver and cone numbers are the dispatch's and my arithmetic.
