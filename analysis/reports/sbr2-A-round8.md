## The three points

1. **Edit 2.** I sign "each read from below and from above". I find no test that separates it from "read from below and from above alike": both say that each extent's far distance is read from both sides and the lesser taken on each side, which is what `LeverBounds::min` does. The tie-break is yours.
2. **The tie's witness.** I sign B's, with one fault fixed.
   - B's two readings of the tree hold: `ConicArc::at` stops at the far end, and a line's `witness` has no stop at `far`.
   - The fault: "at *an* arc length no longer than…" does not fix which point. Any such point is sound, but the parting is levered at its distance, so the sentence must say which point it is.
   - The fix: "at an arc length of the lesser of the face's radii of curvature, or at its far end where that comes first".
3. **The fan's slack.** "With no hand-written slack" stays. That is my position.

## For Ev

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

## For the orchestrator

- **Where my text differs from B's round-6 section:**
  - edit 2 reads "each read from below and from above", your tie-break;
  - the witness sentence fixes the point at the lesser of the face's radii, or at the far end.
- **The witness fix depends on B's reading** that a line's `BoundSupport::witness` has no stop at `far`. The unit adds that stop when the reach leaves the fold.
- **Nothing was built or run.**
