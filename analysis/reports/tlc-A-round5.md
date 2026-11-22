## For Ev

**I sign the other report's two passages with one amendment: keep the records clause.** The coordinator's reading of where we agree is right. On the six differences:

1. **Lever bullet wording: I sign theirs.** "Measured from the site its datum is read at" leaves the site to passage (b), which says what that site is. "A face with no surface yet" names what the measurement found. "The region is not built yet" did not.
2. **"Any point is sound" is no longer true as written,** so (b) replaces it.
   - It holds for a Zero: a point's far distance is never less than the distance from the line the datum turns about.
   - It fails for a definite sign: that same distance over-states the lever. Measured from the edge's middle, it exceeded the exact truth on 99.5% of reads.
   - How this relates to PR 6968: you asked how the midpoint could be sound, since it is no witness. The answer approved there ("any pivot is sound provided the gap and lever are read at it") was true only of the Zero side. (b) answers your original doubt properly: any point proves a Zero, and a definite sign about a known line is levered from that line.
   - The amendment: (b) as drafted also dropped "so a record that verdicts read from different points holds its region, not a lever", which you approved on PR 7141. That clause stays.
3. **Scope: every line edge, and you should know exactly what that rests on.**
   - The comparison is exact for any point set: distance from a line is never more than distance from any point of it.
   - The tilt model underneath is first order in the tilt, at every site. Today's midpoint site rests on that same model.
   - Measured against an exact truth: plane–plane edges whose faces have only straight edges.
   - Not measured: line edges with a curved face or a curved boundary edge (1.12 M reads skipped). Those rest on the exact inequality, not on a truth.
   - Any class the line site moves there is reported to you on the PR before its code merges.
4. **No `## Open` line on the page.** The curved edge and the joint turn go in a tracker row and in the PR's list of what is not yet decided. The page stays present tense.
5. **Order: the text merges with the code that makes it true.** The `[ev]` PR states the question by editing the page. Once you answer, the line-site code joins that same PR and it merges once. Merging the text earlier would put a property in the README that the tree does not have.
6. **Gating is listed below.**

**Passage (a).** In `crates/geom-core/README.md`, "Float rounding", the lever bullet, after "no longer than the site's own measure of it:":

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points of the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above, whatever acts on the verdict. Where the two disagree the reading is in band and is refused as one. Where a face has no surface yet nothing proves a Zero over it. No consumer takes one bound.

**Passage (b).** The consumed-extent bullet, replacing "Its lever is the extent's far distance from that point … holds its region, not a lever.":

> Its lever is the extent's far distance from the site its datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times that point's distance from the line it turns about, which is never more than its distance from any point of that line. So a lever read from a point proves a Zero, and a definite sign of a datum that turns about a known line is levered from that line: a dihedral along a line edge, from the edge's line. A record that verdicts read from different sites holds its region, not a lever.

**What is measured before what.**
- **Gates your approval:** nothing further. (b) rests on an inequality that holds exactly and on the population measured against an exact truth.
- **Gates the line-site code merging into the `[ev]` PR:**
  - the line site run on every line edge, with the classes it moves (judged against the exact truth where one exists, otherwise reported as moved with no truth);
  - the comparison of both bounds again at the line site (the 4 reads that refuse there);
  - the line-into-arc table of the open row `honeone/the-wedge-reach-counts-a-faces-run-along-its-edge`, where the declared profile is the truth.
- **Gates the first unit** (the two-sided rule read from the middle):
  - door against tier 3 (running; target zero);
  - which bodies the few dozen readings that act on a Zero over a face with no surface yet belong to, and whether those faces are the ones whose surfaces the verdict reads;
  - the 6 reads where both faces are covered and the bound from above is still unusable;
  - why `classify_shared_rim` made a different number of decisions on two runs (determinism);
  - closed-form rows that make the two bounds disagree, built from the 8,092 reads where the bound from above is more than 1,000 times the bound from below, since the suites hold none.
- **Gates the loud refusal for out-of-order bounds:** the 29,468 boundary-edge points past the bound from above. Judged by the offending point's distance off the face's surface: on the surface means a defect in the cover; off it means the door's record is not that face.
- **Gates later questions:**
  - curved edges: the truth is the coaxial rim, solved in the profile's meridian plane, judging tangent line against carrier;
  - the joint turn: which region it governs is to be read out of the code first, and no truth exists yet;
  - how often the reach actually set the folded lever (the lever's binding share), before anyone quotes "no disagreement found" as evidence about the reach.

**Asked now:** (a) and (b) together, for every line edge, as one decision. Strike (b) and (a) still says nobody chooses a bound, but the bound from below is then not one at today's site.

**Not yet decided:** the site on a curved edge; the joint turn's site and region; any class the line site moves outside the measured population; the 29,468.

**What happens to the tree, in order:**
1. The two-sided rule lands, read from the middle, with no README change. It moves no verdict where both bounds are finite.
2. The line site lands with (a) and (b) in the `[ev]` PR, after its moved classes are reported to you there.
3. Curved edges and the joint turn follow, once their truths exist.

Confidence: (a) likely. (b): the inequality is sure; the wording and the reach to every line edge are likely.

## For the orchestrator

- **Amendment to the other report's (b):** its replacement span runs to "read at the same one", which deletes the PR 7141 records clause. I restored it with "sites" in place of "points". If the other designer intended to drop it, that is a split for Ev, since Ev "was less sure about the records thing".
- **Conceded to the other report:**
  - its wording of (a);
  - no `## Open` line;
  - text merging with the code that makes it true;
  - the first unit adding closed-form straddle rows.
- **Held, now agreed in substance:** the scope is every line edge, with Ev told exactly what the unmeasured part rests on.
- **Unchecked:** whether other point-site levers prove definite signs correctly, for example a cone apex levered along its slant. (b) binds only a datum that turns about a known line, so those are untouched.
