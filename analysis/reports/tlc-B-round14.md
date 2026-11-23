**(1)** The sweep has to be in first; it is hours.
- **What would change a sentence:** only a finished sweep body whose verdict is wrong on a curved wall with truthful bounds on both sides. Then (B)'s "a dihedral along a line edge" becomes "a dihedral between two planes along a line edge".
- **What would change only the list of what refuses:**
  - A Zero served at an inverted read. `sweep::blend::surgery` is the one door outside certification that the lane's point-by-point note found reading a face off its surface.
  - A cover under the truth with its boundary on the surface. That is a defect to file at the cover home, not a sentence.
- **Why wait:** sweep holds most curved walls of finished bodies, which is the class where (B) rests on argument, and core has only about 15,000 such reads.

**(2)** Nothing changes in (A), (B) or (C).

**(3)** Yes, it is the typed missing bound from above, and that type is step 1's. At certification it is already true in effect. Certification proves definite signs from below alone, and with covers read lazily it never asks for the bound from above, so the case is never evaluated there and no certification verdict changes. Only a decision that asks for a Zero compares the two and meets it.

## For Ev

**Decision asked: approve or strike three passages in `crates/geom-core/README.md`, "Float rounding".** Together they say a lever read off geometry is known from both sides, and that a dihedral along a straight edge is levered from the edge's line. I recommend all three (likely). I ran nothing; every figure is the lane's.

Terms:
- *Lever*: the length a tilt is multiplied by to get a displacement.
- *Bound from below / from above*: a length the region certainly reaches / certainly does not pass.
- *Site*: the point or line a lever's distance is measured from.
- *Dihedral*: the angle between an edge's two faces.
- *Straddle*: one decision whose two bounds read differently.
- *Cover*: a region known to contain a face.
- *Core*: the topo and step-import suites.

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

**Zeros rest on the cover being a bound from above. That was recounted with nothing folded in:** the covers' own far distance from the line, against the bound from below and against the truth. Core, ε = 1e-9, all 4,799 tests.

| face class | reads | cover under the truth by more than ε |
|---|---|---|
| plane, straight edges | 2,422,531 | 0 |
| plane, arcs | 222,780 | 0 |
| cylinder or cone wall | 10,737 | 108 |

- **All 108, and 21 more on inexactly read walls, are at two certification doors mid-construction.** Each has its boundary standing off the surface its face still carries: an edge certified onto a face still carrying its seed surface, or a rebased run before its move lands. No read past ε has its boundary on its surface.
- **No Zero, and no verdict that needs the bound from above, was served at any of them.** Certification reads from below only.
- **Below ε:** 176 hits on plane faces, all rounding, at most 2.4e-14.
- **The design's rule, run on core:** a cover short of the boundary by more than ε is a missing bound from above. Against the earlier run that folded the two bounds, test outcomes changed on 0 of 4,799.
- **Not in:** the same recount on the sweep suite, where most curved walls of finished bodies are; and at 1e-12 and 1e-6.

**The truth for cylinder and cone walls** is the boundary's farthest point from the line, by a convexity argument along rulings. It is checked on paper only; a check against an interior maximiser is not in. One hole, a cone's apex inside its face, is guarded.

**One body built to a known answer on a curved wall:** the open row's extrude case. A plane wall meets a cylinder wall along a ruling, declared tangent. The line site reads the lever 0.05 and the join reads smooth.

**The suites, line site against middle site at 1e-9:** no test changes outcome except the classes below.

### Classes that move

- **Declared-tangent joints off tangent by more than about ε over the arc's radius** (the open row; here 15ε and 19ε on a 1 m arc). Before: built with a sharp edge. Now: the join reads smooth and certification refuses it.
  - What refuses is certification's normal-parallelism check, levered at 1/κ = 1 m and never capped by the face's 0.05 reach.
  - Whether that arm is right is a separate row. Until it is settled, these bodies refuse where the profile accepts them.
- **Faces whose true width is inside the band** (1.5ε to 5.3ε wide, three tests). They built because the lever counted their length. They now refuse in band, and the exact lever agrees.
- **Faces of no width** (20 tests). They refuse at the arm check, as intended. The tests pin the old refusal and get rewritten.
- **One near-tangent keel wall** reads Zero where the middle read Negative. The lever is 0.067; the middle read 1.002. Its test still passes.

### Settled since the last draft

- **The `Dual` lane does not refuse.** The NaN is in a derivative only, from a square root at zero in the cover's frame arithmetic. No decision reads a derivative. Of 116,359 `Dual` verdicts at 1e-9, none is refused, straddled or moved. It has its own row for what it does cost.

### Unmeasured, and told as such

- **The sweep recount**, as above.
- **The curve-reading bound from below on lanes other than f64.** On f64 it is a numerical maximum. On `Interval`, `Dual` and `Sym` it is 17 samples, so those lanes read a shorter lever. Its cost there is not timed.
- **One corpus test newly refuses** (24 reads): must-carry is asked over a face that is still a placeholder mid-construction. Whether production asks it there is not attributed.
- **Curved edges and the joint turn:** no truth built, as (C) says.

### Order in the tree

1. The two-sided rule lands first, with a missing bound from above as a typed case and covers read lazily. It changes no README text.
2. The line site and the curve-reading bound from below join this PR after your answer. Text and code merge together.
3. Curved edges and the joint turn follow once their truths exist.

Confidence:
- The recommendation: likely.
- The inequality in (B): sure.
- The line as the site for two planes: sure.
- For cylinder and cone walls along a ruling: likely (argument on paper, one body built to a known answer, no counter-case).
- That covers bound from above: sure on plane faces (core); likely on cylinder and cone walls (about 15,000 core reads, every hit off its surface) until the sweep is in.
- The two-sided rule's soundness: sure. Its completeness on lanes other than f64: unsure.
- (C): sure that it is open.

## For the orchestrator

Changes to my round-13 section:

- **Lines to replace when the sweep reports:** "Not in: the same recount on the sweep suite…", the first bullet under "Unmeasured", and the confidence line on covers. If the sweep is clean, the sentence can read: "sweep: N reads on cylinder and cone walls, every read past ε off its surface, no Zero served at one". If it is not, see answer (1) for which result does what.
- **On your point C, the first-run figures.** I no longer cite either. The recount's table answers the point with nothing folded in. For the record, the two figures count different things:
  - 1,388,796 is every arc-bounded plane read in the first run.
  - 402,282 is the subset where the vertices under-read the truth. That is the only subset on which the first run's folded bound from above said anything about the cover.
- **Dropped from For Ev: the 282 M paired decisions** as evidence that the choice of bound changes no verdict.
  - That door also folded the bound from below into the bound from above.
  - 74% of its door reads have the two equal.
  - So an equal pair there may be the fold. The premise rests on the arithmetic alone, and the text now says so.
- **Dropped: "no served verdict differs from the truth's".** It follows from the recount plus the construction and is not separate evidence.
- **The `Dual` gate on step 1 is lifted.** Its row stays beside the fork.
- **Step 1's remaining gates:**
  - the `contact_edge_must_carry` attribution;
  - the site mint's cover read off held geometry;
  - door against tier 3.
- **Step 2's gate:** the curve read as one read on every lane.
- **Still owed, on landing and not on the question:**
  - the interior-maximiser check of the ruled truth;
  - the recount at 1e-12 and 1e-6;
  - the unattributed 66-verdict path difference between the design run and the folded run. It should be explained before the design run's "0 of 4,799" is quoted as final. A path difference with identical outcomes is probably the typed case short-cutting a read, but it has not been shown.
- **For the sweep run, please have it report by door.** Blend surgery is the door I named in (1). If it serves a Zero over a strip's neighbour before the swap, the design refuses it, and that is a class for Ev.

Unchecked by me: the recount's own instrument (I read the lane's table as given, not its patch); which suite holds the three in-band-width tests and the keel.