## For Ev

**One decision is asked: two passages of `crates/geom-core/README.md` ("Float rounding") and one line under `## Open`.** The coordinator's reading of what we agree on is right. Below are the six differences settled, then the text I sign. I ran nothing.

Terms:
- *Bound from below / from above*: a length the region certainly reaches / certainly does not pass.
- *Site*: what a lever's distance is measured from.
- *Dihedral*: the angle between an edge's two faces.

**How this relates to your exchange on PR 6968.** You asked what the midpoint was for, since "it doesn't look like a witness". The answer you accepted was that a pivot is the origin of a tilt bound, not a witness.
- That answer holds for what it was about. A gap plus a tilt times a distance bounds movement from above, which proves a coincidence and needs no witness.
- It does not cover a definite sign, which does need a witness: a point that really moves that far. Measured from the midpoint, the points' reach exceeds the exact truth on 99.5% of the reads that have one.
- So the clause is narrowed to what that explanation supported. Your first question was the right one for definite signs.
- You also said to leave the text unless it would be ambiguous to a later reader. It was: this track read "any point is sound" as covering definite signs.

### The six differences

1. **Lever bullet wording.** "Measured from the site its datum is read at" (mine): not every datum turns about something. "Not built yet" (the other report's): the unbuilt faces include planes, cylinders and cones that have a surface and no bounding loop, so "no surface yet" was wrong.
2. **"Any point is sound."** It is true for a Zero and false for a definite sign. The page should not keep it unqualified next to a rule that contradicts it. I keep the inequality, shortened, and take the other report's one-sentence rule as the opening.
3. **Scope: every straight edge.** I withdraw my narrower list.
   - Two planes: the proof is exact whatever the faces' boundaries. The measurement's truth was built for straight-bounded faces only (1.12 M arc-bounded reads skipped).
   - A curved face along a straight edge (a cylinder wall along a ruling): the proof holds to first order. That is the order of the dihedral margin itself, and the fold's curvature arms carry the rest. It is not measured.
   - That case is the extrude half of the open row `the-wedge-reach-counts-a-faces-run-along-its-edge`. It is checked before this reaches you (G1 below).
4. **An `## Open` line on the page: yes.** Curved edges and the joint turn keep the span's middle. The page should say that is undecided, so a later reader takes it as neither ratified nor a violation.
5. **When: the text merges on your answer, before the code.** I give up "text merges with its code". The channel's rule is that the PR edits the doc and merges with the answer, and PR 7194 did so. The sequencing fact you need: until the line-site unit lands, the tree still reads straight edges from the middle, and that unit's row tracks the gap.
6. **What gates what.**
   - **Gates sending this to you (G1):** the line site run on the whole suite for straight edges, plus the open row's extrude table.
     - Reported: every test whose outcome changes.
     - Judged against the exact truth where it applies, and against the profile's declared tangency on the table, which is true by construction.
     - Elsewhere there is no truth, and the moved classes are listed as such.
     - Why it gates: the text binds every straight edge, and only 50 degenerate fixtures and 4 refusals are known to move. This is one run, and it saves you a second visit.
   - **Gates landing or later items only:**
     - the 29,468 reads with an edge point past the bound from above (a defect to file at its source, and the loud refusal waits on it);
     - door against tier 3;
     - how many Zero-acting reads on unbuilt faces newly refuse, and who they are (first unit; any valid class comes to you);
     - whether the reach was ever the least of the fold (it tells how much "no straddle" says);
     - the exact truth extended to arc-bounded plane faces;
     - truths for curved edges and for the joint turn (the `## Open` line).

### The passages I sign

**(A) Lever bullet**, after "no longer than the site's own measure of it:":

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off points of the region and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above, whatever acts on the verdict. Where the two disagree the reading is in band and is refused as one. Where a face is not built yet nothing proves a Zero over it. No consumer takes one bound.

**(B) Consumed-extent bullet**, replacing from "Its lever is the extent's far distance from that point" through "holds its region, not a lever":

> Its lever is the extent's far distance from the site its datum is read at: the line, for a datum that turns about one (a dihedral about a straight edge), else the point the datum is read at; the least over all sites is not owed. A tilt moves a point of the extent by the tilt times its distance from the line it turns about, never more than its distance from a point of that line. So a point is sound for a Zero, provided the datum and its lever are read at the same one, and a definite sign is levered from the line. A record that verdicts read from different sites holds its region, not a lever.

**(C) Under `## Open`:**

> which site a dihedral along a curved edge is levered from (its station's tangent line or the edge's carrier), and the site and region of the joint turn at a pole; both are read from the span's middle.

- **Asked now:** approve or strike (A), (B) and (C). Striking (B) leaves (A) true only of distance from the middle, a length no verdict is owed.
- **Not yet decided:** curved edges, the joint turn, and any valid class the unbuilt-face case newly refuses.
- **Order in the tree:**
  1. the text, on your answer;
  2. the two-sided funnel, the typed unbuilt face and lazily read covers, which move no verdict at finite bounds;
  3. the line site for straight edges, which moves the classes G1 lists;
  4. curved edges and the turn, after their truths.

Confidence: (A) likely; (B) sure on the inequality, likely on the scope beyond planes until G1; the order likely.

## For the orchestrator

- **Signed from the other report:** "not built yet"; the one-sentence site rule as (B)'s opening; the `## Open` line; text before code. **Held:** "read at" over "turns about" in (A); the narrowing of "any point is sound" inside (B); G1 before the PR goes up, not "before landing".
- **Why G1 gates the ask and not the landing.** (B) binds every straight edge on a first-order argument for curved faces. If the extrude table did not read the profile's 0.05, the scope would shrink and the text would be rewritten after Ev had seen it. The suite run with it also puts the moved classes in the decision document, which PR 7140's condition asks for anyway.
- **If the other designer will not sign G1 as a gate,** the split is one line each. Send now, with moved classes reported before code lands: two visits, and scope at risk. Or send after G1: one visit, one run's delay. I would not hold the fork open over it.
- **(B) leaves a known gap on the page.** "Else the point the datum is read at" is not what curved-edge wedges do: the datum is at stations and the lever at the middle. (C) names it. That gap is on main today under the clause being replaced, so it is not new.
- **The first unit's PR body** should say the suites hold no finite straddle, so its rows are closed-form fixtures. The reads with bounds more than 1e3 apart are where to build them.
- **The degenerate-face fixtures** found at the line site become collapsed-arm refusals. G1 should report which check refuses each afterwards.
- **Unchecked by me:**
  - `Cover::distance_range` from a line on pieces other than planes;
  - whether any other lever in the tree proves a definite sign from a point for a datum that turns about a line. (B) would bind it; I surveyed none.
- **Still to chase before the unbuilt-face count is read:** the two lane notes disagree on step-import's snapped cylinders and the contact-edge corpus. The other report flags the same.