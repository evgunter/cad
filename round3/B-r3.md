## For Ev

**Recommendation** (`likely`). I withdraw the further end state I held last round. One difference is left.

1. Tier 3 decides each edge's class once: crease (with its turn), smooth seam, or cusp end. The finished body keeps it, with the edge's reach and each face's pole or apex vertex. The embedding question reads these and never classes an edge.
2. In every cell beside a shared edge or vertex, the question makes one banded reading of its own: the rate at which the two faces part there, times the faces' reach, against K·ε. This stays for good. It is not replaced by a bare sign once tier 3 is repaired.
3. The rate is the two faces' rates off the separator added together, so the number does not depend on which separator the question picked.
4. The class decides which argument may run. The other face's own surface is a separator only across a crease. A cusp end goes to the second-order argument; it is not re-decided from a cell's size. A smooth seam is proved through a separator both faces leave squarely.
5. Where there is no edge (a vertex shared alone, a pole, a face against itself) the question's reading is the only one, levered at the faces' reach from that entity. Tier 3 gains no new check.
6. Whether tier 3's own lever is right on a face that curves back is a separate tier 3 question. Nothing here depends on its answer.

**Terms.**
- *Wedge*: the angle two faces make along a shared edge. *Class*: tier 3's verdict on it.
- *Separator*: a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- *Rate*: the sine of the angle at which a face leaves the separator, enclosed over a cell.
- *Lever*: the length a rate is multiplied by before it is compared with the band K·ε. *Reach*: how far the faces run from what they share. *Fold*: tier 3's lever, the least of the reach and the faces' curvature radii.
- *Cell*: a piece of the question's own subdivision.

**Why I gave up "a sign only, once tier 3 is certified".**
- I had treated the band on the rate as a second copy of tier 3's band on the wedge. It is not. Tier 3's fact is at the edge. The cell's reading says that along this stretch of the two faces, wherever they are still within the band of each other, they part at a rate geometry ε off cannot turn. So the real faces part as the stored ones do and do not meet a second time. That is your PR 7095 test applied to the cell, and no fact about the edge holds it.
- I argued that on plane, cylinder, cone, sphere and torus the edge's fact implies the cell's. I still think so (`likely`), but that is a lemma about kinds of face, resting on three repairs to tier 3. A certificate should not rest on it when the cell can read the thing directly.
- My objection last round was that the banded reading passes a face that flattens to a rate just above K·ε / reach. That is no counter-example. Such a face is a thin wedge that ε cannot turn, and `[topology.census]` already calls that legal ("a definite number is a legal configuration, however thin").
- My other objection was that a second banded reading could refuse a crease tier 3 accepted, because a bisecting plane reads each face at half the wedge. Adding the two faces' rates removes it: the sum is never less than the wedge's own sine.

**The difference that remains: which lever the cell's reading uses beside an edge.** I hold the faces' reach.
- The reach is never shorter than tier 3's lever, and the summed rate is never less than the wedge's sine. So on the edge itself the cell's reading follows from tier 3's crease. The question cannot refuse there what tier 3 accepted. What it adds lies off the edge.
- With tier 3's lever that guarantee goes. Tier 3's lever varies along an edge, since a cone's curvature radius changes along its length. A single lever per edge then falls short of what tier 3 used somewhere, and the question can refuse for good a crease tier 3 passed. Keeping a lever per stretch is the question re-reading tier 3's check cell by cell, which every report rejected.
- Tier 3's lever goes to 0 at a cone's apex. The reach does not. With the reach the apex needs no concession inside the question.
- Tier 3's lever is the thing all three reports now say is unsettled. Tying the cell's reading to it moves the question whenever that is settled, and the two serve different ends. Tier 3's lever answers how far an angle governs a gap. The cell's lever answers how much angle ε of error can make.
- What tier 3's lever would buy is that the question could not read a departure where tier 3 read smooth. The class already gives that (item 4).
- The honest cost: on a tightly curved face the reach is more permissive than your test strictly allows. A 1 mm cylinder ε off turns by ε over 1 mm, not over its reach. At the edge tier 3's class has already applied the curvature. Off the edge, on the analytic faces, the two levers disagree only within about K·ε of where a face comes parallel to the other. `likely`, by hand.

**The one sentence to rule on.** Beside a shared edge, is the rate at which the faces part levered at the faces' reach, with tier 3's class deciding which separators may be used, or at the lever tier 3 decided that edge's class at?

**Premise, unchanged** (`sure`).
- The gate does not ask the embedding question yet, and the finished body keeps no per-edge fact. The hand-over approved on PR 6893 is unbuilt, so PR 7329 had no verdict to read.
- The PR's per-cell number is the edge's wedge only when the separator is the other face's surface.

**Text I would change.** `[topology.tiers]`, "That reading is the edge's wedge as tier 3 reads it, one reading for both: an angle metered at the reach…" (agent-written from your "presumably unified with the other case"). It should say:
- the question reads from the verdict what tier 3 decided at a shared edge or singular vertex, and decides none of it;
- in each cell it reads the rate at which the two faces part, at the reach of the faces it governs, never at a cell of its own subdivision, passing by more than the band;
- where tier 3 holds no fact, that reading is the only one.

This narrows "one reading for both" to one class per edge. "Never re-deciding one over a cell" stays and becomes true.

**What would decide the remaining difference.** One worked case: a cone that owns its apex, cut by a plane through its axis, so an edge runs from the apex to the rim. Say what "the lever tier 3 decided that edge at" is on the stretch nearest the apex. If the only answers are a number per stretch or a concession inside the band of the apex, the reach is the lever.

## For the orchestrator

**(a) What moved me, and to what.**
- Difference 2 (a further end state): I move to the other two. The band per cell is the end state at creases too. Neither report's worked case did it as framed. A's uses a spline face, which tier 3 does not class at all (check 4 is exempt by kind), so my rule already kept the banded reading there. What moved me was working out what the band on the rate certifies: that the faces ε off still part, through the zone where the stored faces are within the band of each other. That is over the cell, not at the edge, as both reports said.
- From C-r2: the summed rate. It answers my bisector objection, and it is what makes the reach provably consistent with tier 3 on the edge.
- From A-r2: "the reach is the more permissive lever, safe only because the class gate and the off-edge distance tests carry the curvature cases." Accepted, and stated to Ev as the cost.
- Difference 1 (the lever): not moved. I hold the reach, with A-r2.

**(b) Is my argument answered, point by point.**
1. *The lever.*
   - "A length at tier 3's lever in a small cell bounds no actual distance": I drop it. At either lever the reading is a rate that ε cannot turn, not a bound on a distance, and that is meaning enough.
   - "Under a bisecting separator it reads half the wedge": answered by C-r2's sum, at either lever.
   - "It can refuse for good a crease tier 3 accepted": answered for the reach (reach ≥ fold, sum ≥ sin θ). Not answered for tier 3's lever. C-r2's own caveat ("it might need a lever per stretch, which the verdict would then keep per stretch") is the point: one lever per edge under-reads somewhere, and a lever per stretch is option B.
   - C-r2's "the table cannot split" is equally true under the reach once the class dispatches the arms. Row 3 (sine 5e-7) is `End`, so no first-order arm runs. That needs the dispatch to cover the plane arm as well as the carrier arm: across an `End` edge the bisecting plane reads 2.5e-7 × 1 m and would certify first-order a body whose exactly-tangent twin refuses `SecondOrder`. A-r2's wording ("the class decides which argument runs") covers it; C-r1's gate on the carrier arm alone does not.
   - C-r2's "the apex stops being the engine's problem: the lever is the entity's" is asserted and marked `unsure` by its author. Tier 3's fold at a station is a point value. Read from below over the edge, as the lever rule in `crates/geom-core/README.md` asks of a definite sign, it is 0 at an apex.
2. *The further end state.*
   - "It certifies the stall": conceded in substance. My reply (a rate just above the band passes) was wrong as a rebuttal: that configuration is legal.
   - "Its by-hand argument leans on the rate staying near the wedge's past the edge": true. I had named it as the weak point; I now treat it as disqualifying for a certificate.
   - Not conceded: A-r2's "close to the exact sign you declined". It was conditional on the verdict. Moot now.

**(c)** As in `For Ev`. If Ev rules for tier 3's lever, what has to be specified with it is the lever on an edge whose curvature varies and on the stretch before an apex.

**(d) Deciders.**
- The apex worked case, as in `For Ev`. It needs no build.
- If a measurement is wanted: the gate census under both levers with full class dispatch, listing bodies whose verdict differs. Prediction: cones that own an apex, and edges where the wedge and the curvature both vary; nothing else. An empty list makes the choice one of ownership only, and then the reach wins on independence from the open tier 3 fork.
- For the summed rate: one unit row through `sign_on` on a crease with a sum just above K·ε / reach, under the carrier arm and under the bisecting plane. Both must decide alike.

**Notes for the build.**
- The verdict can keep every certified edge's reach: check 2 reads it into `reaches` before check 4's spline exemption (`validate.rs`, "One wedge reader for checks 2 and 4"). `arm_of` then stops recomputing it per cell pair.
- The summed bound assumes a unit gradient: exact for a plane, near the edge for a carrier. The graph separator is not checked (C-r2's caveat; I did not check it either).
- Under the carrier arm one face lies on the separator and contributes rate 0; the sum is then the wedge's sine, as today.
- The cell's proved sign at a crease should agree with the turn the verdict holds. A mismatch refuses.
- Still to file, off this decision:
  - the fold's over-read at sine 1e-4, on TOPO or RESTFRONT (all three reports);
  - the reach's site (ONEROUND's fork), which matters more now, since the reach is the cell's lever and the strip probe in review 1 passes on its over-read.

**Not checked.** I ran nothing this round. Read: `round3/dispatch.md`, `round2/A-r2.md`, `round2/C-r2.md`. The claim that the two levers differ off the edge only within about K·ε of a face's parallel point is by hand, for constant-curvature sections only.
