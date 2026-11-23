## For Ev

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

## For the orchestrator

**The three small things.**
- **Edit 2: I sign A's**, "each read from below and from above". I looked for a separating test and found none that decides. The nearest is whether the sentence fixes that the lesser is taken side by side (the lesser of the reads from below, the lesser of the reads from above, as `LeverBounds::min` does). "Alike" says that a little more directly, but A's wording has no natural other reading. So the tie-break is yours, as you say.
- **The witness: I sign mine**; I find no fault in it. One addition for the unit, already in my round 7: a line's witness has no stop at `far` today, so the unit adds one when the reach leaves the fold.
- **The fan's slack: the phrase stays**, and that is my position. The `1.01` written at the site goes and the unit re-poses the rim.

**Where my text still differs from A's round 6 in a word, and why.**
- Item 5, first line. A has "a picture read by plane offsets". I have "read only for signs of distances within it", because `ArcReach::clears_as` decides `Distance::clearance(p, (q, w))`, a distance from the apex to a segment, which is not a plane offset. The final state is the same.
- Item 5 also carries "never a place on the body", so that Ev can see why the cone's displacement does not fall under edit 1.
- The recommendation's last sentence and two lines under "Unmeasured" (the lines, the spiric pinch, the sampled reads) are mine; A's round 6 does not have them. Nothing in them is in dispute.

**Unchanged from earlier rounds:** the brief sentences for the unit (the cone's ray; the end of `Lever::length`, with the displacement read pinned at `link`), the merge-gating measurements, and the list of standing defects.

**Nothing was built or run.**