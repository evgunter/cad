# Round 2, designer B

## 1. What moved me and what did not

**Moved (each checked in the tree).**
- **The link's face-tie fan stays.** I said `link::fan`'s rim goes. Wrong for `FaceSector::cleared`: its doc says the rim stands "past the farthest the face stands from `p` … so a coincidence read there holds over the whole face". That is an enclosure at the face's real scale, sized by `cover::face_reach_from(..).length()`. A is right: it is a named construction sized by a length read from the face's cover. Only the boolean's tangent rays (`sectors::fans`, `line_fans`, `vtxfac`'s `reach.point`) become departures.
- **`Lever::length` gated by type.** I offered a README sentence and review. A's gate makes the bad state fail to compile, which `docs/prompts/designer.md` §4 ranks first. I adopt it.
- **The witness: point first, lever from the point.** In `Tie::read_down` the fold `[face.lever, face.bend.radius()]` is of bare radii; only the bound's reach is a `Lever`. So the abscissa can be built from lengths of geometry and the parting levered at the witness's own distance. A's order is better than my "any point is sound".
- **The type lives in `topo`.** Its constructor needs the body (the edge's cover, the far vertex) and every consumer is in `topo`. `geom_brep` takes pieces (unit directions, an arm, a bend) and knows nothing of it.
- **From below suffices in `sector_shape` for distinct edges.** A definite sign at the lower bound is the same sign at the upper. So my default door and A's "from below" give one outcome there.

**Not moved.**
- **A corner's arm from above is the lesser, not the greater.** `fate::reading_lever` is `bound.min(o)`, and `LeverBounds::min` is "each bound the lesser of the two's, exact where both are". That is the cut both of us call the precedent. A's `[lesser below, greater above]` bounds two different lengths. It also makes two lines of lengths 1 and 3 inexact, so every both-bounds read on a plain polyhedron reads twice and can straddle. A cites the cut only for the lower bound.
- **A divider's lever from above is not the union of two edges' reaches.** Along a divider a face can reach past both edges (a polygon's diagonal), so that union is neither the neighbourhood nor the face. The fork has two ends: the lesser, or the face's reach.
- **Per-predicate bounds.** A's "a Zero only falls through to a subdivision sound at every angle" misses a consumer. At `CornerBounds::SlitTip`, `corner_shape` and `apex_corner_shape` act on the Zero: `reads_slit_tip_turn` turns it into a whole turn. This is the track's failure again, a bound chosen from a description. It also corrects me: a straddle must not read as that Zero.
- **Arm records deleted, not held as bounds.** `census` already calls `edge_wedge_shape(own * arm, next * arm, …)` with the arm in hand, and `Star` and `BoolSector` hold both bounds. A held copy is a third copy kept in step by hand.
- **`Leg::arm` is a method.** `profile::sugar::Leg` holds `len` and `arc` beside `arm`, "folded" from them. A grouped it with `Carrier`; the issue row argues a region only for `Carrier`.
- **`Margin::over_lever` cannot take a `Lever`.** Its divisors in the tree: `perimeter`, `e.surface_area`, `lever.powi(11)`, `ys[j] - ys[i]`, a chart's speed bound.
- **`PlannedRun.order` keeps a lever.** In `held_reads` the arm feeds `Margin::levered(int.norm(), arm)`, `PierceLever::within` and the sagitta charge, all at the pierce point. The ball feeds `one_carrier`, whose `ConsumedExtent` doc asks for "a ball enclosing both faces". A ball from two bounds' covers is not that either. A says it read only the field.
- **The departure holds its reach; it does not hold the curve.** The upper bound is read off the edge's cover, which `topo` builds from the body with the edge's ends. A curve and span rebuilt per record gives a second cover and a second reach for one edge, and it is A's own "one new flow" (spline carriers reaching `bound_side` and the link).

**Does A answer my arguments?** The default door: no (A lists predicates). Arms derived: no. The cone's per-sign gap: moot, we both give the cone departures. `over_lever` as measures: no. `held_reads`' region: no. My "fans go": answered, and I was wrong.

## For Ev

**Recommendation (likely).** A sector bound is a unit direction plus what stands behind it, never a vector scaled by a lever. One departure record serves the star, the bound-side read, the cut and the link. A curved bound's lever is its edge's far distance from the vertex, known from both sides. A lever's length can no longer be read as geometry, by type.

Terms. A *bound* is one side of a face's corner at a vertex: an edge leaving it, or a *divider* (a direction a wide corner is cut at). A *lever* is the length a tilt is multiplied by. *Reach* is how far an edge gets from the vertex. `LeverBounds` is a lever known exactly or between a bound from below and one from above. A *straddle* is the two bounds giving different signs; it refuses.

**Premise check (sure).**
- The cut already has the target shape. `boolean::fate::corner_holds` takes a unit direction, a both-sided reach read from the vertex, and a bend. It reads its own reach because the star's is a diameter from below.
- One concept has four forms: the star's bound, the cut's `Leaving`, `validate::Departure` (the vector at both bounds), and the link's scaled vectors. Two predicates answer "is this direction inside that corner", levered differently.
- The packing is forced from below: `geom_brep::sector_shape` asks for scaled chords and takes its arm from their norms.
- The number passed to the conic bend reading is an arc length for a witness, not a lever.

**The final state.**
1. *One departure record, in `topo`*, built by one door from the body:
   - a unit direction;
   - a reach as `LeverBounds` from the vertex. A line: its chord, exact. A circular arc: exact. An ellipse, spiric or spline: its farthest sampled point from below, its cover's far distance from above (the read the cut uses);
   - what stands behind it: a line's far vertex; a curved edge's bend and arc; a divider's face section and its corner's arm.
2. *A corner's arm* is the lesser of its two edges' reaches, at each bound, exact where both are (`LeverBounds::min`, as the cut does it). It is derived where the two bounds are held. `SectorShape.arm`, `FullTurn.arm`, `ApexShape.arm`, `star::Sector.arm` and `metering_arm` go.
3. *Predicates take unit directions and an arm and decide through one both-bounds door*: one reading where exact, two that must agree otherwise. A site reads one bound only by writing `.below()` or `.above()`. For a definite sign this is the read from below; for a Zero, from above. So the default is the README's rule with no site choosing.
   - `sector_shape` returns only the shape. Between distinct edges an undecided convexity falls to a subdivision that is sound at every angle. At a slit's tip the Zero is acted on (it reads the whole turn), so there it must be the agreed Zero, not a straddle.
4. *Points.* A point the sector code places is a vertex or a point of the edge.
   - The conic bend witness sits at an arc length the bend record bounds off the curve itself.
   - A tie's witness is chosen first, from lengths of geometry (the arc bound, the face's radii). Its lever is then read from it.
   - The split's `p + along` becomes a point of the shared edge.
5. *The link.* The boolean's gathered cone takes departures; a line's ray is still its far vertex. The link's face-tie fan stays a named construction, a polygon past the face's disc, sized by the face cover's far distance read as a length.

**Worked example (sure).** An ellipse with semi-axes 10 and 1, an edge leaving the end of the minor axis and running nearly all the way round. Its diameter is 20. Its far distance from the vertex is about 10.05. Today's lever is a lower bound on 20, on the side that claims a definite sign: a tilt between ε/20 and ε/10 is called definite where the edge reads in band.

**The other records (likely).**
- `PlannedRun.order`: keeps a lever, as bounds; its readers all read at the pierce point. The ball built from it is a different thing, a region for the carrier ladder, and is built from the two faces' covers.
- editor-core's carrier reach: holds its region. `Leg::arm`, `SectionOval::extent`: methods of geometry the record already holds.
- `Margin::over_lever`: not a records question. Its divisors are perimeters, areas, powers and speeds. Each call is first classed as a tilt's arm or a measure.

**The rule you approved.** It decides every record above. It stays two questions, with two things beside it:
- "May hold the lever" means as its bounds, read from the pinned point. One record's readers act on different signs.
- `Lever::length` is reachable only from a margin, by type. The sector bound passed both questions because its lever was multiplied into a direction; no question about fields reaches that.
- I would not make this a third question ending "otherwise the record holds the region". The conic bend needed an arc length, which is neither a lever nor a region.

**What flows where.** `geom_brep::sector_shape` sees less. The star comes to read each edge's cover from the vertex; the cut already reads the same cover there. The link sees departures in place of fabricated points. No spline carrier travels with a bound. The pierce read takes two faces' covers; it already reads those faces. I see no odd flow (likely).

**Faithful, and yours.**
- Faithful to PR 7141: the reach from the vertex; both bounds; arms derived where their source is held; the records by the two questions.
- Yours:
  1. *The gate*: `Lever::length` only from a margin, by type. Recommend yes.
  2. *One README sentence on a corner*: "a turn between two directions is levered at the lesser of their reaches, from each side." Recommend yes. The tree does it; no ratified text says it.
  3. *What a sector's Zero governs*, which sets a divider's lever from above. Either the neighbourhood (the corner's arm, exact on polyhedra), or the face's reach from the vertex (sounder if a tie is read as two faces coinciding; refuses many more). Recommend the neighbourhood, with any reader that claims the whole face reading the face itself, as the link's fan does. Unsure.
- Independent of PR 7619: every lever here is read from a point. `sector_shape` taking its arm as an argument leaves the edge-wedge caller free.

**To measure or read first.**
- Old lever against new bounds per carrier kind, over the boolean, split, census and link-fuzz suites. Judge: lines do not move; every flip is listed with its margin against the band; a valid class that newly refuses is reported to you.
- Straddles by predicate name, under both ends of item 3.
- Item 3 is settled by a reading, not a count: whether any consumer acts on a sector's `On` as a coincidence of faces without the glue door's whole-face verdict.
- The gate: the roughly 90 typed uses of `Lever::length`, classed. Judge: each is a margin, a diagnostic, or a finding.
- Each surviving `.below()` or `.above()` has a row that goes red when swapped.

## For the orchestrator

**Your list of differences, corrected.**
- *The cone*: no difference left. Both give the boolean's gathered cone departures. On the face-tie fan I moved to A.
- *Which bound*: the outcomes coincide wherever every acting verdict is a definite sign. The difference is who is trusted to know that. A's list is, in my design, the list of `.below()` opt-outs, and one entry on it is wrong at slit tips.
- *Where the type lives*: I moved to `topo`.
- *The rule*: we agree on the gate. We differ on packaging (a third question, or the type's contract plus "as its bounds").
- *Missed*: whether the departure holds its reach (mine) or the curve with the reach as a method (A's). This is A's "one new flow".

**Agreed sentences, for comparison.**
- "A sector bound is a unit direction plus what stands behind it, never a vector scaled by a lever."
- "A curved bound's lever is its edge's far distance from the vertex, read at both bounds; a line's is its chord, exact."
- "The conic bend witness is placed at an arc length the bend record bounds off the curve; it takes no lever."
- "A tie's witness is chosen first and its lever read from it."
- "The link's face-tie fan is a named construction sized by the face cover's far distance, read as a length."
- "`Lever::length` is reachable only from a margin."

**What would settle each remaining difference.**
- *Arm from above*: count line-only corners with unequal edges whose both-bounds reads straddle under `[lesser, greater]`. Under `LeverBounds::min` the count is zero by construction. Any is a newly refusing polyhedral class.
- *Divider*: read `boolean::recl`, `fan::Coincident` and `PairRecord::coplanar` ("the pair lies in one plane") for an `On` acted on without `verified_one_carrier`. I did not trace it.
- *Records held or derived*: list the readers of `SectorShape.arm` that hold neither bounds nor arm. I found none: `star::walk`, `census`, and `dihedral_side`, which reads only `bisector`.
- *Reach held or method*: whether `cover::edge_cover_linked` differs from `Curve3::cover_over` on the span. Its doc says it is hulled with the edge's ends; I did not diff the two.
- *`PlannedRun`*: `held_reads`, the lines after its `Unsourced lever` comment.
- *`over_lever`*: the grep above.

**Corrections to my first report.**
- "`fans` … and the fan rim go" conflated `boolean::sectors::fans` (tangent rays, which go) with `link::fan` (the cleared disc's rim, which stays).
- "A straddling convexity falls to the subdivision rung, sound at every angle" holds for `Distinct` bounds only. The unit must keep rung 2's agreed Zero apart from its straddle so `reads_slit_tip_turn` cannot take a straddle for a turn.
- I put the type in `geom_brep`.

**Checked in A's report and found right.**
- `apex_extrema_inside` acts on its Zero (`!= Sign::Positive` returns no extremum) at a from-below arm.
- `sector_face` builds a scaled chord (`ConicDeparture::of(..).dir`) for `chart_normal_beside`; `dihedral_side` then acts only on `bisector.is_none()`.
- `fate::leaving` calls `star::departure` and discards its lever.

**Not checked.**
- `recl` and `fan::Coincident`.
- Whether the gate is expressible across crates. `geom_brep`'s extent home composes levers and Rust has no friend crates, so it may need `Lever` combinators in `geom-core` and a diagnostics exit.
- The link's tangent pieces beyond `FaceSector::tangent` and `second_order::cusp`.
- Nothing was built or run.

**Defects, as before, plus one.**
- `corner_shape` and `apex_corner_shape` act on a Zero read at a from-below arm at slit tips. Benign while slit-tip edges coincide by construction, but it is an acted-on Zero at the wrong bound. Belongs with the unit.
- Still standing: `second_order::reach` reads a fabricated point through the point door; `link::fan`'s `1.01` slack; `wedge_holds_named` and `corner_holds` are two implementations of one predicate; `held_reads`' ball under-states what `one_carrier` consumes.
