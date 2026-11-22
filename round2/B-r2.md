## For Ev

**Recommendation.** Three steps. Every report agrees on the first; the difference between them is whether the third ever happens.

1. **Build now** (`sure`). Tier 3 decides each edge's wedge once, and the finished body keeps it: the class (crease, smooth seam, cusp end), a crease's turn, the edge's reach, and each face's pole or apex vertex. The embedding question takes the finished body and never classes an edge. A cusp edge is read from the verdict, not re-decided from a cell's size. In its cells the question keeps PR 7329's banded reading at every edge, with the lever read from the verdict instead of recomputed per cell.
2. **Repair tier 3's wedge check**, so that "crease" means the faces part by more than the band at their far side, at every point of the edge. Today it does not mean that, in three ways (below).
3. **Then, beside a crease, a cell proves a sign only** (`likely`). The band is applied once, to the wedge. Where tier 3 gives no crease (a smooth seam, an edge beside a spline face, a vertex shared alone, a pole, a face against itself), the question's own banded reading stays for good.

If you would rather tier 3's wedge check stay a cheap sampled classifier, stop after step 1. That is a coherent final state, and it is the one I would then choose.

**Terms.**
- *Wedge*: the angle two faces make along a shared edge. *Crease*: a wedge tier 3 reads as definitely not tangent.
- *Lever*: the length an angle is multiplied by to become a displacement, compared with the band K·ε. *Reach*: how far the faces run from the edge. *Fold*: tier 3's lever, the least of the reach and the faces' curvature radii.
- *Cell*: a piece of the question's own subdivision. *Separator*: a surface through the shared edge; the question shows the two faces lie on opposite sides of it.
- *Station*: a sample point along an edge where tier 3 reads the wedge.

**What I have changed since my first report.**
- I leaned on tier 3's fold being the right meter. It is not shown to be, and on one worked case it is wrong (next section). So step 3 now has step 2 as a stated condition, where before I treated tier 3's verdict as already trustworthy.
- I had a pole or apex as something tier 3 holds no fact about. It does: tier 3 places every pole and apex at a vertex. Which vertex a chart collapses at is read from the verdict; the PR infers it from the shape of a struct.
- The lever of the question's own readings at an edge comes from the verdict, computed once per edge. The PR recomputes it for every pair of cells.

**The three differences.**

*1. What a cell decides beside a crease.*
- The case for keeping a banded reading there: it covers the whole cell, so it can catch a face that leaves the edge at a healthy angle and then runs almost parallel to the other face. No angle at the edge contains that. This is right against tier 3 as it stands, and it is why step 1 keeps the reading.
- It is not a guarantee, though. The reading fails such a cell only when the leaving rate falls under K·ε / reach somewhere in it. A face that flattens to a rate just above that passes, with the two faces still inside the band of each other everywhere. `likely` (by hand)
- The guarantee wanted is one statement per edge: the faces get K·ε apart somewhere within their reach. That is what the wedge check is meant to say. A sign in every cell then finishes it: the gap only grows away from the edge, so once clear of the band it stays clear.
- On analytic faces (plane, cylinder, cone, sphere, torus) a face cannot flatten faster than its own radius allows, so a wedge meter that reads the greatest parting contains the case. On spline faces tier 3 judges nothing, and the question's own reading stays. `likely`
- Reading "one length at the verdict's lever" in each cell is the same product, rate × lever, with tier 3's lever. A small cell does not reach out to the lever, so it bounds no actual distance. It is the wedge decided a second time, and with a bisecting separator it reads half the wedge, so it can refuse for good a crease tier 3 accepted.
- Under step 3 the cell's sign must agree with the turn the verdict holds. A mismatch is a contradiction to refuse, not a second opinion.

*2. New tier 3 facts for a vertex shared alone, or a face's own turning.* No. Each has one reader, the embedding question, so tier 3 would compute a fact only to hand it across. The question is already the home of facts per pair of faces ("one fact per pair of faces, of the two faces and the cells they share"). What I take from the other view: such a reading should be decided once per pair and shared vertex, not again in every cell. `likely`; a second reader would change this.

*3. Whether tier 3's fold is the right meter.* It is not established. `likely`
- A 1 mm cylinder and a plane through one of its straight lines, at a sine of 1e-4. The plane cuts the cylinder again 2e-7 m away, and between the cuts the two part by at most 5e-12 m. Move the plane 5e-12 m and it no longer cuts the cylinder: the edge is not there to within ε.
- Tier 3 reads Corner there (1e-4 × 1e-3 = 1e-7, above K·ε = 1e-8). The reach alone reads it more definite still. Both fail your test.
- The case put for the fold, the same cylinder at 5e-7, does not tell the meters apart: the fold reads tangent, and so does the parting (about 1e-16 m).
- The fold counts a face as turning away over its radius. The parting stops growing at about angle × radius. That is its own question for you. Steps 1 and 3 do not depend on its answer, but step 3 waits for it.

**Tier 3's wedge check today is not a certified fact, in three ways** (`sure` on the first two):
- it reads at stations inside the edge, so a wedge that dips into the band between two of them passes;
- its reach counts a face's run along the edge (the open ONEROUND fork);
- the fold, as above.

**The premise, as corrected in round 1 and still standing.**
- The gate does not ask the embedding question yet, and the finished body keeps no per-edge fact. Both hand-overs are approved (PR 6893) and unbuilt. `sure`
- What the PR bands per cell is the edge's wedge under only one of its three kinds of separator. With a bisecting plane it is half the wedge. With a circular edge's own plane it is unrelated: a capsule's cylinder and dome meet smoothly, tier 3 reads a smooth seam, and the question reads "1 × reach". `sure`

**Text I would change** (`[topology.tiers]`, "That reading is the edge's wedge as tier 3 reads it, one reading for both…"; agent-written from your "presumably unified with the other case").
- If the end is step 3: at an edge tier 3 reads as a crease, the reading is that wedge, taken from the verdict, and a cell proves only a sign; elsewhere the reading is the question's own, an angle metered at a reach of the faces it governs, never at a cell. And in tier 3's page: the wedge is read over the whole edge.
- If the end is step 1: the question reads the class from the verdict and decides none; its cell reading is its own, at the faces' reach. That narrows "one reading" to "one class per edge".

**What would decide the open part.** One measurement: sign-only cells beside tier 3's creases, over the gate census and the tour, listing every cell that newly certifies with the gap at its far side. One such cell with a far-side gap inside the band keeps the cell reading for good; none supports step 3.

## For the orchestrator

**(a) What moved me, and to what.**
- C's premise 1 (the fold over-reads at sine 1e-4): arithmetic checked (chord 2r·sin θ = 2e-7, sagitta r·θ²/2 = 5e-12). I move from "the fold is the more faithful lever" to "the meter is open". My round-1 hand argument said it "leans on tier 3's lever folding the curvature arms"; that sentence is withdrawn. The candidate meter is the lesser of sin θ · reach and the sagitta sin²θ / (2κ_rel), which is `tangent_second_order`'s quantity read before the class is settled. `unsure`, its own fork.
- C's premise 2 (the cell reading holds more than the wedge): accepted against today's tier 3. So C's option A is the state to build now, where I had "PR 7329 as a step". My end state is C's A plus dropping the cell band at creases once tier 3 is repaired.
- A's premise 4 (check 6 places chart singularities): accepted; my round-1 list put a pole or apex wholly under the question's own. The collapse is tier 3's, by key. The angle read there (the sine's limit) stays the question's.
- A's "the lever is read from the verdict, never computed per cell": adopted for the question's own readings at edges. Check 2 already reads every certified edge's reach into `reaches` before check 4's spline exemption (`validate.rs`, "One wedge reader for checks 2 and 4"), so the verdict can keep a reach even for an edge with no wedge verdict. `likely`
- C's unchecked item (the carrier arm's sign against tier 3's convexity): adopted as part of the crease licence.

**(b) Is my argument answered, point by point.**
1. *Cell beside a crease.* Mine: the proof is blind within ε / sin θ of the edge, the wedge bounds that strip, so a sign suffices.
   - C answers part of it. The blind-strip argument covers the sign's robustness, not "the gap never clears the band". Conceded.
   - C's reading does not supply that either: rate_min × reach is an extrapolation, a bound on the far gap only when the cell spans the face and the lever is its width. It catches the flatten case only as the rate nears zero.
   - Not answered by C: in the carrier arm on the edge, C's reading is tier 3's own sine at a longer lever, so it is a weaker re-test that can differ only by enclosure width, off-edge variation and the bisector's half-angle. C's objection to its own option B ("a station reading and a cell enclosure … can still disagree") applies to its A at creases.
   - A does not answer it. "ψ ≥ sine × distance, so sine × arm bounds the separation at distance arm" holds only if the sine bound holds out to `arm`; the cell is smaller. And "splitting resolves that" fails on the bisector arm: sin(θ/2) × L stays under K·ε for a wedge between K·ε and 2K·ε at every depth.
2. *New tier 3 facts.* A's reason is "decided once as a fact of the entity". Answered by the pair-level certificate already being such a home, and by there being one reader. A's own unchecked item (whether `link.rs` holds a reusable vertex reading) is what could reverse this.
3. *The fold.* C answers my leaning; A's 5e-7 case does not discriminate. One nuance on C: the parting stops at θ/κ only where the curved face bends toward the other. Bending away, the gap keeps growing, and what fails there is the edge's position and class under a sub-ε move, not the far gap. A repaired meter has to say which of the two it protects.

**(c)** As in `For Ev`. Confidence: step 1 `sure`, step 3 `likely` and conditional on step 2, difference 2 `likely`, difference 3 `likely` that the fold is wrong and `unsure` on its replacement.

**(d) Measurements.**
- *Difference 1.* On the gate harness: for each shared edge call `topo::edge_wedge` once; where it is `Crease`, decide `sign_on`'s departure at `Band::least()`, else as PR 7329. Over the 275 census tests and the tour, list each touching cell that certifies under the mutant and not under the PR, with the lower end of ψ at the rectangle's far side and the pair's final verdict. Also count cells the PR refuses on a strictly signed rate at a crease. Review 2's sample (claim 8: no undecided read one-signed above rounding size, in eight tests) predicts both lists near empty; then the two end states differ only on constructed bodies.
- *The worked case that would settle it against me:* a body on analytic faces only, which a parting-aware meter classes crease, sign-only certifies, and whose two faces never get K·ε apart. I argue none exists (constant bending). A torus face leaving a spiric edge is the place to look.
- *Difference 3.* Build table row 1 (r = 1e-3, sine 1e-4) as a body twice, the cylinder face on the short-arc side and on the long-arc side. Read check 4's class and whether the edge certifies as an `Intersection`. Move the plane 5e-12 m and read again. A class that changes under a move of ε/200 fails Ev's test by demonstration.

**Corrections to my round-1 report.**
- "The apex stops being a problem in the engine" stands for step 3 only. Under step 1 the engine keeps the reach lever and the sine's limit at an apex, as the PR has it, which also has no apex problem.
- "PR 7329 can land as a step": still so, but the doc fix is now C's list (gate the carrier arm on the crease class, drop "is tier 3's wedge check's own", correct `wedge_reach.rs` and `folded_lever_arm`'s note), not mine.

**For the PR to Ev.** The three reports converge on the next build. The open choice is one sentence: does tier 3's wedge check become the certified statement over the whole edge (then the question's cells stop banding at creases), or does it stay a classifier (then the question's cell reading is the certified one, and "one reading for both" narrows to one class). The fold's meter is a separate question and should not ride this PR.

**Not checked.** I ran nothing in either round. The flatten-case claims are by hand. I did not read `link.rs`, PR 7262, or anything under `round2/` but `dispatch.md`.
