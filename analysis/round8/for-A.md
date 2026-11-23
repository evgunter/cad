# Round 8, the last: your final `For Ev` section

Both round-7 reports are in; the other designer's is below, whole. You are designer A; it is designer B's. Same rules.

Where round 7 leaves the three small things:

1. **Edit 2.** You agree on everything but one word: A "…the lesser of the two extents' far distances from that point, **each read from below and from above**." B "…far distances from that point, **read from below and from above alike**." If you find a test that separates them, give it. If you find none, sign A's ("each read from below and from above"): that tie-break is mine and arbitrary, and I will say so in the log.
2. **The tie's witness.** A: "…the edge's point at the lesser of the face's radii of curvature and the edge's arc length, each read off what the bound holds (its arc, or a line's two points); the parting is levered at that point's distance from the vertex." B: "…the edge's point at an arc length no longer than the face's radii of curvature and no farther along than the edge's far end, each read off the geometry; the parting it witnesses is levered at that point's distance from the vertex", with two readings of `bound_side` behind "far end" (an ellipse's arc length is never computed, `ConicArc::at` stops at the far end; a line's witness has no stop at `far` today, the reach in the fold supplies it). If you find a fault in B's sentence, give it. If none, sign B's.
3. **The fan's slack.** You crossed again, but on whether a phrase is in the text, not on the end state: both of you now say the `1.01` written at the site goes, and the unit re-poses the rim (the division by `cos(π/8)` as a product by a constant; the rim's coordinates, which can stand past the session box, either drawn in or shown inside what the admission rule allows). So "with no hand-written slack" is true of the end state both describe, and it stays in the fan's line. Say so only if that is not your position.

Then give your **complete, final `## For Ev` section**: it is forwarded to Ev verbatim beside the other designer's, so it must stand alone (terms, premise, worked example, the final state, the records, what flows where, the three README edits as exact text and place, what is not Ev's, PR 7619, what is unmeasured). Ev reads it on a phone: lead with the decision, keep it under 90 lines, and use the same words as the other designer wherever you agree, so that Ev can see at a glance that the two say one thing. After it, `## For the orchestrator` with only what is new.

---

# Designer B's round-7 report

# Round 7, designer B

**1. Edit 2: one sentence that passes both tests.** Add the word "read":

> A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' far distances from that point, read from below and from above alike. A direction drawn between them has no extent of its own and takes that lever.

- "Far distances" stays, so no word is defined by itself and a far distance is still one number.
- "Read from below and from above" attaches to the read of it, in the lever bullet's own verb, and matches edit 3 ("its lever, read from below and from above").
- So A's word, with the read named. My test is met by the added word, not by "levers".

**2. The tie's witness: one sentence.**

"A tie's witness is the edge's point at an arc length no longer than the face's radii of curvature and no farther along than the edge's far end, each read off the geometry; the parting it witnesses is levered at that point's distance from the vertex."

- "Extent" is gone. A is right that in this README it names a region.
- "The edge's arc length" is replaced by "no farther along than the edge's far end", for two reasons from `bound_side`:
  - `ConicArc::at` advances by at most the given length and stops at the far end. An ellipse's own arc length is never computed.
  - `BoundSupport::witness` for a line is `base + (far − base) · (l / |far − base|)` with no stop at `far`. Today the stop comes from the reach in the fold. Once the reach leaves the fold, the far end has to be in the sentence, and the unit adds the stop for a line.

**3. The fan's slack: A's side. "With no hand-written slack" stays in the text for Ev.**
- Test: is a site's own constant ever this README's answer to rounding? No. The admission rule is "the one statement about rounding, and no site charges it", and "no `Margin` door takes hand-written slack". Whether or not the rim's rounding falls inside that rule, a `1.01` written at the site is not how it is covered. So the end state is the same either way, and I was wrong to make the phrase wait on how.
- What the `1.01` covers, read at `link::fan`: `radius = reach · 1.01 / cos(π/8)`, then `p + t · radius` over eight directions built from one cross product, sums and a constant `1/√2`. That is a short chain. Two things in it are outside the admission rule's letter ("a short, division-free chain of operations on in-box coordinates"):
  - the division by `cos(π/8)`. It is by a constant that cannot vanish, and the unit re-poses it as a multiplication by a constant;
  - the rim is not in the box. It stands about 1.08 times the face's far distance from `p`, so a face near the box's edge puts rim points past it, at coordinates a few times the box.
- Neither argues for a site constant. The second is for the unit to state when it re-poses the rim: either the rim is drawn in, or its coordinates are shown to stay within what the admission rule's count allows. I did not read the value of `SHORT_CHAIN_ULPS`, so I do not claim which.

**For Ev: as round 6, with these three sentences**, and with "with no hand-written slack" restored to the fan's line in item 5, as A has it.

**For the orchestrator, new only.**
- The line witness's missing stop at `far` is a small defect the unit meets when the reach leaves the fold. Today it is masked, not absent.
- The rim standing outside the session box is a note for the unit's re-posing, not for Ev.
- Nothing was built or run.


---

# For reference, designer B's round-6 `For Ev` section is what B's round 7 amends; yours is your own round 6.
