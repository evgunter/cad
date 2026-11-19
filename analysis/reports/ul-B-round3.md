## Reconciliation round 3: the records rule

Your read of what has met is right. I accept A's `curvature_radius` correction: today it takes a bare `T`. In the final state an arc's radius is read as a point read, centre to end point.

**(a) The test a reviewer can check on a struct.**
- A `Lever` or `LeverBounds` field is allowed only in a **reading record**: the result of one read, built where the read is made, and holding the datum or pivot the arm was read from. `dihedral`'s `Reading`, `vtxfac`'s `HeldRead` and `carrier_pose`'s `PlanePair` are examples.
- Any other record holds its region and no arm. That covers a record whose verdicts come later, at pivots not yet fixed, and a record that already holds the geometry the arm would be read from. In the second case a stored arm is a value that should be derived, and two descriptions of one region would be kept in step by hand.
- Checks a reviewer can run:
  - Is the field built inside a read, beside its datum?
  - Does any consumer read it from another point?
  - Does the struct also hold the geometry?

**(b) What the sector's `arm` and `SectionOval::extent` are under that test.**
- **The sector's `arm` is a region, not a lever.** The sector describes a corner for verdicts still to come, and it already names its bounds. The arm is therefore derived from the bounds, and a stored copy can disagree with them: `boolean::fan`'s fixtures build an arm of 1 with no edge behind it. My round-2 view, that "one pivot is fixed, so it may keep its lever", missed this. A fixed pivot does not stop a stored arm from disagreeing with the geometry it should be read from.
- **`SectionOval::extent` is a region too.** The oval is a description of geometry for later readings, not the result of a reading. It holds its ball, and today's field (the larger half-width) under-states that ball besides.

**(c) The README needs the sentence.**
- The same-point sentence implies the rule, but I proposed a `Lever` field on `SsiDomain` from that same sentence. Text that a careful reader misapplies does not carry the rule.
- Forward A's sentence. That makes three sentences in all.
- This is no longer a choice between defensible answers. Ev approves the wording, as Ev does for any ratified text.

---

## For Ev

**Recommendation (likely): keep the three sources as you ruled on PR 6380, and add no fourth. Delete both `unsourced` doors and their per-file pin, and add three sentences to the contract.** No choice between defensible end states is left. The third sentence can be declined on its own.

**What the 66 arms are.**
- **7 are test code.** They go to the fixture doors that already exist.
- **About 46 are real levers whose geometry is in hand.** The site worked the arm out as a number, or carried it in a record, instead of asking the extent's home.
- **About 13 are not levers at all.** In `x · L` the length is the thing measured, or a tolerance. It went through `levered` because that door would take it.

**Premise check.**
- **"The honest read would move verdicts in the last bits" is no reason to stay unsourced** (sure). `[simple.conditioned]`: pose it well, then treat `f64` as exact.
- **Many of the moves are not last-bit moves.** A sum read against a larger-of is up to 2× off, and an arc length read against a reach is up to π× off. Where checked, the moved verdict is the one that was wrong (worked example below).
- **The door is the cast that DESIGN's margins clause (i) forbids** (sure). The clause says a site no door fits "is a finding, not a cast". `join::off_conic_slack` routes a hand-written slack through the door, which the README also forbids.
- **The exposure is wider than 66.**
  - `sector_shape::metering_arm` is a public function that turns a bare number into a lever through one pinned site. Its 9 callers are invisible to the pin.
  - `Margin::sagitta` and `Margin::over_lever` take their arm as a bare number.
- **"A union's read is a max" is only half right.** It holds where both regions are read from one pivot. Where the operands are pinned apart, the honest read across both is a sum, through `ExtentBall::lever_from_within`.
- **`profile`'s six are a layering problem, not a sourcing one.** Each is the distance between two points or an arc's radius. The reads for those live in a crate above `profile`.

**What each site becomes.**
- **A read the home makes** (about 46). The moved verdicts are accepted.
  - Sector, germ and `census` wedge arms: the edges' own `Lever`. `metering_arm` is deleted, not re-sourced.
  - Sums:
    - the larger of the two where one pivot serves both (`rim_wedge`);
    - `lever_from_within` where the pivots stand apart (`eval::measure`, the mate bundle).
  - Arc lengths and parameter spans (`offset_axial`, `ring_path`, `pose`, `reduce`, `census`): the span's read from the corner.
  - A carrier's stored term (Villarceau's `R`, the spiric chart's `R`): the consumed circle's or oval's own extent.
  - `vtxfac`'s ball, rays, a marched polyline, a mitre: the geometry each consumes.
  - `trace::plane_misses`: today it levers at the plane's stored origin, so one plane stored with two origins gives two verdicts. It re-anchors (`anchored_at`) and reads the scene's extent.
  - A unit arm (`fate::sign`, `overlap::one_sense`): 1 m is the hidden length scale that `[tolerance.epsilon]` rules out. The arms are the corner's edges and the faces' extent.
- **A sourced arm times a pure number, or less an offset.** The number moves into the reading and the offset into the band.
  - `leaned_lever`: the lean multiplies the reading.
  - `pcurves` joint room (`d − 2ε`): the arm is `d`, the vertex's distance from the axis, and the ε goes to the band.
  - `pose` route (`length + 2e`): the home's widen-by-tolerance read.
- **The length is the thing measured:** `Margin::of` or the quotient door. This covers `props` `off_axis` and `intersect`'s five apex reaches.
- **The length is a tolerance:** a band answers the question, and no margin is built.
  - `dihedral` (2): the true arm, which every caller holds, in the band of the tolerance offered.
  - `join::off_conic_slack`: `Band::padded`.
  - `validate`'s `K · rounding` is a rounding charge and goes with the open row on site rounding charges. If it outlives the door, it is `Margin::of(rounding)` in the band `(ε/K, ε)`.
- **A rate** (`pcurve_cache` focal β; unsure): the metric doors.
- **A zero that means "no arm"** (`ring_path`'s lead piece): that piece has no lever field.

**A lever belongs to a reading; a record that describes geometry holds its region.** A lever means something only from the point its reading's datum is pinned at.
- **A reading record keeps its arm.** This is the result of one read, holding the datum it was read from, as `dihedral`'s `Reading` and `vtxfac`'s `HeldRead` do.
- **A record that describes geometry for verdicts still to come holds its region**, because a stored arm there is a value that should be derived. Five records change:
  - **`SsiDomain::extent`** is a bare number that four verdicts read, each from a different point. It becomes the ball its producers already hold, and each verdict reads its own lever. One carried length could be sound from all four pivots only as a diameter, which over-states the arm by up to 2×.
  - **A sector's `arm`**: the sector already names its bounds, so the arm is derived from them. Tests today build sectors whose arm matches no edge.
  - **`SectionOval::extent`** becomes the oval's ball. Today's larger half-width falls short of that ball.
  - **`Piece::Segment`'s `lever`** is removed.
  - **Editor-core's `Arm(f64)`**, a second lever type, goes.
- Confidence: likely.

**`profile`.** The point-level reads move down to `geom-core`, beside `Lever`: two points' distance, and a point set's reach from a pivot. Covers and `Reach` stay in `geom_brep`, so "It is one type" is untouched.
- `profile`'s legs return `Lever`s built from those reads: a line's chord, and an arc's lesser of its radius (centre to end point) and its chord.
- `profile` depends on nothing new and never holds a bare-length door.
- Rejected: `profile` depending on `geom_brep`, which would make a sketch crate depend on the B-rep crate for a chord.
- Rejected: `profile` as a second extent home. It would hold the bare-length door, and its natural call reads a stored `f64` the gate cannot tell from an honest read.
- Confidence: likely.

**Worked example: `rim_wedge::same_circle`.** It levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r. The honest arm is the larger radius.
- Where θ·r lies between Kε/2 and Kε, today's arm reads past Kε and decides "not one circle". The honest read is in band and refuses.
- Where θ·r lies between ε/2 and ε, today's arm refuses a coincidence the honest read accepts.

**Ratified text.** All of this goes in the lever bullet of `crates/geom-core/README.md`, "Float rounding", which was written on PR 6329 with your approval. The reply there said arms that fit no source would come back to you; this is that return. The three sources and "rates are not levers" stay, and `docs/DESIGN.md` is unchanged. Add:

1. > "A length that is the reading itself (a residual, an offset, a stand-off) is the margin, and a tolerance is the band's: neither is an arm. A dimensionless factor belongs to the reading, not the arm. The point-level reads (two points' distance, a point set's reach from a pivot) live beside `Lever` in `geom-core`; covers and `Reach` stay in `geom_brep`."
2. > "A lever belongs to a reading, read from the point that reading's datum is pinned at. A record that describes geometry for verdicts still to come holds its region, never a lever."
3. > "A lever is never shorter than the region its verdict governs and no longer than the site's own measure of it: a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree."

Notes on the three:
- Sentence 2 follows from the existing "the datum and its lever are read at the same one", but a careful reading of that sentence still produced a `Lever` field on `SsiDomain`, so the consequence needs stating.
- Sentence 3 is the rule that `extent.rs`'s code docs hold today. About 20 of the sites to be re-read are arcs or splines, where the two bounds differ and the wrong one gives a wrong answer.
- Sentence 3 is separable: declining it changes nothing above.

**The door and the gate.** Both doors and `lever-sources.counts` are deleted. The gate keeps its two other holds: the extent's own door stays in its home, and the fixtures stay pinned.

What stays possible:
- A lever written by hand outside the door (`Margin::of(x·len)`). That is why the sites that are not levers get honest doors, not only a ban.
- A constructor fed a caller's number, `curvature_radius` and the two-point read included. The constructor-doors work you asked for on PR 6380 is closing this.
- `sagitta`'s and `over_lever`'s bare arms.

Deleting the doors is trivially reversible. If an arm with no source ever appears, a fourth source can be ratified then.

**Not recommended.**
- **A fourth source, "a length the site measures itself".** Any site could call its number that: it is the vouched length the rule exists to stop.
- **Keeping the door as a shrink-only list.** You declined this on PR 6380 ("keep the text and fix").

**Feasibility, kept apart from the ranking.** Perhaps 30 sites move verdicts by more than bits, and each wants its own differential.

**Confidence.**
- No fourth source: likely. About 35 sites are known to me only from their stated reasons.
- Bits are not a reason: sure.
- The door is a cast: sure.
- The 7/46/13 split: likely.
- The `rim_wedge` arm: likely.
- `plane_misses` depends on the stored origin: sure.

## For the orchestrator

- **No remaining difference from A.** The records rule is met on A's line: reading records keep the arm, descriptive records hold their region, and the sentence is added. The sector's `arm` and `SectionOval::extent` become regions.
- **Correction taken:** an arc's radius in `profile` is a point read (centre to end point), not `curvature_radius(r)`, which takes a bare `T` until the constructor-doors work lands.
- **For the unit:** A counts about 34 `Lever`/`LeverBounds` struct fields outside the extent's home. Each wants classifying by the round-3 test, which a reviewer can check: is the field built inside the read, beside its datum, and read only from it? Is the geometry also held on the struct?
- **File as "Hidden levers"** on `extent-reads-made-outside-the-extent-home`:
  - `metering_arm` (9 callers the pin cannot see; delete it);
  - `Margin::sagitta` (12 production calls) and `over_lever` (44), which take bare arms;
  - `plane_misses`'s stored-origin pivot, a defect whatever Ev decides.
- **Doc fix:** `Lever::max`'s "a read of the union" holds only from a shared pivot.
- **The filing row is stale.** Its title says 74 and its body 63, against 66 in the tree. It is missing `star`, `fan`, `chord_rows` and `coincide::replay`, and it has no family for the fixture sites.
- **Sequencing:** the fixtures and the pure bit-movers can go before Ev answers. That is `metering_arm`'s callers, the line spans, `leaned_lever` and the quotient forms. The new `geom-core` point reads should go with `oneround/lever-doors`.
- **Not verified:** about 35 sites are classified from their comments only. `pcurve_cache` focal β, `unit_sign` and `coincide::replay` are unsure. `Lever::curvature` having no callers is A's grep. No builds were run.
