## For Ev

**Recommendation (likely).** All three reports now agree on the final state. I have moved to the other two on the last difference: the cell's banded reading beside an edge is levered at the faces' reach, not at the lever tier 3 decided the edge at.
1. **Tier 3 classes each edge once.** The class is a crease (with its turn), a smooth seam, or a cusp end. The finished body keeps the class, the edge's reach, and each face's pole or apex vertex.
2. **The embedding question takes the finished body and never classes an edge.**
   - The class decides which argument may run. The other face's own surface is a separator only across a crease. A cusp end goes to the second-order argument, read from the verdict's `End`.
   - Across an `End` edge, no first-order separator certifies: not the other face's surface, and not a bisecting plane either.
   - A smooth seam is proved through a separator both faces leave squarely, as on a capsule.
3. **Every cell beside a shared edge or vertex makes one banded reading of its own, for good.** The reading is the two faces' rates off the separator added together, times the faces' reach, against K·ε. The reach is read once per edge or vertex from the verdict, never recomputed per cell. A pass means the faces part by more than the band at their far side: your PR 7095 test, applied over the cell.
4. **Where tier 3 holds no fact** (a vertex shared alone, a face against itself), this reading is the only one, levered at the faces' reach from that entity. A pole or apex is read by its vertex key. Tier 3 gains no new check.
5. **Two levers stay separate questions, and both readers inherit the answers.** One is check 4's lever (the fold), which over-reads a face that curves back. The other is the reach's own recipe (below). Because the reach is one recipe kept on the verdict, a fix to it lands in both readers at once.

**Terms.**
- *Separator*: a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- *Rate*: the sine at which a face leaves the separator, enclosed over a cell.
- *Reach*: how far the faces run from what they share.
- *Fold*: tier 3's lever, the least of the reach and the faces' curvature radii.

**Why the reach, and not tier 3's decided lever** (likely).
- **On the edge itself the cell can never refuse what tier 3 passed.** The reach is never shorter than the fold, and the summed rate is never less than the wedge's sine.
- **A lever kept once per edge cannot promise that.** Tier 3's fold varies along an edge. Where the wedge narrows toward one end and the curvature tightens toward the other (a plane cutting a cone obliquely, for instance), each station passes. The least wedge times the least lever can still fall under the band, so the question would refuse for good a body tier 3 accepted.
- **A lever kept per stretch avoids that**, but it is a second per-stretch structure, kept in step with check 4.
- **What the decided lever would buy is already bought by the class.** Its gain was that the question could not read a crease where tier 3 read smooth, and the class gate gives that.

**The cone worked case.** A cone that owns its apex, cut by a plane through its axis. The edge is a ruling running from the apex to the rim.
- The wedge is a right angle all along the edge: the plane through the axis meets the cone's tangent plane squarely.
- **Under the decided lever**, the verdict's lever is the least over tier 3's stations, all of which lie inside the edge, so it is positive. A near-apex cell reads 1 × that lever and passes. The cell holding the apex vertex goes to the vertex's reading. So the decided lever does not fail on this case. It fails only if the fold is read over the cell, which reaches 0 at the apex, and nobody proposes that.
- **Under the reach**, the cell reads 1 × the reach and passes. The vertex reading is the same.
- So the cone does not decide between the two. The varying edge above does, and it goes against the decided lever.

**The worked case I put for the decided lever is a fault in the reach's recipe, not an argument for tier 3's lever.**
- The case: a 1 mm cylinder face leaves a 1 m plane at a sine of 1e-4. The face is 1e-7 m wide and ends while still leaving at a rate of 2e-8, its far edge 5e-12 m from the plane.
- The reach certifies it: 2e-8 × 1 m. But the reach used is the farther face's run, the plane's metre. The cylinder face's own far side is 1e-7 m away and parts by 5e-12 m, under the band, so it fails your test.
- Levered at the face's own run, the reading is 2e-8 × 1e-7 and refuses.
- This is the same over-read as review 2's slab 20ε thick, whose end edge is levered at its neighbours' metre. It belongs with the recipe: measured across the edge rather than from the edge's middle (ONEROUND's fork), and the farther face against each face's own run.
- I would put that to you as one question about the recipe. Under this recommendation both readers inherit the answer.

**Ratified text I would change.** `[topology.tiers]`, the sentence "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach…". It is agent wording of your "presumably unified with the other case". I would have it read:

> Where faces do meet, or a face is read against itself, the question reads from the verdict what tier 3 decided there (an edge's class and turn, an end, a pole or apex vertex) and decides none. Over each cell it reads the rate at which the faces part, the two faces' rates summed, at the reach of the faces it governs ([tolerance.epsilon]), never at a cell of its own subdivision, and that passes by more than the band.

This narrows "one reading for both" to one class per edge. "Never re-deciding one over a cell", which you approved on PR 6893, stays and becomes true.

**Confidence.** Final state: likely. That reach plus the summed rate never refuses on the edge what tier 3 passed: sure. That the recipe's choice of the farther face over-reads a small, tightly curved face: arithmetic sure; the fix unsure.

## For the orchestrator

**(a) The case against my side, answered.**
- **The cone through its axis** does not break a decided lever kept per edge, as worked above. What breaks it is B's general point: a single lever under-reads an edge along which both the wedge and the fold vary, and a per-stretch lever is a second structure.
- **Combined with the sum and reach ≥ fold**, the reach is provably consistent with tier 3 on the edge, and the decided lever is not. That moves me.

**(b) What moved me.**
- **B-r3's consistency argument** (summed rate ≥ the wedge's sine, reach ≥ fold).
- **Reworking my own case.** It turns on the farther face's run in `wedge_reach`'s recipe ("the reach is the farther face's", the module docs). B-r3's reply to it ("a thin wedge ε cannot turn, legal") does not hold on a 1 mm cylinder: ε turns its normal by about 1e-6, far more than 2e-8. B concedes this as the reach's "honest cost", within about K·ε of where a face comes parallel. My case lies wholly in that zone. So the cost is real, and its home is the recipe.

**(c)** As in `For Ev`.

**(d)** I no longer hold the decided lever, so (d) does not arise. The residual is the recipe question, and it should go to Ev with, or beside, ONEROUND's site fork. In three sentences:
- The reach is the farther face's run, so a small face beside a large one is levered at the large one's extent.
- A face 1e-7 m wide whose far side sits 5e-12 m from the plane then certifies at 2e-8 × 1 m.
- Each face levered at its own run refuses it, and tier 3's reading would adopt the same recipe, since there is one.

**For the measuring lane.**
- **My case**, two levels:
  - As a unit row through `sign_on` at a 1 m arm and a 1e-7 m arm.
  - As a body where the narrow cylinder face's far boundary is shared with the plane (its ends are vertices on the plane), so that no pair sharing nothing trips the distance test. If the reach certifies that body, the recipe fault reaches the body level and should be filed with priority.
- **B's varying edge**: a plane cutting a cone obliquely, with each station's sine × fold compared against the least sine × the least fold.
- **Not run by me.** Everything this round is by hand. I read only the round-4 dispatch and the two round-3 reports.

**To file, off this decision.**
- The fold's over-read at sine 1e-4: all three reports, TOPO or RESTFRONT.
- The recipe's farther face against each face's own run: on the honeone or ONEROUND site row, citing `crates/topo/src/wedge_reach.rs` module docs and review 2's 20ε slab.
- For the build's reviewer: the summed-rate bound assumes a separator whose gradient has unit length; the graph separator is unchecked (C's caveat, still open).
