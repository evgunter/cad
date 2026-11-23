## For Ev

**Recommendation.** A sector bound stops being a vector with a length. It becomes one record, a *departure*, that holds a unit direction and a lever apart, and every sector predicate takes that record. The lever is the edge's far distance from the vertex, known from both sides. No point is placed at a lever's length. Confidence: likely overall; per claim below.

Terms. A *bound* is one side of a face's corner at a vertex: an edge leaving the vertex, or a *divider* (a direction the walk cuts a wide corner at). A *lever* is the length a tilt is multiplied by to give a displacement. *Reach* is how far an edge gets from the vertex. `LeverBounds` (exists today) is a lever known exactly, or between a bound from below and one from above. A *straddle* is the two bounds giving different signs; it refuses.

**Premise check (three corrections).**
- Half of this already exists. `boolean::fate::corner_holds` takes a unit direction, a `LeverBounds` reach read off the edge's cover from the vertex, and a bend, and reads every sign at both bounds. Its `.ray()` is a type conversion, not the packed vector. `fate::leaving`'s doc says it reads its own reach because the star's is the wrong one for it. (sure)
- The issue is evidence of a concept nothing represents once: how an edge leaves a vertex. The tree has four forms of it: `star::walk::Bound` + `BoundSupport` (lever: a diameter from below), `fate::Leaving` (reach from the vertex, both bounds), `validate::Departure` (the vector scaled at both bounds), and `link`'s scaled vectors (`TangentFan`, `Horn.ends`, `ApexLobe.ends`). Two predicates answer "is this direction inside that corner": `validate::wedge_holds_named` and `fate::corner_holds`, levered differently. (sure)
- The packing is forced from below. `geom_brep::sector_shape` asks for two scaled chords and takes the arm from their norms. `star::walk::Bound` already holds direction and support apart and re-packs with `ray()`; `census` multiplies `own * arm` only to call it. (sure)
- The "read as geometry" sites in the gathered cone mostly read the point as direction times reach again (a point's distance from a plane, drawn in to the ray's reach). Only the split's `p + along` and the tie witness use a placed point as a position. (likely)

**The final state.**

1. *One departure type*, in `geom_brep`, replacing the four forms:
   - `dir`: unit. A line's chord direction, a curved edge's tangent at the vertex, a divider's direction.
   - `reach`: `LeverBounds`, read from the vertex. Line: its chord, exact. Circular arc: the cover's far distance, exact. Ellipse, spiric, spline: the farthest sampled point of the edge from below, the cover's far distance from above (`edge_reach_bounds`, already used by `fate`, `validate`, `census`, the offsets). Divider: the lesser of its corner's two edges' reaches (`LeverBounds::min`).
   - what stands behind it: a line's far vertex; a curved edge's bend and its arc (carrier and span); a divider's face section.
   - No `ray()`, no `point()`. `edge_extent_inf` leaves the sector machinery. (likely)

2. *The one number that was three things separates.*
   - Lever for first-order signs: `reach`, above.
   - Arc length for the conic-against-plane bend: a length of the curve, held by the bend record (`BendBounds::Planar.arc`, already there) and bounded from below off the curve itself. `conic_bend_side` loses its `extent` argument. (likely)
   - Scale for a ray: gone.

3. *Predicates take the departure and decide through one both-bounds door* (`validate::decide_at_lever`, moved beside `LeverBounds`): one reading where the lever is exact, two that must agree otherwise. A site reads one bound only by writing `.below()` or `.above()`. This is the answer to the track's repeated wrong bound: the default needs no judgement, and the only places a reviewer must read a consumer are the ones that opt out. (likely)
   - `sector_shape(own, next, arm, normal, bounds)` takes two unit directions and the arm. It returns only the shape: convex, split at a direction, or a full turn. An undecided or straddling convexity already falls to the subdivision rung, which is sound at every angle.
   - `bound_side::read`, the apex predicates, `runs_into`, `first_order_overlap`, `junction_bound_side`, `corner_holds`: the same record, the same door.

4. *Points.* A point the sector code places is a vertex or a point of the edge, by evaluating its curve at a parameter. The tie witness already does this. The split's `p + along` becomes a point of the shared edge, which lies on both faces; today, for a curved edge, it lies on neither. (likely)

5. *The gathered cone and the link's tangent pieces.* Two defensible states:
   - **Rays (my lean).** A tangent sector's wedge holds two departures; a link of real faces keeps its real points. The cone's wedge and ahead signs are a sine at the lesser of two reaches, so they go through the both-bounds door. `fans`, `line_fans`, `second_order::reach`/`toward`, and the fan rim built at `reach · 1.01 / cos(π/8)` go. Nothing fabricated is left.
   - **Named constructions.** The cone stays point-based and is fed "the tangent ray's point at its reach", built by one door, once per bound. Smaller. But the two whole readings agreeing does not make each sign agree, so a mixed truth can pass. (sure of the gap)
   - Confidence in the lean: unsure. I did not read every cone reader.

**Worked example.** A nearly closed flat ellipse (semi-axes a ≫ b) leaving a vertex at an end of its minor axis. Its diameter is 2a. Its far distance from that vertex is about a. Today the bound's lever is a lower bound on 2a, on the side that claims a definite sign, so a tilt between ε/2a and ε/a is called definite where the edge reads in band. And the "ray point" sits 2a along the tangent, a full a past anything the body has.

**The records.**
- `SectorShape.arm`, `FullTurn.arm`, `ApexShape.arm`: deleted. The caller holds the two bounds, so the arm is derived there (question one of the rule). `metering_arm` goes. `star::Sector.arm` and `BoolSector.arm` become methods over their bounds. (likely)
- `PlannedRun.order`: keeps a lever, as bounds; its ordering readers all read at the pierce point. The ball `held_reads` builds from it is a different thing: a region read from carriers' own pivots, so it is built from the two faces' covers. (likely)
- editor-core's carrier reach: holds its region. `Leg::arm`, `SectionOval::extent`: methods of the geometry the record already holds. (likely)
- `Margin::over_lever`: not a records question. Its doc's divisors are a perimeter, a radius, an area. Each call is first classed as a tilt's arm or a measure. (unsure; calls not read)

**Is the two-question rule enough?** For every record above, yes. The sector bound shows two gaps beside it, not a wrong question:
- The rule sees lever *fields*. This lever was multiplied into a direction and stored as the product, so no question reached it.
- "May hold the lever" has to mean both bounds. One record's readers act on different signs; a record holding one bound has chosen for all of them.

**What flows where.**
- `geom_brep::sector_shape` sees less: no chords.
- `star::walk` comes to read each edge's cover from the vertex. `fate` already reads the same cover from the same vertex.
- Dividers carry their corner's arm, as today, now as bounds.
- The cone's readers see directions and reaches, not points.
- `held_reads` reads two faces' covers; it already reads those faces.
- I see no module learning a region it had no business with. (likely)

**Faithful to PR 7141, or yours.**
- Faithful: the curved bound's lever is its reach from the vertex (the README's "far distance from that point"); both bounds read; the three arm records deleted; the item-3 records by the two questions.
- Yours, with my recommendation:
  1. One README sentence: *a lever's length is read only by a margin; nothing is placed or scaled by it, and a record that needs a direction and a lever holds both.* Recommend yes. `Lever::length`'s doc says this; nothing ratified does.
  2. The cone as rays (5). It changes what `link` states a ray is ("fixed by its point"). Recommend rays, after the measurement below.
  3. The divider's lever. A divider has no edge. "The lesser of its corner's edges' reaches" is a convention about how far a corner is read, and no ratified clause states it. Recommend keeping it and stating it. The alternative from above, the face's own reach from the vertex, would refuse many straight corners.
- PR 7619: independent. These levers are read from a point, not about a line. `sector_shape` taking its arm as an argument is what keeps the edge-wedge caller free to pass whatever that fork decides.

**To measure first, and against what.**
- Old lever against new bounds, per carrier kind, over the boolean, split and census suites; every changed verdict listed. Lines must not move. Circular arcs past a half turn lengthen (today's read is short of the diameter there). Judge: any class of valid bodies that newly refuses is reported to you (PR 7140).
- Straddle rate on ellipse, spiric and spline bounds, and on dividers beside them. Judge: zero on lines and circles by construction; the rest is the class reported.
- The cone: the same suites with rays against today's points. Judge: every decided sign equal wherever the lever is exact. A difference is a place the cone used a point as a position, to be read before (5) is claimed.
- `conic_bend_side` with the curve's own arc bound. Judge: no tie row on a circle moves; ellipse rows listed.
- Every `.below()` / `.above()` that survives needs a row that goes red when the bound is swapped.

## For the orchestrator

**Errors in the brief.**
- `corner_holds(edge.ray(), …)` in `fate` and `overlap` is not a reader of the packed vector. `Leaving::ray` converts `Leaving` to `Ray`; the predicate already takes direction, `LeverBounds` and bend. It is the precedent, not a site.
- The cone's "read as geometry" sites (`Cone::gathered`'s rays, `sectors::fans`, `vtxfac`'s `reach.point`) feed `wedge_distance` and `ahead_distance`, which draw the probe in to the ray's reach and read a point-plane distance: a sine times the lesser reach. I read those two and `ArcReach`; not the rest.
- "Made at" misses: `census.rs` near `edge_wedge_shape(own * arm, next * arm, …)`; `link`'s `TangentFan::Corner`, `Horn.ends`, `ApexLobe.ends` ("each scaled to its reach"); `second_order::thorns`' `along(d, l)`; `validate::wedge_holds_named`, which takes its lever from a vector's norm inside `Margin::of(n·(x×y)/|x|)` with no `Lever` at all.
- `metering_arm`'s nine callers check out: `sector_shape` 2, `star::walk` 3, `topo::sector_shape` 2, `splitting::insert` 2.
- The row `sector-bounds-pack-a-direction-and-its-lever-into-one-vector` is cited by `sizes-are-read-back-out-of-extent-levers.md` and is not in this tree.

**Not checked.**
- `link.rs` beyond `Sector`, `Cone`, `Fan`, `gathered`, `wedge_distance`, `ahead_distance`, `ArcReach`, `fan`, `FaceSector::tangent`. The parity reading's internal references (a wedge's centroid, "carried to q's distance") I only read in docs. The rays recommendation rests on that.
- Whether the "ray fixed by its point" claim is in `crates/topo/README.md` and who ratified it. I ran no `git log -S` on it.
- `geom_brep::chart_normal_beside`'s sector predicates; `sector_face.rs` reads `ExtentBall::of_cover_from(net, at).lever()` before calling it and I did not follow it.
- The 43 `over_lever` calls; `vtxfac` beyond `PlannedRun` and `held_reads`; `splitting::neighborhood`.
- Each consumer's action per sign. I traced `sector_shape`'s three rungs and `bound_side::read`/`descend` only. The both-bounds default is meant to make the rest unnecessary; the opt-outs are not.
- Nothing was built or run. Every "moves" above is from reading.

**Assumed.**
- A vertex is always an end of its edge's span, so the reach is read from a span end.
- `cover::edge_reach_bounds` returns `None` only for null scaffolding, which the star already refuses (`StarFault::NullEdge`).
- A non-`Copy` cover need not be held by a bound: all a bound's readers read at the vertex, so by question two it holds the lever.

**Design notes for the unit.**
- The both-bounds doors (`decide_agreed`, `decide_levered`, `decide_at_lever`) live in `topo::validate`. `geom_brep::sector_shape` needs them, so they move down beside `LeverBounds`; `k_stats::straddle` is already in `geom-core`.
- `sector_shape`'s rung 0 (a chord's length is a finite, non-underflowed number) is a question about a chord, not a corner. It moves to where the departure normalises the chord.
- `BoundSupport::Extent` is test-only (`expect(dead_code)` outside tests); it becomes a fixture departure.
- The tie witness's abscissa (`witnessed(leading, arm.length())`) picks which real point of the edge to read. Any point of the edge is sound; taking the bound from below keeps it no farther than the parting was read. Say that in its doc so it is not read as a lever used as geometry.
- `edge_extent_inf`'s doc names point-read data as levered at the diameter; it changes with its callers, as `diameter-levers-of-a-datum-read-at-a-point` already says.

**Defects off the question** (not filed; I am read-only).
- `link::second_order::reach` reads a scaled vector's norm through the point door, `ExtentBall::point(apex + d).lever_from(apex)`: a fabricated point entering the "consumed points" door, invisible to both lever gates. Belongs on `unsourced-levers-await-a-source` or the records unit.
- `link::fan` sets its rim at `reach · 1.01 / cos(π/8)`, "a hundredth to spare for the rounding": hand-written slack in a construction.
- `validate::wedge_holds_named` and `fate::corner_holds` are two implementations of one predicate with different levers (the second direction's length; the lesser of the two reaches). One should go whatever is decided here.
- `conic_bend_side`'s `BendBounds::Unbounded` arm decides `0 − |slope · extent|` under `decide_positive`, which can never pass: a refusal spelled as a decision.
- `held_reads` hands `one_carrier` a ball of the pierce's shortest arm where `ConsumedExtent`'s own doc requires a ball enclosing both faces. The existing row notes it can under-state; it is the wrong-answer direction for that ladder.
