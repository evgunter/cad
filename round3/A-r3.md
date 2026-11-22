## For Ev

**Recommendation (likely).**
- **Tier 3 decides each edge's wedge once and the finished body keeps it.** The verdict holds two things per edge: its class (a crease and which way it turns, a smooth seam, or an end), and the lever the class was decided at. It also keeps each face's pole or apex vertex.
- **The embedding question takes the finished body.** It never classes an edge. A cusp reads tier 3's `End` rather than deciding it from a cell's size.
- **Beside a shared edge, each cell makes one banded reading, and it is a length.** For each face, take its least rate of leaving the separator over the cell, times the edge's lever from the verdict. Add the two faces' figures together and compare the sum with K·ε.
- **Where tier 3 holds no fact** (a vertex the faces share alone, a face's own turning), the lever is the faces' reach from that entity, computed once per entity, never per cell. Tier 3 gains no new checks.
- **This is the end state, not a step.** The cell's band is not retired once tier 3's check is repaired.
- **Whether check 4's lever is the right meter is a separate tier 3 question.** Whatever it settles, it governs both readings, because both take the lever from the verdict.

**Terms.**
- *Separator*: a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- *Rate*: how fast a face leaves the separator, per unit of distance along the face.
- *Lever*: the length a rate or angle is multiplied by to give a distance compared with K·ε.
- *Reach*: how far the faces run from the edge.
- *Tube*: the strip beside an edge inside which the two faces may be within K·ε of each other.

**What the cell reading guarantees.** On one face, the distance from the separator is at least the cell's least rate times the distance from the edge. So if the least rate × L reaches K·ε, then no point of that face more than L from the edge is within the band. In a cell smaller than L, every point is within L of the edge anyway. So in any cell, the reading proves this: every point where the faces may be within the band lies within L of the edge. Adding the two faces makes it the same number whichever separator the question picks. Today a plane that bisects a crease reads each face at sin(θ/2), half what the face's own surface reads as separator. (likely; argued by hand)

**The two differences, and where I stand.**

1. **The lever: the edge's lever from the verdict, not the reach.** I move back to this from my last round.
   - By the guarantee above, L is the width of the tube, the strip where being inside the band is put down to the edge.
   - At L = reach that guarantee is empty. The reach is how far the faces run, so a tube that wide covers the whole face. All that is left is an extrapolation to the far side. That objection is fair against the reach; read as the tube, it does not apply to the decided lever.
   - L should be the scale at which nearness is still the edge's. That is exactly tier 3's meter question. One lever on the verdict settles it once for both readings.
   - Worked case: a 1 mm cylinder face leaves a 1 m plane at a sine of 1e-4. Over its 1e-7 m width it curls toward the plane and ends while still leaving at a rate of 2e-8. Its far edge is then 5e-12 m from the plane.
   - At the reach (1 m): 2e-8 × 1 = 2e-8, above K·ε = 1e-8. The cell certifies, and a face lying wholly inside the band is accepted as "the edge".
   - At check 4's lever today (1e-3): 2e-11. The cell refuses and splits, and the distance test then refuses the far end.
   - The reach is safe here only if some other face pair happens to trip the distance test. Nothing in the reading itself catches it.

2. **No retirement of the cell's band after tier 3 is repaired** (likely).
   - The cell's band and the wedge's band decide two different facts. The wedge's band says the angle at the edge is real. The cell's band says the nearness is confined to the tube.
   - A whole-edge, certified check 4 speaks for the edge. It does not say a face stays out of the band beyond the tube.
   - The argument that analytic faces cannot flatten that way is not shown. It also leaves the reading needed anyway for spline-adjacent edges, vertices and seams. So retiring the band buys a second code path and a guarantee that is only argued.
   - A sign-only cell certifies the worked case above with any lever, since there is no lever at all.

**Other answers, as final states.**
- **Cell banded at the reach, kept on the verdict.** Simpler, and no worse at the far side. But its tube is the whole face. Two meters for one nearness question would have to be kept in step if tier 3's meter changes.
- **Sign only beside creases once tier 3 is certified over the whole edge, the band applied once.** Literally one band. It accepts a face that stays inside the band beyond the tube, and its safety rests on an argument about faces, not a check.

**Text I would change.** `[topology.tiers]`, the sentence starting "That reading is the edge's wedge as tier 3 reads it, one reading for both". It is agent wording of your "presumably unified with the other case". I would have it read:

> The question reads from the verdict what tier 3 decided at a shared edge or singular vertex, its class and the lever it was decided at, and decides neither. Over each cell it reads one length: that the two faces' nearness lies within that lever of the entity, which passes by more than the band. Where tier 3 holds no fact (a vertex shared alone, a face's own turning) the lever is the faces' reach from that entity; never a cell of the question's own subdivision.

Tier 3's page: the wedge is read over the whole edge. "Never re-deciding one over a cell" stays, and becomes true.

**If a split remains, the one sentence to rule on:** is a cell's banded reading beside an edge levered at the lever tier 3 decided that edge at, so one meter governs the wedge and the nearness beside it, or at the faces' reach, so the cell checks only that the faces part by the band at their far side?

## For the orchestrator

**(a) What moved me.**
- C's summed separation answers B's half-angle objection to a decided lever. It is also what B's "bounds no distance" needs.
- B's "bounds no distance" made me state what the reading does prove, the tube. Once stated, that guarantee is empty at the reach, which reverses my round-2 move.
- B's correction stands: the pole and apex fact is tier 3's, read by key.
- B's "decide once per pair and vertex, not per cell" for the no-fact readings is adopted.

**(b) Answered, point by point.**
- **Lever.** My round-2 reason for the reach was that the class gate and the off-edge distance tests carry the curvature cases. The worked case shows they carry it only when another face pair trips. I withdraw that reason.
- **Lever, B's objections.** B's "a small cell bounds no actual distance" is answered by the tube statement. B's "refuses for good a crease tier 3 accepted" holds under one-face bisector readings only, and is answered by summing. With summing, as cells shrink the reading tends to tier 3's own margin, which passed, so a refusal is only one more split.
- **End state.** B's "rate just above Kε/reach passes with the faces in band everywhere" is a true objection to the reach. At the decided lever it is the tube by definition: nearness within the edge's lever is accepted.
- **End state, B's analytic-face argument** (a parting-aware meter contains flattening). Not refuted. It is unshown, and it covers only the faces tier 3 judges.

**(c)** As in `For Ev`. Confidence: lever likely; end state likely; the summed-separation bound sure for planar separators and likely for curved carriers near the edge (C's caveat on the graph separator stands).

**(d) Deciding case.** A unit probe through `sign_on`: plane against a cylinder of radius 1e-3, sine 1e-4, the cylinder's part ending at a rate of 2e-8. It certifies at arm 1 m and refuses at 1e-3. Then build it as a body whose far boundary is shared with the plane, so that no non-adjacent pair trips the distance test: a sliver of cylinder whose ends are vertices on the plane. See whether the reach certifies a body with a face inside the band. For B's step 3, B's measurement (count the cells that sign-only newly certifies with the far-side gap inside the band) cannot show that no such cell exists. The constructed case is what decides it.

**Caveats.**
- If tier 3's meter becomes min(reach, θ/κ), the worked edge's class goes in-band and tier 3 refuses it first. The case then shows the lever difference only under today's fold. That is still a case for one meter.
- Whole-edge check 4 may need a lever per stretch where the curvature varies (C's caveat). The verdict would then keep it per stretch.
- I ran nothing this round, and read only the dispatch and the two round-2 reports.
