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
