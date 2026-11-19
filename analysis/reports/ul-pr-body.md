**The decision: no fourth source for a lever. The `unsourced` doors and their pin are deleted, each of the 66 arms is re-posed by what its length actually is, and the README gains two sentences and one clause.** Both designers agree after four rounds; nothing is split. The diff is the lever and consumed-extent bullets of `crates/geom-core/README.md` ("Float rounding").

**What the 66 arms turned out to be** (the designers' count, about 25 to 30 sites read in code, the rest from their stated reasons):
- about 7 are test code, and go to the fixture doors;
- about 46 are real levers whose geometry is in hand; the site worked the arm out as a number instead of asking the extent's home;
- about 13 are not levers: the length is the thing measured, or a tolerance, and went through `levered` because that door would take it.

**The decisions.**
1. **No fourth source; delete `Lever::unsourced`, `ExtentBall::unsourced` and the per-file pin.**
   - Rejected: a fourth source, "a length the site measures itself" (any site could call its number that), and a list that only shrinks (you declined it on PR 6380).
2. **"It would move bits" is not a reason to stay unsourced, and many of the moves are not bits.** `rim_wedge::same_circle` levers a tilt at `r_a + r_b` where the arm is the larger radius, so it is up to 2× off. Perhaps 30 sites move a verdict by more than bits, and each gets its own differential.
3. **`profile` reads point-level reads that move down to `geom-core`**, beside `Lever` (two points' distance, a point set's reach from a pivot). Covers and `Reach` stay in `geom_brep`.
   - Rejected: `profile` depending on `geom_brep`, and `profile` as a second extent home (it would hold the bare-length door).
4. **Records.** A record that already holds what the lever is read from derives it (a sector's `arm`, `SectionOval::extent`). A record several verdicts read from different points holds its region (`SsiDomain` holds its ball). A reading record keeps the arm it was read at.

**What I am asking.** Approve 1 to 4 and the text in the diff? The which-bound sentence and the records clause can each be declined alone; a standalone sentence in place of the clause is acceptable to both designers.

**Going ahead without waiting, unless you object.** The seven test sites move to the fixture doors, and the arms whose honest read moves only last bits are re-read. Both designers read `[simple.conditioned]` and your PR 6380 ruling as already covering those.

**Not checked.** Nothing was built or measured. About 35 to 40 of the 66 sites were classified from their comments; `pcurve_cache`'s focal β, `unit_sign` and `coincide::replay` are unsure.

Rounds, what each designer could not check, and the exposure the pin cannot see (`metering_arm`, `Margin::sagitta`, `Margin::over_lever`): `work/oneround/log.md` (2026-11-19), `work/oneround/unsourced-levers-await-a-source.md` and `docs/DESIGN-FORK-LOG.md` row 290.

---

## A

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

---

## B

**Recommendation (likely): keep the three sources as you ruled on PR 6380, add no fourth, and delete both `unsourced` doors and their per-file pin.** Add two sentences to the contract and one clause to an existing sentence. No choice between defensible end states is left. The second sentence and the clause can each be declined alone.

**What the 66 arms are.**
- **7 are test code.** They go to the fixture doors that already exist.
- **About 46 are real levers whose geometry is in hand.** The site worked the arm out as a number, or carried it in a record, instead of asking the extent's home.
- **About 13 are not levers.** In `x · L`, the length is the thing measured, or a tolerance. It went through `levered` because that door would take it.

**Premise check.**
- **"The honest read would move verdicts in the last bits" is no reason to stay unsourced** (sure). `[simple.conditioned]` says: pose it well, then treat `f64` as exact.
- **Many of the moves are not last-bit moves.** A sum read against a larger-of is up to 2× off, and an arc length read against a reach is up to π× off. Where I checked, the moved verdict is the one that was wrong (worked example below).
- **The door is the cast DESIGN's margins clause (i) forbids** (sure). That clause says a site no door fits "is a finding, not a cast". `join::off_conic_slack` routes a hand-written slack through the door, which the README also forbids.
- **The exposure is wider than 66.**
  - `sector_shape::metering_arm` is a public function that turns a bare number into a lever through one pinned site. Its 9 callers are invisible to the pin.
  - `Margin::sagitta` and `Margin::over_lever` take their arm as a bare number.
- **"A union's read is a max" is half right.** It holds where both regions are read from one pivot. Where the operands are pinned apart, the honest read across both is a sum, through `ExtentBall::lever_from_within`.
- **`profile`'s six are a layering problem, not a sourcing one.** Each is two points' distance or an arc's radius, and the reads for those live in a crate above `profile`.

**What each site becomes.**
- **A read the home makes** (about 46), accepting that verdicts move:
  - Sector, germ and `census` wedge arms: the edges' own `Lever`. `metering_arm` is deleted, not re-sourced.
  - Sums of reaches:
    - the larger where one pivot serves both (`rim_wedge`);
    - `lever_from_within` where the pivots stand apart (`eval::measure`, the mate bundle).
  - Arc lengths and parameter spans (`offset_axial`, `ring_path`, `pose`, `reduce`, `census`): the span's read from the corner.
  - A carrier's stored term (Villarceau's `R`, the spiric chart's `R`): the consumed circle's or oval's own extent.
  - `vtxfac`'s ball, rays, a marched polyline, a mitre: the geometry each one consumes.
  - `trace::plane_misses`: it levers at the plane's stored origin, so one plane stored with two origins gives two verdicts. It re-anchors (`anchored_at`) and reads the scene's extent.
  - A unit arm (`fate::sign`, `overlap::one_sense`): 1 m is the hidden length scale `[tolerance.epsilon]` rules out. The arms are the corner's edges and the faces' extent.
- **A sourced arm times a pure number, or minus an offset.** The number moves into the reading and the offset into the band:
  - `leaned_lever`: the lean multiplies the reading.
  - The `pcurves` joint room (`d − 2ε`): the arm is `d`, the vertex's distance from the axis, and the ε goes to the band.
  - `pose` route (`length + 2e`): the home's widen-by-tolerance read.
- **The length is the thing measured:** `Margin::of` or the quotient door. This covers `props` `off_axis` and `intersect`'s five apex reaches.
- **The length is a tolerance:** a band answers the question, and no margin is built.
  - `dihedral` (2): the true arm, which every caller holds, in the band of the tolerance offered.
  - `join::off_conic_slack`: `Band::padded`.
  - `validate`'s `K · rounding` is a rounding charge and goes with the open row on site rounding charges. If it outlives the door, it becomes `Margin::of(rounding)` in the band `(ε/K, ε)`.
- **A rate** (`pcurve_cache` focal β; unsure): the metric doors.
- **A zero meaning "no arm"** (`ring_path`'s lead piece): that piece has no lever field.

**Records: a lever is read off what the record holds, at the reader's own point.** Two ratified texts decide each case. A reviewer asks two questions of any struct with a `Lever` field:
1. **Does it already hold what the lever is read from** (a carrier, a cover, a ball, end points, or the levers the arm is folded from)? Then the lever is a method on the struct, never a second field. `[one-way.places]` says a reach "is a method on [the cover], never a second reading".
   - **A sector's `arm`:** the sector holds its two bounds, each with its own reach, and the arm is the shorter of them. Today it is computed a second way, from departure vectors, which is why it differs from the bounds and why it was pinned. It becomes their `min`.
   - **`SectionOval::extent`:** the oval holds its carrier. The conic beside it stores no lever and reads its carrier's cover. The oval does the same, and the field goes. Today's value, the larger half-width, also falls short of the oval's ball.
2. **If not, do all its readers pin their data at one point?** Then it may keep the `Lever`, as a reading record does: a datum together with the arm read at it. If its readers pin their data at different points, it holds its region. The README already says "any point is sound provided the datum and its lever are read at the same one".
   - **`SsiDomain::extent`** is a bare number that four verdicts read at four different points. One carried length is sound from all four only as a diameter, which over-states the arm by up to 2×. The record holds the ball its producers already have, and each verdict reads its own lever. That also leaves one description of the domain, where today `center` and `half_extent` sit beside `extent`.
   - **`Piece::Segment`'s `lever`** is removed, and **editor-core's `Arm(f64)`**, a second lever type, goes.
- Confidence: likely.

**`profile`.** The point-level reads (two points' distance, and a point set's reach from a pivot) move down to `geom-core`, beside `Lever`. Covers and `Reach` stay in `geom_brep`, so "It is one type" is untouched.
- `profile`'s legs return `Lever`s built from those reads: a line's chord, and an arc's lesser of its radius (centre to end point) and its chord.
- `profile` depends on nothing new and never holds a bare-length door.
- Rejected: `profile` depending on `geom_brep`, which would make a sketch crate depend on the B-rep crate for a chord.
- Rejected: `profile` as a second extent home. It would hold the bare-length door, and its natural call reads a stored `f64` the gate cannot tell from an honest read.
- Confidence: likely.

**Worked example: `rim_wedge::same_circle`.** It levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r. The honest arm is the larger radius.
- Where θ·r lies between Kε/2 and Kε, today's arm reads past Kε and decides "not one circle". The honest read is in band, and refuses.
- Where θ·r lies between ε/2 and ε, today's arm refuses a coincidence that the honest read accepts.

**Ratified text.** All of it is in `crates/geom-core/README.md`, "Float rounding", which was written on PR 6329 with your approval. The reply there said arms that fit no source would come back to you; this is that return. The three sources and "rates are not levers" stay, and `docs/DESIGN.md` is unchanged.
1. **Added to the lever bullet:**
   > "A length that is the reading itself (a residual, an offset, a stand-off) is the margin, and a tolerance is the band's: neither is an arm. A dimensionless factor belongs to the reading, not the arm. The point-level reads (two points' distance, a point set's reach from a pivot) live beside `Lever` in `geom-core`; covers and `Reach` stay in `geom_brep`."
2. **Added to the lever bullet:**
   > "A lever is never shorter than the region its verdict governs and no longer than the site's own measure of it: a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree."

   This is the rule `extent.rs`'s code docs hold today. About 20 of the sites to be re-read are arcs or splines, where the two bounds differ and the wrong one gives a wrong answer.
3. **Appended to the consumed-extent bullet's existing sentence:**
   > "…read at the same one, so a record that verdicts read from different points holds its region, not a lever."

   The sentence already implies this, but a careful reading of it once produced a `Lever` field on `SsiDomain`. A separate sentence saying the same is an acceptable alternative.

Items 2 and 3 are each separable: declining either changes nothing else.

**The door and the gate.** Both doors and `lever-sources.counts` are deleted. The gate keeps its two other holds: the extent's own door stays in its home, and the fixtures stay pinned.

What stays possible:
- A lever written by hand outside the door (`Margin::of(x·len)`). This is why the non-levers get honest doors, not only a ban.
- A constructor fed a caller's number, including `curvature_radius` and the two-point read. The constructor-doors work you asked for on PR 6380 is closing this.
- `sagitta`'s and `over_lever`'s bare arms.

Deleting the doors is trivially reversible. If an arm with no source ever appears, a fourth source can be ratified then.

**Not recommended.**
- **A fourth source, "a length the site measures itself":** any site could call its number that. It is the vouched length the rule exists to stop.
- **Keeping the door as a shrink-only list:** you declined this on PR 6380 ("keep the text and fix").

**Feasibility, kept apart from the ranking.** Perhaps 30 sites move verdicts by more than bits, and each wants its own differential.

**Confidence.**
- No fourth source: likely. About 35 sites are known only from their stated reasons.
- Bits are not a reason: sure.
- The door is a cast: sure.
- The 7/46/13 split: likely.
- The `rim_wedge` arm: likely.
- `plane_misses` depends on the stored origin: sure.

---

🤖 Generated with [Claude Code](https://claude.com/claude-code)

https://claude.ai/code/session_01EdpJs4dRmhgaKbdqQzfNmw
