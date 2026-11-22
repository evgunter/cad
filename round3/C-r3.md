## For Ev

**Recommendation.** All three reports now agree on what to build first:
- Tier 3 decides each edge's class (a crease and its turn, a smooth seam, a cusp end) once. The finished body keeps the class, each edge's reach, and each face's pole or apex vertex.
- The embedding question takes the finished body and never classes an edge. The other face's own surface is used as the separator only across an edge tier 3 classed a crease. A cusp is read from the verdict's `End`, and a pole or apex by its vertex key.
- In every cell beside a shared edge or vertex, the question keeps its own banded reading, as PR 7329 builds it, levered at the faces' reach. The reach is read once per edge or vertex, not recomputed for every pair of cells.
- Where tier 3 holds no fact (a vertex shared alone, a face's own turning), the question's reading is the only one, and tier 3 gains no new check.

I hold that this is the final state, not a step toward a sign-only cell. Confidence: **likely**.

Terms:
- **Separator:** a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- **Rate:** the sine at which a face leaves the separator, enclosed over a cell.
- **Reach:** how far the faces run from what they share.
- **Band:** K·ε.

### What the cell reading guarantees

The reading passes a cell when the faces' least rate, times the reach, clears the band. Two refinements:
- **Sum the two faces.** With a separator that bisects a crease, each face leaves it at half the wedge's angle. Summing makes the reading at the edge at least the wedge's sine, so the choice of separator cannot refuse a crease for good.
- **What a pass means:** every point where the faces are within the band of each other lies within the reach of the shared entity, and the faces are a band apart by their far side. That is your PR 7095 test ("a wedge that opens by less than about 2ε at the faces' far side could really open the other way"), applied wherever a cell reaches. It is not a second classing of the edge.

A face that leaves cleanly, flattens, and creeps away at a rate just above K·ε/reach still passes. That is not a hole: it is exactly the far-side test you accepted. **likely**

### The one open choice

**Is "the faces part by more than the band within their reach" a fact of the edge, or of the pair of faces?**

**(i) A fact of the pair, read by the question over its cells (my lean).**
- Tier 3 keeps a class, which its other readers need: constructors decide from it what an edge must store, and a blend reads its convexity.
- The parting depends on the faces over their whole extent, not only on what they do at the edge. The question already reads that extent, and DESIGN already makes its certificate "one fact per pair of faces".
- Tier 3's wedge check can stay a classifier, and its sampling at points along the edge, its lever and its reach stay tier 3's own concerns.
- One band is applied per question: the class's band in tier 3, the parting's band in the question.

**(ii) A fact of the edge, certified by tier 3, and the cell proves only a sign.**
- The band is applied once to one fact, which is "one reading for both" taken literally.
- It needs tier 3's wedge check rebuilt first: read over the whole edge rather than at sample points, its reach not counting the faces' run along the edge, and a meter that reads the faces' greatest parting.
- Edges beside a spline face, smooth seams, vertices and poles keep the banded cell reading for good. So two mechanisms remain, chosen by kind.
- Tier 3 would compute the parting only for the question, which is the reason all three reports gave for adding no tier 3 facts at a vertex.

Both are sound and both can be reversed. If you choose (ii), (i) is the state to build first in any case.

### Ratified text

`[topology.tiers]`, "That reading is the edge's wedge as tier 3 reads it, one reading for both…". It is agent wording of your "presumably unified with the other case". Under (i):

> Where faces do meet, or a face is read against itself, the question reads from the verdict what tier 3 decided there (an edge's class and turn, an end, a pole or apex vertex) and decides none. Over each cell it reads how far the faces are guaranteed to part at the reach of the faces it governs ([tolerance.epsilon]), never at a cell of its own subdivision, and that passes by more than the band.

Under (ii), the second sentence instead applies only where tier 3 reads no crease, and tier 3's page says the wedge holds over the whole edge.

### Separate, and not part of this choice

Check 4's lever, the least of the faces' reach and their curvature radii, over-reads a face that curves back toward the other.
- A cylinder of radius 1e-3 m meets a plane along one of its straight lines, at a sine of 1e-4.
- The plane cuts the cylinder again 2e-7 m away, and between the two cuts they part by at most 5e-12 m. Moving the plane that far removes the edge.
- Check 4 still reads a crease (1e-4 × 1e-3 = 1e-7, above the band).

Arithmetic **sure**; the replacement meter **unsure**. It should be its own tier 3 question.

## For the orchestrator

**(a) What moved me.**
- On the lever I move to the reach, which is A's and B's position. My reason for tier 3's lever was that the question could not then read a crease where tier 3 read smooth; the class gate on the carrier arm does that (my round 1). A is also right that tier 3's lever is itself unsettled. Tying the cell to it imports the fold's error, and the curving-back cases are carried by the class gate and the distance tests, not by the lever.
- I also concede A's point that my round-2 "only ever refuses on the safe side" was not argued: the reach is at least the fold, so the cell reading is the more permissive of the two. Its safety rests on the class gate plus the distance tests, which is now stated rather than assumed.
- From B: the guarantee wording. The banded reading does not exclude a slow creep, and I now say it is the far-side test, not more.

**(b) Point by point.**
1. *Lever.*
   - B's objection to tier 3's lever (a small cell bounds no actual distance) applies equally to the reach: both are extrapolations. Only the far-side test makes either meaningful, and that test is levered at the reach.
   - B's bisector objection (half the wedge, refused for good) is answered by summing both faces' separations, which neither report took up. Keep the summing under the reach too.
2. *A further end state.*
   - B's point that the band does not exclude the flatten-and-rise case is right. It is answered by reading the band as your far-side test, which the creeping face does pass by construction.
   - B's step 3 is coherent and not refuted. My objection to it is where it puts the fact, not whether it is sound: it hands tier 3 a fact about the faces' extent that only the question reads.

**(c)** As in `For Ev`. If the split remains, the sentence for Ev: *"Is 'the faces part by more than the band within their reach' a fact of the edge, which tier 3 certifies and the question only signs against, or a fact of the pair of faces, which the question reads over its cells while tier 3 keeps the class?"*

**(d) What would decide it.** B's mutant is the one measurement. On the gate harness, decide `sign_on` by sign alone across edges `topo::edge_wedge` reads `Crease`, then over the 275 census tests and the tour list every cell that newly certifies, with ψ's lower end at its far side and the pair's verdict.
- If none newly certifies with a far-side gap inside the band, (i) and (ii) give the same verdicts on every built body, and Ev's choice is placement alone.
- If any does, (ii) needs tier 3's parting meter before it is safe, which strengthens (i).

No worked case on analytic faces separates them. A jet meter with the reach folded in reads what the cell reading reads, and I found no case where trims or curvature tell them apart.

**Other notes.**
- The summing claim assumes a separator whose gradient has unit length. That holds for a plane; it is approximate for a face's own surface near the edge and unchecked for the edge-graph separator. Give it to the build's reviewer.
- Also for the build's reviewer: B's crease licence (the carrier arm's sign must match the turn the verdict holds) and A's per-entity reach kept on the verdict. Check 2 already reads every certified edge's reach (B's citation in `validate.rs`).
- I ran nothing. Every measurement cited is the reviews' or the rows'.
