## For Ev

**Recommendation (likely).** A sector bound carries a **unit direction plus what stands behind it**, and never a vector scaled by a lever. Each predicate takes unit directions and reads the lever its own verdict needs from that backing. The bound's reach is a method, never a stored scalar. Every point the machinery places is either a point of the body or a named construction whose size is read from geometry. The boolean cut already works this way (`boolean::fate::Leaving`), so this extends a form the tree already has rather than inventing one. The two-question rule you approved is right about *where* a lever lives, but on its own it lets this defect through. I recommend adding one test to it (§3).

**1. The premise, corrected (sure).**
- `corner_holds` does not read the packed vector. Its `edge.ray()` is `fate::Leaving::ray`, which is a unit direction, the edge's reach from the vertex read at both bounds (`cover::edge_reach_bounds`), and its bend. Every sign it reads is read at both bounds. That is the target shape.
- `fate::leaving` calls `star::departure` and throws away the star's lever. So one bound has two levers in the tree today: the cut's (honest) and the star's (the edge's diameter, read from below).
- `star::Bound.dir` is already a unit vector. The packing lives in a handful of places:
  - `Orbit::departures`, `Bound::ray`, `BoundSupport::point` and `ConicDeparture.dir`;
  - the chord that `sector_face` builds for `chart_normal_beside`;
  - the link's own way of passing departures around ("the point it reaches", `link::second_order::reach`);
  - `validate::Departure::along`, which is honest about its bounds but still packs them.
- `conic_bend_side`'s `extent` argument is not a lever. It is an arc length from below, used to place the witness point, and `BendBounds::Planar.arc` already carries one. On `Unbounded` the lift is zero, so `extent` makes no difference there.
- `bound_side::read` acts on every sign: its Zero is a `Tie` that consumers key on. Yet a curved bound or a divider is read there at a single lever.
- `apex_extrema_inside` acts on its Zero: "no extremum inside" means the sector is not cut. Yet it is levered at the bare `s.arm.length()`, a length that is really a lever read from below.

**2. The final state.**

*What a bound is.* There is one departure type, shared by the star, `bound_side`, the cut and the link: `Departure { dir: UnitVec3, behind: Behind }`. `fate::Leaving` folds into it, adding only a secant variant. `Behind` is one of:
- **Line** `{ base, far }`. Its reach is `|far − base|`, exact. This is today's `Chord`.
- **Curve** `{ the edge's curve and span, which way it is walked, bend }`. Its reach is a method: the edge's cover read from the vertex, at both bounds. For a circle or an ellipse, the witness arc is read off the same curve. `ConicDeparture` becomes just `{ tangent, bend }`. This replaces `Conic.extent` and `ConicArc` as separate fields.
- **Divider** `{ arm: CornerArm, bend }`. Its lever is its sector's arm: a divider has no edge behind it, and all its readers pin at the vertex.
- **Probe** (test only).

*What the predicates take.* They take unit directions and a `CornerArm`, which is a `LeverBounds` whose lower bound is the lesser of the two bounds' reaches from below and whose upper bound is the greater of their reaches from above. Each verdict reads the bound its action needs:
- `geom_brep::sector_shape` / `corner_shape` and `chart_normal_beside`'s convexity check read from below. Every verdict there that acts is a definite sign, and a Zero only falls through to a subdivision that is sound at every angle.
- `topo::sector_shape`'s apex window read and `apex_over_half_turn` read from below, for the same reason.
- `apex_extrema_inside` reads both bounds, because it acts on its Zero.
- `bound_side::read`'s first order for a curved bound or a divider reads both bounds and acts where they agree. This is `validate::decide_levered`, the same read the cut uses. A line bound stays an exact distance, bit for bit.
- Rung 0 (a chord length that is not a number) moves to where the direction is normalised, and to the lever's own construction.
- `metering_arm` and its pin are deleted.

*Where points are placed, and from what.*
- **The `conic_bend_side` witness:** the edge's point at arc length `min(arc, lifts_to)`, where `arc` is the carrier's own lower bound on arc length (on an ellipse, at least its chord). `extent` is dropped.
- **`Tie::witnessed`:** the point is chosen first, as the edge's own far point or its point at the lift length. The parting's lever is then read from that point (a two-point read). Today it goes the other way: the lever is read first and a point placed at its length.
- **`splitting::rules`' `p + along`:** replaced by a point of the edge off the apex. The cone's normal there is read by direction (`normal_along`), which needs no point.
- **The link** (`Cone::gathered`, `boolean::sectors::fans`, `vtxfac`, `recl`, `second_order`) takes `Departure`s, not points:
  - A line's ray is its far vertex, unchanged.
  - A curved bound's side of a plane is read as the apex's offset plus `levered(d̂·n, reach)` at both bounds.
  - `second_order`'s residue `Distance::of(p + t̂c·l, p + t̂a·l, m)` is exactly that levered tilt, re-posed.
  - The face-tie fan stays a named construction: a polygon around the face's disc about `p`. Its radius is the face cover's far distance, read as a length from geometry, not a `Lever` read back.
- **`validate::Departure::along`:** becomes `(UnitVec3, LeverBounds)`, with its cross products read through `decide_levered`.

**3. The records.** The rule you approved asks two questions:
- **Q1:** does the record already hold what the lever is read from? If so, the lever is a method.
- **Q2:** if not, do all its readers pin their data at one point? If so, the record may hold the lever; if not, it holds its region.

*How the sector records come out (likely):*
- `SectorShape.arm`, `FullTurn.arm`, `ApexShape.arm` and `star::Sector.arm` become `CornerArm`. Under Q2 a record may hold a lever: all readers pin at the vertex. Today they hold a bare `T`.
- `BoundSupport::Conic.extent` goes. Under Q1 it becomes a method, because the record holds the curve.
- `Bisector.arm` stays under Q2.

*Item 3's records (likely):*
- **`PlannedRun.order`** holds its region: an `ExtentBall` about the vertex from the two bounds' covers. The pierce ball is consumed as a region, so it is geometry, and `ExtentBall::unsourced` goes.
- **`Leg::arm` and editor-core's `Carrier` reach** hold their region (`ExtentBall::of_cover_from`), as their issue already argues. Their readers read from different pivots.
- **`SectionOval::extent`** becomes a method under Q1, if the oval's half-widths are held (unsure: I did not read it).
- **`Margin::over_lever` / `over_lever_down`** take a `Lever<T>`. That is a typing change, not a records question.

**Is the rule asking the wrong thing?** Not wrong, but incomplete (likely). The sector bound passes both questions today: the record doesn't hold its curve, and every reader pins at the vertex. It still fails in three ways:
- the lever is the wrong measure (a diameter, not the far distance from the vertex);
- it is a single bound held for readers that need both;
- its length is read back as geometry.

I recommend adding **Q3: the held lever is the far distance from the pinned point, holds every bound its readers need, and no reader reads its length as geometry. Otherwise the record holds the region.** Enforce the last part by type: make `Lever::length` reachable only from `Margin`. That turns the ratified "the exit, not an entrance" into a compile error, and that rule is what the PR 7628 sites were breaking. **Yours to decide:** whether to adopt Q3, and the type-level gate.

**4. What flows where (your condition on records).**
- `bound_side`, the star's readers and the link gain a view of a spline or spiric edge's carrier and span. Today they see only the edge's tangent and bend; they already hold circle and ellipse carriers through `ConicArc`. This is read-only and inside `topo`, through `geom_brep`'s cover door. It is the one new flow.
- `sector_shape` (geom-brep) sees less: unit directions and a lever, with no scaled vectors.
- The link sees less: departures instead of points it was told were "reaches".
- No module comes to read a region it had no business knowing.

**5. Faithful elaborations, and what is yours.**
- **Faithful (sure):**
  - the curved bound's lever is its reach from the vertex (the README's "far distance from the point the datum is read at");
  - both bounds wherever a verdict acts on every sign;
  - the witness is an arc length, not a lever;
  - the records placed per Q1 and Q2.
- **Yours:**
  - (a) **Q3 and the gate.** I recommend both.
  - (b) **The lesser of the two reaches is a corner's lever from below.** The tree does this already (rung 1 asks "whether a corner's edges are long enough"; the cut uses "the shorter of the two directions' own reaches"). The README doesn't say it, and its "far distance of the consumed extent" would say the greater. The principle underneath: a turn between two directions is resolved only at the shorter, since each direction is fixed by its own extent. I recommend stating it in `geom-core/README.md`. That is a change to a design page, so it needs you.
  - (c) **A divider's lever from above:** the union of its two bounds' reaches (recommended, unsure), or the face's reach from the vertex. The face's reach is sounder if a divider's side verdict governs the whole sector of the face, but it is much looser. Measure first.
- **Independent of PR 7619 (likely).** Every turn here is about a point, the vertex, not about an edge's line. The wedge levers are untouched. If 7619 rules that curved edges need a turn lever known from both sides, that fits this design.

**Worked example (sure).** Take an ellipse with semi-axes 10 and 1, and an edge leaving the end of the minor axis that runs almost all the way round. The edge's diameter is about 20, between the two ends of the major axis, and `edge_extent_inf` samples it as such. The edge's far distance from the vertex is only about 10.05. So today the curved bound's lever from below overstates the true lever about twofold: a definite sign there can come from a datum the honest lever reads within the band. A near-tangent decision amplifies the error further.

**6. Measure before this goes to you.** Judge each against your conditions: a verdict that moves toward refusing is accepted, and a valid class that newly refuses is reported.
- (1) For every site in §2: how often the old lever and the new bound differ, and which verdicts flip (golden bodies, the `link` fuzz, the slow suites). Each from-below flip must be toward refusal; any other flip is a finding.
- (2) The new straddle refusals where both bounds are now read (`bound_side` first order, `apex_extrema_inside`, the link), counted by predicate name, under both choices in 5(c).
- (3) The `conic_bend` witness without `extent`: rows where `extent` exceeded `arc` (ellipses).
- (4) Whether any row reaches `rules`' curved edge at a cone's apex.
- (5) For every flip, its margin against the band, not the ratio of the two lengths.

## For the orchestrator

- **Errors in the brief:**
  - `corner_holds` takes `fate::Leaving::ray`, not `star::Bound::ray`. `fate`/`overlap` are already honest; they are the precedent, not a site.
  - `conic_bend_side(extent.length())` is an arc length for the witness and is redundant with `BendBounds::Planar.arc`.
  - `star::Bound.dir` is unit. The packing is in `Orbit::departures`, `ray()` and `point()`.
- **Found off the brief, for the unit:**
  - `apex_extrema_inside` acts on Zero yet is levered at a bare arm length read from below.
  - `bound_side::first_order_as` acts on every sign at one lever.
  - `splitting::rules::edge_wedge`'s `p + along` hands `classify_dihedral` a point on the tangent ray (reachability unknown).
  - `fate::leaving` discards the star's lever.
- **Not read or checked:**
  - `vtxfac::PlannedRun` beyond its field;
  - `SectionOval` beyond its doc;
  - `link::fan`'s callers beyond the two shown;
  - whether `Lever::of_consumed_extent` already refuses non-finite lengths, which rung 0's move relies on;
  - the exact `LeverBounds` API names.
- **Assumed:** the ellipse example's `edge_extent_inf` reads about 20 from its 16 samples. It was not computed.
- **Filing:** the `sector-bounds-pack-a-direction-and-its-lever-into-one-vector` row cited by PR 7628 is not in this tree, and should be filed with the findings above. `a-levers-length-used-outside-a-margin-has-no-gate.md` is the home for the type-level gate in 3/Q3.
- **Size:** the link is the largest piece and could be its own unit after the star, `bound_side` and `sector_shape`.
