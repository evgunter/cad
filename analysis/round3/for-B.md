# Round 3

Both round-2 reports are in. The other designer's round-2 report is below, WHOLE and as delivered. You are designer B; it is designer A's. The same rules hold: read-only, the same tree, `docs/prompts/designer.md` binds you, no helper on another model.

Round 2 moved both of you, and on several points you CROSSED: each took the other's first position. That is not agreement. On each crossed point one of the two positions is better supported by the tree; find which by reading, not by deferring again. My reading of where round 2 leaves you (a hypothesis; correct it):

**Agreed now** (say so only if it is true of your position): the departure holds a unit direction and its reach as a field, read once at both bounds from the vertex, and does not hold the curve; a corner's arm is the lesser of the two reaches at each bound (`LeverBounds::min`); the three arm records and `star::Sector.arm` as a stored field go, the arm derived where the two bounds are held; one both-bounds door is the default and a single bound is a visible opt-out with a swap row; `PlannedRun.order` keeps a lever as bounds and the pierce's ball is built apart from the two faces' covers; `Leg::arm` and `SectionOval::extent` are methods; editor-core's carrier reach holds its region; `Margin::over_lever`'s calls are classed first; `conic_bend_side` loses `extent`; independent of PR 7619.

**Crossed, to settle by the tree:**
1. **The link's face-tie fan** (`link::fan`, `FaceSector::cleared`): A's round 2 says the fans go and the cone reads rays throughout (the side and ahead readings are a sine times a reach); B's round 2 says the face-tie fan stays a named construction, because its rim is an enclosure past the farthest the face stands from the point, sized by the face cover's far distance. Which readers of the cleared face's rim use it as an enclosure of the face and which as a direction at a reach? Read `FaceSector::cleared`, `reads_over`, `closes` and the parity reading's centroids, which both of you left unread, far enough to say.
2. **Where the departure type lives**: A's round 2 says `geom_brep` (nothing in it is `topo`'s, and it replaces `ConicDeparture`); B's round 2 says `topo` (its constructor needs the body's cover and far vertex, and every consumer is in `topo`). What does `geom_brep::sector_shape` take in each, and which crates' code constructs one?
3. **How "a lever's length is not geometry" is held**: A's round 2 says a README sentence plus a typed count (`clippy::disallowed-methods`, a reason at each kept site), not visibility, because about 90 typed references are margin arithmetic in other crates; B's round 2 says `Lever::length` reachable only from a margin by type, and notes it may need combinators in `geom-core` and a diagnostics exit because Rust has no friend crates. Read enough of those references to say which is buildable and which final state is better, and what, if anything, of it is Ev's to decide rather than code.
4. **The tie's witness**: both now say the point comes first from lengths of geometry and its lever is read from it. Say it in one sentence you both could sign, and say whether the README sentence (if one goes to Ev) needs an exception for it.

**Open on both sides:**
5. **`sector_shape` at a slit's tip**: B's round 2 finds `corner_shape` and `apex_corner_shape` act on the Zero at `CornerBounds::SlitTip` (`reads_slit_tip_turn` reads a whole turn), so an agreed Zero must be kept apart from a straddle there. A's round 2 says `sector_shape` needs no opt-out. Check it.
6. **A divider's lever from above**: both recommend the corner's arm; B leaves it to Ev pending one reading (whether any consumer acts on a sector's `On` as a coincidence of faces without the glue door's whole-face verdict: `boolean::recl`, `fan::Coincident`, `PairRecord::coplanar`). Make that reading now and say whether anything is left for Ev.
7. **The text for Ev.** Write the exact README sentences you would put to Ev (to `crates/geom-core/README.md`, "Float rounding", and where in it), and the exact wording of the clarification to the records rule of PR 7141. Ev's feedback on an earlier fork of this track: ratified text says what a thing IS, not a list of past misuses, and a sentence that alludes to an error nobody would naturally make again is confusing. Keep to that.

Send, as your final message, at most 150 lines: (1) for each numbered point, where you stand and the reading that puts you there; (2) your position now as a full `## For Ev` section that replaces your last, and a `## For the orchestrator` section. If you now agree on a point, use the same words you would put to Ev, so the two texts can be compared sentence by sentence.

---

# Designer A's round-2 report

## 1. What moved me and what did not

**Moved, each checked in the tree:**
- **A corner's arm from above is the lesser of the two reaches' upper bounds, not the greater.**
  - `LeverBounds::min` (`geom_brep::extent`) is "the bounds on the lesser of two lengths": `[min(below), min(above)]`.
  - The lever is the lesser reach (`boolean::sectors::reading_lever`, `fate::corner_holds`). Its upper bound is therefore the lesser of the upper bounds.
  - My "greater upper bound" bounded the union of the two edges, which is a different quantity. I was wrong.
- **The three arm records are deleted, not kept as bounds.**
  - Once `sector_shape` takes its arm as an argument, `SectorShape.arm`, `FullTurn.arm` and `ApexShape.arm` would only hand the input back.
  - Every caller already holds both bounds: `star::walk` has the two departures, `census` passes `e.len`, and `chart_normal_beside` ignores the arm.
  - `star::Sector.arm` can be a method over the sector's two bounds. A sub-sector bounded by a divider takes the corner's arm, because the divider carries it and it is the lesser.
- **A divider's lever is its corner's arm: the lesser of the corner's two edges' reaches, as bounds.**
  - My face-reach alternative is withdrawn. Nothing in the tree reads a divider at the face's reach.
  - It would also refuse widely, as the other report says.
- **`PlannedRun.order` keeps its lever as bounds, and the pierce's ball is built separately, from the two faces' covers.**
  - `ConsumedExtent`'s doc (`carrier_eq.rs`) requires "a ball enclosing both faces".
  - `vtxfac::held_reads` today builds it from the shortest arm about `p`. That ball is in the under-stating direction.
  - My "ball about the vertex from the bounds' covers" was the wrong region.
- **`Leg::arm` becomes a method of the leg's own geometry, not a held region.**
  - The stored-reach issue says "the leg's own geometry is what the lever should be read from". That is question one of the rule.
- **`Margin::over_lever` needs each call classed first; it is not a pure typing change.**
  - Its doc's divisors are a perimeter, a radius, an area, a straddle height and powers of a lever (`quartic.rs`: `lever.powi(11)`).
  - There are about 52 calls outside `geom-core`, and many are not tilt arms.
- **A curved departure holds its reach as a field.** Question one asks whether the record holds *what the lever is read from*. That is the edge's cover read from the vertex, which no bound holds. The carrier it could be rebuilt from is not that, and re-sampling the cover for every reader is the wrong way round. So question two applies, and the field is allowed.
- **Both bounds by default, with visible opt-outs.**
  - The track's given is that units repeatedly picked the wrong bound from a description.
  - A both-bounds door makes the safe reading need no judgement. Only an opt-out needs its consumer read, and then a row that goes red when the bound is swapped.
  - My per-predicate list survives as the list of opt-outs to check. These are the sites where a Zero leads to a sound path that does not refuse, but a straddle would be mapped to a refusal: `apex_over_half_turn` and `tangent_extrema_inside`'s callers.
  - `sector_shape` needs no opt-out. Its `Ok(Zero) | Err(_)` already falls through to the subdivision, which is sound at every angle.
- **The gathered cone is fed rays, and the fans go.**
  - `link::wedge_distance` and `ahead_distance` draw the probe in to the ray's reach (`within`) and read a point's distance from a plane. That is a sine times the reach, not a position.
  - So a wedge of two departures reads the same thing, and the rim's hand-written `1.01` slack goes with `fan`.
  - My "keep the face-tie fan as a named construction" is withdrawn. What the cone's other readers (the parity reading, `reads_over`) do with ray points is unread by both of us, so this stays unsure until measured.
- **The departure type lives in `geom_brep`, beside `LeverBounds` and in place of `ConicDeparture`.** Nothing in it is `topo`'s: the half-edge key stays on `star::Bound`. `sector_shape` itself takes `(UnitVec3, UnitVec3, LeverBounds)`, not the record, because `census` passes sheet directions that are not departures.
- **PR 7619.** The other report shows that `census` hands `edge_wedge_shape` the wedge's arm `e.len`, which is a turn about an edge. That is 7619's lever. Taking the arm as an argument is what keeps this design independent of that fork.

**Not moved:**
- **No separate "a ray is fixed by its point" decision goes to Ev.** The phrase is a code doc in `link.rs` (lines 781 and 923), not ratified text: `topo/README.md` has no such clause. Changing it is a refactor that falls under Ev's records condition, not a ratified decision.
- **The rule itself.**
  - Both reports diagnose it the same way: it sees lever *fields*, not products, and "may hold the lever" has to mean its bounds.
  - I drop my "third question" framing in favour of two clarifications of the approved wording, which is the same content.
  - On the gate I move partway. I no longer propose making `Lever::length` private by visibility: about 90 typed references are margin arithmetic in other crates (`a-levers-length-used-outside-a-margin-has-no-gate.md`). The typed count that issue proposes (`clippy::disallowed-methods` with a reason at each kept site) is the enforcement. It is code, not a decision for Ev.
- **The witness point.**
  - The other report keeps `witnessed(leading, arm.length())` and documents that any point of the edge is sound.
  - Under the sentence both reports put to Ev ("nothing is placed by a lever's length"), that sentence would bind its own exception.
  - So the abscissa should come from geometry: the edge's reach from below read as a length off its cover, the same number from a geometry door. Or it is named in the sentence as its one exception. Either settles it. I lean to the geometry door.

**My first-report arguments the other report answers:** all but the witness point. It agrees with me on:
- the lever is the reach from the vertex, at both bounds;
- `conic_bend_side` drops `extent`;
- `splitting::rules`' `p + along` becomes a point of the edge;
- `second_order`'s residue becomes a levered tilt;
- `star::walk` comes to read the edges' covers, the same read `fate` already makes;
- PR 7619 is independent.

## For Ev

**Recommendation (likely).** A sector bound becomes one record, a *departure*. It holds a unit direction and the edge's reach from the vertex, known from both sides, as two separate things. Every sector predicate takes unit directions and a reach. No lever's length is used to scale a vector or to place a point.

**Terms.**
- **Bound:** one side of a face's corner at a vertex. It is either an edge leaving the vertex or a *divider*, a direction the walk cuts a wide corner at.
- **Lever:** the length a tilt is multiplied by to give a displacement.
- **Reach:** how far an edge gets from the vertex.
- **`LeverBounds`** (exists today): a lever known exactly, or between a bound from below and a bound from above.
- **Straddle:** the two bounds giving different signs. A straddle refuses.

**Premise (sure).**
- Half of this exists already. `boolean::fate::corner_holds` takes a unit direction, a `LeverBounds` reach off the edge's cover from the vertex, and a bend, and reads every sign at both bounds.
- The defect is a concept the tree holds in four forms, with different levers:
  - the star's bound, which today packs a direction with a lever that is the edge's diameter read from below;
  - the cut's `Leaving`;
  - `validate`'s scaled departures;
  - the link's scaled vectors.
- `geom_brep::sector_shape` forces the packing: it takes two scaled chords and reads its arm from their lengths.

**Worked example (sure).** Take a nearly closed flat ellipse, semi-axes `a` much larger than `b`, leaving a vertex at an end of its minor axis.
- Its diameter is `2a`. Its far distance from the vertex is about `a`.
- Today's lever is a lower bound on `2a`, so a tilt between `ε/2a` and `ε/a` is called definite where the edge reads within the band.
- Today's "ray point" sits `2a` along the tangent, about `a` past anything the body has.

**The final state.**
1. **One departure type, in `geom_brep`.** It has a unit direction, a reach and what stands behind it.
   - **Line:** its far vertex; the reach is the chord, exact.
   - **Curved edge:** its bend and its arc (carrier and span). The reach is read once from the edge's cover from the vertex, both bounds. This is exact on a circular arc, which also lengthens arcs past a half turn that are read short today.
   - **Divider:** its face's section; its reach is its corner's arm (item 3).
   - There is no `ray()` and no `point()`. `metering_arm` and `edge_extent_inf` leave the sector machinery.
2. **The one number that was three things separates.**
   - The lever for first-order signs is the reach.
   - The arc length the conic-against-plane witness needs is a length of the curve, held by the bend record (`BendBounds::Planar.arc`). `conic_bend_side` loses its `extent` argument.
   - The scale for a ray is gone.
3. **A corner's arm** is the lesser of its two bounds' reaches. As bounds, each bound of the arm is the lesser of the two (`LeverBounds::min`). Each direction is fixed only to the band over its own length, so the angle between two of them is resolved only at the shorter.
   - `sector_shape` takes two unit directions and this arm, and returns the shape alone: convex, split at a direction, or a full turn.
   - `SectorShape.arm`, `FullTurn.arm` and `ApexShape.arm` are deleted: the caller holds both bounds.
   - `star::Sector.arm` becomes a method over its bounds.
4. **One both-bounds door decides every sector predicate.**
   - It reads once where the lever is exact, and at both bounds otherwise, deciding only where they agree.
   - A site reads one bound only by writing `.below()` or `.above()`, and each such site has a row that goes red when the bound is swapped.
5. **Points.** Every point placed is a vertex or a point of an edge's curve.
   - The split's `p + along` becomes a point of the shared edge, which lies on both faces. Today, for a curved edge, it lies on neither.
   - The tie witness reads the edge's point at the edge's reach from below, taken as a length off its cover.
6. **The gathered cone** reads rays: a wedge is two departures, and a face held whole is its directions, levered at that face's reach from the point. Its side and ahead readings are already a sine times a reach, so they go through the door in item 4. The fans and the rim slack go. (unsure until the measurement below)

**The records.** These follow from the two questions you approved (likely):
- `BoundSupport::Conic.extent` is replaced by the departure's reach.
- `Bisector.arm` becomes the corner's arm, as bounds.
- `PlannedRun.order` keeps a lever, as bounds: its readers all read at the pierce point. The pierce's ball is a separate thing, built from the two faces' covers, because the region it stands for must enclose both faces.
- editor-core's carrier reach holds its region: its readers read from different pivots.
- `Leg::arm` and `SectionOval::extent` become methods of the geometry those records hold.
- `Margin::over_lever` is not a records question. Each of its ~52 calls is first classed as a tilt's arm (typed `Lever`) or a measure (left as it is).

**Is the rule enough?** For every record above, yes. The sector bound shows two places where its wording is short, not a wrong question:
- the rule sees lever *fields*, and this lever was stored multiplied into a direction;
- "may hold the lever" has to mean the lever's bounds, because one record's readers act on different signs.

**What flows where.**
- `geom_brep::sector_shape` sees less: no chords.
- `star::walk` comes to read each edge's cover from the vertex, which the cut already reads from the same vertex.
- The cone sees directions and reaches instead of points.
- `held_reads` reads the two faces' covers, faces it already reads.
- No module learns a region it had no business with.

**Faithful to PR 7141 (sure):**
- the reach from the vertex as the lever (the README's "far distance from that point");
- both bounds read;
- the arm records deleted;
- the item-3 records by the two questions.

**Yours to decide, with my recommendation:**
1. *Add two sentences to `geom-core/README.md`.* Recommend yes. Today only `Lever::length`'s doc says the first; nothing ratified does, and no ratified clause states the second.
   - The first: "A lever's length is read only by a margin. Nothing is placed or scaled by it, and a record that needs a direction and a lever holds both."
   - The second: "A turn between two directions is levered at the lesser of their reaches, and a divider at its corner's."
2. *Read "may hold the lever" in your records rule as "holds the lever's bounds its readers need".* Recommend yes.

**PR 7619:** independent. These levers are read from a point, not about a line. `sector_shape` taking its arm as an argument lets the edge-wedge caller pass whatever that fork decides.

**To measure first, and against what.** The judge in every case: a verdict that moves toward refusing is accepted; a class of valid bodies that newly refuses is reported to you before landing.
- **Old lever against new bounds**, per carrier kind, over the boolean, split, census and link suites, listing every changed verdict. Lines must not move. Circular arcs past a half turn lengthen.
- **The straddle rate** on ellipse, spiric and spline bounds and on the dividers beside them. It is zero on lines and circles by construction.
- **The cone as rays against today's points.** Every decided sign must be equal wherever the lever is exact. Any difference marks a reader that used a point as a position, and is read before item 6 is claimed.
- **`conic_bend_side` with the curve's own arc bound.** No circle row moves; the ellipse rows are listed.
- **Every surviving `.below()` / `.above()`** has its swap row red.

## For the orchestrator

- **Agreed with the other designer, in substance:** every point above except the witness abscissa. On that, either its geometry door or naming it as an exception in README sentence 1 settles it; it is a question of wording, not one to measure.
- **Still unread by both of us:** the gathered cone's parity readers (`reads_over`, `closes`, centroids). The rays recommendation rests on them; the third measurement above settles it.
- **Defects for the unit to file** (the other report found most of them; I checked them against the tree):
  - `vtxfac::held_reads` builds its `ConsumedExtent` as `ExtentBall::unsourced(p, shortest arm)`, against that type's "ball enclosing both faces".
  - `link::fan`'s rim carries `reach · 1.01 / cos(π/8)`, hand-written slack.
  - `conic_bend_side`'s `Unbounded` arm decides `0 − |residue|` under `decide_positive`: a refusal spelled as a decision.
  - `link::second_order::reach` feeds a fabricated point to `ExtentBall::point(..).lever_from`.
  - `validate::wedge_holds_named` (with `Margin::of(n·(x×y)/|x|)`, no `Lever`) and `fate::corner_holds` implement one predicate with two levers. One should go; `corner_holds` is the fuller.
  - `apex_extrema_inside` acts on Zero yet is levered at a bare `s.arm.length()`.
  - The row `sector-bounds-pack-a-direction-and-its-lever-into-one-vector` is cited and absent.
- **Where the doors move:** `decide_agreed` / `decide_levered` / `decide_at_lever` move from `topo::validate` down beside `LeverBounds` in `geom_brep`, because `sector_shape` needs them.
- **Not checked:** whether `cover::edge_reach_bounds` ever returns `None` for a certified edge; the `over_lever` calls one by one; `SectionOval`'s fields; whether any row reaches `rules`' curved edge at a cone's apex.
- **Nothing was built or run.**
