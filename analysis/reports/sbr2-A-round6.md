## 1. The cone's ray

**Test: what the gathered cone's readers do with the vector.**
- `Cone::settle_side` reads `Distance::of(q, p, outward)`: the sign of a point's offset from a plane through `p`.
- `wedge_side` / `ahead` (through `Sector::place`) draw a point `x` in to the ray's length with `within(p, x, |r − p|)`, then read its offset from a plane through `p`.
- `Cone::crossing` forms `q + (w − q)·t` and `on_plane(q, w, …)`, then hands that point to `Sector::place`, which again reads only signs of plane offsets.
- The parity reference is the wedge's centroid `(a − p) + (b − p)`, carried to `q`'s distance or out to the shorter ray's length. It too is read only by plane offsets.
- In the gathered cone, `q` is itself a bound's ray (`vtxfac` / `recl` `reach.point`).

**Finding.**
- Every reading is the sign of an offset from a plane through `p`, and each offset is linear in the rays' vectors.
- A ray vector's part along a plane's normal `n` is `(d̂·n)·l`: a tilt times a lever. The lever bullet already defines that: "the length at which a tilt … displaces the geometry it governs".
- `within`'s fold with `|r − p|` is the lever folded with another length, which is margin arithmetic.
- No gathered-cone reader treats the vector as a place on the body or compares it with a real point at full scale. The real points in the picture are line far vertices and the cone's own apexes. They enter the same plane offsets as plain lengths, not as places compared with the rays.
- The readers that do read a norm back off a held point are `Link::of`'s `reach` and `second_order::reach`. They are filed defects, and in the final state each carries the departure's lever.

**Under either wording of edit 1,** "direction times lever, read only against planes" is a displacement, not a size or position of geometry. So the final state is my round-5 form:
- the departure holds a `LeverBounds` field;
- `link` forms each ray's displacement with one `geom-core` read, "a ray's displacement at a lever", at each bound;
- every sign is decided at both bounds, in step.

**B's need for a length dissolves on the tree.**
- B argues that `boolean::sectors::fans` has no body, so the departure must carry an extent-home measure giving both a length and a lever. That would hold only if the cone needed a *length*, and it needs only a displacement.
- The other length B names, the tie witness's abscissa, needs no reach at all:
  - `ConicArc::at` stops at the edge's far end;
  - a line's witness is read along its chord to its far vertex;
  - so the abscissa is the lesser of the face's radii of curvature and the edge's own arc length (for a line, its chord), all read off geometry;
  - the parting is then levered at the witness point's distance from the vertex, which is no farther than the edge reaches, so today's cap at the reach is implied.
- So the departure carries no second measure, and item 1 of the final state stands as written.

## 2. The three edits

- **Edit 1: r5.**
  - Test: say what the thing is, not what is forbidden. r5 says what a length that places or sizes geometry *is*; r4 is a prohibition.
  - The cone complies: its vectors are displacements, which no reader uses to place or size geometry.
  - The witness complies: its length is read from the curve and the face.
- **Edit 2: r5.**
  - Test: no word may be defined by itself. "Its lever is the lesser of … levers" does that; "far distances" is the bullet's own definition of a lever.
- **Edit 3: r5.**
  - Test: use the README's own words. It says "from below" and "from above".
  - "As it is known" adds only a hedge. An exact lever is read the same from both sides, so it needs no hedge.

## 3. Other changes

- **The witness, worded for both of us:** "A tie's witness is the edge's point at the lesser of the face's radii of curvature and the edge's arc length, each read off the geometry; the parting it witnesses is levered at that point's distance from the vertex."
  - It replaces B's "edge's own extent", which in this README names a region, not a length.
  - It replaces my "reach from below", which is not needed.
- **The fan's slack.** "No hand-written slack" stays as the end state.
  - The `1.01` also covers the rounding of the eight turn directions, but that is a short chain of operations on in-box values. The README's admission rule covers it ("the one statement about rounding, and no site charges it").
  - So the unit re-poses the rim and does not derive a new charge.
  - The rim's radius is the face cover's far distance read as a length, over `cos(π/8)`.

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
   - A tie's witness is the edge's point at the lesser of the face's radii of curvature and the edge's arc length, each read off the geometry; the parting it witnesses is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** The cone stays a picture read by plane offsets.
   - The boolean's gathered cone is fed departures. Each ray enters it as its displacement, direction times lever, at each bound, and every sign is compared in step.
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
   > A turn between two directions out of one point, each with its own extent, is consumed where both run: its lever is the lesser of the two extents' far distances from that point, from below and from above alike. A direction drawn between them has no extent of its own and takes that lever.
3. In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point may hold its lever, read from below and from above, and a record that verdicts read from different points holds its region, not a lever.

**Edit 3 is the whole clarification to PR 7141's records rule;** its two questions stand.

**Not yours.** How edit 1 is enforced is code: `Lever::length` is removed, and each use becomes a named read.

**PR 7619:** independent. These levers are read from a point, not about a line.

**Unmeasured.**
- Nothing has been built or run.
- Which verdicts move, which curved bodies newly refuse, and the straddle rate are all unknown.
- None of them bears on the three edits.
- The unit that builds this measures them and reports any newly refusing class of valid bodies before it merges.

## For the orchestrator

- **New: the cone ray is settled by a test, not by side.**
  - Every gathered-cone reader reads signs of plane offsets, each linear in the ray vectors. That makes them displacements by the lever bullet's own definition.
  - So the departure holds a `LeverBounds` field. The unit's gate sentence keeps "a ray's displacement for `link`'s picture" among its named reads.
  - B's extent-home measure in the departure is not needed: the cone needs no length, and the witness's length comes from the arc and the face's radii.
- **New: the witness abscissa drops the reach.**
  - `ConicArc::at` stops at the far end, and a line's witness is read along its chord to its far vertex.
  - The parting is levered at the witness point's exact distance, no farther than the edge reaches.
- **New: the fan's `1.01` covers the rounding of the eight turn directions too.** That rounding is inside the README's admission rule, so the unit re-poses the rim instead of deriving a new charge.
- **Still unread by either of us:** `rest::judge` on gathered cones. I found it reading `Link`s only, not `Cone::gathered`; worth one check by the unit.
- **Nothing was built or run.**
