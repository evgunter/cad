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
