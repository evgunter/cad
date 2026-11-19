## Reconciliation round 4: answers

**1. Is the struct-level state met?** Yes. A's question 1 says the same thing as my round-3 test ("a record that already holds the geometry the arm would be read from derives it"), and A's question 2 is the same-point test. No struct comes out differently:
- `BoolSector::arm` goes, replaced by the `min` of `start_reach` and `end_reach`;
- `SectionOval::extent` goes, replaced by a read off its carrier's cover, as `SectionConic` already does;
- `SsiDomain` holds its ball;
- the reading records keep their arm.

A's wording is sharper because it cites a ratified home for question 1 (`[one-way.places]`: a reach "is a method on [the cover], never a second reading"). I forward A's two questions as the test.

**2. New sentence or appended clause in the README?** It is only wording. Forward the appended clause: "…read at the same one, so a record that verdicts read from different points holds its region, not a lever". It states the consequence where its rule already lives, and adds no third statement of a rule the text already holds. A new sentence is an acceptable alternative. Either version carries what a careful reader missed once.

## For Ev

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

## For the orchestrator

- **No remaining difference from A.** We have met on:
  - the struct test, stated as A's two questions;
  - the sector's `arm`, now the `min` of the bounds' reaches;
  - `SectionOval`, which reads its carrier's cover;
  - the appended clause, with a separate sentence noted as the acceptable alternative.
- **For the unit:**
  - Classify A's ~34 `Lever`/`LeverBounds` struct fields outside the extent home by the two questions.
  - `metering_arm`'s callers move to `BoolSector`'s `min` of its bounds' reaches.
- **File as "Hidden levers"** on `extent-reads-made-outside-the-extent-home`:
  - `metering_arm` (9 callers);
  - `Margin::sagitta` (12) and `over_lever` (44), which take bare arms;
  - `plane_misses`'s stored-origin pivot, which is a defect whatever Ev decides.
- **Doc fix:** `Lever::max`'s "a read of the union" holds only from a shared pivot.
- **The filing row is stale:**
  - its title says 74 and its body 63, against 66 in the tree;
  - it is missing `star`, `fan`, `chord_rows` and `coincide::replay`;
  - it has no fixture family.
- **Sequencing:** the fixtures and pure bit-movers can go before Ev answers (`metering_arm`'s callers, the line spans, `leaned_lever`, the quotient forms). The new `geom-core` point reads go with `oneround/lever-doors`.
- **Not verified:**
  - about 35 sites are classified from their comments only;
  - `pcurve_cache` focal β, `unit_sign` and `coincide::replay` are unsure;
  - "`Lever::curvature` has no callers" is A's grep, not mine;
  - `BoolSector`'s fields are as A reports them; I did not re-read them;
  - no builds were run.
