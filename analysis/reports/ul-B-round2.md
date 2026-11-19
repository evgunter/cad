## Reconciliation round 2: answers

Your read is right, with one correction: on `SsiDomain` we already met in round 1, both holding the ball. A's orchestrator note still lists it as a difference, but it no longer is one.

**(1) The README's text: forward the two-sentence version.**
- In the final state, a lever's definition and the never-carried rule would each be a second statement of text already ratified:
  - the definition repeats `[tolerance.epsilon]`'s "the reach at which the angle displaces the geometry it governs";
  - "never carried" is the consumed-extent bullet's "any point is sound provided the datum and its lever are read at the same one", applied to a record.
- Two statements of one rule are a defect.
- The which-bound rule is new as ratified text. It is binding, it exists today only in `extent.rs`'s code docs, and it decides the direction of a wrong answer. It earns its place.
- So: the role sentence, which carries the placement of the point-level reads, plus the which-bound sentence.
- The union point becomes a doc fix on `Lever::max`. It is not README text.

**(2) How absolute "never carried" is: forward the qualified rule.**
- The same-point sentence is the test. A record whose own geometry fixes one region and one pivot satisfies it and may keep its `Lever`, for example a sector's arm at its own vertex.
- My blanket list was wrong for the sector and for `SectionOval`. `SectionOval` keeps a lever field, but the value must be the oval's ball and not its larger half-width.
- Two of my items stand for reasons of their own:
  - editor-core's `Arm(f64)` goes, because it is a second lever type;
  - `Piece::Segment` loses its lever field, because "no arm" should be unrepresentable.
- `SsiDomain` holds its ball, because four verdicts read it from four pivots.

**(3) `validate::resolved_as`: both are defensible; a unit settles it, not worth Ev's attention.**
- The band re-pose is the honest form wherever the function stands: `Margin::of(rounding)` in the band `(ε/K, ε)`.
- The site-rounding row may retire the function altogether.
- Forward the re-pose, and note the row.

**(4) The `pcurves` joint room: forward lever-at-`d` with a padded band; a unit detail.**
- `sin(step/4)/2` is the chord fraction of a circle of radius `d`. It is a pure number applied to a radius, not a parameter span crossing to metres, so it is not a rate.
- The `−2ε` is the ends' uncertainty, and that belongs to the band.

**(5) A's two corrections: both accepted.**
- Lowering the point reads does not fix `geom`'s spiric torus ball. `R + r` is a carrier's ball, not a point read, so the gate's exempt item stays.
- `sign_of` does not fit `fate::sign`. It is the door for an enclosure sign only, and a half-turn count summed from `atan2` is not exact. The corner's edges give the arm.

**`profile`: do not put it to Ev as a question.**
- There is no longer a defensible second end state. A second home inside `profile` would need the same point read anyway, and it would hand `profile` the bare-length door.
- Ev still sees it, because the placement of the point reads is part of the role sentence Ev is asked to approve.

---

## For Ev

**Recommendation (likely): keep the three sources as you ruled on PR 6380, add no fourth, and delete both `unsourced` doors (`Lever::unsourced` and `ExtentBall::unsourced`) and their per-file pin. Add two sentences to the contract.** Nothing here is a fork between defensible end states. The second sentence can be declined on its own.

**What the 66 arms are.** They are not one kind of thing waiting for a source:
- **7 are test code.** They go to the fixture doors that already exist.
- **About 46 are real levers whose geometry is in hand.** The site worked the arm out as a number, or carried it in a record, instead of asking the extent's home.
- **About 13 are not levers.** In `x · L` the length is the quantity measured, or a tolerance. It was written through `levered` because that door would take it.

**Premise check.**
- **"The honest read would move verdicts in the last bits" is no reason to stay unsourced** (sure). `[simple.conditioned]` says: pose it well, then treat `f64` as exact.
- **Many of the moves are not last-bit moves.** A sum read against a larger-of is up to 2× off. An arc length read against a reach is up to π× off. Where checked, the moved verdict is the one that was wrong (worked example below).
- **The door is the cast DESIGN's margins clause (i) forbids** (sure). The clause says "a site no door honestly fits is a finding, not a cast". `join::off_conic_slack` routes a hand-written slack through the door, which the README also forbids.
- **The exposure is wider than 66.** `sector_shape::metering_arm` is a public function that turns a bare number into a lever through one pinned site. Its 9 callers are invisible to the pin. `Margin::sagitta` and `Margin::over_lever` take their arm as a bare number.
- **"A union's read is a max" is half right.** It holds where both regions are read from one pivot. Where operands are pinned apart, the honest read across both is a sum, through `ExtentBall::lever_from_within`.
- **`profile`'s six are a layering problem, not a sourcing one.** Each is two points' distance or an arc's radius. The two-point read lives in a crate above `profile`.

**What each site becomes.**
- **A read the home makes** (about 46), accepting that verdicts move:
  - **Sector, germ and `census` wedge arms:** the edges' own `Lever`. `metering_arm` is deleted, not re-sourced.
  - **Sums:**
    - the larger of the two where one pivot serves both (`rim_wedge`);
    - `lever_from_within` where the pivots stand apart (`eval::measure`, the mate bundle). Editor-core's own `Arm(f64)` lever type goes.
  - **Arc lengths and parameter spans** (`offset_axial`, `ring_path`, `pose`, `reduce`, `census`): the span's read from the corner.
  - **A carrier's stored term** (Villarceau's `R`, the spiric chart's `R`): the consumed circle's or oval's own extent.
  - **`vtxfac`'s ball, rays, a marched polyline, a mitre:** the geometry each consumes.
  - **`chord_join`'s oval:** the oval's ball. Its larger half-width falls short of it.
  - **`trace::plane_misses`:** it levers at the plane's stored origin, so one plane stored with two origins gives two verdicts. It re-anchors (`anchored_at`) and reads the scene's extent.
  - **A unit arm** (`fate::sign`, `overlap::one_sense`): 1 m is the hidden length scale `[tolerance.epsilon]` rules out. The corner's edges and the faces' extent give the arm.
- **A sourced arm times a pure number, or less an offset.** The number moves into the reading and the offset into the band:
  - `leaned_lever`'s lean multiplies the reading;
  - `pcurves` joint room (`d − 2ε`): lever at `d`, with the ε through `Band::padded`;
  - `pose` route (`length + 2e`): the home's widen-by-tolerance read.
- **The length is the quantity measured:** `Margin::of` or the quotient door. This covers `props` `off_axis` and `intersect`'s five apex reaches.
- **The length is a tolerance.** The question is about the band, and a band answers it:
  - `validate::resolved_as` is `K·rounding` decided in `(ε, Kε)`. It becomes `Margin::of(rounding)` decided in `(ε/K, ε)`. The verdict is the same, and it is now stated honestly. The open site-rounding row may retire it.
  - `dihedral` (2): the true arm, which every caller holds, in the band of the tolerance offered.
  - `join` `off_conic_slack`: `Band::padded`.
- **A rate** (`pcurve_cache` focal β; unsure): the metric doors.
- **A zero that means "no arm"** (`ring_path`'s lead piece): that piece has no lever field.

**A record holds what its verdicts can read from their own pivot.** The contract already says a datum and its lever are read at the same point.
- `SsiDomain::extent` is a bare number that four verdicts read, each pinned at a different point. They are the boundary strip, the tube's transversality, the march at its current point, and the section's crossing. One carried length cannot be the reach from all four.
- Its producers already hold a ball. The record holds that ball, and each verdict reads its lever from its own pivot.
- This also leaves one description of the domain where there are two today: `center` and `half_extent` sit beside `extent`.
- A record whose own geometry fixes one region and one pivot keeps its lever. A sector's arm, read at its own vertex, is the example.
- This needs no new text (likely).

**`profile`.** The point-level reads move down to `geom-core`, beside `Lever`: two points' distance, and a point set's reach from a pivot. Covers and `Reach` stay in `geom_brep`.
- `Seg` and the sugar legs return `Lever`s built from those reads: a line's chord, and an arc's lesser of radius and chord.
- Rejected: `profile` depending on `geom_brep`, a sketch crate leaning on the B-rep crate for a chord.
- Rejected: a second extent home inside `profile`. It would need the same point read, and it would hand `profile` the bare-length door.
- Confidence: likely.

**Worked example: `rim_wedge::same_circle`.** It levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r. The honest arm is the larger radius.
- Where θ·r lies between Kε/2 and Kε, today's arm reads past Kε and decides "not one circle". The honest read is in band, and refuses.
- Where θ·r lies between ε/2 and ε, today's arm refuses a coincidence that the honest read accepts.

**Ratified text.** All of it is the lever bullet of `crates/geom-core/README.md`, "Float rounding", written on PR 6329 under your approval. The reply there said arms that fit no source would come back to you; this is that return. The three sources and "rates are not levers" stay. `docs/DESIGN.md` is unchanged. Add:

1. > "A length that is the reading itself (a residual, an offset, a stand-off) is the margin, and a tolerance is the band's: neither is an arm. A dimensionless factor belongs to the reading, not the arm. The point-level reads (two points' distance, a point set's reach from a pivot) live beside `Lever` in `geom-core`; covers and `Reach` stay in `geom_brep`."
2. > "A lever is never shorter than the region its verdict governs and no longer than the site's own measure of it: a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree."

Sentence 2 is the rule `extent.rs`'s code docs hold today; the README does not carry it. About 20 of the sites to be re-read are arcs or splines, where the two bounds differ and the wrong one gives a wrong answer. It is separable: declining it changes nothing above.

**The door and the gate.** Both doors and `lever-sources.counts` are deleted. The gate keeps its two other holds: the extent's own door stays in its home, and the fixtures stay pinned.

What stays possible:
- A lever written by hand outside the door, as `Margin::of(x·len)`. That is why the sites that are not levers get honest doors, not only a ban.
- A constructor fed a caller's number, the two-point read included. The constructor-doors work you asked for on PR 6380 is closing this.
- `sagitta`'s and `over_lever`'s bare arms.

Deleting the doors is trivially reversible. If an arm with no source ever appears, a fourth source can be ratified then.

**Not recommended.**
- **A fourth source, "a length the site measures itself".** Any site could call its own number that, which is the vouched length the rule exists to stop.
- **Keeping the door as a list that only shrinks.** You declined this on PR 6380 ("keep the text and fix").

**Feasibility, kept apart from the ranking.** Perhaps 30 sites move verdicts by more than bits, and each wants its own differential.

**Confidence.**
- No fourth source: likely. About 35 sites are known only from their stated reasons.
- Bits are not a reason: sure.
- The door is a cast: sure.
- The 7/46/13 split: likely.
- The `rim_wedge` arm: likely.
- `plane_misses` depends on the stored origin: sure.

## For the orchestrator

- **No remaining differences from A on substance after this round.**
  - The text: two sentences.
  - "Never carried": qualified by the same-point test.
  - `SsiDomain` holds its ball.
  - `validate` and the `pcurves` joint room are re-posed as above; both are unit details.
  - A's two corrections are accepted.
- **The `profile` question is dropped.**
- **Corrections taken:**
  - The spiric torus ball (`geom/src/curves.rs`, `R + r`) is not fixed by lowering the point reads, so the gate's exempt item stays.
  - `sign_of` does not fit `fate::sign`; the corner's edges give its arm.
- **Doc fix:** `Lever::max`'s doc ("a read of the union") holds only from a shared pivot.
- **Hidden levers to file:**
  - `metering_arm`: 9 callers invisible to the pin. Delete it, do not re-source it.
  - `Margin::sagitta` (12 production calls) and `Margin::over_lever` (44 calls) take bare arms.
  - Both belong in "Hidden levers" on `extent-reads-made-outside-the-extent-home`.
  - Also file `trace::plane_misses`'s stored-origin pivot; it is a defect whatever Ev decides.
- **Off the question:**
  - `Lever::curvature` appears to have no callers (A's grep); I did not verify it.
  - Check whether `intersect`'s apex rounding charges are on the site-rounding row.
- **The filing row is stale:**
  - Its title says 74 and its body 63; the tree has 66.
  - It misses `star`, `fan`, `chord_rows` and `coincide::replay`, and it has no fixture family for the 7 test sites.
- **Sequencing:** the fixtures and pure bit-movers can go before Ev answers. Those are `metering_arm`'s callers, the line spans, `leaned_lever` and the quotient forms. The new `geom-core` point reads should take geometry from the start, together with `oneround/lever-doors`.
- **Not verified:** about 35 sites are classified from their comments only. `pcurve_cache` focal β, `unit_sign` and `coincide::replay` are unsure. No builds were run.
