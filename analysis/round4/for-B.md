# Round 4

Both round-3 reports are in. The other designer's round-3 report is below, WHOLE and as delivered. You are designer B; it is designer A's. The same rules hold: read-only, the same tree, `docs/prompts/designer.md` binds you, no helper on another model.

My reading of where round 3 leaves you (a hypothesis; correct it). **Agreed:** everything on round 3's agreed list; the departure type lives in `topo` and `geom_brep::sector_shape` takes two unit directions and an arm; the link's face-tie fan stays a named construction sized from the face's cover, its hand-written slack gone; a divider is levered at its corner's arm and nothing but the sentence is Ev's there; at a slit's tip an agreed Zero is kept apart from a straddle, which refuses; a tie's witness is a point of the edge chosen from lengths of geometry and the parting is levered at that point's distance from the vertex; how "a lever's length is not geometry" is enforced is code and not Ev's; three README edits go to Ev; independent of PR 7619.

**Not yet one text. Settle each, by the tree where it can be:**
1. **The three README edits.** You each wrote them, in different words and at different places in "Float rounding". Read the other's three against the README bullets as they stand (`crates/geom-core/README.md`) and against each other, and give ONE text for each edit with its exact place, or say which of the two you would sign and why the other is worse. Test each sentence: does it say what the thing IS; is every word defined in the README where it stands; does it bind something that should not be bound (B's "read only in its product with a tilt": the curvature sagitta squares a lever; A's "read only by a margin": is a `Margin` a README term); does "a region a site needs is read from the geometry the lever was read from" restate the records clause beside it.
2. **The gathered cone's interior.** A's round 3: a ray of the gathered cone is a real point or a departure read at both bounds, the cone's readers using a ray only as a direction at its reach. B's round 3: the cone stays a picture of points shared with real-scale links (`rest::judge` and `crossing` read chord points as positions; the parity reference's centroid depends on the two reaches), fed by one door in `link` that realises each ray at its reach, at both bounds, each `Distance` deciding both. Is there a difference left in the FINAL STATE, or only in how the unit gets there? If there is one, which reader decides it?
3. **The gate's end state.** A's round 3: `Lever::length` removed, with margin combinators in `geom-core`, a report exit and a test-support exit. B's round 3: each remaining read becomes a named read or a typed parameter, and a typed count holds the raw read at zero outside the two homes, because `geom_brep`'s extent and curvature homes are another crate and compose levers. Is there a difference left, given both say it is code? One sentence each for the unit's brief.
4. **What gates the QUESTION to Ev, and what gates only the MERGE of the unit that builds it.** Both of you list measurements "first". Each needs a build (the old lever logged beside the new bounds at the star's departures; straddles by predicate). For each measurement say: could its result change any of the three README sentences or the recommendation, or does it only decide which bodies' verdicts move, which Ev's standing conditions already cover (a verdict that moves toward refusing is accepted, PR 7141; a class of valid bodies that newly refuses is reported before it lands, PR 7140)? If a measurement gates the question, say exactly what is logged, at which doors, over which suites, and against what it is judged, so that a lane can be handed it today. If none does, say so plainly, and say what the PR to Ev must state as unmeasured.

Send, as your final message, at most 120 lines: (1) the four points; (2) your `## For Ev` section, final unless the other's answer moves you, with the README edits as exact text and place; (3) `## For the orchestrator`. If you agree on a sentence, use the same words.

---

# Designer A's round-3 report

## 1. Where I stand on each point, and the reading behind it

**Agreed now, as the brief lists it, with one correction.** The departure does hold an arc (carrier and span) for a circle's or an ellipse's bend witness. It never reads its reach from that arc. The reach is a field, read once at both bounds from the vertex off the body's edge cover. Everything else on the brief's agreed list is my position too.

**1. The link's face-tie fan: B is right, and the fan stays.**
- `Link::of` reads the cleared fan's chords as segments. It builds the link's ball from `to_segment(p, wedge)`, folded with `LinkFace::clear`.
- `Link::of` also reads the link's reach as the far distance of the rim points.
- `FaceSector::cleared`'s doc says the rim stands "past the farthest the face stands from `p` … so a coincidence read there holds over the whole face". That is an enclosure, sized from `cover::face_reach_from`.
- The cone's own readers (`wedge_side`, `ahead`, `closes`, `Sector::place`, and the parity reference's centroid and `ArcReach::Out`) use a ray only as a direction at its reach. `within` draws points in to `|r − p|`; the centroid is `(a−p)+(b−p)`, carried to `q`'s distance; `ArcReach::Out` reads at the wedge's shorter reach.
- So a wedge's ray is one of two things:
  - a real point: a line's far vertex, or the cleared rim's vertex;
  - a departure, a direction with `LeverBounds`, read at both bounds.
- The boolean's tangent rays (`sectors::fans`, `line_fans`, `vtxfac`/`recl`'s `reach.point`) become departures. `link::fan` stays as a named construction. Its `1.01` slack goes, because its reach is read from the cover.

**2. Where the departure type lives: in `topo`. B is right.**
- Its builder needs the body: `cover::edge_reach_bounds(body, e, pivot)` reads `edge_cover_linked`, a cover hulled with the edge's ends, and the far vertex.
- Every consumer is in `topo`: the star, `bound_side`, `fate`, `validate`, the link, the split.
- `geom_brep::sector_shape` takes the same pieces in either design: two unit directions, a `LeverBounds` arm, a normal and the corner bounds. `census` passes it sheet directions that are no departures. Nothing in `geom_brep` builds or reads the type.
- `ConicBend` and `BendBounds` stay in `geom_brep`. The departure holds them.

**3. "A lever's length is not geometry": B's final state, enforced by type. It is buildable, and it is code.**
- I read the uses of `Lever::length` outside geom-core, about 105 lines outside tests. They fall into five kinds:
  - (a) An arm-positivity gate, `Margin::of(arm.length())` (in `enters`, `dihedral`, `battery`). This becomes one `Margin` door.
  - (b) A bare-`T` parameter that is really a lever (`regular_jet`, `parting_arm`, `chart_arm`, `irregular_reach`). Each becomes `Lever`.
  - (c) A fold with a radius (`PierceLever::within(reach.length())`, `bound_side`'s fold of `[face.lever, radius]`). Each becomes `Lever::curvature_radius` and `Lever::min`.
  - (d) A diagnostics payload (`ReadFault::Departure.arm`, `battery`'s `measured`). These take a report exit that returns a type with no arithmetic.
  - (e) A geometry use. These are this question's sites and the findings.
- Rust has no friend crates, and `link::metric` builds its departure margins in `topo`. So "only from a margin" needs four things: geom-core margin combinators for the formulas sites write today (levered, levered squared, sagitta, the curvature charge), the report exit, a test-support exit, and `length()` removed.
- With those, the bad state fails to compile, and a clippy count is no longer needed.
- None of that is for Ev to decide. The README sentence that states the principle is (item 7).

**4. The tie's witness.** The sentence I would sign:
- "A tie's witness is the edge's point at the least of the face's curvature radius, the face's curvature lever and the edge's reach from below, each read as a length of the geometry; the parting it witnesses is levered at that point's distance from the vertex."
- That distance is a two-point read, exact, so it gives one reading and no straddle.
- No point is placed at a lever's length, so the README sentence needs no exception. The witness's numbers are radii and a cover's far distance, all lengths of geometry. `bound_side::read_down` builds today's arm from exactly those (`face.lever`, `face.bend.radius()`, `reach.lever()`).

**5. `sector_shape` at a slit's tip: B is right, and I was wrong to say it needs no care.**
- At `CornerBounds::SlitTip`, `corner_shape` turns rung 3's definite cos-Positive rejection into a whole turn. It reaches rung 3 only when rung 2 is not definite, and today a straddle would get there too.
- `apex_corner_shape` turns an agreed Zero of the window chord into a whole turn (`reads_slit_tip_turn(.., Sign::Zero)`).
- So the final state reads rung 2 through the both-bounds door. Between distinct edges, Zero and straddle both fall through to the subdivision. At a slit tip, only an agreed Zero together with a definite rung 3 is the turn, and a straddle refuses.
- It is benign today: slit tips are coincident line edges (exact levers), and a wrap tip's two departures are one edge's, so their sine is exactly zero. But it should be stated by construction, not argued.

**6. A divider's lever from above: nothing is left for Ev beyond stating the convention.**
- `fan::meeting` decides `bool_faces_parallel` at `a.arm.min(b.arm)` on every sign:
  - Positive: rays.
  - Zero, glued: `Coincident::Glued`, the glue door's verdict.
  - Zero, a curved face unglued: `Coincident::Frontier`, which `recl` refuses (`CurvedBooleanUnsupported`).
  - Zero, unglued planes: `Parallel`, which refuses where the sectors overlap and otherwise only records no crossing at this vertex.
- `PairRecord::coincident` is built only from those arms. `recl` lumps only `Glued`, and its tangent descent reads each bound at its own reach.
- So no consumer acts on a sector's Zero as two faces coinciding without the glue door. A sector's Zero claims its neighbourhood only, and the corner's arm, the lesser at each bound, is the right lever from both sides. `meeting` itself takes the default both-bounds door.

**7.** The text for Ev is in the section below.

**My earlier arguments B has not answered:** none remain open. B answered the witness point by adopting it.

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
- The tree has four forms of one concept, with different levers.
- `geom_brep::sector_shape` forces the packing: it asks for scaled chords and reads its arm from their lengths.

**Worked example (sure).** Take an ellipse with semi-axes 10 and 1, and an edge leaving the end of the minor axis and running nearly all the way round.
- Its diameter is 20. Its far distance from the vertex is about 10.05.
- Today's lever is a lower bound on 20, on the side that claims a definite sign. So a tilt between ε/20 and ε/10 is called definite where the edge reads within the band.

**The final state.**
1. **One departure record, in `topo`, built by one door from the body.** It holds:
   - a unit direction;
   - a reach, as `LeverBounds` read from the vertex:
     - a line: its chord, exact;
     - a circular arc: exact;
     - an ellipse, spiric or spline: the edge's farthest sampled point from below, and its cover's far distance from above;
   - what stands behind it:
     - a line's far vertex;
     - a curved edge's bend and, for a circle or an ellipse, its arc;
     - a divider's face section and its corner's arm.
2. **A corner's arm is the lesser of its two bounds' reaches, at each bound.** It is exact where both reaches are, and derived where the two bounds are held. `SectorShape.arm`, `FullTurn.arm`, `ApexShape.arm`, the stored `star::Sector.arm` and `metering_arm` go. A divider is levered at its corner's arm: a sector's Zero claims only its neighbourhood, and no reader takes it as two faces coinciding without the glue door's verdict.
3. **Predicates take unit directions and an arm, and decide through one both-bounds door.** The door reads once where the lever is exact, and at both bounds otherwise, deciding only where they agree. A site reads one bound only by writing `.below()` or `.above()`, with a row that goes red when the bound is swapped. At a slit's tip, only an agreed Zero reads as the whole turn.
4. **Points.** Every point the sector code places is a vertex or a point of an edge.
   - The conic bend witness sits at an arc length the bend record bounds off the curve.
   - A tie's witness is the edge's point at the least of the face's curvature radius, the face's curvature lever and the edge's reach from below, each read as a length of the geometry; the parting it witnesses is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** A ray of the gathered cone is a real point (a line's far vertex) or a departure read at both bounds. The link's face-tie fan stays a named construction: a polygon past the face, sized by the face cover's far distance, read as a length.
6. **A lever's length is reachable only from a margin, by type.** Diagnostics take a report exit that does no arithmetic. This is code, not a decision for you.

**The other records (likely).**
- `PlannedRun.order` keeps a lever, as bounds: its readers all read at the pierce point. The pierce's ball is built from the two faces' covers.
- editor-core's carrier reach holds its region.
- `Leg::arm` and `SectionOval::extent` become methods of geometry their records hold.
- `Margin::over_lever`'s calls are each classed first, as a tilt's arm or a measure.

**What flows where.**
- `geom_brep::sector_shape` sees less.
- The star comes to read each edge's cover from the vertex, which the cut already reads there.
- The link's cone sees departures in place of fabricated points.
- The pierce read takes two faces' covers; it already reads those faces.
- No spline carrier travels with a bound. I see no odd flow.

**For you to ratify** (`crates/geom-core/README.md`, "Float rounding", the bullet "A verdict's lever arm", after "...and acts only where they agree."):
- *"A lever is read only by a margin: a point, a scale or a region a site needs is read from the geometry the lever was read from."*
- *"A turn between two directions read at one point is levered at the lesser of their levers from that point, at each bound; a direction with no geometry behind it, a divider of a corner, takes its corner's."*
- *The records rule of PR 7141*, its second question to read: "if not, do all its readers pin their data at one point (then it may hold the lever, as the bounds its readers act on, read from that point; else it holds its region)."
- Recommend all three. The tree does each already in places; no ratified text says them.

**Independent of PR 7619.** Every lever here is read from a point. `sector_shape` takes its arm as an argument, which leaves the edge-wedge caller free to pass whatever that fork decides.

**To measure first, and against what.** Against all of these: a verdict that moves toward refusing is accepted, and a class of valid bodies that newly refuses is reported to you before landing.
- **Old lever against new bounds**, per carrier kind, over the boolean, split, census and link-fuzz suites. Lines do not move. Every flip is listed with its margin against the band.
- **Straddles counted by predicate name.** Zero on lines and circles by construction.
- **The cone with departures against today's points.** Every decided sign must be equal where the lever is exact.
- **`conic_bend_side` with the curve's own arc bound.** No circle row moves.
- **Every surviving `.below()` / `.above()`** has its swap row red.

## For the orchestrator

- **Convergence:** on every point, the two positions now share one final state. The sentences in "For Ev" are worded to compare line by line with B's.
- **Read for this round:**
  - `Link::of`'s ball and reach, `FaceSector::cleared`, `Cone::closes`, `reads_given`'s references, `Sector::place`, `wedge_distance`, `ahead_distance`, `within`;
  - `fan::meeting`, `recl`'s use of `Coincident`, `PairRecord::coincident` / `coplanar`;
  - `bound_side::read_down` and `witnessed`;
  - `corner_shape`, `apex_corner_shape`, `reads_slit_tip_turn`;
  - `cover::edge_reach_bounds`;
  - a sample of the `Lever::length` uses.
- **Not read:** `slit.rs`'s `coplanar()?`, which is a different function; the link's `TangentFan` / `Horn` / `ApexLobe` beyond their docs; the 52 `over_lever` calls one by one.
- **For the unit:**
  - The geom-core margin combinators the gate needs (levered, levered squared, sagitta, the curvature charge, the min with a radius) are about five to ten doors. I counted them from the formulas I read, not exhaustively.
  - `link::metric`'s departure doors either move to geom-core or are written over those combinators.
- **Defects still standing** (file with the unit):
  - `held_reads`' ball is built from an arm (`ExtentBall::unsourced`), against `ConsumedExtent`'s "both faces".
  - `link::fan`'s `1.01` slack.
  - `conic_bend_side`'s `Unbounded` arm is a refusal spelled as a decision.
  - `second_order::reach` reads a fabricated point through the point door.
  - `validate::wedge_holds_named` and `fate::corner_holds` are one predicate written twice.
  - `apex_extrema_inside` acts on a Zero read at a lever from below.
  - The row `sector-bounds-pack-a-direction-and-its-lever-into-one-vector` is cited and absent.
- **Nothing was built or run.**
