## For Ev (addendum: the answer to your comment on edit 2)

**Yes: as sent, edit 2 is false for a valid consumer, on a Zero.** It fixed a corner's lever by what the datum is: two directions out of a point. The bullet it sits in fixes an extent by what the verdict is used over. The lever has to follow the use, and the new text below does.

**What each sign serves.**
- **A definite sign** is served at the lesser far distance, read from below. Both directions have points of the body that far out, and they stand apart by more than the band. So it is right for every consumer whose region reaches at least that far.
- **A Zero** is served at the lesser far distance, read from above. It proves "within the band" only out to the shorter direction's end. So it is right only for a consumer that uses it over no more than that part.
- **The same rule, either way:** a consumer whose region is larger or smaller than that part is a verdict over a different extent, and reads that extent's lever.

**Worked case.**
- Take a flat face with a vertex in the middle of a straight side. The two edges at the vertex are each 1 long, and the face runs 100 deep.
- A plane contains the side's line and is tilted off the face by 0.5ε per unit length.
- The divider (square to the side, into the face) reads 0.5ε at lever 1: a Zero.
- At depth 100 the face stands 50ε off the plane: definitely apart.
- A consumer that takes the Zero as "nothing parts from the plane near this vertex" is right.
- A consumer that takes it as "this face lies in the plane" (keep it as a wall, merge it, drop one of two faces) is wrong.

**Such consumers are valid, and the tree has three.** Each levers at the face, not at the corner:
- the split's coplanar rule (`splitting::rules`, rule (a), metered at the face's extent from the vertex);
- the boolean's glue door, whose extent is a ball enclosing both faces;
- the link's fan, sized by the face's far distance.

Today the invariant holds, but in part only because each function does the right thing. It is not yet held by the types.

**What holds it in the final state: the types, in two places.**
1. **The corner's arm is a named type, built only from the corner's two departures.** It is the lever of the part within the lesser of the two edges' far distances from the vertex. Sector predicates take it, not a bare `LeverBounds`, so a Zero decided at it is a neighbourhood Zero and nothing more.
2. **Whatever acts on a whole face takes a whole-face verdict in its signature.** That verdict is minted only by a door that takes the face and levers at the face's far distance.
   - The boolean's lump already works this way: it accepts only the glue door's one-carrier verdict.
   - The split's rule (a) is the same shape for a tangent plane.

With both, a neighbourhood Zero is a different type and cannot be handed to a face-wide action.

**The three edits.** All go in `crates/geom-core/README.md`, "Float rounding". Edit 1 is unchanged since the PR was opened; edits 2 and 3 are changed.

1. *(unchanged)* In **A verdict's lever arm**, after "…or the session box where no geometry is in hand.":
   > The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever.
2. *(changed)* In **The consumed extent**, after "…the least over all points is not owed.":
   > A verdict on the turn between two directions out of one point, each with its own extent, that is used over the part where both run has that part as its extent: its lever is the lesser of the two far distances from the point, each read from below and from above. A direction drawn between them has no extent of its own, and a verdict on it has the extent it is used over: that same part, or the face the direction lies in.
3. *(changed)* In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point over one extent may hold its lever, read from below and from above, and a record that verdicts read from different points, or use over different extents, holds its region, not a lever.

**Why each changes.**
- **Edit 2:** the old text asserted the use; the new one makes the lever conditional on it, so it is true of every consumer by construction.
- **Edit 3:** the same flaw sat there too. Two readers that share a point but use its lever over different regions need different levers, so a record may hold one only if they share the extent as well.
- **"Point":** the word this bullet uses on main today. If PR 7619's "site" merges first, both edits follow the page.

**What else changes in the final state.**
- **The divider** still carries its corner's arm, now as that named type. A reader that needs a divider's side over the whole face reads the face's far distance through a whole-face door.
- **Item 2's last line** ("no reader takes it as two faces coinciding without the glue door's verdict") is replaced by the two type statements above.
- **The both-bounds door loses its opt-outs.**
  - Your approved lever text says no consumer takes one bound, and a read at both bounds already serves a definite sign at the bound from below and a Zero at the bound from above.
  - No sector predicate is wrong under it, only looser where a Zero falls through to a sound path. At such a site a straddle takes the same path as the Zero, since neither is definite.
  - So `.below()` / `.above()` and their swap rows come out, and every sector predicate reads both.
- **The link's fan** is unchanged. It is the pattern, not an exception.
- **Cost, unmeasured:** none from this change on today's bodies, since no reader moves to a longer lever.

## For the orchestrator

- **Settled, with the test for each:**
  - *Point 1:* both statements are true, because the lever follows the use. Neither designer's sentence was wrong.
  - *Point 2:* B's types. Test: does a type hold it today? Only for the lump (`Coincident::Glued(CarrierRelation)`).
  - *Point 3:* B's text with "point". "Less" is already the bullet's first sentence ("the region a verdict is used over"), so it needs no second sentence.
  - *Point 4:* B's edit 3, with "uses" corrected to "use" so the verb agrees with "verdicts".
  - *Point 5:* the opt-outs come out. No predicate is wrong under the both-bounds read; a straddle at a fall-through site takes the Zero's sound path.
- **New, for the unit: one `ParallelFace` is minted at a neighbourhood lever.**
  - `splitting::rules::apex_reading` mints `ParallelFace` levered at `bound.reach.lever()`, a bound's reach, not the face's extent. The tangent-plane path, in contrast, uses `face_extent`.
  - So the whole-face verdict type is not yet minted only from the face.
  - The unit checks whether the apex graze acts face-wide, along the ruling past the corner's arm. If it does, it levers at the face's extent along the ruling, and `ParallelFace` gets a door that takes the face.
  - The unit also lists every `ParallelFace`, `CarrierRelation` and `SideCode::On` / `PlaneSide::On` reader that keeps, merges or lumps a face.
- **Defect to file:** `PairRecord::coplanar()`'s doc ("the pair lies in one plane") over-claims four neighbourhood codes.
- **Read this round:** the approved PR 7619 text (`git show baf9df0720`: "No consumer takes one bound"), and `splitting::rules`' `face_extent` use and `apex_reading`.
- **Nothing was built or run.**
