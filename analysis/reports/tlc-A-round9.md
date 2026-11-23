## For Ev

(A), as I sign it, after "no longer than the site's own measure of it:"

> a lever read off a consumed extent is known from both sides, each measured from the site its datum is read at: from below off curves and points known to lie on the region, and from above off its cover. A verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above. Where the two disagree the reading is in band and is refused as one.

(B) and (C) are unchanged.

## For the orchestrator

1. **The bound from below: I take the other designer's wording, "curves and points known to lie on the region".** My reason for keeping "points" was that what the bound reads at a line is code, not text. That reason is answered. The new wording says what the bound from below is made of, which is what Ev asked for on PR 7410, and it does not point at a mistake. It is still true at every site, including the middle one.
2. **The not-built face: I agree to remove the sentence.** "From above off its cover" already implies it: a face with no surface or loop yet has no cover, so nothing bounds it from above, so no Zero is served. Keeping a sentence for a case that only arises mid-construction is the kind of extra wording Ev's PR 7410 answer told us to cut. The typed not-built case stays in the code (step 1), where that rule actually lives.
3. **The arc cover's distance from a line: it is part of step 2's code, but not part of Ev's question.**
   - It belongs in step 2 because the line site's bound from above is that read. Loose arc covers (14,129 reads more than 10× past the truth) turn into Zeros refused on valid bodies. So the line site should not land without it, or the second run's completeness losses on arc-bounded faces will be attributed to the site when the cover's looseness is the cause.
   - It is not Ev's because it is a tighter closed form in the cover home with no design content and no text, an implementer's fix.
   - It is different from the 80 reads where the bound from above fell below the truth. If those are covers that do not hold their arc, that is a soundness defect in `topo::cover`, and it gates landing.
   - So step 2 is: the line site, plus the bound from below reading the boundary's curves, plus the arcs' closed-form distance from a line in the cover home.

No remaining split with the other report. The order of work, (B) and (C), and the second run before the `[ev]` PR stand as both of us described them in round 8.

**Checks I have not done:** whether the arc closed form holds for an arc that is not in its face's plane, and whether a curved face's farthest point from its edge's line can lie inside the face rather than on its boundary. If the second, a curved face along a line would need more than boundary curves in its bound from below. The ruling truth for a cylinder or cone wall that the second run adds would show this.
