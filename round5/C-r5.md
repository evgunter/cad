**AGREE.** I agree with the final state the dispatch states. The narrower face's run is part of this answer, not a separate question: it is what makes the cell's reading a statement about this pair of faces. Whether tier 3's wedge check (check 4) takes the same extent is tier 3's own question. I think it should, but this answer does not need it to.

## For Ev

**Decision.** In each cell next to a shared edge or vertex, the embedding question levers its banded reading at how far the narrower of the two faces runs from what they share, measured across the edge. That number is kept once per edge or vertex on the finished body. Tier 3 decides each edge's class, and the question reads that class and never decides one. Confidence: **likely**, and all three reports agree.

**Terms.**
- **Class:** tier 3's verdict on an edge: a crease and which way it turns, a smooth seam, or a cusp end.
- **Separator:** a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- **Rate:** how fast a face leaves the separator, per unit of distance across the edge, enclosed over a cell.
- **Run:** how far a face's boundary stands from the edge's line, measured across the edge and not along it.
- **Band:** K·ε.
- **Fold:** check 4's lever today, the least of the faces' extent and their curvature radii.

**The final state.**
1. Tier 3 decides each edge's class once. The finished body keeps the class, each face's pole or apex vertex, and the lever below.
2. The question takes the finished body and decides no class. The class decides which argument may run:
   - The other face's own surface is a separator only across a crease.
   - A cusp end goes to the second-order argument, read from the verdict's `End`; no first-order separator certifies across it.
   - A smooth seam is proved through a separator both faces leave squarely, as on a capsule.
3. In each cell the question makes one banded reading, for good:
   - Add the two faces' least rates.
   - Multiply the sum by the shorter of the two faces' runs.
   - It must exceed K·ε.
4. Where tier 3 holds no fact (a vertex the faces share alone, a face's own turning), the same kind of extent is measured from that entity. A pole or apex is read by its vertex key. Tier 3 gains no new check.
5. Whether check 4's lever is the right meter is a separate tier 3 question.

**What a pass means.** Suppose the slowest rate in the cell held across the narrower face. Then the two faces would be more than the band apart at that face's far side. That is your PR 7095 test ("a wedge that opens by less than about 2ε at the faces' far side could really open the other way"), with the far side taken as the pair's. Adding the two rates makes the number independent of which separator the question chose. A plane that bisects a crease reads each face at half the wedge's angle; the sum is never less than the wedge's sine.

**Why the narrower face.** The two faces can be compared only where both exist. Past the narrower face's far side, the wider face's points are read against other faces by the distance test. `[topology.census]` already reads a meeting pair "over the shorter cell".

**The three worked cases.**

| Case | Today's lever (the farther face's run) | Check 4's lever | The narrower face's run |
|---|---|---|---|
| Curling strip | 2e-8, certifies (wrong) | 2e-11, refuses | 2e-15, refuses |
| Flat strip | 5e-5, certifies (wrong) | 5e-5, certifies (wrong) | 5e-12, refuses |
| Cone through its axis | holds | breaks near the apex | holds |

- **The curling strip:** a 1 mm cylinder face, 1e-7 m wide, leaves a 1 m plane at a sine of 1e-4, curls back and ends 5e-12 m from the plane while still leaving at 2e-8. Today's lever is the plane's metre. Only the narrower run refuses it for the right reason.
- **The flat strip:** the same, but flat, 1e-7 m wide, folded off the plane at a sine of 5e-5, with its far edge 5e-12 m away. Neither face is curved, so check 4's lever is the metre too, and it certifies a face lying wholly inside the band.
- **The cone:** a cone that owns its apex, cut by a plane through its axis. The plane leaves the cone squarely along the whole ruling, and the narrower run is one positive number for the edge. Check 4's lever is the cone's curvature radius, which goes to 0 at the apex: one number per edge refuses for good somewhere, and one per stretch is check 4 read again cell by cell.

**Ratified text to change.** `[topology.tiers]`, the sentence beginning "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach…". It is agent wording of your "presumably unified with the other case". Replace it with:

> Where faces do meet, or a face is read against itself, the question reads from the verdict what tier 3 decided there (an edge's class and turn, an end, a pole or apex vertex) and decides none of it. Over each cell it reads the rate at which the two faces part, the faces' rates summed, levered at how far the narrower of them runs from what they share, measured across it ([tolerance.epsilon]), never at a cell of its own subdivision, and that passes by more than the band. Where tier 3 holds no fact, that reading is the only one.

"Never re-deciding one over a cell" (approved on PR 6893) stays, and with this it becomes true.

**Depends on, and left open.**
- "Measured across the edge" is what ONEROUND's open question already proposes for where the reach is measured from. If that settles otherwise, the run along a long edge can still exceed a narrow face's width.
- Check 4 should take the same narrower run, since today a 1 µm chamfer tilted 1e-5 off a metre face reads as a crease. That belongs with check 4's lever question, raised to you separately.

## For the orchestrator

- **One residual, and it is placement, not a lever difference.** A would put the farther-or-narrower recipe to Ev as its own question beside ONEROUND's. For the question's own cell reading I would decide it here: it is the pair's separation, a fact the question owns. That means the verdict keeps a pair extent per edge, possibly distinct from tier 3's reach for check 4. If check 4 later adopts it, the two become one number.
- **Not checked:**
  - Whether the summed bound holds under the edge-graph separator (it assumes a separator whose gradient has unit length).
  - A torus face leaving a spiric under the narrower run.
  - The vertex-alone case: `reach_from_vertex` takes the farther face too and needs the same change.
- **Off this decision, for the measuring lane and filing:**
  - Add the flat strip to the lane's builds.
  - File: check 4's fold over-reads at a sine of 1e-4.
  - File: `wedge_reach.rs` says "the reach is the farther face's", and its "caps the lever in any case" note is stale.
- I ran nothing. All the arithmetic above is by hand.
