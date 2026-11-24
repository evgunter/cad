**Updated 2026-11-24 after your question on the second sentence. You were right: as first sent it was false for a consumer that could legitimately be written. Sentences 2 and 3 are changed in this PR's diff; sentence 1 is as it was. The two designers agree on the design and on sentences 1 and 3 word for word, and differ on one clause of sentence 2, marked below for you to keep or strike.**

**One decision: three sentences added to "Float rounding" (`crates/geom-core/README.md`).** Each says something the tree already does in places and no ratified text says. Recommended by both designers: all three.

**1. A lever's length is not geometry** (unchanged). In the lever bullet, after its list of sources:

> The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever.

*Why it is asked.* A bound of a face's corner at a vertex reaches the corner predicates today as one vector: a direction scaled by a lever's length. For a curved edge that is the unit tangent times the edge's extent lever, which ends on no point of the body, and the code reads that one vector both as a lever and as geometry. *Rejected:* leaving it to `Lever::length`'s doc and a count of uses.

**2. A corner's lever follows what its verdict is used over** (changed). In the consumed-extent bullet, after the sentence that defines a lever from a point:

> A verdict on the turn between two directions out of one point, each with its own extent, when used over the part where both run, is levered at the lesser of the two extents' far distances from that point, each read from below and from above. *A verdict used over more or less reads the extent it is used over.* A direction drawn between them has no extent of its own, and a verdict on it reads the extent it is used over: that same part, or the face the direction lies in.

*What was wrong with the first version.* It said a turn "is consumed where both run", fixing the lever by what the datum is. The bullet it sits in fixes an extent by what a verdict is used over. They agree only for a reader that uses the corner's verdict over the corner's own neighbourhood. The case: a flat face with a vertex mid-side, the two edges there each 1 long, the face 100 deep, a plane through the side's line tilted 0.5ε per unit. The divider (the direction a wide corner is cut at) reads 0.5ε at lever 1, a Zero; at depth 100 the face stands 50ε off the plane. "Nothing parts from the plane near this vertex" is right; "this face lies in the plane" is wrong, and wanting the second is legitimate. The other way round: a definite sign at the shorter edge's reach (say 0.01) says nothing inside a ball of 0.001.

*The clause in italics is the one difference between the designers.* One keeps it: without it no clause covers a verdict used over less, or an edge-against-edge Zero used over a whole edge. The other strikes it: it restates the bullet's opening ("the region a verdict is used over"), and the conditional first sentence already withholds the lesser distance from any other use. Both hold the text true either way. It is in the diff; say "strike the middle sentence" and it goes.

**3. A record holds one lever only where its verdicts share a point and an extent** (changed). The same bullet's closing clause becomes:

> so a record whose verdicts all read from one point over one extent may hold that extent's lever, read from below and from above, and a record whose verdicts read from different points, or are used over different extents, holds its region, not a lever.

*Why it changed.* The same flaw sat in it: readers that share a point but use the lever over different regions need different levers. It is still the one clarification of the records rule you approved on PR 7141; its two questions stand.

**What now holds the invariant your question was about: types, not the readers' good behaviour.** Both designers checked whether a function can write a whole-face verdict without the face, and it can today, in both lanes: the boolean's is a plain public value its caller supplies, and the split mints its own at a single bound's lever in one branch (at a cone's apex) and at the face's extent in the other. Three readers act on a whole face and each reads the face (the boolean's glue door, the split's coplanar rule, the link's fan), so nothing is known wrong; but nothing stops the next one. In the design as it now stands:
- the corner's arm is a named type, built only from the corner's two bounds; a Zero decided at it is a neighbourhood Zero and nothing more;
- whatever acts on a whole face (keeping one of two coincident faces, treating a face as lying in a plane) takes a whole-face verdict in its signature, a type with private fields and one mint, a door that takes the face and levers at its far distance.

A neighbourhood Zero is then a different type and cannot be handed in.

**What else changed with the answer.** The both-bounds door has no single-bound opt-outs: the lever text you approved on PR 7619 says no consumer takes one bound, and reading both already serves a definite sign from below and a Zero from above. The rest of the design is as before (the departure record; the link's fan, which reads the face itself and is the pattern).

**What flows where** (your condition on PR 7141). The star comes to read each edge's cover from the vertex, which the cut already reads there; `geom_brep`'s corner predicate sees less; the link's cone holds departures where it held fabricated points; the pierce read takes two faces' covers, faces it already reads. Neither designer finds a module that comes to read a region it had no business with.

**Not measured: nothing was built or run.** No count could change a sentence. Known direction: a curved bound's lever from below shortens, so definite signs on ellipse, spiric and spline bounds move toward the band; lines do not move; circular arcs past a half turn lengthen. The answer to your question moves no reader to a longer lever today, with one site to check (the split's graze at a cone's apex). The unit that builds this counts what moves, and any class of valid bodies that newly refuses comes back to you on its PR before it lands (PR 7140).

**Independent of PR 7619**: every lever here is read from a point, not about a line.

**Sequencing, which is mine.** After your answer this PR stays open and the first unit builds on this branch, so that the text merges with code that makes it true. The rounds are in `work/oneround/log.md`, the entries of 2026-11-23 from 20:54Z and of 2026-11-24 from 00:27Z.

The designers' answers to your question follow verbatim, then their texts as first sent.

---

## Designer A, on your question (2026-11-24)

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

*(Afterwards A signed the shorter second sentence, without "A verdict used over more or less reads the extent it is used over", and B's wording of the third.)*

---

## Designer B, on your question (2026-11-24)

**Yes, your worry is right.** Edit 2 as sent fixes the lever by what the datum is (two directions out of one point). The README fixes an extent by what a verdict is used over. Those agree only for a reader that uses the corner's verdict over the corner's own neighbourhood, out to the shorter edge.

**What is true of each sign.**
- A definite sign, served at the lesser far distance read from below, is right for every reader whose region reaches at least that far. A reader that uses it over less reads its own region.
- A Zero, served at the lesser far distance read from above, is right only for a reader whose region reaches no farther. A reader that uses it over more reads its own region.

**Worked case, the Zero.** A flat face with a vertex in the middle of a straight side. The two edges at the vertex are each 1 long; the face runs 100 deep. A plane contains the side's line and is tilted off the face by 0.5ε per unit length.
- The divider, square to the side and into the face, reads 0.5ε at lever 1: a Zero.
- At depth 100 the face stands 50ε off the plane.
- A reader that took the Zero as "this face lies in the plane" would be wrong. A reader that took it as "nothing parts from the plane near this vertex" is right.

**Worked case, the definite sign.** A touch reading inside a ball of radius 0.001, at a corner whose shorter edge reaches 0.01. A sign that is definite at 0.01 can be within the band everywhere inside that ball.

**What holds the invariant: today, the functions; in the final state, the types.**
- Today's readers are safe, and the three that claim a whole face each read the face: the boolean's glue door, the split's coplanar rule ("metered at the face's extent from the base vertex"), and the link's fan.
- But nothing in the types holds that. Two readings of the tree show it:
  - The boolean's whole-face verdict is `Coincident::Glued(CarrierRelation)`. `CarrierRelation` is a plain public enum, and `fan::meeting` takes it as an argument its caller supplies. Any function could write one from a corner's Zero.
  - The split's whole-face verdict is `ParallelFace`, documented as decided "over its face extent". `apex_reading` mints the same type at a bound's lever (`bound.reach.lever()`), not the face's.
- The final state has two type statements:
  1. The corner's arm is a named type, built only from the corner's two departures. Sector predicates take it. A Zero decided at it is a neighbourhood Zero.
  2. Whatever acts on a whole face (keeping one of two coincident faces, treating a face as lying in a plane) takes a whole-face verdict in its signature. That verdict has private fields and one mint, a door that takes the face and levers at the face's far distance. The tree has this pattern already: `geom_brep::ParallelNormal` has `normal_alignment` as "its only mint" and carries the arm it was decided at.
- Then a neighbourhood Zero is a different type from a whole-face verdict and cannot be handed in.

**The three edits to `crates/geom-core/README.md`, "Float rounding".**

1. **Unchanged.** In **A verdict's lever arm**, after "…or the session box where no geometry is in hand.":
   > The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever.

2. **Changed.** In **The consumed extent**, after "…the least over all points is not owed.":
   > A verdict on the turn between two directions out of one point, each with its own extent, when used over the part where both run, is levered at the lesser of the two extents' far distances from that point, each read from below and from above. A verdict used over more or less reads the extent it is used over. A direction drawn between them has no extent of its own, and a verdict on it reads the extent it is used over: that same part, or the face the direction lies in.

   - The first sentence no longer asserts the use; it is conditioned on it.
   - The second covers both worked cases.
   - The third no longer gives a divider the corner's lever unconditionally. The first worked case is a divider.
   - "Point" is the bullet's word on main today. If PR 7619's "site" merges first, the edit follows the page.

3. **Changed.** In the same bullet, the closing clause becomes:
   > so a record whose verdicts all read from one point over one extent may hold that extent's lever, read from below and from above, and a record whose verdicts read from different points, or are used over different extents, holds its region, not a lever.

   - The same flaw sat here. As sent, a record whose readers share a point could hold one lever for readers that use it over different regions.

**What else changes in the final state.**
- **The divider.** It carries its corner's arm, as the named type. Its verdicts are used over the corner's neighbourhood. A reader that needs a divider's side over the whole face reads the face through the whole-face door.
- **Item 2's divider line** now reads: "A divider takes its corner's arm: its verdicts are used over the corner's neighbourhood. A claim about a whole face is a different type, minted only by a door that takes the face."
- **The both-bounds door has no opt-outs.** The lever text you approved on PR 7619 says a verdict is served "whatever acts on the verdict" and "no consumer takes one bound". Reading both serves a definite sign at the bound from below and a Zero at the bound from above, which is what each sign needs. So no reader gets a more correct verdict by reading one bound. The `.below()` / `.above()` exits and their swap rows come out.
- **The link's fan** is unchanged. It is the pattern.
- **Unmeasured:** nothing was built. No reader moves to a longer lever, so I expect no verdict to move from this addendum; the one exception to check is the split's graze at a cone's apex, below.

---

# As first sent (2026-11-23), superseded where the above differs

## Designer A

**Recommendation (likely).** A sector bound is a unit direction plus what stands behind it, never a vector scaled by a lever. One departure record serves the star, the bound-side read, the cut and the link.

**Terms.**
- **Bound:** one side of a face's corner at a vertex. It is either an edge leaving the vertex or a *divider*, a direction a wide corner is cut at.
- **Lever:** the length a tilt is multiplied by to give a displacement.
- **Reach:** how far an edge gets from the vertex.
- **`LeverBounds`** (exists today): a lever known exactly, or between a bound from below and a bound from above.
- **Straddle:** the two bounds giving different signs. A straddle refuses.

**Premise (sure).**
- The cut already has this shape. `boolean::fate::corner_holds` takes a unit direction, a reach from the vertex known at both bounds, and a bend.
- The tree holds one concept in four forms, with different levers.
- `geom_brep::sector_shape` forces the packing: it asks for scaled chords and reads its arm from their lengths.

**Worked example (sure).** Take an ellipse with semi-axes 10 and 1, and an edge leaving the end of the minor axis and running nearly all the way round.
- Its diameter is 20. Its far distance from the vertex is about 10.05.
- Today's lever is a lower bound on 20, on the side that claims a definite sign. So a tilt between ε/20 and ε/10 is called definite where the edge reads within the band.

**The final state.**
1. **One departure record, in `topo`, built by one door from the body.** It holds:
   - a unit direction;
   - a reach, as `LeverBounds` read from the vertex:
     - a line's chord, exact;
     - a circular arc's, exact;
     - for an ellipse, spiric or spline, the edge's farthest sampled point from below and its cover's far distance from above;
   - what stands behind it:
     - a line's far vertex;
     - a curved edge's bend and, for a circle or an ellipse, its arc;
     - a divider's face section and its corner's arm.
2. **A corner's arm is the lesser of its two bounds' reaches, at each bound.**
   - It is exact where both reaches are, and derived where the two bounds are held.
   - `SectorShape.arm`, `FullTurn.arm`, `ApexShape.arm`, the stored `star::Sector.arm` and `metering_arm` go.
   - A divider is levered at its corner's arm. A sector's Zero claims only its neighbourhood, and no reader takes it as two faces coinciding without the glue door's verdict.
3. **Predicates take unit directions and an arm, and decide through one both-bounds door.**
   - The door reads once where the lever is exact, and at both bounds otherwise, deciding only where they agree.
   - A site reads one bound only by writing `.below()` or `.above()`, with a row that goes red when the bound is swapped.
   - At a slit's tip, only an agreed Zero reads as the whole turn.
4. **Points.** Every point placed is a vertex, a point of an edge, or a named construction sized by a length of geometry.
   - The conic bend witness sits at an arc length the bend record bounds off the curve.
   - A tie's witness is the edge's point at an arc length of the lesser of the face's radii of curvature, or at its far end where that comes first, each read off the geometry; the parting it witnesses is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** A ray of the gathered cone is a real point (a line's far vertex) or a departure read at both bounds.
   - Where the cone's reading needs a position in its own picture, it forms the ray's displacement, direction times reach, at each bound. That is read only for signs of distances within the picture, never as a place on the body.
   - The link's face-tie fan stays a named construction: a polygon past the face, sized by the face cover's far distance read as a length, with no hand-written slack.
6. **A lever's length is reachable only through named reads, by type.** This is code, not a decision for you.

**The other records (likely).**
- `PlannedRun.order` keeps a lever, as bounds. The pierce's ball is built from the two faces' covers.
- editor-core's carrier reach holds its region.
- `Leg::arm` and `SectionOval::extent` become methods of geometry their records hold.
- `Margin::over_lever`'s calls are each classed first, as a tilt's arm or a measure.

**What flows where.**
- `geom_brep::sector_shape` sees less.
- The star comes to read each edge's cover from the vertex, which the cut already reads there.
- The link's cone sees departures in place of fabricated points.
- The pierce read takes two faces' covers; it already reads those faces.
- No spline carrier travels with a bound. I see no odd flow.

**For you to ratify: three edits to `crates/geom-core/README.md`, "Float rounding".** I recommend all three. The tree does each in places, and no ratified text says them.

1. In the bullet **A verdict's lever arm**, straight after "…or the session box where no geometry is in hand.":
   > The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever.
2. In the bullet **The consumed extent**, straight after "…the least over all points is not owed.":
   > A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' far distances from that point, each read from below and from above. A direction drawn between them has no extent of its own and takes that lever.
3. In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point may hold its lever, read from below and from above, and a record that verdicts read from different points holds its region, not a lever.

The two questions of PR 7141 stand as you approved them, and edit 3 is their one clarification.

**Not yours.** How edit 1 is enforced is code: `Lever::length` is removed, and each use becomes a named read.

**Unmeasured. Nothing was built or run.**
- How many verdicts move, and on which bodies, is not counted. Lines keep their chord, and circular arcs past a half turn lengthen.
- The straddle rate on ellipse, spiric and spline bounds is not known. The upper bound is known to be loose near a spiric's pinch, so refusals there are expected.
- It is not known whether the cone read at both bounds agrees with today's points where the lever is exact.
- The named reads that replace `Lever::length` are listed from a sample of about 100 uses, not all of them.
- None of this bears on the three edits. Your standing conditions cover it: the unit's PR carries the counts, and a class of valid bodies that newly refuses comes back to you there before it lands.

**Independent of PR 7619.** Every lever here is read from a point. `sector_shape` takes its arm as an argument, which leaves the edge-wedge caller free to pass whatever that fork decides.

---

## Designer B

**Recommendation (likely).** A sector bound is a unit direction plus what stands behind it, never a vector scaled by a lever. One departure record serves the star, the bound-side read, the cut and the link. Three sentences in `crates/geom-core/README.md` are yours to ratify; they are at the end.

**Terms.**
- **Bound:** one side of a face's corner at a vertex. It is either an edge leaving the vertex or a *divider*, a direction a wide corner is cut at.
- **Lever:** the length a tilt is multiplied by to give a displacement.
- **Reach:** how far an edge gets from the vertex.
- **`LeverBounds`** (exists today): a lever known exactly, or between a bound from below and a bound from above.
- **Straddle:** the two bounds giving different signs. A straddle refuses.

**Premise (sure).**
- The cut already has this shape. `boolean::fate::corner_holds` takes a unit direction, a reach from the vertex known at both bounds, and a bend.
- The tree holds one concept in four forms, with different levers.
- `geom_brep::sector_shape` forces the packing: it asks for scaled chords and reads its arm from their lengths.

**Worked example (sure).** Take an ellipse with semi-axes 10 and 1, and an edge leaving the end of the minor axis and running nearly all the way round.
- Its diameter is 20. Its far distance from the vertex is about 10.05.
- Today's lever is a lower bound on 20, on the side that claims a definite sign. So a tilt between ε/20 and ε/10 is called definite where the edge reads within the band.

**The final state.**
1. **One departure type, in `topo`, built by one door.** It holds a unit direction, a reach and what stands behind it:
   - **Line:** its far vertex. The reach is the chord, exact.
   - **Curved edge:** its bend and, for a circle or an ellipse, its arc. The reach is read once from the edge's cover from the vertex, from below and from above, and is exact on a circular arc.
   - **Divider:** its face's section. Its reach is its corner's arm.
2. **A corner's arm is the lesser of its two bounds' reaches, each bound the lesser of the two's (`LeverBounds::min`).**
   - It is derived where the two bounds are held.
   - `SectorShape.arm`, `FullTurn.arm`, `ApexShape.arm`, the stored `star::Sector.arm` and `metering_arm` go.
   - `geom_brep::sector_shape` and `chart_normal_beside` take two unit directions and an arm.
   - A divider takes its corner's arm. A sector's Zero claims only its neighbourhood, and no reader takes it as two faces coinciding without the glue door's verdict.
3. **One both-bounds door decides every sector predicate.**
   - It reads once where the lever is exact, and at both bounds otherwise, deciding only where they agree.
   - A site reads one bound only by writing `.below()` or `.above()`, and each such site has a row that goes red when the bound is swapped.
   - At a slit's tip, where a Zero reads the whole turn, a straddle refuses.
4. **Points.** Every point placed is a vertex, a point of an edge, or a named construction sized by a length of geometry.
   - The conic-against-plane witness sits at an arc length the bend record bounds off the curve, and `conic_bend_side` loses `extent`.
   - A tie's witness is the edge's point at an arc length no longer than the face's radii of curvature and no farther along than the edge's far end, each read off the geometry; the parting it witnesses is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** The cone stays a picture, read only for signs of distances within it.
   - The boolean's gathered cone is fed departures. Each ray enters it as its displacement, direction times lever, at each bound, and every sign is compared in step. That displacement is never a place on the body.
   - The link's face-tie fan stays a named construction: a polygon past the face, sized by the face cover's far distance read as a length, with no hand-written slack.

**The records (likely).**
- `PlannedRun.order` keeps a lever, read from below and from above. The pierce's ball is built separately, from the two faces' covers.
- editor-core's carrier reach holds its region.
- `Leg::arm` and `SectionOval::extent` become methods.
- `Margin::over_lever`'s calls are each classed first, as a tilt's arm or a measure.

**What flows where.**
- `sector_shape` sees less.
- The star reads each edge's cover from the vertex, which the cut already reads there.
- The cone sees departures in place of fabricated points.
- The pierce read takes two faces' covers, faces it already reads.
- No module learns a region it had no business with (likely).

**Yours: three edits to `crates/geom-core/README.md`, "Float rounding".** I recommend all three. Each states what the tree already does in places.

1. In **A verdict's lever arm**, after "…or the session box where no geometry is in hand.":
   > The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever.
2. In **The consumed extent**, after "…the least over all points is not owed.":
   > A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' far distances from that point, each read from below and from above. A direction drawn between them has no extent of its own and takes that lever.
3. In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point may hold its lever, read from below and from above, and a record that verdicts read from different points holds its region, not a lever.

**Edit 3 is the whole clarification to PR 7141's records rule;** its two questions stand.

**Not yours.** How edit 1 is enforced is code: `Lever::length` is removed, and each use becomes a named read.

**PR 7619:** independent. These levers are read from a point, not about a line. `sector_shape` takes its arm as an argument, which leaves the edge-wedge caller free to pass whatever that fork decides.

**Unmeasured.**
- Nothing has been built or run.
- Which verdicts move, which curved bodies newly refuse, and the straddle rate are all unknown. Lines keep their chord. The reach from above is known to be loose near a spiric's pinch, so refusals there are expected.
- The named reads that replace `Lever::length` are listed from a sample of about 100 uses, not all of them.
- None of this bears on the three edits.
- The unit that builds this measures them and reports any newly refusing class of valid bodies before it merges.

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EdpJs4dRmhgaKbdqQzfNmw
