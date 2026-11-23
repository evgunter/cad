# Round 6, designer B

## 1. The cone's ray

**Test: is the vector ever read as anything but the sign of a distance inside the cone's own picture?** It is not, so it is a displacement, and my round-5 objection was wrong.

What the readers do with it, in the tree:
- `Cone::settle_side`: `Distance::of(q, p, s.outward)`, the probe's distance from a plane through `p`. That is `Margin::levered(d̂·n, L)`.
- `Sector::place` (`wedge_distance`, `ahead_distance`): the same, after `within` scales the probe in to the ray's length.
- The parity reference: `w = p + c·(at / |c|)`, built from `(a − p) + (b − p)`. `ArcReach::clears_as` then decides `Distance::clearance(p, (q, w))`, a distance from `p` to a segment of the picture.
- `Cone::crossing`: the point where the segment `q`–`w` meets a plane, then `Sector::place` and `Sector::misses` on it. Signs again.
- What leaves: `reads_tied` returns a `Material`, which `vtxfac` and `recl` map to a `SideCode` or a bool. `fan::nappe_classes` returns classes. No point is returned, stored on the body, or compared with a body position at full scale.
- A line's ray in the same cone is its far vertex, which is its direction at its exact chord. So every ray of a gathered cone is one kind of thing.

The lever bullet defines a lever as "the length at which a tilt … displaces the geometry it governs". Direction times lever, read for those signs, is that displacement. It is not a size or position of geometry under either wording of edit 1.

**The final state.**
- The departure holds a `LeverBounds` field. There is no extent-home measure; I withdraw it.
- A reader that needs a position in the picture forms the displacement from that field through one named read.
- Item 1 of "For Ev" is unchanged ("a reach … read once from the edge's cover").
- The measure had one other possible customer, the tie witness's length. It needs none: see point 3.

## 2. The three edits

**Edit 1: r5.** "The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever."
- Test: does it say what a thing is, and does it avoid repeating its neighbour? r4's "a lever is taken from its sources" repeats the sentence before it ("It is built only from its sources"), and its second half is a prohibition.
- The cone complies with both wordings for the same reason: its vector places no geometry. r5 is the one I can square with that without argument, since it speaks of a length that "places or sizes geometry" and the cone's does neither. In round 5 I read r4's "position" literally and got the cone wrong by it.

**Edit 2: r4.** "…its lever is the lesser of the two extents' levers from that point, from below and from above alike. …"
- Test: what does "from below and from above alike" attach to? It attaches to reads. `LeverBounds::min` takes the lesser of the two reads from below and the lesser of the two from above. A far distance is one true number; a lever is what is read from below or from above ("reads the extent from below", the lever bullet). So "levers" is the exact word.
- "The extents' levers from that point" is defined by the sentence just before it.

**Edit 3: r5.** "…may hold its lever, read from below and from above, and…"
- Test: is every word one this section already uses for this? "Read … from below … from above" is the lever bullet's own verb. "As it is known" is not used of a lever anywhere in "Float rounding"; it is the phrase of the undecided PR 7619 passage ("a lever known from both sides"), and this text should not lean on that.

## 3. What else the other's round 5 changes

- **The tie witness.** We swapped wordings. Test: which lengths does the witness's reader hold as geometry, with no lever read back? `Tie::witnessed` has the bound and no body. The bound holds the arc (a circle's or an ellipse's carrier and span) or a line's two points, and the face's radii are bare numbers. "The edge's reach from below, read as a length" is not in hand once `Lever::length` is gone. So I sign: "A tie's witness is the edge's point at an arc length read from geometry, no longer than the edge's own extent and the face's radii; the parting is levered at that point's distance from the vertex."
- **The fan's `1.01`.** Agreed with A's round 5: a filed defect the unit re-poses. "No hand-written slack" comes out of the text for Ev, because the turns' rounding still has to be covered and how is not decided.
- **Brief sentence (a).** I sign A's with two words changed, because `clears_as` reads a clearance and not a plane distance: "A ray of the gathered cone is a real point (a line's far vertex) or a departure read at both bounds; a reader that needs a position in the cone's picture forms the ray's displacement, its direction times its lever, at each bound, reads it only for signs of distances within the picture, and decides every sign at both bounds in step."
- **Brief sentence (b).** I sign A's as written.

## For Ev

**Recommendation (likely).** A sector bound is a unit direction plus what stands behind it, never a vector scaled by a lever. One departure record serves the star, the bound-side read, the cut and the link.

**Terms.** A *bound* is one side of a face's corner at a vertex: an edge leaving the vertex, or a *divider*, a direction a wide corner is cut at. A *lever* is the length a tilt is multiplied by to give a displacement. *Reach* is how far an edge gets from the vertex. `LeverBounds` (exists today) is a lever known exactly, or between a bound from below and a bound from above. A *straddle* is the two bounds giving different signs; it refuses.

**Premise (sure).**
- The cut already has this shape. `boolean::fate::corner_holds` takes a unit direction, a reach from the vertex known at both bounds, and a bend.
- The tree holds one concept in four forms, with different levers.
- `geom_brep::sector_shape` forces the packing: it asks for scaled chords and reads its arm from their lengths.

**Worked example (sure).** An ellipse with semi-axes 10 and 1, and an edge leaving the end of the minor axis and running nearly all the way round. Its diameter is 20. Its far distance from the vertex is about 10.05. Today's lever is a lower bound on 20, on the side that claims a definite sign. So a tilt between ε/20 and ε/10 is called definite where the edge reads within the band.

**The final state.**
1. **One departure record, in `topo`, built by one door from the body.** It holds:
   - a unit direction;
   - a reach, as `LeverBounds` read from the vertex: a line's chord, exact; a circular arc's, exact; for an ellipse, spiric or spline, the edge's farthest sampled point from below and its cover's far distance from above;
   - what stands behind it: a line's far vertex; a curved edge's bend and, for a circle or an ellipse, its arc; a divider's face section and its corner's arm.
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
   - A tie's witness is the edge's point at an arc length read from geometry, no longer than the edge's own extent and the face's radii; the parting is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** A ray of the gathered cone is a real point (a line's far vertex) or a departure read at both bounds.
   - Where the cone's reading needs a position in its own picture, it forms the ray's displacement, direction times reach, at each bound. That is read only for signs of distances within the picture, never as a place on the body.
   - The link's face-tie fan stays a named construction: a polygon past the face, sized by the face cover's far distance, read as a length.
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
   > A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' levers from that point, from below and from above alike. A direction drawn between them has no extent of its own and takes that lever.
3. In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point may hold its lever, read from below and from above, and a record that verdicts read from different points holds its region, not a lever.

The two questions of PR 7141 stand as you approved them, and edit 3 is their one clarification.

**Unmeasured. Nothing was built or run.**
- How many verdicts move, and on which bodies, is not counted. Lines keep their chord, and circular arcs past a half turn lengthen.
- The straddle rate on ellipse, spiric and spline bounds is not known. The upper bound is known to be loose near a spiric's pinch, so refusals there are expected.
- It is not known whether the cone read at both bounds agrees with today's points where the lever is exact.
- The named reads that replace `Lever::length` are listed from a sample of about 100 uses, not all of them.
- None of this bears on the three edits. Your standing conditions cover it: the unit's PR carries the counts, and a class of valid bodies that newly refuses comes back to you there before it lands.

**Independent of PR 7619.** Every lever here is read from a point. `sector_shape` takes its arm as an argument, which leaves the edge-wedge caller free to pass whatever that fork decides.

## For the orchestrator

**New this round.**
- **Withdrawn:** my round-5 extent-home measure, and its consequence that the departure's reach becomes a method. The departure holds `LeverBounds`.
- **Why we crossed on the cone.** Each round one of us tested the vector against the words of edit 1, not against what its readers do. The readers settle it: signs only, and a `Material` out.
- **For the unit, on the displacement read.** It returns a bare vector whose norm is the lever's length, so it is a raw exit in another shape. It has one legitimate caller, `link`'s picture. Pin it there, counted, as `lever-doors.sh` pins the others. `within`, `Link::of`'s `reach` and `second_order::reach` then take the ray's reach from the departure and stop reading a norm back.
- **The witness's length** comes from the arc or the line's two points the bound already holds. If the unit finds a tie whose bound holds neither (a spiric or spline has no arc), the witness is already absent there today (`ReadFault::Unwitnessed`), so nothing new is needed.
- **Where A's round 5 and this report differ in text:** edit 1 (A signed r4, I pick r5), edit 3 (A signed r4, I pick r5), the witness sentence, and two words of brief sentence (a). Edit 2 and brief sentence (b) are identical. Each difference has its test stated above; none changes the final state.
- **Checked this round:** `vtxfac`'s and `recl`'s use of `reads_tied`'s result, `Cone::crossing`, `ArcReach::clears_as`, `Sector::misses`, and the README's lever bullet for "known" and "read".
- Nothing was built or run.
