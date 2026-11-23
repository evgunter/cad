# Round 4

Both round-3 reports are in. The other designer's round-3 report is below, WHOLE and as delivered. You are designer A; it is designer B's. The same rules hold: read-only, the same tree, `docs/prompts/designer.md` binds you, no helper on another model.

My reading of where round 3 leaves you (a hypothesis; correct it). **Agreed:** everything on round 3's agreed list; the departure type lives in `topo` and `geom_brep::sector_shape` takes two unit directions and an arm; the link's face-tie fan stays a named construction sized from the face's cover, its hand-written slack gone; a divider is levered at its corner's arm and nothing but the sentence is Ev's there; at a slit's tip an agreed Zero is kept apart from a straddle, which refuses; a tie's witness is a point of the edge chosen from lengths of geometry and the parting is levered at that point's distance from the vertex; how "a lever's length is not geometry" is enforced is code and not Ev's; three README edits go to Ev; independent of PR 7619.

**Not yet one text. Settle each, by the tree where it can be:**
1. **The three README edits.** You each wrote them, in different words and at different places in "Float rounding". Read the other's three against the README bullets as they stand (`crates/geom-core/README.md`) and against each other, and give ONE text for each edit with its exact place, or say which of the two you would sign and why the other is worse. Test each sentence: does it say what the thing IS; is every word defined in the README where it stands; does it bind something that should not be bound (B's "read only in its product with a tilt": the curvature sagitta squares a lever; A's "read only by a margin": is a `Margin` a README term); does "a region a site needs is read from the geometry the lever was read from" restate the records clause beside it.
2. **The gathered cone's interior.** A's round 3: a ray of the gathered cone is a real point or a departure read at both bounds, the cone's readers using a ray only as a direction at its reach. B's round 3: the cone stays a picture of points shared with real-scale links (`rest::judge` and `crossing` read chord points as positions; the parity reference's centroid depends on the two reaches), fed by one door in `link` that realises each ray at its reach, at both bounds, each `Distance` deciding both. Is there a difference left in the FINAL STATE, or only in how the unit gets there? If there is one, which reader decides it?
3. **The gate's end state.** A's round 3: `Lever::length` removed, with margin combinators in `geom-core`, a report exit and a test-support exit. B's round 3: each remaining read becomes a named read or a typed parameter, and a typed count holds the raw read at zero outside the two homes, because `geom_brep`'s extent and curvature homes are another crate and compose levers. Is there a difference left, given both say it is code? One sentence each for the unit's brief.
4. **What gates the QUESTION to Ev, and what gates only the MERGE of the unit that builds it.** Both of you list measurements "first". Each needs a build (the old lever logged beside the new bounds at the star's departures; straddles by predicate). For each measurement say: could its result change any of the three README sentences or the recommendation, or does it only decide which bodies' verdicts move, which Ev's standing conditions already cover (a verdict that moves toward refusing is accepted, PR 7141; a class of valid bodies that newly refuses is reported before it lands, PR 7140)? If a measurement gates the question, say exactly what is logged, at which doors, over which suites, and against what it is judged, so that a lane can be handed it today. If none does, say so plainly, and say what the PR to Ev must state as unmeasured.

Send, as your final message, at most 120 lines: (1) the four points; (2) your `## For Ev` section, final unless the other's answer moves you, with the README edits as exact text and place; (3) `## For the orchestrator`. If you agree on a sentence, use the same words.

---

# Designer B's round-3 report

# Round 3, designer B

Your "Agreed now" list is true of my position, every item.

## 1. The numbered points

**1. The link's face-tie fan: it stays. I hold my round-2 position; "rays throughout" does not survive the readers.**
- `link::fan` has four callers, all building a `Link`'s `FaceSector`: `cleared` (sized by `face_reach_from`), `tangent`'s `Edge` and `Inside`, and `chord`. The boolean's `Cone::gathered` never calls it: a face held whole there is a closed sector with no wedges, "which no ray of a construction's own choosing bounds or divides".
- Readers that use the rim as an enclosure or a position:
  - `rest::judge` cuts each wedge into pieces (`Link::pieces`: "the two real points of the face its bounding rays meet the wedge's far side at"), probes points of the chord (`c0.mid(c1)`, the ladder `c0 + (c1 − c0)·t`) at full scale, then again drawn into the ball;
  - its `on_plane` takes both chord ends reading Zero as "the whole piece lies on its plane";
  - `Link::of` reads `ball` as the least distance from `p` to a chord (`to_segment`), folded with `clear`.
- Readers that use it as a direction at a reach: `closes`, `Sector::place` (`wedge_side`, `ahead`, each drawn in by `within`), and `pairing`, levered at the farthest ray point.
- The parity reading (`reads_given`) is a picture, not a list of sines. Its reference is a wedge's centroid `(a − p) + (b − p)`, whose direction depends on the two reaches. Its clearance is `p`'s distance from the segment `q`–`w`. `crossing` places the point where that segment meets a plane (`on_plane`), which stands nearer `p` than either end.
- `Cone` and these readers are shared by real-scale links and gathered cones. So the cone stays a picture of points. The gathered cone is fed departures, and one door in `link` realises each ray at its reach.

**2. Where the type lives: `topo`.**
- `geom_brep::sector_shape` takes `(unit, unit, LeverBounds)` in both designs. So does `chart_normal_beside`, which must be handed an arm for the `sector_shape` it calls. No `geom_brep` function takes the record.
- Every constructor is in `topo`: `star::departure`, `fate::leaving`, `validate::side`, `census::curved`, `sector_face`'s bound, the link's tangent pieces, the star's dividers. `rg` finds `ConicDeparture`, `BoundSupport` and `Leaving` used in no other crate.
- The four forms being replaced are all `topo`'s. `geom_brep` keeps the curve's own reads: the unit tangent and bend (`ConicDeparture`, shrunk) and `edge_reach_bounds`.
- A's "nothing in it is `topo`'s" is true of the field types. It is not true of what builds it: the far vertex, the edge's cover, the corner a divider divides.

**3. How "a lever's length is not geometry" is held: named reads, counted to zero outside the homes.**
- I read about 100 `.length()` sites on lever-named values. They fall into five kinds:
  - "is this arm a length the band reads" (`Margin::of(arm.length())`, a dozen gates);
  - a bare `T` handed to a callee that levers it again (`parting_arm`, `apex_extrema_inside`, `regular_jet`, `chart_arm`, `sagitta_charge`): a parameter's type;
  - the angle a length subtends (`slack / lever.length()`) and the turn over it (`kappa * extent.length()`);
  - diagnostics (`ReadFault::Departure.arm`);
  - geometry (`dir * lever.length()`, `ExtentBall::unsourced(p, arm.length())`, `side_reader`'s `start = reach.length() * −2`).
- Both mechanisms are buildable. Visibility alone is not: `geom_brep`'s extent and curvature homes are another crate. The tree's cross-crate mechanism is a counted gate (`lever-doors.sh`), and a typed count is the exact form of it. A is right on that.
- The better final state is not a reason at each of 90 kept sites. That is the `// Unsourced lever:` pin list, which PR 7141 retired for "each site re-posed by the role its length plays". The first four kinds get a named read or a typed parameter; the typed count then stands at zero outside the two homes.
- Not airtight either way: `Margin::value()` is an open exit by design.
- Ev's: the README sentence only. The mechanism is code.

**4. The witness.** "A tie's witness is the edge's point at an arc length read from geometry, no longer than the edge's own extent and the face's radii; the parting is levered at that point's distance from the vertex." No exception is needed in the README sentence: no lever's length is read. `ConicArc::at` already stops at the far end, so any such length gives a point of the edge.

**5. The slit tip: A's "no opt-out" holds for distinct edges and for one edge; not at a slit's tip.**
- `corner_shape` returns `FullTurn` when rung 2 read `Ok(Zero) | Err(_)` and rung 3 rejected a positive cosine (`reads_slit_tip_turn(.., Sign::Positive)`). `apex_corner_shape` does the same on a Zero window. That is a Zero acted on.
- `SlitTips` is keyed by `(vertex, face)`, and `SlitTips::of` inserts both ends of the shared segment. So every corner of that face at that vertex reads as `SlitTip`, not only the one between the two slit edges.
- So a straddle (Zero from below, definite from above) must refuse there, not read as the turn.

**6. The divider: read; nothing is left for Ev but the sentence.**
- `boolean::fan::meeting` reads two faces parallel at `a.arm.min(b.arm)`, then: glued one carrier by the glue door, a coincidence; a curved face unglued, `Coincident::Frontier`, which `recl` refuses (`CurvedBooleanUnsupported`); two planes unglued, `Meeting::Parallel`, which `pair_search` refuses where the sectors overlap.
- Its doc states the rule: "an unglued Zero acts over no extent, and a coplanar lump speaks for both faces' whole spans."
- So no sector's Zero is acted on as two faces coinciding without the whole-face verdict. The corner's arm is the right lever for a divider. The one reader that claims a whole face, `FaceSector::cleared`, reads the face itself.
- Not traced: the split lane's consumers of an in-plane divider.

**7. Text: in "For Ev" below.**

## For Ev

**Recommendation (likely).** A sector bound becomes one record, a *departure*. It holds a unit direction and the edge's reach from the vertex, known from both sides, as two separate things. Every sector predicate takes unit directions and a reach.

**Terms.** A *bound* is one side of a face's corner at a vertex: an edge leaving it, or a *divider*, a direction a wide corner is cut at. A *lever* is the length a tilt is multiplied by to give a displacement. *Reach* is how far an edge gets from the vertex. `LeverBounds` is a lever known exactly, or between a bound from below and one from above. A *straddle* is the two bounds giving different signs; it refuses.

**Premise (sure).**
- Half of this exists. `boolean::fate::corner_holds` takes a unit direction, a both-sided reach read off the edge's cover from the vertex, and a bend.
- One concept is held in four forms with different levers: the star's bound (a direction packed with the edge's diameter read from below), the cut's `Leaving`, `validate`'s scaled departures, and the link's scaled vectors.
- `geom_brep::sector_shape` forces the packing: it takes two scaled chords and reads its arm from their lengths.

**Worked example (sure).** An ellipse with semi-axes 10 and 1, an edge leaving the end of the minor axis and running nearly all the way round. Its diameter is 20; its far distance from the vertex is about 10.05. Today's lever is a lower bound on 20, on the side that claims a definite sign.

**The final state.**
1. **One departure type, in `topo`**, built by one door: a unit direction, a reach, and what stands behind it.
   - Line: its far vertex; the reach is the chord, exact.
   - Curved edge: its bend and its arc. The reach is read once from the edge's cover from the vertex, both bounds; exact on a circular arc.
   - Divider: its face's section; its reach is its corner's arm.
2. **A corner's arm** is the lesser of its two bounds' reaches. As bounds, each bound of the arm is the lesser of the two (`LeverBounds::min`). It is derived where the two bounds are held; the three arm records and `star::Sector.arm` go.
3. **One both-bounds door decides every sector predicate.** It reads once where the lever is exact, and at both bounds otherwise, deciding only where they agree. A site reads one bound only by writing `.below()` or `.above()`, and each such site has a row that goes red when the bound is swapped. At a slit's tip, where a Zero reads the whole turn, a straddle refuses.
4. **Points.** Every point placed is a vertex or a point of an edge's curve.
   - The conic-against-plane witness sits at an arc length the bend record bounds off the curve; `conic_bend_side` loses `extent`.
   - A tie's witness is the edge's point at an arc length read from geometry, no longer than the edge's own extent and the face's radii; the parting is levered at that point's distance from the vertex.
   - The split's `p + along` becomes a point of the shared edge.
5. **The link.** Its face-tie fan stays a named construction: a polygon past the face's disc, sized by the face cover's far distance read as a length. The touch test probes points of its chords. The boolean's gathered cone is fed departures, and one door inside `link` realises each ray at its reach, at both bounds.

**The records (likely).** `PlannedRun.order` keeps a lever, as bounds; the pierce's ball is built apart, from the two faces' covers. editor-core's carrier reach holds its region. `Leg::arm` and `SectionOval::extent` are methods. `Margin::over_lever`'s calls are classed first as a tilt's arm or a measure.

**What flows where.** `geom_brep::sector_shape` sees less. The star reads each edge's cover from the vertex, which the cut already reads there. The pierce read takes two faces' covers, faces it already reads. No module learns a region it had no business with (likely).

**Yours: three edits to `crates/geom-core/README.md`, "Float rounding".** I recommend all three.
1. In **A verdict's lever arm**, after the sentence that ends "…or the session box where no geometry is in hand":
   > A lever is read only in its product with a tilt, which is a displacement; a length that places or sizes geometry is a read of that geometry.
2. In **The consumed extent**, after "…the least over all points is not owed":
   > A turn between two directions out of one point is consumed where both run: its lever is the lesser of their reaches from that point, at each bound. A direction that divides a corner has no edge behind it, and is read over that corner.
3. In the same bullet, the closing clause "so a record that verdicts read from different points holds its region, not a lever" becomes:
   > so a record whose verdicts all read from one point may hold its lever, as the bounds it is known between, and a record that verdicts read from different points holds its region, not a lever.

Edit 3 is the whole clarification to the records rule of PR 7141; its two questions stand. I would add no sentence about a lever multiplied into a direction: with edit 1 enforced in code, nobody can write it.

**Not yours.** How edit 1 is enforced is code: each remaining read of a lever's length becomes a named read (the arm as a length the band reads, the angle a length subtends at it, the turn over it), and a typed count holds the raw read at zero outside the two homes.

**PR 7619:** independent. These levers are read from a point, not about a line.

**To measure first.** The judge in every case: a verdict that moves toward refusing is accepted; a class of valid bodies that newly refuses is reported to you before landing.
- Old lever against new bounds, per carrier kind, over the boolean, split, census and link suites. Lines must not move.
- The straddle rate on ellipse, spiric and spline bounds and the dividers beside them. It is zero on lines and circles by construction.
- The gathered cone read at both bounds: every sign compared, not only the final answer.
- `conic_bend_side` with the curve's own arc bound. No circle row moves.
- Every surviving `.below()` / `.above()` has its swap row red.

## For the orchestrator

**Where the two of us now differ, and what settles it.**
- *The cone's interior.* A: rays, each reading a sine times a reach. Me: the picture stays, fed by one door. Settled by `rest::judge` and `crossing`, which read chord points as positions. A direction-space parity reader is a rewrite with different levers (the crossing point stands nearer `p` than the probe), so A's test "every decided sign equal where exact" would fail by construction near the band.
- *The gate.* We agree on the instrument (the typed count; the deprecation build in `a-levers-length-used-outside-a-margin-has-no-gate.md`). We differ on the end state: reasons at kept sites, or named reads and a count of zero.
- *The type's home.* By the list of constructors under point 2.
- *The witness.* A takes the abscissa from "the edge's reach from below, as a length off its cover". I take the edge's own extent and the face's radii. Both are lengths of geometry; the sentence in point 4 covers either.

**For the unit.**
- The gathered cone at both bounds must compare each sign in step. Two whole readings that agree do not make each sign agree. The mechanical form: each ray point is a pair, each `Distance` decides both.
- The cone's picture is "direction times reach" as a vector. It complies with README edit 1 as a displacement, provided no norm of it is read back as a lever. `second_order::reach` and `Link::of`'s `reach` do read one back; they carry the reach instead.
- `sector_shape` must keep rung 2's agreed Zero apart from its straddle, so `reads_slit_tip_turn` cannot take a straddle for a turn. Between distinct edges the fall-through is sound. For one edge the two bounds are one half-edge and the sine is exactly zero.
- `chart_normal_beside` gains an arm argument: `dihedral_side` calls `sector_shape` and reads only `bisector.is_none()`.

**Defects found this round.**
- `SlitTips` keys by `(vertex, face)` and inserts both ends of a shared segment. A slit rooted on a boundary vertex would mark the face's ordinary corners there as `SlitTip`, and a spike among them would read as a whole turn instead of refusing. I did not confirm such a body is reachable. Belongs with the star.
- `side_reader` places a ray's start at `reach.length() * −2`: a lever's length as a parameter. Belongs on `sizes-are-read-back-out-of-extent-levers`.
- `link.rs` near line 3242: `let lever = face.clear.min(rho) * 0.5` is a length of geometry named `lever` and handed to `fan`.

**Corrections to my earlier rounds.**
- Round 1 leaned to rays in the cone. Wrong for a shared `Cone`: `rest::judge` reads chord points as positions.
- Round 2 said "`Lever::length` reachable only from a margin, by type". Visibility cannot do that across crates. The claim is now named reads plus a count of zero.

**Not checked.**
- The split lane's consumers of an in-plane divider.
- `Link::offset` and `lowest_on`.
- Whether the three named reads cover every arithmetic use: `ring_path`'s `charge(extent.length(), slack)`, `coincide::replay`'s power-of-two arm, and `intersect`'s `separation_range` were seen but not followed.
- Nothing was built or run.
