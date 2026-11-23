**(a)** (B) stands as signed. No verdict is wrong with truthful bounds on both sides, and no refusal on a valid body traces to the site. Config B is not needed to say so: it can only tighten the bound from above, and config A has no refusal from a loose one.

**(b)** Nothing changes. (A), (B) and (C) are quoted in full below.

**(c)** Part of the build that follows, in step 2. The first run showed the line site cannot land without it. It is not in Ev's text, which says "points known to lie on the region". Ev is told, because it reverses "not built".

**(d)** A consequence Ev is told is unmeasured. It gates step 1 landing, not the question. If every covered face refused on `Dual`, no sentence would change. It would be a defect in how a bound is read on that lane, and a whole lane newly refusing would come to Ev as a class before step 1 lands.

**(e)** A row beside it. It is certification's own decision, on a curvature arm this fork does not touch. It is named for Ev because it is what now refuses the open row's bodies.

## For Ev

**Decision asked: approve or strike three passages in `crates/geom-core/README.md`, "Float rounding".** Together they say a lever read off geometry is known from both sides, and that a dihedral along a straight edge is levered from the edge's line. I recommend all three (likely). I ran nothing; every figure is the lane's.

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
- **The cover as a bound from above** was never under an independently computed truth on plane pairs in the first run: 0 of 11.93 M straight-edged reads. On arcs, 80 reads were under it. All 46 examined are an arc standing 1.0 to 2.7 m off its face's plane before a move lands, where that truth does not apply. The other 34, in the same tests, were not examined one by one.
- **For cylinder and cone walls** the truth is the boundary's farthest point, by a convexity argument along rulings that was checked on paper only. One hole, a cone's apex inside its face, is now guarded.
- **"Equal to the truth" is partly circular.** On exactly read faces the truth is the bound from below.
- **The one check made by construction on a curved face** is the open row's extrude case: the declared tangency reads smooth.

### Classes that move, for you to see with the question

- **Declared-tangent joints off tangent by more than about ε over the arc's radius** (the open row; here 15ε and 19ε on a 1 m arc). Before: built with a sharp edge. Now: the join reads smooth and certification refuses it.
  - What refuses is certification's normal-parallelism check, levered at 1/κ = 1 m and never capped by the face's 0.05 reach.
  - Whether that arm is right is a separate row. Until it is settled, these bodies refuse where the profile accepts them.
- **Faces whose true width is inside the band** (1.5ε to 5.3ε wide, three tests). They built because the lever counted their length. They now refuse in band, and the truth agrees.
- **Faces of no width** (20 tests). They refuse at the arm check, as intended. The tests pin the old refusal and get rewritten.

### Unmeasured, and told as such

- **The `Dual` lane.** The bound from above carries a NaN derivative on covered faces in 188 of 202 sampled reads. Whether the two-sided rule then refuses them is not known.
- **The curve-reading bound from below on the other lanes.** On f64 it is a numerical maximum. On `Interval`, `Dual` and `Sym` it is 17 samples, so those lanes read a shorter lever than f64 does. Its cost there is not timed.
- **One corpus test newly refuses** (24 reads): must-carry is asked over a face that is still a placeholder mid-construction. Whether production asks it there is not attributed.
- **Curved edges and the joint turn:** no truth built, as (C) says.

### Order in the tree

1. The two-sided rule lands first, with a face not built yet as a typed case and covers read lazily. It changes no README text.
2. The line site and the curve-reading bound from below join this PR after your answer. Text and code merge together.
3. Curved edges and the joint turn follow once their truths exist.

Confidence:
- The recommendation: likely.
- The inequality in (B): sure.
- The line as the site for two planes: sure (exact truth).
- For cylinder and cone walls along a ruling: likely (argument on paper, one check by construction, no counter-case).
- The two-sided rule's soundness: sure. Its completeness on lanes other than f64: unsure.
- (C): sure that it is open.

## For the orchestrator

**Things to confirm before the body quotes the lane's "independent checks"**
- **"Bound from above never below the bound from below" may hold by construction.** In `measure.patch` at the named commit, the line-site door starts the bound from above at `far(own).max(lo)`: the points are folded in. If the second run's door does the same with the edge points, that zero is not evidence. That is why For Ev rests the cover's soundness on the first run's comparison with an independently computed truth. One recount with the fold removed would make it evidence.
- **"No served verdict differs from the truth's" is circular on exact reads** for the same reason the lane gives for "equal". I told Ev so.
- **The convexity truth has no machine check.** One test against a dense interior maximiser on a cylinder wall and a cone frustum would turn "on paper" into "checked", and would catch a second hole like the apex.

**Gates on landing, none on the question**
- **Step 1:**
  - the `Dual` NaN;
  - the `contact_edge_must_carry` attribution (a harness artefact, or a door that production asks mid-construction);
  - the site mint's cover read off held geometry, which the pole-cap turns need (6 + 1 refusals in this build);
  - door against tier 3.
- **Step 2:** the curve read as one read on every lane.
  - As built it is a golden-section maximum on f64 and 17 samples elsewhere, so lanes disagree on the lever and the verdict-diff between lanes will see it.
  - The squared distance of a conic from a line is a degree-two trigonometric polynomial; its critical points have a closed form that every lane can take.
  - A missed maximum is safe for the bound from below. It is not safe if config B's bound from above uses the same maximiser.
- **Config B:** nothing waits on it. With no refusal from a loose cover, the arc piece's closed-form bound from above drops out of step 2. If config B is later adopted, its bound from above must not come from a bracketed search.

**Rows to file beside the fork**
- **`tangent_normal_parallel`'s arm.** It reads `Margin::levered_inv(sin θ, |κ_rel|)`, which is D4's tangency lever, so it is ratified form and not a plain bug. The question is which displacement is held against the tolerance for a declared tangency: the locus offset `sin θ / κ`, or the faces' separation over their reach. The page's opening sentence from PR 7410 puts it that way. It probably ends as an `[ev]` question of its own, and it is what the honeone row now waits on.
- **The carrier-ends mismatch.** 11,556 reads at 1e-9 where a boundary edge's carrier ends are not its face's vertices by more than 1e-9, and `certify_rebased_run` reading an arc up to 2.7 m off its face before a move lands. On those reads the "points known to lie on the region" do not. That is the door handing a face record that is not the face it governs, the loud-refusal case agreed in round 3.

**Unchecked by me**
- The second run's patch: `measure.patch` at this commit shows the first run's door.
- The 34 unexamined reads.
- Whether a ruled spline face along a line (17 samples, no truth) occurs in any suite body.
- What the `Dual` lane's classification reads of a bound's derivative.