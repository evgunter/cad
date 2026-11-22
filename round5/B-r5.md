AGREE.

1. I agree the final state is as the dispatch states it.
2. No difference is left on my side.
3. Whether tier 3's wedge check takes the same extent is not part of this answer. It should go to Ev as a tier 3 question of its own, beside the one about that check's curvature lever.

## For Ev

**Decision asked for** (`likely`). Where two faces of a solid meet, tier 3 decides what kind of edge it is, once. The embedding question reads that and adds one reading of its own in each cell: how fast the two faces part, levered at how far the narrower face runs from the edge, against the band K·ε.

**Terms.**
- *Tier 3*: the per-entity checks a finished body passes before the embedding question is asked. Its wedge check reads the angle two faces make along a shared edge.
- *Class*: tier 3's verdict on an edge. It is a crease (with which way it turns), a smooth seam, or a cusp end.
- *Embedding question*: the check that a solid's boundary does not meet itself. It subdivides the faces into *cells*.
- *Separator*: a surface through the shared edge. The question shows the two faces lie on opposite sides of it.
- *Rate*: the sine of the angle at which a face leaves the separator, taken at its least over a cell.
- *Lever*: the length a rate is multiplied by before it is compared with the band.
- *Run*: how far a face's boundary stands from the edge, measured across the edge and not along it.

**The final state.**
1. Tier 3 decides each edge's class once. The finished body keeps the class, each face's pole or apex vertex, and one lever per shared edge or vertex.
2. The embedding question takes the finished body and never classes an edge.
3. The class decides which argument may run:
   - the other face's own surface is a separator only across a crease;
   - a cusp end goes to the second-order argument, and is not re-decided from a cell's size;
   - a smooth seam is proved through a separator both faces leave squarely.
4. In every cell beside a shared edge or vertex the question makes one banded reading: the two faces' rates added, times the lever, against K·ε. Adding the two makes the number the same whichever separator was picked.
5. The lever is the run of the narrower of the two faces. It is one number per edge or vertex, never a cell's size and never recomputed per cell.
6. Where tier 3 holds no fact (a vertex two faces share alone, a pole, a face against itself), this reading is the only one, at the same kind of extent. Tier 3 gains no new check.

**What a pass means.** Had the slowest parting in that cell held across the narrower face, the two faces would be more than the band apart at its far side. That is your PR 7095 test, with "far side" read as the far side of the pair. It is the question's own fact about the pair of faces, not a second classing of the edge.

**Why the narrower face's run.** Three levers were weighed on two cases.
- *A sliver*: a strip of a 1 mm cylinder, 1e-7 m wide, leaves a 1 m plane at a sine of 1e-4 and ends 5e-12 m from it, still leaving at a rate of 2e-8.
  - The reach as PR 7329 builds it is the plane's metre: 2e-8 × 1 m passes. That is wrong: the strip lies inside the band everywhere.
  - Tier 3's lever (reach capped by curvature radii) refuses it, but only because the strip is curved. A flat strip of the same width, folded at a sine of 5e-5, passes at tier 3's lever too.
  - The narrower face's run is 1e-7 m. Both strips refuse.
- *A cone with its apex*, cut by a plane through its axis. Tier 3's lever shrinks to 0 toward the apex, so it cannot be one number for the edge. The narrower face's run is one number and stays positive.
- The rule is already in the kernel twice: `[topology.census]` reads a meeting pair "over the shorter cell", and the profile's 2-D joint reading uses the shorter of the two legs.

**Left as separate questions.**
- Whether tier 3's wedge check should use the same extent. Today it levers at the farther face. It passes the flat strip above as a crease; the embedding question then refuses the body.
- Whether tier 3's curvature lever is right. A 1 mm cylinder and a plane at a sine of 1e-4 read as a crease, though moving the plane 5e-12 m removes the edge.
- Where the run is measured from. This answer needs it measured across the edge, as the open ONEROUND question proposes. Measured from the edge's middle, a long narrow face still over-reads.

**The premise, corrected** (`sure`).
- The gate does not ask the embedding question yet, and the finished body keeps no per-edge fact. Both were approved on PR 6893 and are unbuilt, so PR 7329 had no verdict to read.
- What PR 7329 bands per cell is the edge's wedge only when the separator is the other face's surface. With a bisecting plane it is half the wedge. With a circular edge's own plane it is unrelated.

**Text to change.** `[topology.tiers]`, "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach…". It is agent wording of your "presumably unified with the other case". It should say:
- the question reads from the verdict what tier 3 decided at a shared edge or singular vertex, and decides none of it;
- over each cell it reads the rate at which the two faces part, levered at how far the narrower of them runs from what they share, never at a cell of its own subdivision, passing by more than the band;
- where tier 3 holds no fact, that reading is the only one.

This narrows "one reading for both" to one class per edge. "Never re-deciding one over a cell", approved on PR 6893, stays and becomes true.

**Confidence.** The final state is `likely`. The arithmetic of the two cases is `sure`. Nothing was run; the measuring lane's numbers are not in.

## For the orchestrator

- **No split.** A-r4 keeps "the faces' reach" and would put the choice of face to Ev separately. I read that as placement, not disagreement: A-r4's own reworking refuses the sliver "levered at the face's own run". The two framings do differ in one consequence. If one recipe on the verdict serves both readers, changing it changes tier 3's class as well. So the PR to Ev should say plainly that this answer fixes the question's lever and leaves tier 3's to its own question.
- **A guarantee I claimed in round 3 no longer holds.** With the narrower face's run, the lever can be shorter than tier 3's. The question can then refuse for good an edge tier 3 classed a crease; the flat strip is the example. That is the right verdict at the wrong door, and it is the argument for tier 3 taking the same extent.
- **Take from C-r4:** the lever is the lesser of the two runs because the summed rate needs both faces measured at one distance. The same rule changes `reach_from_vertex`, which today takes the farther face.
- **Take from A-r4:** across a cusp-end edge no first-order separator may certify, the bisecting plane included.
- **For the measuring lane:** add the flat strip, and run all three cases under the narrower run. On the census, list bodies whose verdict moves between today's reach and the narrower run; I expect thin faces beside large ones only.
- **To file, off this decision:**
  - the curvature lever's over-read at sine 1e-4, on TOPO or RESTFRONT;
  - `wedge_reach.rs`'s "the reach is the farther face's", on the honeone or ONEROUND row, citing review 2's 20ε slab.
- **For the build's reviewer:** the summed bound assumes a separator with a unit gradient. The graph separator is unchecked.
- **Read this round:** `round5/dispatch.md`, `round4/A-r4.md`, `round4/C-r4.md`. I ran nothing.
