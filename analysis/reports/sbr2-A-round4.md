## 1. The four points

**1. The README edits: one text for each, below.**
- **Edit 1, "A verdict's lever arm": B's second clause, my first clause.**
  - B's first clause, "read only in its product with a tilt", binds too much. The README names `levered_inv` in the same bullet, `Margin::sagitta` squares an arm, and a dozen gates decide `Margin::of(arm)` itself. None of those is a product with a tilt.
  - "Margin" stands where it is in that bullet: "a length that is the reading itself … is the margin".
  - My round-3 second clause, "a point, a scale or a region … read from the geometry the lever was read from", restates the consumed-extent bullet ("built only from the geometry consumed"). B's "a length that places or sizes geometry is a read of that geometry" says what such a length is, so I take it.
- **Edit 2, the turn and the divider: B's place and frame, with two words changed.**
  - "Consumed where both run" makes the lesser follow from the bullet's own "its lever is the extent's far distance". That is better than my stating it as a rule.
  - "Reach" is not a README term (`geom_brep::Reach` is the consumed extent itself). "Far distance" is, so I use it.
  - "At each bound" leans on a word the README never uses for this. It says "from below" and "from above", so the text says that.
- **Edit 3, the records clause: B's, verbatim.**
  - It edits the one sentence the rule already lives in, and adds no question.
  - My round-3 wording sat in a work file, not in ratified text, so it was in the wrong place.

**2. The gathered cone's interior: there is a final-state difference, and B is right.**
- `Cone::crossing` places a point on the segment from `q` to `w`, then asks `Sector::place` whether that point lies inside a sector. `w` is the reference point, built from a wedge centroid `(a − p) + (b − p)`. The test is affine: it needs the ray points, not only their directions. `rest::judge` and `Link::pieces` read chord points as positions too, and `Cone` is shared with real-scale links.
- So the cone stays a picture of points. The gathered cone is fed departures, and one door in `link` realises each ray at both bounds. Each `Distance` decides both, and every sign is compared in step.
- The reader that decides it is `Cone::crossing`.
- For edit 1 to hold, the door reads its point's distance off the edge's cover as a length. It does not read it back out of the departure's `Lever`. The same numbers then give both the point and the lever.

**3. The gate: no difference is left. B's end state is the buildable form of mine.**
- `geom_brep`'s extent and curvature homes compose levers across a crate boundary. Removing `length()` outright would only move that read into a pub raw exit, and B's count of zero outside the homes is that exit, counted.
- **Brief sentence (both of us):** "Each read of a lever's length outside geom-core and geom_brep's extent and curvature homes becomes a named read (the arm as a length the band reads, the angle a length subtends at it, the turn over it, a diagnostic) or a parameter typed `Lever`, and a typed count of raw reads (`clippy::disallowed-methods` on `Lever::length`) holds at zero outside those homes."

**4. Nothing gates the question to Ev. Every measurement gates only the merge.**
- None of the measurements can change any of the three sentences or the recommendation:
  - the old lever logged beside the new bounds at the star's departures;
  - straddles counted by predicate;
  - the gathered cone at both bounds;
  - `conic_bend_side` with the curve's own arc bound;
  - the swap rows.
- Each decides only which bodies' verdicts move. Ev's standing conditions already cover that: a verdict that moves toward refusing is accepted (PR 7141), and a class of valid bodies that newly refuses is reported before it lands (PR 7140).
- The one result that could bear on the design is a high straddle rate on spline bounds. That would argue for tighter reach reads, which is a matter of implementation. It would not change what the lever is.
- One reading could have touched edit 2's divider clause. I made it. `splitting::rules` reclassifies a divider read `On` from its two neighbours, or from its wall's graze. That is a claim about the neighbourhood, as `fan::meeting`'s is.
- The PR to Ev must state that nothing was built or measured. It does not yet know how many verdicts move, which curved bodies newly refuse, or the straddle rate. The unit that builds this reports any newly refusing class of valid bodies on its own PR before merging.

## For Ev

**Recommendation (likely).** A sector bound becomes one record, a *departure*. It holds a unit direction and the edge's far distance from the vertex, known from below and from above, as two separate things. Every sector predicate takes unit directions and that lever.

**Terms.**
- **Bound:** one side of a face's corner at a vertex. It is either an edge leaving the vertex or a *divider*, a direction a wide corner is cut at.
- **Lever:** the length a tilt is multiplied by to give a displacement.
- **Reach:** how far an edge gets from the vertex.
- **`LeverBounds`** (exists today): a lever known exactly, or between a bound from below and a bound from above.
- **Straddle:** the two bounds giving different signs. A straddle refuses.

**Premise (sure).**
- Half of this exists already. `boolean::fate::corner_holds` takes a unit direction, a two-sided reach read off the edge's cover from the vertex, and a bend.
- One concept is held in four forms with different levers:
  - the star's bound, a direction packed with the edge's diameter read from below;
  - the cut's `Leaving`;
  - `validate`'s scaled departures;
  - the link's scaled vectors.
- `geom_brep::sector_shape` forces the packing: it takes two scaled chords and reads its arm from their lengths.

**Worked example (sure).** Take an ellipse with semi-axes 10 and 1, and an edge leaving the end of the minor axis and running nearly all the way round.
- Its diameter is 20. Its far distance from the vertex is about 10.05.
- Today's lever is a lower bound on 20, on the side that claims a definite sign.

**The final state.**
1. **One departure type, in `topo`, built by one door.** It holds a unit direction, a reach and what stands behind it:
   - **Line:** its far vertex. The reach is the chord, exact.
   - **Curved edge:** its bend and its arc. The reach is read once from the edge's cover from the vertex, from below and from above, and is exact on a circular arc.
   - **Divider:** its face's section. Its reach is its corner's arm.
2. **A corner's arm is the lesser of its two bounds' reaches, each bound the lesser of the two's (`LeverBounds::min`).**
   - It is derived where the two bounds are held.
   - `SectorShape.arm`, `FullTurn.arm`, `ApexShape.arm`, the stored `star::Sector.arm` and `metering_arm` go.
   - `geom_brep::sector_shape` and `chart_normal_beside` take two unit directions and an arm.
3. **One both-bounds door decides every sector predicate.**
   - It reads once where the lever is exact, and at both bounds otherwise, deciding only where they agree.
   - A site reads one bound only by writing `.below()` or `.above()`, and each such site has a row that goes red when the bound is swapped.
   - At a slit's tip, where a Zero reads the whole turn, a straddle refuses.
4. **Points.** Every point placed is a vertex, a point of an edge, or a named construction sized by a length of geometry.
   - The conic-against-plane witness sits at an arc length the bend record bounds off the curve, and `conic_bend_side` loses `extent`.
   - A tie's witness is the edge's point at an arc length read from geometry, no longer than the edge's own extent and the face's radii; the parting is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** The cone stays a picture of points.
   - The boolean's gathered cone is fed departures. One door in `link` realises each ray at its far distance from below and from above, read off the edge's cover, and every sign is compared in step.
   - The link's face-tie fan stays a named construction: a polygon past the face, sized by the face cover's far distance read as a length, with no hand-written slack.

**The records (likely).**
- `PlannedRun.order` keeps a lever, as bounds. The pierce's ball is built separately, from the two faces' covers.
- editor-core's carrier reach holds its region.
- `Leg::arm` and `SectionOval::extent` become methods.
- `Margin::over_lever`'s calls are each classed first, as a tilt's arm or a measure.

**What flows where.**
- `sector_shape` sees less.
- The star reads each edge's cover from the vertex, which the cut already reads there.
- The pierce read takes two faces' covers, faces it already reads.
- No module learns a region it had no business with (likely).

**Yours: three edits to `crates/geom-core/README.md`, "Float rounding".** I recommend all three. Each states what the tree already does in places.

1. In **A verdict's lever arm**, after "…or the session box where no geometry is in hand.":
   > A lever enters only a margin; a length that places or sizes geometry is a read of that geometry.
2. In **The consumed extent**, after "…the least over all points is not owed.":
   > A turn between two directions out of one point is consumed where both run: its lever is the lesser of their far distances from that point, from below and from above alike. A direction that divides a corner has no edge behind it, and is levered at that corner's.
3. In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point may hold its lever, as the bounds it is known between, and a record that verdicts read from different points holds its region, not a lever.

**Edit 3 is the whole clarification to PR 7141's records rule;** its two questions stand.

**Not yours.** How edit 1 is enforced is code: named reads, and a typed count of raw reads held at zero outside geom-core and geom_brep's extent and curvature homes.

**PR 7619:** independent. These levers are read from a point, not about a line.

**Unmeasured.**
- Nothing has been built or run.
- Which verdicts move, which curved bodies newly refuse, and the straddle rate are all unknown.
- None of them bears on the three edits.
- The unit that builds this measures them and reports any newly refusing class of valid bodies before it merges.

## For the orchestrator

- **One text now:**
  - the three README edits, as above (B's edits 2 and 3 with "far distance" for "reach" and "from below and from above" for "at each bound"; edit 1 combined);
  - the cone stays points, fed by a door that realises each ray at both bounds;
  - the gate (the brief sentence in point 3);
  - nothing gates the question.
- **For the unit: the merge-gating measurements**, each judged against PR 7140 and PR 7141:
  - log the old lever beside the new `LeverBounds` at `star::departure`, `fate::leaving` and `validate::side`, per carrier kind, over the boolean, split, census and link-fuzz suites;
  - count straddles by predicate name (zero on lines and circles by construction);
  - compare each sign of the gathered cone at both bounds, in step;
  - run `conic_bend_side` with the curve's arc bound (no circle row moves);
  - every surviving opt-out needs a swap row that goes red.
- **The door that realises cone rays** reads its distances off the cover as lengths, not out of the `Lever`. `second_order::reach` and `Link::of`'s `reach` carry the reach instead of reading a norm back.
- **Defects still standing:**
  - `held_reads`' ball is built from an arm;
  - `link::fan`'s 1.01 slack;
  - `conic_bend_side`'s `Unbounded` arm;
  - `second_order::reach`'s fabricated point;
  - `wedge_holds_named` / `corner_holds` are one predicate written twice;
  - `apex_extrema_inside`'s Zero at a lever from below;
  - B's `SlitTips` keyed by `(vertex, face)` (unconfirmed reachable);
  - `side_reader`'s `reach.length() * −2`;
  - the absent row `sector-bounds-pack-a-direction-and-its-lever-into-one-vector`.
- **Read this round:** `Cone::crossing`, `splitting::rules`' divider reclassification, the README bullets. Not read: `rest::judge` beyond B's account, `Link::offset`.
- **Nothing was built or run.**
