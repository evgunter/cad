**Reconciliation round 1: answers.** Your read of where the reports stand is right. One addition: B found exposure I missed (`sector_shape::metering_arm`, `Margin::sagitta`, `Margin::over_lever`), and I have adopted it.

**(1) `profile`'s six arms.**
- *What moved me:* B's lowering is much lighter than the version I weighed and rejected (a dimension-generic ball). Only the point-level reads move. My own option needed a point read inside `profile` anyway: `path`'s corner arm is two anchors' separation, not a `Seg`. So it would have duplicated the read B puts in `geom-core`.
- *Is my argument answered:* yes. I drop the fork. One residue: a public two-point door in `geom-core` is the hand-made-`q` violation the constructor-doors row lists. It is no wider than today's `ExtentBall::point(q).lever_from(p)`, so the constructor-doors work holds it either way.
- *Position now:* B's. I add only that `Seg::arm` and the sugar legs' `arm` return a `Lever` built from those reads, so `profile` names which read is a leg's arm and callers stop passing `T`.

**(2) What a record holds.**
- *What moved me:* "never carried" was too absolute. A record whose own geometry fixes one region and one pivot (a sector's arm at its own vertex) can keep its lever; B's cheaper form is right there.
- *Is my argument answered:* not for `SsiDomain::extent`. A `Lever` field is one number read from one pivot, and four verdicts pinned at different points consume it: the boundary strip, the tube's transversality, the march's `decide_transversality` at `x`, and the section's crossing. The ratified text already says "any point is sound provided the datum and its lever are read at the same one". B's field also discards the centre of a ball that B says the producers already hold. And the record already carries a region beside it (`center`, `half_extent`), so a `Lever` field keeps two descriptions of one domain.
- *Position now:* `SsiDomain` holds its ball (four scalars, the same cost) and each verdict reads from its own pivot. This needs no new ratified text; it applies the sentence above.

**(3) What the ratified text gains.**
- *What moved me:* B is right that my definition of a lever restates `[tolerance.epsilon]`, and a second statement is itself a defect. My union item is a code-doc fix (`Lever::max`), not README text. My record item is the existing sentence applied. All three are dropped as text.
- *Is my argument answered:* not on which bound. B's R1 says "the edge's extent read", and the home has two (`edge_extent_inf`, `edge_extent_sup`) plus `LeverBounds`. The rule choosing between them is in `extent.rs`'s function docs only, and the wrong one is the wrong-answer direction. About 20 of the re-read sites are arcs or splines, where the two differ.
- *Position now:* two sentences. B's (adopted), and one on which bound, separable.

## For Ev

**Recommendation (likely): no fourth source. Keep the three, delete the `unsourced` doors and their pin, and add two sentences to the contract.** Nothing here is a fork between defensible end states; the second sentence can be declined on its own.

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
  - *Unit vector times an extent* (sectors, germs, the `census` wedge): the edges' own `Lever`. `metering_arm` is deleted, not re-sourced.
  - *Sums of radii or reaches*: the larger where both are read from one pivot (`rim_wedge`). Where one pivot stands off the other, the sum is honest and the home already has that read (`lever_from_within`): `eval::measure`, the mate bundle. Editor-core's own `Arm(f64)` lever type goes.
  - *Arc lengths and parameter spans* (`offset_axial`, `ring_path`, `pose`, `reduce`): the span's read from the corner. `offset_axial` chose arc length because a closed edge's chord is zero; the home's read now handles closure.
  - *A carrier's term* (Villarceau's `R`, the spiric chart's `R`): the consumed circle's or oval's own extent.
  - *`vtxfac`'s ball*: a ball about the point out to a sourced `Lever`, not a number.
  - *A ray, a marched polyline, a mitre*: the segment, the points, the edges they consume.
  - *`plane_misses`* levers at the plane's stored origin, so one plane stored with two origins gives two verdicts. It re-anchors (`anchored_at`) and reads the scene's extent.
  - *A unit arm on a pure number* (`fate` `sign`, `overlap` `one_sense`): 1 m is the hidden length scale `[tolerance.epsilon]` rules out. Both sites hold geometry (the runs' reach, the faces' extent).
- **A sourced arm times a pure number, or less an offset:** the number moves into the reading, the offset into the band.
  - `star::order` `leaned_lever`: the lean multiplies the reading.
  - `pcurves` joint room (`d − 2ε`): lever at `d`, the ε through `Band::padded`.
  - `pose` `route` (`length + 2e`): the home's widen-by-tolerance read.
- **The length is what is measured:** `Margin::of` or the quotient door. `props` `off_axis` (an offset times `cos α`); `intersect`'s apex reaches (5, a stand-off over a slope).
- **The length is a tolerance:** the question is about the band and is answered with a band.
  - `validate::resolved_as` is `K · rounding` in the band (ε, Kε). It becomes `Margin::of(rounding)` in the band (ε/K, ε): the same verdict, said honestly.
  - `dihedral` (2): the true arm, which every caller holds, in the band of the tolerance offered.
  - `join` `off_conic_slack`: `Band::padded`.
- **A rate** (`pcurve_cache` focal β; unsure): the metric doors.
- **A zero meaning "no arm"** (`ring_path`'s lead piece): that piece has no lever field.

**A record that several verdicts read holds its region, not a length.** The contract already says "any point is sound provided the datum and its lever are read at the same one". `SsiDomain::extent` is a bare number that four verdicts read, each pinned at a different point (the boundary strip, the tube's transversality, the march at its current point, the section's crossing). One carried length cannot be the reach from all four.
- Its producers have a ball in hand. The record holds that ball (four scalars, the same cost) and each verdict reads its own lever from its own pivot.
- The record already carries `center` and `half_extent` beside `extent`; holding the ball leaves one description of the domain, not two.
- A record whose own geometry fixes one region and one pivot (a sector's arm at its own vertex) may keep the lever.
- Confidence: likely.

**`profile`.** The point-level reads (two points' distance, a point set's reach from a pivot) move down to `geom-core`, beside `Lever`. Covers and `Reach` stay in `geom_brep`.
- `profile`'s `Seg` and legs return `Lever`s built from those reads: a line's chord, an arc's lesser of radius and chord.
- `profile` then depends on nothing new.
- Rejected: `profile` depending on `geom_brep` (a sketch crate depending on the B-rep crate for a chord), and a second extent home inside `profile` (it would need the same point read again).
- Confidence: likely.

**Worked example: `rim_wedge` `same_circle`.** It decides whether two rims are one circle, and levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r: the honest arm is the larger radius.
- With θ·r between K·ε/2 and K·ε, today's arm reads past K·ε and decides "not the same circle". The honest read is in band and refuses.
- With θ·r between ε/2 and ε, today's arm refuses a coincidence the honest read accepts.

**Ratified text.** All of it is the lever bullet of `crates/geom-core/README.md`, "Float rounding". Agents wrote it on PR 6329 and you approved it; the reply that explained the three sources said the arms were unclassified and "any that don't come back to you". The three sources and "rates are not levers" stay. `docs/DESIGN.md` is unchanged. Add:

1. > "A length that is the reading itself (a residual, an offset, a stand-off) is the margin, and a tolerance is the band's: neither is an arm. A dimensionless factor belongs to the reading, not the arm. The point-level reads (two points' distance, a point set's reach from a pivot) live beside `Lever` in `geom-core`; covers and `Reach` stay in `geom_brep`."
2. > "A lever is never shorter than the region its verdict governs and no longer than the site's own measure of it: a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree."

Sentence 2 is the rule `extent.rs`'s docs hold today; the README does not carry it. About 20 of the re-read sites are arcs or splines, where the two bounds differ and the wrong one gives a wrong answer. It is separable: declining it changes nothing above.

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

## For the orchestrator

**Where I still differ from B after this round.**
- **`SsiDomain::extent`.** B makes it a `Lever` built by its producers; I make the record hold its ball, with each of the four verdicts reading from its own pivot. My reasons are in the preface. If B's premise is that all four verdicts want the pivot-free diameter, that is checkable per verdict and I have not done it. I read the field's definition (`ssi.rs`, `SsiDomain`) and the four call sites' lines, not their derivations.
- **Sentence 2 (which bound).** B adds none. I keep it, marked separable. If it does not go to Ev it still wants a row: nothing ratified says which of `edge_extent_inf`, `edge_extent_sup` or `LeverBounds` a re-sourced site reads.

**Converged this round.**
- **`profile`**: B's lowering to `geom-core`; my fork is dropped.
- **Text**: my items 1 (definition) and 3 (union) are dropped. The union point becomes a doc fix on `Lever::max`, whose "a read of the union of their extents" holds only from a shared pivot.
- **`validate::resolved_as`**: B's band re-pose.
- **`pcurves` joint room**: B's lever at `d` plus `Band::padded`. I had called it a rate.
- **Standing-off sums**: B's `lever_from_within`, with no new home read.

**How I checked.**
- `rg 'Unsourced lever' crates` gives 66 markers, matching the 66 pins.
- First round: every marker comment, and the code at about 25 sites.
- This round: `sector_shape::metering_arm` (9 references by grep, confirming B), the `SsiDomain` struct and `pair_section::domain_about`, `profile`'s `arc_lever`, `path` and `sugar` arms, and `geom/src/curves.rs`'s gate-exempt item.
- The test-module check on the 7 fixtures was a scan for the nearest `#[cfg(test)]` above each line; `link/chord_rows.rs` is inferred from its `mod` line.

**Corrections to B's report.**
- B's premise 4 says `geom`'s spiric torus ball "is the same problem" as `profile`. It is the same layering problem, but the point-level lowering does not fix it: that arm is `major_radius + minor_radius`, a carrier's ball and not a point read. The gate's exempt item stays unless the spiric reads a cover.
- B offers `sign_of` for `fate` `sign` "where the reading is exact". `Margin::sign_of` is for `Band::enclosure_sign` only, and a half-turn count summed from `atan2` is not exact. The corner's edges are the arm.

**Errors and staleness in the filing row.**
- Title 74, body 63, tree 66.
- Gone from the tree: `splitting::containment` `extent_from`, `boolean::recl`'s corner ball, `checks::at_rest` `constant_pairing`, `unglued_flat_sector`.
- New and not on the row: `star::order` (2), `star::cut`, `coincide::replay`, `link::chord_rows`, `boolean::fan` (2).
- It has no fixture family for the 7 test sites.

**Not checked.**
- No builds were run and nothing was measured. The count of about 30 verdict-moving sites and the count of about 20 arc or spline sites are estimates from the arm formulas.
- `pcurve_cache`'s focal β and `unit_sign` are classified by their comments and one read each; unsure. `unit_sign` reads to me as the circle's axial rise at its own radius, an extent lever.
- `coincide::replay`'s power-of-two arm is a symbolic-lane device; I take it as a read from above, unverified.
- I did not open any `analysis/*` branch.

**Off the question (file or route).**
- `Lever::curvature(kappa)` has no callers under `crates` (grep), against 51 `curvature_radius` and 3 `levered_inv`. It is dead and is a second spelling of `levered_inv`'s curvature case.
- `trace` `plane_misses` reads a verdict off a plane's stored origin, against `[tolerance.residuals]`. This is a defect whatever Ev decides; it belongs with "Pivot not the centre" on `extent-reads-made-outside-the-extent-home`.
- `Margin::sagitta` and `Margin::over_lever` taking bare arms (B's finding): append to that row's "Hidden levers".
- `intersect`'s apex block still charges rounding (`gap_err`, bare `f64::EPSILON` multiples) inside the arms pinned here. I did not check it is on `site-rounding-charges-owed-a-re-posing`.
- editor-core `mate::coset::Arm`, a second lever type: the same class as `consumed-extent-wraps-extent-ball`.

**Sequencing, not design.** The fixtures (7) and the pure bit-movers can go before Ev answers: `[simple.conditioned]` already covers them. That means `metering_arm`'s callers, the line parameter spans, `leaned_lever`, and `intersect`'s quotient forms. The new `geom-core` point reads should take geometry from the start, sequenced with `oneround/lever-doors`. The verdict-moving sites and `SsiDomain` wait for the answer.