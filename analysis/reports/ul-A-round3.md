**Reconciliation round 3: the records rule.** Your read of what has met is right. This round I read the two structs in dispute instead of arguing from the rule, and that settles it. The answer is neither of our round-2 positions.

**(a) The test, as two questions a reviewer asks of a struct with a `Lever` field.**
1. **Does the struct already hold what the lever is read from?** That means the geometry (a carrier, a cover, a ball, end points) or the levers it is folded from. If so, the lever is a method on the struct, not a second field. This is already ratified: `[one-way.places]` says every "reach and ball is a method on [the cover], never a second reading".
2. **If it does not: does the struct fix the one point every reader's datum is pinned at?** If so, it may hold the `Lever` (a reading record: a datum and the arm it was read at). If its readers pin their data at different points, it holds its region. This is the existing same-point sentence.

My round-2 line ("reading record or geometry record") was question 2 only. B's round-2 line ("one region and one pivot keeps its lever") is also question 2 only. Question 1 decides both disputed structs.

**(b) The two structs.**
- **The sector's `arm`: a method, by question 1.** `BoolSector` holds `start_reach` and `end_reach`, each a `BoundSupport` that already carries its bound's reach: `Chord { base, far }`, `Conic { extent }` (the edge's `edge_extent_inf`), `Bisector { arm }`. The sector's arm is documented as "the shorter of its bounds' reaches", so it is their `min`.
  - Today it is computed a second way, as the norm of departure vectors in `sector_shape`. That is why it differs from the bounds' own extents in the last bits, and why it sat `unsourced`. It is one value stated twice and disagreeing.
  - B is right that it is sound from its one vertex. That only means question 2 would pass if question 1 did not apply first.
  - The value is the same as B's ("the edges' own `Lever`"). The only difference left is a cached field against a `min` of two fields, which has no cost.
- **`SectionOval::extent`: a method, by question 1.** The struct holds its `carrier`. Its sibling `SectionConic` has no lever field: `SectionCurve::lever` reads `ExtentBall::of_covered(c.carrier.cover()).lever()` for the conic and the stored field for the oval. Once the oval's arm is its ball (B's value, which I adopt), both arms are the same expression and the field is redundant. B's "keeps a lever field whose value must be the oval's ball" leaves a field kept in step by hand with the carrier beside it.

**(c) No new README sentence.** Both questions are carried by ratified text (`[one-way.places]` and the same-point sentence), and a third statement would be the same defect we both refused for the definition. I withdraw my round-2 sentence. Because a careful reader did miss the application once, I offer one clause appended to the existing sentence, marked separable. B has none.

**Is the remaining difference Ev's?** No. On the two structs, B's field and my method produce the same values; a unit settles it, and the code (the conic sibling, the bounds' own reaches) points to the method. The clause is a detail you can drop without changing anything else.

## For Ev

**Recommendation (likely): no fourth source. Keep the three, delete the `unsourced` doors and their pin, and add two sentences to the contract.** No choice between defensible end states is left; the second sentence and a one-clause amendment can each be declined alone.

**What the 66 arms are.** They are not one kind of thing waiting for a source. I read every call's stated reason and about 25 of the sites in code; the counts are mine.
- **7 are tests' stated scales**: `star::cut`, `star::order`'s probe, `boolean::fan` (2), a `join` fixture, `link::chord_rows`, `ssi::certify`'s test twin. They go to the fixture doors that exist.
- **About 46 are real levers whose geometry is in hand.** The site worked the arm out as a number, or carried it in a record, instead of asking the extent's home. Their source exists: the consumed extent.
- **About 13 are not levers.** In `x · L` the length is the thing measured, or a tolerance, and it was written through `levered` because that door would take it.

**Premise check.**
- **"The honest read would move verdicts in the last bits" is not a reason to stay unsourced** (sure). `[simple.conditioned]` is your rule from PR 6329: pose it well, then treat `f64` as exact.
- **Many of the moves are not last-bit moves.** A sum against a larger-of is up to 2×, an arc length against a reach up to π×. Where I checked, the moved verdict is the one that was wrong.
- **The door is the cast `[tolerance.margins]` forbids** (sure). `Margin::levered(x, Lever::unsourced(y))` makes a margin of any product; that clause says a site no door fits "is a finding, not a cast".
- **The exposure is wider than 66.**
  - `sector_shape::metering_arm` is a public function from a bare number to a lever through one pinned site; its 9 callers are invisible to the pin.
  - `Margin::sagitta` and `Margin::over_lever` take their arm as a bare number.
- **`profile`'s six are a layering problem, not a sourcing one.** Each is a distance between two points or an arc's radius, and the reads for those live in a crate above `profile`.

**What each site becomes.**
- **A read the extent's home makes** (about 46), accepting the moved verdicts:
  - *Sector, germ and `census` wedge arms*: the edges' own `Lever`. `metering_arm` is deleted, not re-sourced.
  - *Sums of radii or reaches*: the larger where both are read from one pivot (`rim_wedge`). Where one pivot stands off the other, the sum is honest and the home has that read (`lever_from_within`): `eval::measure`, the mate bundle. Editor-core's own `Arm(f64)` lever type goes.
  - *Arc lengths and parameter spans* (`offset_axial`, `ring_path`, `pose`, `reduce`, `census`): the span's read from the corner. `offset_axial` chose arc length because a closed edge's chord is zero; the home's read now handles closure.
  - *A carrier's term* (Villarceau's `R`, the spiric chart's `R`): the consumed circle's or oval's own extent.
  - *`chord_join`'s oval*: the oval's ball, read off its carrier's cover as the conic beside it already is. Its larger half-width falls short of the ball.
  - *`vtxfac`'s ball, a ray, a marched polyline, a mitre*: the geometry each consumes.
  - *`plane_misses`* levers at the plane's stored origin, so one plane stored with two origins gives two verdicts. It re-anchors (`anchored_at`) and reads the scene's extent.
  - *A unit arm on a pure number* (`fate` `sign`, `overlap` `one_sense`): 1 m is the hidden length scale `[tolerance.epsilon]` rules out. The corner's edges and the faces' extent give the arm.
- **A sourced arm times a pure number, or less an offset:** the number moves into the reading, the offset into the band.
  - `leaned_lever`: the lean multiplies the reading.
  - `pcurves` joint room (`d − 2ε`): lever at `d`, the ε through `Band::padded`.
  - `pose` `route` (`length + 2e`): the home's widen-by-tolerance read.
- **The length is what is measured:** `Margin::of` or the quotient door. `props` `off_axis`; `intersect`'s five apex reaches.
- **The length is a tolerance:** the question is about the band and is answered with a band, with no margin built.
  - `dihedral` (2) compares `sin θ` with `1/K`: the true arm, which every caller holds, in the band of the tolerance offered.
  - `join` `off_conic_slack`: `Band::padded`.
  - `validate`'s `K · rounding` is a rounding charge and goes with the open row on site rounding charges. If it outlives the door it is `Margin::of(rounding)` in the band (ε/K, ε).
- **A rate** (`pcurve_cache` focal β; unsure): the metric doors.
- **A zero meaning "no arm"** (`ring_path`'s lead piece): that piece has no lever field.

**Records: a lever is read off what the record holds, at the reader's own point.** Two ratified sentences already decide every case; no new rule is needed.
- **A record that holds what the lever is read from derives it; it does not store it beside.** `[one-way.places]`: a reach "is a method on [the cover], never a second reading".
  - *A sector's `arm`.* The sector already holds its two bounds, each with its own reach, and the arm is the shorter of them. Today it is computed a second way, from departure vectors. That second computation is why it differs from the bounds in the last bits and why it was pinned. It becomes the `min` of the two.
  - *`SectionOval`'s `extent`.* The oval holds its carrier. The conic beside it stores no lever and reads its carrier's cover; the oval does the same and the field goes.
- **A record whose readers pin their data at different points holds its region.** The contract: "any point is sound provided the datum and its lever are read at the same one".
  - *`SsiDomain::extent`* is a bare number read by four verdicts at four points: the boundary strip, the tube's transversality, the march at its current point, the section's crossing. One carried length is sound from all four only as a diameter, which over-states the arm by up to 2×.
  - Its producers have a ball in hand. The record holds that ball (four scalars, the same cost) and each verdict reads its own lever.
  - That also leaves one description of the domain where `center` and `half_extent` sit beside `extent` today.
- **A reading record keeps its arm.** It pairs a datum with the lever it was read at, so the arm travels with its own point.
- Confidence: likely.

**`profile`.** The point-level reads (two points' distance, a point set's reach from a pivot) move down to `geom-core`, beside `Lever`. Covers and `Reach` stay in `geom_brep`, so "It is one type" is untouched.
- `profile`'s legs return `Lever`s built from those reads: a line's chord, an arc's lesser of radius and chord.
- `profile` depends on nothing new and never holds a bare-length door.
- Rejected: `profile` depending on `geom_brep` (a sketch crate depending on the B-rep crate for a chord). Also rejected: `profile` as a second extent home. It would hold the bare-length door, and its natural call reads a stored `f64` that the gate cannot tell from an honest read.
- Confidence: likely.

**Worked example: `rim_wedge` `same_circle`.** It decides whether two rims are one circle, and levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r: the honest arm is the larger radius.
- With θ·r between K·ε/2 and K·ε, today's arm reads past K·ε and decides "not the same circle". The honest read is in band and refuses.
- With θ·r between ε/2 and ε, today's arm refuses a coincidence the honest read accepts.

**Ratified text.** All of it is in `crates/geom-core/README.md`, "Float rounding". Agents wrote it on PR 6329 and you approved it; the reply that explained the three sources said the arms were unclassified and "any that don't come back to you". The three sources and "rates are not levers" stay. `docs/DESIGN.md` is unchanged. Add to the lever bullet (the wording is yours):

1. > "A length that is the reading itself (a residual, an offset, a stand-off) is the margin, and a tolerance is the band's: neither is an arm. A dimensionless factor belongs to the reading, not the arm. The point-level reads (two points' distance, a point set's reach from a pivot) live beside `Lever` in `geom-core`; covers and `Reach` stay in `geom_brep`."
2. > "A lever is never shorter than the region its verdict governs and no longer than the site's own measure of it: a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree."

Sentence 2 is the rule `extent.rs`'s docs hold today; the README does not carry it. About 20 of the re-read sites are arcs or splines, where the two bounds differ and the wrong one gives a wrong answer. Declining it changes nothing above.

Optional, and separable: a clause on the extent bullet's existing sentence, "…read at the same one, *so a record that verdicts read from different points holds its region, not a lever*". It adds no rule. It is there because the bare sentence was read once without that consequence being drawn.

**The door and the gate.** `Lever::unsourced` and `ExtentBall::unsourced` are deleted, and `lever-sources.counts` with them. The gate keeps the extent's door in its home, and the fixture pins.

What stays possible:
- `Margin::of(x * len)`, a lever outside the door that no gate sees. Closing the door raises the pull toward it, which is why the non-levers get honest doors rather than only a ban.
- A constructor fed a caller's number, the two-point read included. That is the constructor-doors work already under way.
- `sagitta`'s and `over_lever`'s bare arms.

Deleting the door is trivially reversible: if an arm with no source ever appears, a fourth can be ratified then.

**Options not recommended.**
- **A fourth source, "a length the site measures itself"**: any site could call its number that. It is the vouched length the rule exists to stop.
- **Keeping the door as a shrink-only list**: what you declined on PR 6380 ("keeping the text and fixing").

**Feasibility, apart from the ranking.** Perhaps 30 sites move verdicts beyond bits, and each wants its own differential.

**Confidence.**
- No fourth source: likely (about 40 sites are known to me only by their comments).
- Bits are not a reason: sure.
- The door is a cast: sure.
- The 7/46/13 split: likely, ±5 between the last two.
- The `rim_wedge` arm: likely.
- `plane_misses` depends on the stored origin: sure.
- The sector's arm is the `min` of its bounds' reaches: likely (read from the record and its docs, not derived through `sector_shape`).
- Least certain sites: `pcurve_cache`'s focal β and `unit_sign`, `coincide::replay`'s power-of-two arm.

## For the orchestrator

**Remaining differences from B, as of B's round-2 report.**
- **The sector's `arm` and `SectionOval::extent`: field or method.** B keeps a `Lever` field on each (the sector's at its vertex; the oval's with the ball as its value). I make each a method, because each struct already holds what the lever is read from. The values agree. A unit settles it; it is not Ev's.
- **The optional clause.** B adds no text for records. I add no sentence either, only the separable clause. Drop it freely.
- **`validate::resolved_as`.** B forwards the band re-pose and notes the row; I forward the row and note the re-pose. The content is the same.
- Nothing else differs. `SsiDomain` met in round 1, as B says; my round-2 note listing it was about B's round-0 text.

**What I read this round.**
- `topo::boolean::sectors::BoolSector`: its fields, and about 25 uses of `.arm` across `recl`, `vtxfac`, `sectors`, `fan`, `insert`. All are germ readings at the base vertex, by their call lines.
- `topo::bound_side::BoundSupport`: `Chord`, `Extent` (test-only), `Conic { extent, .. }`, `Bisector { arm, .. }`.
- `topo::star::walk::Sector`: also holds `arm`, "the shorter chord of the corner the sector is part of".
- `topo::chord_join`: `SectionOval`, `SectionConic` and `SectionCurve::lever`.

**Not checked.**
- That `min(start_reach, end_reach)` reproduces the corner's arm for a sub-sector bounded by a bisector. I reasoned it (a bisector's reach is the corner's arm, which is no longer than either edge's) but did not trace `Star::refine`.
- `star::walk::Sector` holds no bounds itself (the `Star` does), so there the arm is a method on the star, or the field stays under question 2. A unit decides.
- A spiric's `cover()` on each `lever()` call costs more than a stored field. The conic sibling already pays it; unmeasured.
- No builds were run and nothing was measured. The counts of about 30 verdict-moving sites and about 20 arc or spline sites are estimates.
- The scan of about 34 lever fields outside the home (round 2) is a regex heuristic; which pass question 1 or 2 is a pass for a unit.
- I opened no `analysis/*` branch.

**Carried from earlier rounds (unchanged).**
- **Corrections B has accepted:** the spiric torus ball is not fixed by lowering the point reads, so the gate's exempt item stays; `sign_of` does not fit `fate` `sign`.
- **Still open with B's text:** an arc's radius in `profile` should be a point read (centre to end), since `Lever::curvature_radius` takes a bare `T` until `oneround/lever-doors` lands.
- **The filing row is stale.** Title 74, body 63, tree 66. Gone: `splitting::containment` `extent_from`, `recl`'s corner ball, `at_rest` `constant_pairing`, `unglued_flat_sector`. New: `star::order` (2), `star::cut`, `coincide::replay`, `link::chord_rows`, `boolean::fan` (2). It has no fixture family.
- **Off the question, to file or route:**
  - `Lever::curvature` has no callers (grep) and is a second spelling of `levered_inv`'s curvature case.
  - `Lever::max`'s doc holds only from a shared pivot.
  - `plane_misses`' stored-origin pivot is a defect whatever Ev decides.
  - `sagitta`'s and `over_lever`'s bare arms belong with "Hidden levers" on `extent-reads-made-outside-the-extent-home`.
  - Check whether `intersect`'s apex rounding charges are on the site-rounding row.
- **Sequencing:** the fixtures and the pure bit-movers can go before Ev answers (`metering_arm`'s callers, the line spans, `leaned_lever`, the quotient forms). With the sector's arm a method, `fan`'s two fixtures state bounds through the test-only `BoundSupport::Extent`. The `geom-core` point reads take geometry from the start, with `oneround/lever-doors`. The verdict-moving sites and the records wait for the answer.