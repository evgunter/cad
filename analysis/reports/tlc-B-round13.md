**(a)** No. It was the fold, not the covers. The first run's door starts the bound from above at `far(own).max(lo)`, and on straight-edged plane faces the truth is the farthest vertex, which is `lo`. What replaces it: the first run's arc-bounded plane reads where the vertices under-read the truth (402,282), on which the cover was at or above the truth on all but 80; and the lane's point-by-point count, which found no cover missing a boundary point that lies on its face's surface.

**(b)** Instrument. The design orders the two bounds within ε only. On the lane's note of boundary points outside their covers, I return to the typed "no bound from above" case for a larger inversion. Every such read there is mid-construction with the recorded surface as the stale side, so the bound from below stands and nothing proves a Zero. My round-3 "neither bound trusted" would stop certification at its busiest doors. The build must not copy the fold.

**(c)** A gate on asking Ev. The fold hid exactly the reads the design treats differently: where the cover falls short of a boundary point, the run served Zeros on the bound from below, and the design refuses them. So the run's list of what is still refused, which my text gives Ev, is not the designed build's list. The recount also has to report every Zero served at such a read. On cylinder and cone walls, nothing independent yet shows the cover read from a line is a bound.

## For Ev

**Decision asked: approve or strike three passages in `crates/geom-core/README.md`, "Float rounding".** Together they say a lever read off geometry is known from both sides, and that a dihedral along a straight edge is levered from the edge's line. I recommend all three (likely). I ran nothing; every figure is the lane's. One recount is owed before this is sent, named below.

Terms:
- *Lever*: the length a tilt is multiplied by to get a displacement.
- *Bound from below / from above*: a length the region certainly reaches / certainly does not pass.
- *Site*: the point or line a lever's distance is measured from.
- *Dihedral*: the angle between an edge's two faces.
- *Straddle*: one decision whose two bounds read differently.
- *Cover*: a region known to contain a face.

### The passages

**(A)** Lever bullet. After "no longer than the site's own measure of it:", replacing the sentence that sorts verdicts into three kinds:

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** Consumed-extent bullet. Replacing "Its lever is the extent's far distance from that point … holds its region, not a lever.":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

### Premise, and the text this replaces

- **The fork was opened on "which bound does each of about 41 callers need". That is the wrong question.** A definite sign is proved by the bound from below and a Zero by the bound from above, whoever acts on it. Measured at today's site over about 282 M paired decisions, the choice of bound changes no verdict.
- **The measured defect is the site.** On straight edges between plane faces, the points' reach from the edge's middle exceeds the exact lever on 99.5% of reads. It counts the faces' run along the edge.
- **(B) narrows "any point is sound provided the datum and its lever are read at the same one".** That clause is agent-written. You approved it on PR 6968 after asking how a midpoint that "doesn't look like a witness" could be sound.
  - The answer given then holds for a bound from above, which needs no witness.
  - A definite sign does need one: a point that really moves that far. Your question was the right one for it.
- The sentence (A) replaces is also agent-written, approved on PR 7141.

### Final states

1. **(A), (B) and (C): recommended.**
   - Every levered decision reads both bounds.
   - Wedges on straight edges read from the edge's line.
   - Curved edges and the joint turn stay at the middle, named as open.
   - Left possible: a door building its region from too little geometry. Review holds that, as today.
2. **(A) and (C), with (B) struck.** Zeros become sound. Definite signs stay over-read exactly as today. The open row `the-wedge-reach-counts-a-faces-run-along-its-edge` stays as it is: a sharp edge built on a declared-tangent joint, passing tier 3.
3. **(B) narrowed to two planes.** Cylinder and cone walls along a ruling stay at the middle. That row's extrude case is one of them, so it stays wrongly built.
4. **Nothing.** Today.

Each is reversible: the site is one read in the consumer, and the text is two sentences.

**Worked example.** A strip 4 long and 0.05 wide, at 15ε to its neighbour along its long edge.
- From the middle the lever is 2, so the reading is 30ε: a corner.
- From the edge's line the lever is 0.05, so the reading is 0.75ε: the tangent joint that was declared.

A second case: a half-disc along its diameter. Both vertices are on the line, so its bound from below has to read the arc; the lever is the radius.

### What the second run shows

Line site, both bounds at every decision, the bound from below reading each boundary edge's farthest point. Three tolerances, four suites at 1e-9.

- **No served verdict differs from the verdict at the true lever**, wherever a truth exists (9.1 M verdicts at 1e-9). The truth exists for plane faces and for cylinder and cone walls.
- **No refusal on a valid body comes from the site, or from a loose cover.**
- **One served verdict moves against the middle site.** A near-tangent keel wall reads Zero where the middle read Negative. Both bounds and the truth give a lever of 0.067; the middle read 1.002.

What that rests on, so you can weigh it:
- **Definite signs are sound by construction.** Every point read lies on the boundary.
- **The run cannot speak for the cover.** Its door took the larger of the cover's reach and the bound from below as its bound from above. So three of its counts hold by construction and are not evidence about covers:
  - "above never below below";
  - "no wrong-direction pair";
  - "never under the truth" on straight-edged plane faces, where the truth is the farthest vertex. The earlier runs' counts on those faces hold the same way.
- **What is independent evidence that a cover bounds from above:**
  - Arc-bounded plane faces in the first run, where the vertices under-read the truth (402,282 reads). The cover was at or above the truth on all but 80. All 46 of those examined are an arc standing 1.0 to 2.7 m off its face's plane before a move lands, where that truth does not apply. The other 34, in the same tests, were not examined one by one.
  - A point-by-point count at 1e-9. No cover misses a boundary point that lies on its face's surface. Every boundary point outside its cover is off the surface by about as much, mid-construction; at rest, only on fixtures built to be refused.
- **For cylinder and cone walls,** nothing independent yet shows the cover read from a line is a bound. The truth there is the boundary's farthest point, by a convexity argument along rulings checked on paper only. One hole, a cone's apex inside its face, is now guarded.
- **"Equal to the truth" is partly circular.** On exactly read faces the truth is the bound from below.
- **The one check made by construction on a curved face** is the open row's extrude case: the declared tangency reads smooth.

**The recount owed before this is sent.** The cover's own reach, unfolded, against the bound from below and against the truth, by face class. With it: every Zero the run served where the cover fell short of a boundary point.
- The design does not take the larger of the two. A cover that falls short by more than ε means the face's recorded surface is not yet its own. That is a bound from above that is missing: definite signs stand and nothing proves a Zero.
- So those Zeros are refusals in the build as designed, and the run's list of what is still refused does not include them.

### Classes that move, for you to see with the question

- **Declared-tangent joints off tangent by more than about ε over the arc's radius** (the open row; here 15ε and 19ε on a 1 m arc). Before: built with a sharp edge. Now: the join reads smooth and certification refuses it.
  - What refuses is certification's normal-parallelism check, levered at 1/κ = 1 m and never capped by the face's 0.05 reach.
  - Whether that arm is right is a separate row. Until it is settled, these bodies refuse where the profile accepts them.
- **Faces whose true width is inside the band** (1.5ε to 5.3ε wide, three tests). They built because the lever counted their length. They now refuse in band, and the truth agrees.
- **Faces of no width** (20 tests). They refuse at the arm check, as intended. The tests pin the old refusal and get rewritten.
- **Whatever the recount adds:** Zeros asked over a face whose recorded surface is not yet its own.

### Unmeasured, and told as such

- **The `Dual` lane.** The bound from above carries a NaN derivative on covered faces in 188 of 202 sampled reads. Whether the two-sided rule then refuses them is not known.
- **The curve-reading bound from below on the other lanes.** On f64 it is a numerical maximum. On `Interval`, `Dual` and `Sym` it is 17 samples, so those lanes read a shorter lever than f64 does. Its cost there is not timed.
- **One corpus test newly refuses** (24 reads): must-carry is asked over a face that is still a placeholder mid-construction. Whether production asks it there is not attributed.
- **Curved edges and the joint turn:** no truth built, as (C) says.

### Order in the tree

1. The two-sided rule lands first, with a missing bound from above as a typed case and covers read lazily. A face not built yet and a face whose surface is not yet its own are both that case. It changes no README text.
2. The line site and the curve-reading bound from below join this PR after your answer. Text and code merge together.
3. Curved edges and the joint turn follow once their truths exist.

Confidence:
- The recommendation: likely.
- The inequality in (B): sure.
- The line as the site for two planes: sure for definite signs (the bound from below is the exact truth there).
- For cylinder and cone walls along a ruling: likely (argument on paper, one check by construction, no counter-case).
- That covers bound from above: likely on plane faces; unsure on cylinder and cone walls read from a line, until the recount.
- The two-sided rule's soundness: sure. Its completeness on lanes other than f64: unsure.
- (C): sure that it is open.

## For the orchestrator

Changes to my round-12 section:

- **The first bullet under "to confirm" is now confirmed, and wider than I wrote.** It reaches the straight-edged counts in the first run and in the sites note too. I had rested the cover on exactly that class.
- **The bounds' constructor: I leave my round-3 position.** That was "past ε, neither bound is trusted and every decision refuses". The lane's note on boundary points outside their covers shows why it is wrong:
  - Every such read is a boundary point off the recorded surface, mid-construction: an edge certified onto a face still carrying its seed's or its predecessor's surface, a rebased run before the move lands, the blend's surgery before the swap, import's adopt.
  - It counts 37,150 such reads at the middle site, 35,015 of them at `euler::certify_rebased_run`.
  - The stale side is the surface, so the cover is the bound that is missing. The points are what the door holds.
  - The right case is therefore the typed missing bound from above, with definite signs standing.
  - At rest every such read is a fixture built off its face, which tier 3 refuses by incidence.
- **What the recount must carry, beyond what the lane named:**
  - Per door, the decisions that served a Zero at a read where the unfolded cover is short of the bound from below by more than ε. That is the class the design refuses and the run served.
  - The cover's line read against the ruled truth on cylinder and cone walls. A cover under that truth with every boundary point on the surface is a defect in the line read of a wall or frustum piece. File it at the cover home; it gates step 2.
- **The PR waits for it.** This is the same reason as in round 8. The second run was to be the build as designed, and in this one respect it was not.
- **Unchanged:** the landing gates on steps 1 and 2, the rows beside the fork, and the note that config B's bound from above must not come from a bracketed search.

Unchecked by me: the second run's whole door beyond the quoted line; whether any at-rest read has an unfolded cover short of an edge point read from a line. The point-by-point note says none from a point, and a line read of the same cover is a different read.