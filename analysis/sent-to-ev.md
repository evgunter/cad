**One decision: three sentences added to "Float rounding" (`crates/geom-core/README.md`), in this PR's diff. Two designers agree on all three, word for word, and on the design under them (A and B below). Recommended: all three.** Each says something the tree already does in places and no ratified text says.

**1. A lever's length is not geometry.** In the lever bullet, after its list of sources:

> The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever.

*Why it is asked.* A bound of a face's corner at a vertex reaches the corner predicates today as one vector: a direction scaled by a lever's length. For a line edge that is its chord. For a curved edge it is the unit tangent times the edge's extent lever, which ends on no point of the body, and the code reads that one vector both as a lever (its norm) and as geometry (its end point). `Lever::length`'s doc already calls it "the exit, not an entrance"; nothing ratified does. *Rejected:* leaving it to that doc and a count of uses, which binds only where a reviewer remembers it.

**2. A turn between two directions out of one point is levered at the lesser of their far distances.** In the consumed-extent bullet, after the sentence that defines a lever from a point:

> A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' far distances from that point, each read from below and from above. A direction drawn between them has no extent of its own and takes that lever.

*What it commits.* A corner's lever is the shorter of its two edges' far distances from the vertex, known from both sides; a divider (a direction a wide corner is cut at) takes its corner's. The boolean's cut already reads its corners so. The star's own bound instead reads a curved edge at a lower bound on its *diameter*, which is not a lower bound on its far distance from the vertex: on a nearly closed ellipse with semi-axes 10 and 1 leaving an end of its minor axis, 20 against about 10.05, on the side that claims a definite sign. *Rejected:* levering a divider at its face's far distance from the vertex. That is owed only if some reader takes a corner's Zero as two faces coinciding; both designers read the consumers and found none does without the glue door's whole-face verdict, and it would refuse many straight corners.

**3. A record read from one point holds its lever from both sides.** The same bullet's closing clause becomes:

> so a record whose verdicts all read from one point may hold its lever, read from below and from above, and a record that verdicts read from different points holds its region, not a lever.

*What it is.* The one clarification of the records rule you approved on PR 7141; its two questions stand. A record whose readers act on different signs cannot hold one side of a lever without choosing for them. *Rejected:* a third question added to the rule (one designer's first proposal, withdrawn: the case that prompted it needed an arc length, which is neither a lever nor a region).

**What the sentences license, in one paragraph.** A sector bound becomes a *departure*: a unit direction, the edge's far distance from the vertex read once from below and from above, and what stands behind it (a line's far vertex; a curved edge's bend and arc; a divider's face section). The four forms the tree holds of "how an edge leaves a vertex" become that one. Corner predicates take unit directions and an arm and decide through one door that reads both sides and acts where they agree; reading one side is a visible opt-out with a row that goes red when it is swapped. Every point placed is a vertex, a point of an edge, or a named construction sized by a length of geometry. How a lever's length is kept from use as geometry is code (it is reachable only through named reads), not a decision for you.

**What flows where** (your condition on PR 7141). The star comes to read each edge's cover from the vertex, which the cut already reads there. `geom_brep`'s corner predicate sees less (no chords). The link's cone holds departures where it held fabricated points. The pierce read takes two faces' covers, faces it already reads. Neither designer finds a module that comes to read a region it had no business with.

**Not measured: nothing was built or run.** Both designers hold that no count could change a sentence: a count says which bodies' verdicts move. The direction is known: a curved bound's lever from below shortens, so definite signs on ellipse, spiric and spline bounds move toward the band (refusals or escalations); lines do not move; circular arcs past a half turn lengthen. How many, and whether any class of valid bodies newly refuses, is unknown. The unit that builds this counts them, and any such class comes back to you on its PR before it lands (your condition on PR 7140).

**Independent of PR 7619** (the turn lever about an edge): every lever here is read from a point, not about a line.

**Sequencing, which is mine.** After your answer this PR stays open and the first unit builds on this branch, so that the text merges with code that makes it true. The rounds, the points on which the designers crossed and how each was settled by a reading of the tree, and the defects they found on the way (filed as `work/oneround/the-sector-machinerys-defects-found-while-weighing-what-a-bound-holds.md`) are in `work/oneround/log.md`, the entries of 2026-11-23 from 20:54Z.

The two designers' final texts follow verbatim.

---

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
