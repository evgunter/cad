# Round 10: one answer for Ev

Both of you answered Ev's question alone, and both said the same first thing: yes, edit 2 as sent is false for a valid consumer, because it fixes a corner's lever by what the datum is where the bullet fixes an extent by what the verdict is used over; the lever must follow the use. The other designer's round-9 answer is below, whole. You are designer B; it is designer A's. Same rules; do not take the other's side because it is the other's: give the test that decides each point.

Where you differ, to settle now so that Ev gets ONE answer and one text:

1. **The definite sign.** A: it too can be wrong, for a reader that uses it over LESS than the shorter edge (a touch reading inside a ball of 0.001 at a corner whose shorter edge reaches 0.01: definite at 0.01, in band everywhere inside the ball). B: the definite sign is safe for every consumer whose region reaches at least that far. These do not contradict; say together what is true of a definite sign and of a Zero, in two sentences Ev can check.
2. **What holds the invariant.** A: no new type; every claim larger than the corner already comes from a door that takes the larger region (the glue door, a face's cover), and no door turns a corner's verdict into one; state that as the design. B: two type statements: the corner's arm is a named type built only from the corner's two departures, and whatever acts on a whole face takes a whole-face verdict in its SIGNATURE, minted only by a door that takes the face; then a neighbourhood Zero is a different type and cannot be handed in. Ev favours structural enforcement over discipline. Is A's "no door turns a corner's verdict into one" a fact the types hold today, or a fact about today's functions? B found a third whole-face consumer neither had cited (the split's rule (a), metered at the face's extent) and a doc that over-claims (`PairRecord::coplanar`: "the pair lies in one plane" of four neighbourhood codes); B did NOT check that every face-wide action in the split and boolean lanes is behind a whole-face verdict in its signature. Read enough to say which final state is right and what the unit must check.
3. **Edit 2's text.** Two candidates:
   - A: "A verdict on the turn between two directions out of one point, each with its own extent, is used where both run: that is its extent, and its lever is the lesser of the two extents' far distances from that point, each read from below and from above. A verdict used over more or less reads the extent it is used over. A direction drawn between them has no extent of its own and takes that lever."
   - B: "A verdict on the turn between two directions out of one site, each with its own extent, that is used over the part where both run has that part as its extent: its lever is the lesser of the two far distances from the site, each read from below and from above. A direction drawn between them has no extent of its own, and a verdict on it has the extent it is used over: that same part, or the face the direction lies in."
   A's first sentence still ASSERTS the use ("is used where both run"), which is the flaw Ev caught, restated, and then corrects it in a second sentence; B's conditions on the use throughout. A's is shorter and its second sentence covers "less" as well as "more". Give one text. "Point" or "site": the README on main says "point" in this bullet today; PR 7619's approved text, not yet merged, says "site"; use the word of the bullet as it stands on main and say that it follows the page.
4. **Edit 3.** B adds "over one extent": "so a record whose verdicts all read from one site over one extent may hold its lever, read from below and from above, and a record that verdicts read from different sites, or uses over different extents, holds its region, not a lever." A left edit 3 unchanged. Does the same flaw sit in edit 3 as sent (a record whose readers share a point but use its lever over different regions)? One text.
5. **The opt-outs.** B: PR 7619's approved text says a verdict is served whatever acts on it and no consumer takes one bound, so the `.below()` / `.above()` opt-outs and their swap rows in the both-bounds door contradict ratified-to-be text and come out; every sector predicate reads both. A had the opt-outs as visible exceptions with swap rows. Decide: is there any sector predicate for which reading both is wrong (not merely looser), given that a definite sign at the bound from below and a Zero at the bound from above are what the both-bounds read serves anyway?

Send, as your final message, at most 80 lines: a complete `## For Ev` ADDENDUM that stands alone as the answer to Ev's comment (the direct answer first, in plain words with one worked case; what holds the invariant; the three edits as exact text and place, marking what changed since the PR was opened; what else changes in the final state), using the same words as the other designer wherever you agree; then `## For the orchestrator` with only what is new.

---

# Designer A's round-9 answer

## For Ev

**Yes, your worry is right.** Edit 2 as sent fixes the lever by what the datum is (two directions out of one point). The README fixes an extent by what a verdict is used over. Those agree only for a reader that uses the corner's verdict over the corner's own neighbourhood, out to the shorter edge. Edit 2 gives the wrong lever in two cases:

- **A Zero used over more than that neighbourhood is levered too short.**
  - Example: a corner whose edges reach 1 and 10, or a divider on a face that runs 100 along it, and a datum tilted by δ.
  - At the lesser reach, δ·1 < ε reads Zero. But the long edge's far end stands 10δ off, and the face's far part 100δ, which can exceed ε.
  - A reader that took that Zero as "both edges lie in the plane" or "the face's sector lies in the plane" would be wrong. Its Zero is served at a bound from above that is not above what it is used over.
- **A definite sign used over less is levered too long.**
  - Example: a touch reading inside a ball of radius 0.001, at a corner whose shorter edge reaches 0.01.
  - A sign that is definite at 0.01 can be within the band everywhere inside that ball.

**Today no reader does either.** Two-face coincidence reaches the lump only as `Coincident::Glued(CarrierRelation)`, which the glue door builds from the faces. A curved face's unglued Zero refuses (`Frontier`). The one reader that claims a whole face, the link's cleared fan, reads the face's own cover. So the coincidence case is already held by a type, not only by good behaviour. The text must say why, not lean on it.

**The fix states what is already true: the lever is the use's.**
- The consumed-extent bullet already says an extent is "built only from the geometry consumed … never from a length a caller vouches for". A verdict used over a whole face consumes the face, so it reads the face's cover. A verdict used inside a smaller ball consumes that ball.
- What edit 2 should say is what a corner's verdict is used over, the neighbourhood where both directions run, and that its lever is that neighbourhood's.
- The structure that holds this already exists: every claim larger than the corner comes from a door that takes the larger region (the glue door, a face's cover). No door turns a corner's verdict into one. I would state that as the design rather than add a new type. A corner's Zero carries the arm it was read at (`Decided`), and a whole-face door does not take a corner's verdict as its input.

**Edit 2, to put to you in place of the one sent** (same place: the consumed-extent bullet, straight after "…the least over all points is not owed."):

> A verdict on the turn between two directions out of one point, each with its own extent, is used where both run: that is its extent, and its lever is the lesser of the two extents' far distances from that point, each read from below and from above. A verdict used over more or less reads the extent it is used over. A direction drawn between them has no extent of its own and takes that lever.

- Every term is the bullet's own: verdict, extent, used over, far distance, from below and from above.
- The second sentence is a consequence of the bullet's first, stated where a reader of edit 2 would otherwise over-read it.
- Edits 1 and 3 are unchanged.

**The rest of the final state.**
- **Unchanged:** the divider, the both-bounds door, the link's fan, which reads the face itself.
- **Item 2's divider line reworded:** "A divider takes its corner's arm: its verdicts are used over the corner's neighbourhood. A claim that two faces coincide is the glue door's, which reads the faces, and no door makes one from a corner's verdict."

## For the orchestrator

- **Read for this round:**
  - `fan::meeting` (Parallel / Glued / Frontier) and `recl`'s match on `Coincident` (`Frontier` refuses `CurvedBooleanUnsupported`);
  - `splitting::rules`' divider branch (an `On` divider takes its neighbours' side, a local claim);
  - `FaceSector::cleared` (reads `face_reach_from`);
  - the README's consumed-extent bullet ("built only from the geometry consumed").
- **New:** the definite-sign case, an over-long lever for a reader inside a ball smaller than the shorter edge. Neither designer raised it before.
  - The link's touch readings use their own `clear` and reach, so they are not corner-arm consumers.
  - I did not check every `BoolSector.arm` reader for one used inside a smaller ball. The unit should list them.
- **Not proposed:** a typed "neighbourhood verdict" wrapper. The existing doors (`CarrierRelation`, covers) already make a whole-face claim need the face. If Ev wants it in the type anyway, the shape is a corner verdict that carries its `CornerArm` and is accepted by no face-level door.
- **Nothing was built or run.**
