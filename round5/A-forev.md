## For Ev

**Recommendation (likely): two changes.**
- The embedding question reads each edge's class from tier 3's verdict and never decides one.
- In each cell beside a shared edge or vertex, it makes one banded reading of its own: how fast the two faces part, levered at how far the narrower of the two faces runs from what they share.

**Terms.**
- **Class**: tier 3's verdict on an edge. It is a crease (with its turn), a smooth seam, or a cusp end, where the faces meet at wedge 0 or 2π.
- **Separator**: a surface through the shared edge that the question shows the two faces lie on opposite sides of.
- **Rate**: how fast a face leaves the separator per unit of distance across the edge, taken as its least value over a cell.
- **Run**: how far a face's boundary stands from the edge's line, measured across the edge, not along it.
- **Lever**: the length the rate is multiplied by before it is compared with the band K·ε.
- **Cell**: a piece of the question's own subdivision of a face.

**The final state.**
1. **Tier 3 decides each edge's class once.** The finished body keeps the class, the lever for each edge and vertex (below), and each face's pole or apex vertex. This is the hand-over you approved on PR 6893. It is not built yet: today the question takes a bare body and the finished body keeps no per-edge fact.
2. **The class decides which arguments the question may run.**
   - The other face's own surface is a separator only across a crease.
   - A cusp end goes to the second-order argument. No first-order separator certifies across it, a bisecting plane included.
   - A smooth seam is proved through a separator both faces leave squarely: on a capsule, the plane of its circle.
3. **The reading in each cell beside a shared edge or vertex.** Add the two faces' rates. Multiply by the lever: the shorter of the two faces' runs, kept once per edge or vertex on the verdict and never recomputed per cell. Compare with K·ε.
   - A pass means: had the slowest rate in the cell held across the narrower face, the faces would be more than the band apart at its far side. That is your PR 7095 test ("opens by less than about 2ε at the faces' far side") applied to the pair.
   - Adding the two rates makes the reading the same whichever separator the question picked. Today a plane bisecting a crease reads each face at half the wedge.
   - This reading stays in the end state. It is not replaced by a bare sign once tier 3 improves: it checks the faces' parting over the cell, which no fact about the edge holds.
4. **Where tier 3 holds no fact**, the question's own reading is the only one, levered at the narrower face's run from that entity. That covers a vertex the faces share alone and a face against itself. A pole or apex is read by its vertex key. Tier 3 gains no new check.

**Why this lever.** Two worked cases decide it.
- **A strip 1e-7 m wide, its far edge 5e-12 m from a 1 m plane.** One version is a curling 1 mm cylinder; the other a flat strip at a sine of 5e-5.
  - Levered at today's reach (the farther face's, 1 m), both certify a face lying inside the band. Wrong.
  - Levered at check 4's lever, the flat one certifies too: with no curvature, check 4's lever is the 1 m reach. Wrong.
  - Levered at the strip's own run, both refuse. Right.
- **A cone that owns its apex, cut by a plane through its axis.**
  - Check 4's lever (the least of the reach and the faces' curvature radii) shrinks to 0 at the apex. One such lever per edge would refuse for good a crease tier 3 passed. A lever per stretch is check 4 read again, cell by cell.
  - The narrower face's run is one positive number for the whole edge. Right.

**Text I would change.** `[topology.tiers]`, the sentence starting "That reading is the edge's wedge as tier 3 reads it, one reading for both". It is agent wording of your "presumably unified with the other case". I would have it say:

> The question reads from the verdict what tier 3 decided at a shared edge or singular vertex, and decides none of it. Over each cell it reads the rate at which the two faces part, levered at how far the narrower of them runs from what they share ([tolerance.epsilon]), never at a cell of its own subdivision, passing by more than the band. Where tier 3 holds no fact, that reading is the only one.

"Never re-deciding one over a cell", which you approved on PR 6893, stays and becomes true.

**Separate questions, not this one.**
- **Check 4's lever over-reads a face that curves back.** A 1 mm cylinder at a sine of 1e-4 against a plane through one of its straight lines: the plane cuts the cylinder again 2e-7 m away, the two part by at most 5e-12 m between the cuts, and check 4 still reads a crease.
- **Check 4 should probably take the same narrower run.** Otherwise it passes creases the question rightly refuses.
- **Where the run is measured from.** ONEROUND's open question proposes the edge's line rather than its middle, and this answer relies on that proposal.

**Confidence.**
- The final state: likely.
- The worked cases' arithmetic: sure.
- That adding the rates bounds the faces' separation: exact for a plane separator, close for a face's own surface near the edge, unchecked for the separator built from the edge's graph.
