# Round 6

Both round-5 reports are in. The other designer's round-5 report is below, WHOLE and as delivered. You are designer B; it is designer A's. The same rules hold: read-only, the same tree, `docs/prompts/designer.md` binds you, no helper on another model.

Where round 5 leaves you. In round 5, A signed B's ROUND-4 wording of all three edits; B, in the same round, rewrote all three as merges with A's round-4 wording. So there are now two candidate texts for each edit, both written by B:

- **Edit 1.** (r4) "The read runs one way: a lever is taken from its sources, and no size or position of geometry is taken from a lever." / (r5) "The read runs one way: a length that places or sizes geometry is a read of that geometry, not of a lever."
- **Edit 2.** (r4) "…its lever is the lesser of the two extents' levers from that point, from below and from above alike. …" / (r5) "…its lever is the lesser of the two extents' far distances from that point, from below and from above alike. …" (the rest identical).
- **Edit 3.** (r4) "…may hold its lever, from below and from above as it is known, and…" / (r5) "…may hold its lever, read from below and from above, and…".

And on the gathered cone you have now crossed THREE times. Round 5: A says a reader that needs a position forms the ray's displacement, its direction times its lever, at each bound, read only against a plane (a levered quantity, never a place on the body), and drops the door that read distances off the cover as a second source for a reach the departure already holds. B says one door in `link` places the ray at the edge's far distance read as a length of the edge's geometry and never out of a `Lever`, because direction times lever "takes a position from a lever, against edit 1"; and that, since `boolean::sectors::fans` holds no body, the departure must then carry the far distance as an extent-home measure that gives both a length and a lever, its reach a method of that measure.

**Do not take the other's side because it is the other's; that is how you crossed.** Decide each by a test and say the test:

1. **The cone's ray.** Under each candidate wording of edit 1, is "direction times a lever's length, read only against a plane" a size or position of geometry, or a displacement (which the lever bullet already defines a lever by: "the length at which a tilt … displaces the geometry it governs")? What do `Cone::crossing`, `Sector::place`, `within` and the parity reference actually do with that vector: only signs of plane distances, or also a point later treated as on the body or compared with real points at full scale? Then say which of the two the final state is, and what the departure holds in it (a `LeverBounds` field; or an extent-home measure with the reach a method), and whether that changes item 1 of the `For Ev` final state ("a reach … read once from the edge's cover").
2. **The three edits.** For each, pick r4 or r5 (or one last merge), with the one test that decides it. For edit 1, the choice must be consistent with your answer to point 1: say which wording your cone complies with and why.
3. Anything else the other's round 5 changes for you (the tie witness's wording; "no hand-written slack" given the fan's `1.01` also covers the rounding of eight turn directions).

Then your `## For Ev` section, final, and `## For the orchestrator` with only what is new. At most 100 lines. Where you agree, use the same words.

---

# Designer A's round-5 report

## 1. The three edits: one sentence each

I would sign B's text for all three. Each is checked against `crates/geom-core/README.md` as it stands.

- **Edit 1. I sign B's:** "The read runs one way: a lever is taken from its sources, and no size or position of geometry is taken from a lever."
  - My "a lever enters only a margin" binds too much. `Lever::min`, the fold with a curvature radius and a refusal's report all read a lever, and none of them is a margin.
  - B's binds none of those, because none of them forms a size or a position of geometry.
  - "Its sources" is the bullet's own term ("It is built only from its sources").
  - It states the bullet's existing "a length that is the reading itself … is the margin" from the other side, and does not restate it.
- **Edit 2. I sign B's:** "A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' levers from that point, from below and from above alike. A direction drawn between them has no extent of its own and takes that lever."
  - "Corner" and "divider" are `topo`'s words and do not appear on this page. Mine used both.
  - "The extents' levers from that point" is defined by the sentence just before it: "its lever is the extent's far distance from that point".
- **Edit 3. I sign B's:** "so a record whose verdicts all read from one point may hold its lever, from below and from above as it is known, and a record that verdicts read from different points holds its region, not a lever."
  - "From below and from above as it is known" says what the lever is, in the same words as edit 2.
  - My round-4 "as the bounds it is known between" uses "bounds", a word this README never defines.

## 2. The unit's brief: two sentences I would sign

**(a) The cone.**
- The sentence: "A ray of the gathered cone is a real point (a line's far vertex) or a departure read at both bounds; a reader that needs a position in the cone's picture forms the ray's displacement, its direction times its lever, at each bound, reads it only against a plane, and decides every sign at both bounds in step."
- I drop my round-4 door that read its distances off the cover as lengths. It would be a second source for a reach the departure already holds.
- The displacement is a levered quantity: a reading of it against a plane is `Margin::levered(n·d, lever)`. It is never a place on the body, so it falls under neither edit 1 nor the defect being fixed.

**(b) The gate.**
- The sentence: "Remove `Lever::length`: each use becomes a `geom-core` read named for what it forms (the arm as a length the band reads, a levered tilt, a sagitta, the curvature charge, the angle a length subtends, a fold with a radius, a ray's displacement for `link`'s picture), a `Lever` parameter, a report exit with no arithmetic, or a finding; the extent and curvature homes hold their measures as lengths and mint levers at their exits, and a home that cannot keeps its raw read behind one counted door pinned there."

## 3. What B's report changes in my "For Ev", beyond those sentences

- **The witness sentence** takes B's wording: "the face's radii of curvature". `FaceDatum.lever` is a bare radius ("the face's smallest radius of curvature within the bound's reach"), so naming it a lever would put a lever read as a length in front of Ev.
- **The fan's `1.01`:** I said it was simply gone. Its doc gives it to the rounding of the turns too, which remains, so it is a filed defect that the unit re-poses.
- **The gate item** takes B's wording: "reachable only through named reads, by type".

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
4. **Points.** Every point the sector code places is a vertex or a point of an edge.
   - The conic bend witness sits at an arc length the bend record bounds off the curve.
   - A tie's witness is the edge's point at the least of the face's radii of curvature and the edge's reach from below, each read as a length of the geometry; the parting it witnesses is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** A ray of the gathered cone is a real point (a line's far vertex) or a departure read at both bounds.
   - Where the cone's reading needs a position in its own picture, it forms the ray's displacement, direction times reach, at each bound. That is read only against a plane, never as a place on the body.
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
   > The read runs one way: a lever is taken from its sources, and no size or position of geometry is taken from a lever.
2. In the bullet **The consumed extent**, straight after "…the least over all points is not owed.":
   > A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' levers from that point, from below and from above alike. A direction drawn between them has no extent of its own and takes that lever.
3. In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point may hold its lever, from below and from above as it is known, and a record that verdicts read from different points holds its region, not a lever.

The two questions of PR 7141 stand as you approved them, and edit 3 is their one clarification.

**Unmeasured. Nothing was built or run.**
- How many verdicts move, and on which bodies, is not counted. Lines keep their chord, and circular arcs past a half turn lengthen.
- The straddle rate on ellipse, spiric and spline bounds is not known. The upper bound is known to be loose near a spiric's pinch, so refusals there are expected.
- It is not known whether the cone read at both bounds agrees with today's points where the lever is exact.
- The named reads that replace `Lever::length` are listed from a sample of about 100 uses, not all of them.
- None of this bears on the three edits. Your standing conditions cover it: the unit's PR carries the counts, and a class of valid bodies that newly refuses comes back to you there before it lands.

**Independent of PR 7619.** Every lever here is read from a point. `sector_shape` takes its arm as an argument, which leaves the edge-wedge caller free to pass whatever that fork decides.

## For the orchestrator

- **New this round:** the two designers now share one text for all three README edits (B's round-4 wording) and for both brief sentences (2a, 2b).
- **2a replaces my round-4 door** that realised each ray off the cover. With it gone, the departure's `LeverBounds` is the one source for a ray's reach. The vector read `link` needs is the gate's "a ray's displacement for `link`'s picture".
- **Nothing else changed.** The merge-gating measurements and the standing defects are as in B's round-4 orchestrator section, which I would sign.
