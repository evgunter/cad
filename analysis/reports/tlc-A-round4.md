## For Ev

**Recommendation: one decision now, not two.** The lever sentence, as sent, says a lever is known "from below off points of the region". The measurements show that at today's site this is false of the lever a verdict is owed, on nearly every read where we have an exact truth. So the sentence goes to you together with the site rule for edges whose datum turns about a line (likely). The remaining site cases, curved edges and the joint turn, are not yet askable.

Terms:
- **Bound from below / from above**: a length the region certainly reaches / certainly does not pass.
- **Site**: what a lever's distance is measured from.
- **Hinge**: the line a reading turns about. For a dihedral (the angle between an edge's two faces) on a straight edge, that is the edge's line.
- **Straddle**: one decision whose two bounds read differently.

### What the measurements changed

1. **The defect is the site, not the choice of bound (sure).**
   - Across about 282 million paired decisions, at three ε and on four number lanes, the two bounds never gave different verdicts where both were finite.
   - On plane-plane straight edges with an exact truth (the farthest face vertex's distance from the edge's line), the "bound from below" read from the span's middle exceeds the truth on 99.5% of reads.
   - On 50 reads both bounds then agree on a wrong verdict: a corner or a refusal where the truth is a flat join. All 50 are degenerate test faces whose vertices lie on the edge's line.
   - Read from the line, the bound from below equals the truth. The only differences are 4 reads where the bound from above alone reads a corner and the truth is in band. Those 4 refuse, which is the safe direction.
2. **A missing bound from above means "this face has no geometry yet"** (sure). Every uncovered face is a placeholder surface or an unbounded plane, cylinder or cone with no loop yet, read mid-construction. No described spline face lacks a cover.
3. **Adding boundary edges to the bound from below cures nothing**: there was nothing to cure. It costs 40–90× more on the symbolic lanes than on plain floating point. **I would not build it.**
4. **No consumer needs a coarser question** such as "is it definitely Negative?". Every pair at the two such consumers agreed. **I would not build one.**

### Answers to the coordinator's questions

- **Is the sentence true of the tree after the first unit?** Only literally.
  - The points give a bound from below on distance from the middle point. The README's own "far distance from that point" makes that quantity the lever.
  - But a tilt about a line moves a face point by the tilt times its distance from that line. A distance from a point on the line is longer, so it over-states what a definite sign needs.
  - Approving the sentence alone would approve a rule the code meets in name and breaks in substance. Its own figure is 99.5% of the reads with a truth.
- **What is askable about the site.**
  - Askable now: datums that turn about a straight line, from the measurement.
  - Not askable: which line on a curved edge (the station's tangent line, or the edge's carrier itself), and where the joint turn is read. No truth was built for either.
  - Because the sentence is only true with the site rule, it is one decision with the straight-line site, plus an open item for the rest.
- **The missing bound from above becomes a typed case named for what it is: the face is not built yet.**
  - It is not a cover failure.
  - Definite signs still stand from below, so certification, by far the largest reader of such faces, is unchanged.
  - Nothing proves a Zero.
  - The finished body's tier-3 check stays the decision of record. It re-reads every edge with real faces.
- **Cost changes one thing in the final state: covers are read lazily.**
  - A door hands back the region, and a face's cover is computed only when a decision needs the bound from above.
  - Each face's cover is kept once per body state, not rebuilt for every edge.
  - Today certification computes covers it never reads, and the cone-lift cycle recomputes the same unlifted covers once per lift. Those two account for 75% of cover computations.

### What I would have you approve now (approve or strike each)

**(A) The lever bullet**, in `crates/geom-core/README.md`, "Float rounding". After "no longer than the site's own measure of it:":

> a lever read off a consumed extent is known from both sides, from below off points of the region and from above off its cover, each read from the site its datum turns about, and a verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above, whatever acts on the verdict. Where the two disagree the reading is in band and is refused as one. Where the region is not built yet nothing proves a Zero. No consumer takes one bound.

**(B) The consumed-extent bullet.** Replace "Its lever is the extent's far distance from that point" with:

> Its lever is the extent's far distance from the site its datum turns about: the line, for a datum that turns about one (a dihedral about a straight edge), else the point the datum is read at.

**(C) A line in that page's `## Open` section:**

> which line a dihedral along a curved edge turns about (its station's tangent line or the edge's carrier), and where the joint turn's lever is read.

**What landing (A)–(C) does:**
- Wedge readings on straight edges read their lever from the line.
- Curved edges and the joint turn keep the span's middle until (C) is settled; a row tracks it.
- The verdicts that move are the 50 degenerate fixtures listed above, plus 4 reads that now refuse. Before landing, the full suite is re-run with the line site to show which tests change outcome.

**Not decided yet, and not asked:**
- curved edges' line, and the joint turn's site and region (they need truths that do not exist yet);
- any class of valid bodies the typed "not built yet" case newly refuses (measurement N5 below).

**What an agent lands without you** (all already agreed):
- The lever stays two-sided down to each decision, in one `geom-core` home.
- A disagreement leaves through each verdict's existing in-band arm.
- The typed "not built yet" case.
- Ordered bounds.
- Lazily read, kept covers.
- No boundary-edge reads in the bound from below, and no coarser gate.

### Still to measure before you decide

- **N1. The line site on the whole suite, straight edges only.**
  - Instrument: test outcomes that change.
  - Truth: M-e's exact truth for each verdict. The test's own stated intent is reported beside it and judged against nothing.
- **N2. Extend the exact truth to plane faces bounded by arcs as well as lines** (1.12M reads were skipped). An arc's largest distance from a line is closed-form. Judged as in M-e.
- **N3. Curved carriers.**
  - Population: a plane face meeting a cylinder along a circle, including the open row's line-into-arc table.
  - Truth: the largest distance of the plane face's points from the cylinder, and the reverse. This is closed-form and assumes no site.
  - Compared: tangent line against carrier as the site. This is what (C) waits on.
- **N4. The joint turn.**
  - Population: hand-built closed-form fixtures (a cone apex, a sphere pole).
  - Compared: the sites against the exact turn.
  - Where no truth exists, the sites' verdicts are reported side by side as "no truth".
- **N5. The typed "not built yet" case on the 82,619 affected pairs.**
  - Instrument: how many then refuse (a reading at the bound from below that is not definite).
  - Truth: none mid-construction. As a consistency check, the finished body's verdict on the same edge.
  - Any class of bodies that builds today and refuses then comes to you.
- **N6. Whether the lever moved the margin at all**, on the pairs that agreed.
  - Instrument: the share where the folded arm is the reach rather than a curvature arm.
  - Truth: none; this tells how much "no straddle" says.
- Already running: M-c (29,468 boundary-edge points past the bound from above) and M-f (door against tier 3).

Confidence:
- (A)+(B) as one decision: likely.
- The over-read figures and the 50 wrong verdicts: sure (measured against an exact truth).
- Laziness as a final-state property: likely.
- Not building edge reads or a coarser gate: likely.

## For the orchestrator

**What moved me.** M-e. My round-3 position, "the site goes to Ev after its measurement, the sentence first", is answered against me in one part. The sentence is not true of the tree without the site, so they cannot be split. M-e also answers the measurement it was waiting for, but only on straight edges; the rest goes into the `## Open` line (C). This keeps away from the rewrite-after-measurement pattern: nothing curved is asked.

**Not built.**
- M-d and M-b(iii) retire boundary edges in the bound from below and the coarser gate. Neither goes to Ev as a question; they appear in For Ev as information only.
- The two-sided funnel stays on soundness grounds even with zero observed straddles: a Zero proved from below is unsound in principle. N6 says whether the corpus ever tested it.

**Attribution I could not check.**
- The 6 reads in `disagreement.md` with both faces covered and the bound from above still poison.
- The `classify_shared_rim` 135-against-45 decision mismatch (a cache filled by the first run, unconfirmed).
- `disagreement.md` names step-import's snapped cylinders and `blend::reach` among the poisoned-net sources, but `m-a.md` attributes every refusal to placeholders, unbounded faces and one probe. One of the two is wrong for those rows. Chase them before N5 is read.
- M-c's 29,468 reads are either a cover missing its own edge (a defect for the slate that owns `topo::cover`; the session box reads face covers too) or an edge off its face. Whichever it is, file it at the source.

**For whoever builds (A)/(B).**
- Each of `wedge_reach` and `stored_reach`/`edge_reach` takes a site value: a line or a point.
- `Cover::distance_range(Site::Line)` already exists; M-e used it.
- The degenerate-face fixtures in M-e (`tier3_tests::sliver_dihedral_at_rest_is_rejected` and others) may stop being refused by this check. N1 shows whether another check still refuses them, and that goes in the PR body.

**Grep precondition.** Satisfied: no caller acts on a certification refusal's decided sign.

**Not filed (I am read-only).** The middle-of-span site on curved edges and the joint turn becomes the row behind (C).
