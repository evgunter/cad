**(1)** Yes, ask now, with the design run on the sweep stated as running.
- Both things I held the question for are answered by the recount. No Zero, and no verdict needing the bound from above, stands at an inverted read at any door. No at-rest door has a cover short of its boundary past ε.
- With no Zero at an inverted read, the design's rule cannot serve differently from the folded run, so the run in progress is a confirmation.
- If it changes an outcome, that amends the list of what refuses before any code lands. It changes no sentence.

**(2)** Nothing in (A), (B) or (C) changes, and none of the four states belongs in the text.
- They are mid-construction states of four doors. The page already says the bound from above is read off a cover, and a face with none has no such bound.
- The typed missing bound from above is still the build's answer for all four, entered by the door, which knows its own state. An inversion found without a door having said so falls into the same case and is counted.
- One caution for the build. At two of the four (a surface set before its boundary moves; a boolean's working face) the stale side is the boundary, so the bound from below is read off points that are not yet on the face. Detail is in the orchestrator's section.

## For Ev

**Decision asked: approve or strike three passages in `crates/geom-core/README.md`, "Float rounding".** Together they say a lever read off geometry is known from both sides, and that a dihedral along a straight edge is levered from the edge's line. I recommend all three (likely). I ran nothing; every figure is the lane's.

Terms:
- *Lever*: the length a tilt is multiplied by to get a displacement.
- *Bound from below / from above*: a length the region certainly reaches / certainly does not pass.
- *Site*: the point or line a lever's distance is measured from.
- *Dihedral*: the angle between an edge's two faces.
- *Straddle*: one decision whose two bounds read differently.
- *Cover*: a region known to contain a face.
- *Core* and *sweep*: the two test suites measured (topo with step-import; the sweep crate).

### The passages

**(A)** Lever bullet. After "no longer than the site's own measure of it:", replacing the sentence that sorts verdicts into three kinds:

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points known to lie on the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

**(B)** Consumed-extent bullet. Replacing "Its lever is the extent's far distance from that point … holds its region, not a lever.":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**(C)** Under `## Open`, replacing "Nothing on this page.":

> Which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

### Premise, and the text this replaces

- **The fork was opened on "which bound does each of about 41 callers need". That is the wrong question.** A reading multiplied by its lever grows with the lever. So a definite sign is proved at the bound from below and a Zero at the bound from above, whoever acts on it. This is an argument, not a count.
- **The measured defect is the site.** On straight edges between plane faces with straight boundaries, the exact lever is the farthest vertex's distance from the edge's line. The points' reach from the edge's middle exceeds it on 99.5% of 8.78 M reads. It counts the faces' run along the edge.
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
2. **(A) and (C), with (B) struck.** Zeros become sound. Definite signs stay over-read as today. The open row `the-wedge-reach-counts-a-faces-run-along-its-edge` stays as it is: a sharp edge built on a declared-tangent joint, passing tier 3.
3. **(B) narrowed to two planes.** Cylinder and cone walls along a ruling stay at the middle. That row's extrude case is one of them, so it stays wrongly built.
4. **Nothing.** Today.

Each is reversible: the site is one read in the consumer, and the text is two sentences.

**Worked example.** A strip 4 long and 0.05 wide, at 15ε to its neighbour along its long edge.
- From the middle the lever is 2, so the reading is 30ε: a corner.
- From the edge's line the lever is 0.05, so the reading is 0.75ε: the tangent joint that was declared.

A second case: a half-disc along its diameter. Both vertices are on the line, so the bound from below has to read the arc; the lever is the radius.

### The evidence

**Definite signs need no measurement.** The bound from below is read off points of the boundary, so the region reaches at least that far.

**Zeros rest on the cover being a bound from above. That was counted with nothing folded in:** the covers' own far distance from the line, against the exact lever. ε = 1e-9, every test in both suites.

| face class | reads, core | reads, sweep | cover short by more than ε, core / sweep |
|---|---|---|---|
| plane, straight edges | 2,422,531 | 1,768,879 | 0 / 0 |
| plane, arcs | 222,780 | 554,854 | 0 / 0 |
| cylinder or cone wall | 10,737 | 539,124 | 108 / 5,246 |

- **Every read past ε is at a door working on a face that is not finished.** There are four such states:
  - a run of edges re-based before its move lands;
  - an edge certified onto a face still carrying its seed surface;
  - a surface set before its boundary moves (offsets and shells; the boundary stands exactly the offset distance off);
  - a boolean's working face.
- **Of 725 such wall reads examined, each has a wall face 0.02 to 2 off its own surface.** That is a sample: at most five per door and class per test, with every one of the 179 affected tests represented.
- **No door that reads a finished body has a cover short past ε.** The largest shortfall there is 5e-10, and it equals the boundary's own distance off its surface.
- **No Zero stands at any such read.** The 564 verdicts served at them are all definite signs, proved from below.
- **The design's rule** treats a cover short by more than ε as a missing bound from above. Run on core, it changed 0 of 4,799 test outcomes against the run that folded the two bounds. The same run on the sweep is in progress. Since no Zero stands at such a read, it is expected to change nothing.

**The exact lever for cylinder and cone walls** is the boundary's farthest point from the line, by a convexity argument along rulings. It is checked on paper only; a check against an interior maximiser is not in. One hole, a cone's apex inside its face, is guarded.

**One body built to a known answer on a curved wall:** the open row's extrude case. A plane wall meets a cylinder wall along a ruling, declared tangent. The line site reads the lever 0.05 and the join reads smooth.

**The suites, line site against middle site at 1e-9:** no test changes outcome except the classes below.

### Classes that move

- **Declared-tangent joints off tangent by more than about ε over the arc's radius** (the open row; here 15ε and 19ε on a 1 m arc). Before: built with a sharp edge. Now: the join reads smooth and certification refuses it.
  - What refuses is certification's normal-parallelism check, levered at 1/κ = 1 m and never capped by the face's 0.05 reach.
  - Whether that arm is right is a separate row. Until it is settled, these bodies refuse where the profile accepts them.
- **Faces whose true width is inside the band** (1.5ε to 5.3ε wide, three tests). They built because the lever counted their length. They now refuse in band, and the exact lever agrees.
- **Faces of no width** (20 tests). They refuse at the arm check, as intended. The tests pin the old refusal and get rewritten.
- **One near-tangent keel wall** reads Zero where the middle read Negative. The lever is 0.067; the middle read 1.002. Its test still passes.

### Unmeasured, and told as such

- **The design's rule on the sweep:** running, as above.
- **The counts above at 1e-12 and 1e-6.**
- **The curve-reading bound from below on lanes other than f64.** On f64 it is a numerical maximum. On `Interval`, `Dual` and `Sym` it is 17 samples, so those lanes read a shorter lever. Its cost there is not timed.
- **One corpus test newly refuses** (24 reads): must-carry is asked over a face that is still a placeholder mid-construction. Whether production asks it there is not attributed.
- **Curved edges and the joint turn:** no exact lever built, as (C) says.

Settled since earlier drafts: the `Dual` lane does not refuse. Its NaN is in a derivative that no decision reads, and none of its 116,359 verdicts is refused, straddled or moved.

### Order in the tree

1. The two-sided rule lands first, with a missing bound from above as a typed case that an unfinished face's door enters, and covers read lazily. It changes no README text.
2. The line site and the curve-reading bound from below join this PR after your answer. Text and code merge together.
3. Curved edges and the joint turn follow once their exact levers exist.

Confidence:
- The recommendation: likely.
- The inequality in (B): sure.
- The line as the site for two planes: sure.
- For cylinder and cone walls along a ruling: likely (argument on paper, one body built to a known answer, no counter-case in 550,000 reads).
- That covers bound from above: sure on plane faces; likely on cylinder and cone walls (none short at any finished body; the unfinished ones examined by sample).
- The two-sided rule's soundness: sure. Its completeness on lanes other than f64: unsure.
- (C): sure that it is open.

## For the orchestrator

Changes to my round-14 section:

- **The gate on asking is lifted.** Replace the one "in progress" line when the sweep's design run reports. If it changes an outcome, attribute it by door before anything else: by the recount it should not be able to.
- **How the typed case is entered.** I sign "entered from what the door knows", with the inversion check kept as the fallback into the same case.
  - A door that knows its face is unfinished should say so, and then no cover is computed at all. That is also the lazy read.
  - An inversion past ε at a door that did not say so is the count to watch. On finished bodies it is zero in both suites. A nonzero count there is a defect to file at its source.
- **The stale side differs by door, and the build should not treat the four alike without saying so.**
  - Rebased run, seed surface: the surface or the faces are stale. The points are what the door holds, so the bound from below is good.
  - Surface set before its boundary moves, and the boolean's working face: the boundary is stale. The bound from below is read off points that stand up to the offset distance, or 0.16 to 0.56, off the face.
    - The 560 `tangent_second_order` Positives at the graft door are served on such points.
    - That is today's behaviour, and tier 3 re-asks at rest. But there "points known to lie on the region" is not true of what the door reads.
    - I would file it as a row beside the fork: those two doors should read their bound from below off the boundary as they leave it, or hold their definite verdicts as provisional by type.
- **The 342 wall reads where the cover is short of the face's own vertices** are the same states, with a vertex off its surface. Nothing separate to file.
- **What the off-surface attribution rests on.** The by-door figures are samples, capped at five per door and class per test. For Ev says so. A full by-door count of reads past ε, without detail lines, would make "every read" exact. I would run it with the design run and not wait for it.
- **Withdrawn:** the 66-verdict path difference (fuzz inputs) and blend surgery as a separate door (it reaches the kernel through `certify_edge_spec`).
- **Unchanged:**
  - step 1's gates: the `contact_edge_must_carry` attribution, the site mint's cover, door against tier 3;
  - step 2's gate: one curve read on every lane;
  - owed on landing: the interior-maximiser check and the recount at the other two tolerances;
  - the rows beside the fork: `tangent_normal_parallel`'s arm and the `Dual` derivative.

Unchecked by me: the recount's instrument; whether `set_face_surfaces_certifying` re-certifies its edges after the boundary moves, or leans on tier 3 alone.