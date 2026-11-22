## For Ev

**Recommendation (likely).** Tier 3 decides each edge's wedge class once. The verdict keeps it, and the embedding question reads it and never decides a class. In each cell beside a shared edge or vertex, the question still makes one banded reading of its own: the separation the face is guaranteed at the faces' reach, checked against K·ε.
- That guaranteed separation is the sine at which the face leaves the separator, enclosed over the cell, times the reach. It is a length.
- It is levered at the faces' reach, never at the cell, and computed once per edge or vertex by the one recipe tier 3 uses.
- The class decides which argument runs. The other face's surface is used as the separator only across an edge classed as a crease. The cusp reads tier 3's `End` and does not decide it from a cell's size.
- Where tier 3 holds no fact (faces sharing only a vertex, a pole or apex, a face against itself), the question's own reading is the only one. Tier 3 gets no new checks. The apex vertex is read from the verdict by key (as approved on PR 6893), not from a struct's shape.

Two things I would change in ratified text:
- **The "one reading for both" sentence** in `[topology.tiers]` should say what is unified: the edge's class, which the question reads and never decides. The question's cell reading is its own, levered at the reach.
- **Check 4's lever is a separate tier 3 question** and needs its own row; see point 3 below.

**Terms.**
- *Separator*: a surface through the shared edge that the question shows the two faces lie on opposite sides of.
- *Sine*: the face's angle off the separator, enclosed over a cell.
- *Reach*: how far the faces run from what they share.
- *Fold*: check 4's lever today, the least of the reach and the faces' radii of curvature.

**The three differences.**

1. **What a cell beside a crease decides: a banded length, not a bare sign** (likely).
   - A bare sign means the question accepts any rate that stays one-signed, however small. That proves the faces do not cross. It does not prove they part.
   - Worked case: plane F1, and a spline face F2 that leaves the edge at a clean angle, flattens to run within K·ε of F1 for a stretch, then rises away. Tier 3 reads the edge a crease, since the curvature at the edge is low. Over a cell holding the edge and the flat stretch, F2's rate stays one-signed with a tiny minimum, so a bare sign certifies the whole cell. That passes two faces lying within the band off any shared entity, which the finished-body clause refuses.
   - Under the banded reading that minimum fails the band, the cell splits, and the cells on the flat stretch meet the distance test and refuse.
   - What the band per cell guarantees: every point of the face within the band of the separator lies within the reach of the edge. That is your PR 7095 test ("opens by less than about 2ε at the far side"), applied to the cell.
   - So the band is not applied twice to one fact. Tier 3 bands the wedge at the edge; the cell bands the face's parting from the separator over the cell, which no fact about the edge contains.

2. **No new tier 3 facts where there is no edge** (likely).
   - I had proposed tier 3 facts for a vertex shared alone and for a face's own turning. Nothing but the question would read them, so they would be a mechanism nothing needs.
   - The rule that covers those cases is enough. An angle the question reads is levered at an extent of the body (the reach from the vertex, the face's own reach), never at a cell, and the question decides it only where tier 3 holds no fact.

3. **Check 4's fold over-reads a face that curves back** (arithmetic sure; the fix unsure).
   - I said the fold was right; I was right only at sine 5e-7.
   - The case is the reviewers' table: a cylinder of radius 1e-3 and a plane through its ruling at sine 1e-4. The plane cuts the cylinder a second time 2e-7 m away, and between the two cuts they part by at most 5e-12 m. Moving the plane 5e-12 m removes the edge.
   - Check 4 still reads Corner: 1e-4 × 1e-3 = 1e-7, above K·ε.
   - The scale over which the angle governs the gap is about sinθ divided by the curvature toward the other face, not the radius.
   - This is tier 3's question, for its own row or fork. The recommendation above does not depend on it: whatever lever tier 3 settles on, it is used once, in tier 3.
   - The cell reading at the reach does not over-certify here. Cells near the second cut have no zero side, and the distance test refuses them.

**Why the cell reading uses the reach rather than tier 3's lever.**
- I first proposed reading the cell at tier 3's lever. Point 3 shows that lever is itself unsettled.
- Point 1 shows that a face curving back is caught by the class (tier 3) and by the distance test off the edge, not by the lever.
- What the cell reading uniquely guarantees is parting at the far side, and that is the reach.
- The reach is computed once per edge or vertex from the body (`wedge_reach::reach_over`, `reach_from_vertex`), so ONEROUND's fix of where it is measured from lands once. Measured across the edge, it also refuses the reviewers' strip: 4 long, 0.05 wide, folded so its far edges part by 0.9ε.

**The other answers.**
- *Bare sign at creases.* Answered by the worked case in point 1. It is also close to the exact sign you declined on 7095.
- *One function, one arm, read by both.* The arm choice reopens the apex refusal. A station and a cell enclosure can still disagree. And "never re-deciding" would have to go.
- *PR 7329 as it is.* The question certifies a crease where tier 3 reads the edge smooth (the 5e-7 row). That is harmless only because tier 3 runs first, and no type enforces that order.

**Premise, unchanged from the first round** (sure):
- The gate does not ask the embedding question on main.
- `embedded` takes a bare `Side`, and `AtRestBody` keeps no per-edge fact. The hand-over approved on 6893 is unbuilt, so PR 7329 had nothing to read.
- The sine is the wedge only when the separator is the other face's surface. With the bisecting plane it is the sine of half the wedge. With a circular edge's own plane it is unrelated: a capsule's smooth seam reads 1 × reach, which is correct, because the faces really are on opposite sides of that plane.

**Single decider for difference 1.**
- Build the worked case and run both readings: bare sign certifies, banded refuses.
- Also count, over the suites, cells certified by a bare sign that contain a point within K·ε of the other face beyond the edge's reach. If no valid body puts that count above zero, the band costs nothing there.

## For the orchestrator

- **What moved me:**
  - C's premise 1. I checked the arithmetic: the chord half-length is r·sinθ and the sagitta r·θ²/2.
  - C's point that the cell reading covers more than the edge, which turned me against B's bare sign.
  - Both other reports' implicit test that nothing but the question would read new tier 3 facts. I dropped them.
  - Because of C's premise 1, I dropped the verdict's lever in favour of the reach for the cell reading.
- **Where B's argument fails:** its hand argument bounds only the blind strip left by the stored edge's ε offset. B names its own weak point (a rate that decays past the edge) and leans on the fold to cover it, and C's case shows the fold does not.
- **Where C's argument is incomplete:**
  - "The arms that still differ can only cost a refusal on the safe side" is not argued. The reach is at least the fold, so the cell reading is the more permissive one.
  - It is safe only because the class gate and the off-edge distance tests carry the curvature cases. Ask C or a reviewer to confirm with the cylinder at sine 1e-4 on a built body.
- **To file:**
  - Check 4's fold over-reads a face curving back (`geom_brep::folded_lever_arm`, `wedge_of`), with C's numbers. This is TOPO or RESTFRONT ground.
  - `AtRestBody`/`KeptSign` keep no per-entity facts. That is lane B/C's, under 6893.
- **Not run:** the worked case. A spline face is needed for the inflection; with only analytic faces, a far boundary inside the band is caught by a face pair that shares nothing.
- I read nothing beyond the dispatch and the three round-1 reports this round.
