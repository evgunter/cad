## Reconciliation answers (round 1)

Your read of where we stand is right. Two smaller differences you missed are at the end of this list.

**(1) `profile`'s six arms.**
- *What moved me:* nothing on the substance. One correction to how A frames my option: I do not lower a ball generic over dimension, and I do not move `Reach` or covers. Only two point-level reads move into `geom-core` beside `Lever`: two points' distance, and a point set's farthest point from a pivot.
- *Is my argument answered?* No. Under A's (a), `profile` becomes a second home that may call the bare-length door `Lever::of_consumed_extent`. The natural call is `of_consumed_extent(self.len)`. `Seg::len` is a stored `f64`, which is exactly a length the caller vouches for, and the gate cannot tell it from an honest read. That is the hole Ev's PR 6380 ruling is closing for the constructors. Under my position `profile` never sees the bare door. It calls `Lever::between(a, b)` and `Lever::curvature_radius(r)`, both of which take geometry. (a) also changes the ratified "It is one type", which mine leaves alone.
- *Position now:* unchanged, but I put it to Ev as a fork with both options laid out, leaning to lowering the reads (likely).

**(2) A record that needs a lever later.**
- *What moved me:* A's rule, "a lever is read at the verdict, not carried". I was wrong to make `SsiDomain::extent` a `Lever`.
- *Why:* a lever means something only from a pivot (README: the datum and the lever are read at the same point). `SsiDomain`'s verdicts pivot at different points: the march point, the boundary points, the crossings. A carried scalar can be sound from every one of those pivots only if it is a diameter. Its producers today pass `ball.radius()·2`, which is that diameter, and it over-states the arm by up to 2×. The README calls an over-stated arm unsound on a two-sided row that serves a class.
- *Position now:* the record holds its region (a `Reach` or its ball), and each verdict reads `lever_from` its own pivot. The same applies to the sector `arm`, editor-core's `Arm`, `Piece::Segment`'s `lever` and `SectionOval::extent`.

**(3) What the ratified text gains.**
- *What moved me:* A's items 1 (what a lever is), 2 (read at the verdict, not carried) and 5 (which bound). Each is a binding rule that the README does not state today. Item 5 is currently held only in `extent.rs`'s code docs.
- *What I keep:* item 3 (two regions read as one region from the pivot) follows from item 1 and need not be its own sentence. Item 4 is my sentence, and I keep its point-level-read placement clause.
- *Position now:* four items, set out below.

**Smaller differences.**
- **`validate::resolved_as`:** A's routing is better. The function says of itself that it stands only until `site-rounding-charges-owed-a-re-posing` lands, so it goes with that row. My band re-posing is the interim form only if it outlives the row.
- **`pcurves` joint room:** A reads it as a rate through the metric doors; I read it as an arm at the radius plus a padded band. The step is an angular parameter, so A's reading is defensible (unsure).
- **`trace` `plane_misses`:** I adopt A's diagnosis. It reads a verdict off a plane's stored origin, which is a defect and is fixed by re-anchoring with `anchored_at`.

---

## For Ev

**Recommendation (likely): keep the three sources as you ruled on PR 6380, add no fourth, and delete `Lever::unsourced`, `ExtentBall::unsourced` and their per-file pin.** Each of the 66 sites is re-posed by **the role its length plays in the reading**. The README's lever bullet gains four rules. One question is left for you, on `profile` (near the end).

**Premise check (sure unless marked).**
- **The five families sort the sites by why bit-identical typing failed, not by what each arm is.** PR 6391's acceptance test was "bit-identical", so any arm whose honest read rounds differently got parked. `[simple.conditioned]` lets no site protect its bits.
- **Many of the moves are not last-bit moves.** A sum read against the larger value is up to 2× off. An arc length against a reach is up to π×. A torus's major radius stands in for the circle's own radius. These arms are wrong.
- **The door is the cast DESIGN's margins clause (i) forbids** ("a site no door honestly fits is a finding, not a cast"). `join::off_conic_slack` routes a hand-written slack through it, which the README also forbids.
- **7 of the 66 are test code** (`star`, `fan` ×2, `chord_rows`, a `join` fixture, the `ssi::certify` twin). They belong on the fixture doors that already exist.
- **"A union's read is a max" is half right.** It holds where both regions are read from one pivot. Where operands are pinned at separate points, the honest read across both is a sum, through `ExtentBall::lever_from_within`.

**The roles, and what each site becomes.**
- **A read of consumed geometry not yet made through the home** (most of families 1, 3 and 4). The site reads its region. Where the read rounds differently, the verdicts move.
  - Sector and germ arms (`sector_shape::metering_arm`, which serves 9 callers through one pin) and `census`' wedge arm: the edges' own `Lever`.
  - `rim_wedge`: the larger radius (worked example below).
  - `eval::measure` and the mate `Arm`: the read across two pinned operands, made in the home. Editor-core's `Arm`, a second lever type, goes.
  - Arc lengths and line parameter spans: the span's reach.
  - `chart_region`'s `hyp`: the cylinder piece's region.
  - Villarceau and spiric `R`: the circle's or oval's own region.
  - The marched polyline: its points. A ray: its own segment.
  - `plane_misses`: today it levers at the plane's stored origin, so the same plane stored with a different origin gives a different verdict. That is a defect. It is fixed by re-anchoring (`anchored_at`).
- **A dimensionless factor folded into an arm.** It multiplies the reading instead. For example, `leaned_lever`'s `unsourced(L·min(s₀,s₁))` becomes `levered(x·min(s₀,s₁), L)`.
- **The length is what the reading is about.** It goes through `Margin::of` or `levered_inv`: `props::curved` `off_axis`, and `intersect`'s apex reaches (`levered(1/d, size)` becomes `levered_inv(size, d)`).
- **A tolerance used as the arm.** The question belongs to the band, through its own doors (`padded`, an offered tolerance), with no margin built.
  - The `dihedral` pair compares `sinθ` with `1/K`.
  - `join::off_conic_slack`.
  - `validate`'s `K · rounding` is a rounding share, and goes with the open row on site rounding charges.
- **A unit or zero standing in for a missing arm.**
  - `fate::sign` and `overlap::one_sense` lever at 1 m, the hidden length scale `[tolerance.epsilon]` rules out. Both sites hold geometry to read.
  - `ring_path`'s zero placeholder becomes a piece with no lever field.
- **Tests:** fixture doors.

Confidence: the role cut is sure for the sites named. That all 66 fit a role is likely; about 35 are known only from their stated reasons. Least certain: `pcurve_cache` focal β and `unit_sign`, the `pcurves` joint room, and `coincide::replay`'s rounding of an arm up to a power of two.

**Worked example: `rim_wedge::same_circle`.** It levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r. The honest arm is the larger radius.
- Where θ·r lies between Kε/2 and Kε, today's arm reads past Kε and decides "not one circle". The honest read is in band and refuses.
- Where θ·r lies between ε/2 and ε, today's arm refuses a coincidence the honest read accepts.

**What the contract gains** (`crates/geom-core/README.md`, "Float rounding", the lever bullet; the wording is yours). Today's three sources and "rates are not levers" stay.
1. **What a lever is.** The farthest the geometry a verdict governs stands from the point where the reading's datum is pinned. This is `[tolerance.epsilon]`'s "reach at which the angle displaces the geometry it governs", stated so a site can check itself. Two regions are therefore read as one region from that pivot.
2. **A lever is read at the verdict, never carried.** A record that needs one later holds its region (a `Reach` or its ball), never a length. A carried length is pivot-blind: to be sound from every pivot it must be a diameter, which over-states the arm.
   - `SsiDomain::extent` (today a `pub f64`) becomes the domain's ball.
   - A sector's `arm`, the mate `Arm`, `Piece::Segment`'s `lever` and `SectionOval::extent` change the same way.
3. **Neither a tolerance nor a length the reading is about is an arm.** A tolerance is the band's, and the length is the margin. A dimensionless factor belongs to the reading.
4. **Which bound.** A lever is never shorter than the region it governs and never longer than the site's measure of it. A reading that acts on either sign reads both bounds. `extent.rs` holds this rule today and the README does not.

**The door and the gate.** Both `unsourced` doors and `lever-sources.counts` are deleted. The gate keeps two holds: the extent's own door stays in its homes, and the fixtures stay pinned.

What remains possible:
- A lever written by hand outside the door, as `Margin::of(x·len)`.
- A bare-`T` arm at `Margin::sagitta` and `Margin::over_lever`.
- A constructor fed a caller's number. The constructor-doors work you asked for on PR 6380 is closing this.

Deleting the doors is trivially reversible, and each verdict move is its own change.

**Not recommended.**
- **A fourth source, "a stated length".** It makes the sentence true by emptying it.
- **A permanent list that only shrinks.** This was put to you on PR 6380, and you chose "keep the text and fix".

**The question for you: `profile`'s six arms.** Each is a 2-D leg's chord, or the lesser of the leg's radius and its chord. `profile` depends only on `geom-core`, and the extent home lives in `geom_brep`.
- **(a) Lower the point-level reads into `geom-core`, beside `Lever`** (I lean here, likely). Two points' distance (`Lever::between(a, b)`) and a point set's reach from a pivot move down. Covers and `Reach` stay in `geom_brep`, so "It is one type" is untouched.
  - `profile` reads `min(curvature_radius(r), between(a, b))`, from constructors that take geometry. It never holds the bare-length door.
  - Cost: two small functions and a 2-D form.
- **(b) `profile` is the extent home for its own geometry.** `Seg` returns `Lever`s, and the gate lists `profile` beside the one `geom` item it lists today.
  - Ownership is clean: `profile` owns `Seg`.
  - But `profile` gains the bare-length door. The natural call, `of_consumed_extent(self.len)`, reads a stored `f64`, which the gate cannot tell from an honest read.
  - "It is one type" becomes "one home per representation".

**Ratified text.** All of it is in the README's "Float rounding" section, which your `[ev]` commit `31576a6c6` wrote on PR 6329. The reply that explained the sources to you there said unclassified arms would "come back to you", and this is that return. `docs/DESIGN.md` does not change.

**Feasibility (kept apart from the ranking).** Perhaps 30 sites move verdicts by more than bits, and each wants its own differential. A record holding a ball costs one read per verdict where it held a number.

## For the orchestrator

- **Still apart from A:**
  - `profile`: I lean (a), lowering the reads; A leans to `profile` as its own home. Both options are put to Ev.
  - Item 3: I fold "two regions read as one region from the pivot" into item 1 instead of making it a separate sentence.
  - `pcurves` joint room: A says the metric door; I am unsure.
  - Converged on everything else, including item 2 and the routing of `validate`.
- **The pin understates exposure.** `sector_shape::metering_arm(arm: T) -> Lever<T>` is a `pub` laundering door: one pin, 9 callers the gate cannot see. Delete it; do not re-source its body. `Margin::sagitta(.., arm: T)` (12 production calls) and `Margin::over_lever(.., lever: T)` (44 calls) take bare arms. They belong with "hidden levers" on `extent-reads-made-outside-the-extent-home`.
- **The row is stale.** Its title says 74 and its body 63; the tree has 66. Its site list misses `star`, `fan`, `chord_rows` and `coincide::replay`.
- **Sequencing:** the fixtures and the pure bit-movers can go before Ev answers. The rest waits on the answer.
- **Overlap with `oneround/lever-doors`:** the same final state (constructors take geometry). New home reads should take regions from the start.
- **Provenance:** `31576a6c6`, plus PR 6329's body and comments and Ev's comment 6594257162 on PR 6380. The shallow checkout was not a blocker.
- **Not verified:** about 35 sites are classified from their comments only. No builds were run.
