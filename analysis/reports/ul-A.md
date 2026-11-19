## For Ev

**Recommendation (likely): no fourth source. Keep the three, delete the `unsourced` door and its pin, and add the sentence the contract lacks: what makes a length a lever.** One real fork is left for you, on `profile` (end of this section).

**What the 66 arms are.** They are not one kind of thing waiting for a source. I read every call's stated reason and about 25 of the sites in code; the counts are mine.

- **7 are tests' stated scales**: `star::cut`, `star::order`'s probe, `boolean::fan` (2), a `join` fixture, `link::chord_rows`, `ssi::certify`'s test twin. They belong on the fixture door that already exists.
- **About 46 are real levers whose geometry is in hand.** The site worked the arm out as a number itself, or carried it in a record, instead of asking the extent's home. Their source already exists: the consumed extent.
- **About 13 are not levers.** The product `x · arm` is something else, written through `levered` because that door would take it.

**Premise check.**

- **"The honest read would move verdicts in the last bits" is not a reason to stay unsourced** (sure). `[simple.conditioned]` is your own rule from PR 6329: pose it well, then treat `f64` as exact. No site may protect its bits.
- **Many of these moves are not last-bit moves.** A sum against a larger-of is up to 2×, an arc length against a reach up to π×, a torus's major radius against the circle's own. These arms are wrong, and the door has let them stand.
- **"The union's read is a max" is half right.** It holds only when both regions are read from one pivot. Read each from its own reference point and the honest read is across both, which is a sum.
- **The door is the cast `[tolerance.margins]` forbids** (sure). `Margin::levered(x, Lever::unsourced(y))` makes a margin of any product. That clause says a site no door fits "is a finding, not a cast".

**What the contract should say** (new text for `crates/geom-core/README.md`, "Float rounding"; the wording is yours):

1. **What a lever is.** The farthest that the geometry a verdict governs stands from the point where the reading's datum is pinned. This is `[tolerance.epsilon]`'s "reach at which the angle displaces the geometry it governs", stated so a site can check itself. The three sources are the three ways to have it: read off the region, the curvature arm for a tangency, the session box where no region is in hand.
2. **A lever is read at the verdict, not carried.** A record that needs one later holds its region (a `Reach` or its ball), never a length. The 46 trace back to records that store an arm:
   - `SsiDomain::extent`, a bare `f64`;
   - a sector's `arm`;
   - editor-core's `Arm(f64)`, a second lever type whose doc calls it "the one door a lever is formed through";
   - `Piece::Segment`'s `lever`;
   - `SectionOval`'s `extent`.
3. **Two regions are read as one region from the reading's pivot.** `Lever::max` is that read only from a shared pivot.
4. **A tolerance is never an arm, nor is a length the reading is about.** ε times a pure number is a threshold and belongs on the band's side. A length times or over a pure number is a length already.
5. **Which bound.** A lever is never shorter than the region it governs and no longer than the site's measure of it; a reading that acts on every sign reads both bounds. `extent.rs`'s docs hold this today and the README does not. Most of the 46 never said which bound they are.

**What each family reads instead.**

- **Family 1, "a source exists" (and most of family 4):** the home's read.
  - *Unit vector times an extent* (sectors, germs, the `census` wedge): the extent's own `Lever`. A sine factor (`leaned_lever`) multiplies the reading, not the arm.
  - *Sums* (`rim_wedge`, `eval::measure`, the mate `Arm`, `pose` `route`): one region enclosing both, read from the pivot. `measure`'s and the mate's sums are honest reads across two separately pinned operands; the home should make that read, and `Arm` goes.
  - *Arc lengths* (`offset_axial`, `ring_path`, `pose`): the span's reach from the corner. `offset_axial` chose arc length because a closed edge's chord is zero; the extent's read now handles closure.
  - *Carrier terms* (Villarceau's `R`, the spiric chart's `R`): the consumed circle's or oval's own region, which the site holds.
  - *`vtxfac`'s ball*: a ball about the point out to a sourced `Lever`. The constructor takes a `Lever`, not a number.
- **Family 3, "a caller's length":** the ratified text already answers it. Either there is geometry and the site reads it, or there is none and the arm is the session box.
  - `SsiDomain` holds its region; its test literals go to the fixture door.
  - A ray is its own segment; a marched polyline is its points.
  - `plane_misses` levers at the plane's stored origin, so the same plane stored with a different origin gives a different verdict. It re-anchors (`anchored_at`) and reads the scene's extent.
  - `ring_path`'s zero placeholder becomes no lever at all: that piece has none.
- **Not levers (family 2 and strays):**
  - *ε or K·ε as the arm* (`dihedral` 2, `join` `off_conic_slack`): each compares a pure number with a constant (`sin θ ≤ 1/K`). Asked through `Band`'s own doors (offered tolerance, padding), with no margin built.
  - *A length over or times a pure number* (`intersect` apex 5, `props` `off_axis`): the quotient door or `Margin::of`.
  - *`K · rounding`* (`validate`): a rounding charge, which goes with `site-rounding-charges-owed-a-re-posing`.
  - *Rates* (`pcurve_cache` `beta`, `pcurves` joint room): the metric doors. The joint room's `− 2ε` is hand slack and goes.
  - *A unit arm on a pure number* (`fate` `sign`, `overlap` `one_sense`): 1 m is the hidden length scale `[tolerance.epsilon]` rules out. Both sites hold geometry (the runs' reach, the faces' extent).

**Worked example: `rim_wedge` `same_circle`.** It decides whether two rims are one circle, and levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r: the honest arm is the larger radius.
- With θ·r between K·ε/2 and K·ε, today's arm reads past K·ε and decides "not the same circle". The honest read is in band and refuses.
- With θ·r between ε/2 and ε, today's arm refuses a coincidence the honest read accepts.

**The door and the gate.** `Lever::unsourced` and `ExtentBall::unsourced` are deleted, and `lever-sources.counts` with them. The gate keeps what it holds now: the extent's door only in the listed homes, and the fixture pins.

What this still leaves possible:
- `Margin::of(x * len)`, a lever outside the door that no gate sees. Closing the door raises the pull toward it, which is why the non-levers need honest doors rather than only a ban.
- A geometric constructor fed a caller's number (the constructor-doors work already under way).
- A wrong pivot or bound inside the home.

Deleting the door is trivially reversible; each verdict move is its own change.

**Options not recommended.**
- **A fourth source, "a stated length"**: makes the sentence true by emptying it; any number is an arm again.
- **Keeping the door as a permanent shrink-only list**: what was put to you on PR 6380, where you leaned to "keeping the text and fixing".

No site I read needs a new source (likely; the 40 I know only by their comments could hold one).

**The fork: `profile`'s six.** They are true extent reads of a 2D leg (its chord; its arm, the lesser of radius and chord), and `profile` depends only on `geom-core`.
- **(a) `profile` is the extent home for its own geometry.** `Seg` returns `Lever`s, and the gate lists it beside the one `geom` item it already lists. The README's "It is one type" becomes one home per geometry representation. Small, and ownership is clean: `profile` owns `Seg`. Cost: two homes to keep to one rule.
- **(b) One home, lowered to where both reach** (`geom-core`), with a ball generic over dimension. One rule in one place, but new machinery for six sites, and `Reach`'s substance (covers) cannot move down anyway.

I lean (a) (likely). A sketch leg is not a carrier, edge or face, so `[one-way.places]`' one-cover row is not breached.

**Ratified text this changes.** All of it is in the README's "Float rounding" section, written by agents on PR 6329 (commit `31576a6c6`) and approved by you with "looks good". The reply that explained the three sources to you said the arms were not yet classified and "any that don't come back to you": this is that return.
- The lever bullet gains items 1 to 5.
- "It is one type" changes under (a).
- `[tolerance.epsilon]` is unchanged.

**Feasibility, apart from the ranking.**
- Perhaps 30 sites move verdicts beyond bits, and each wants its own differential.
- A record holding a region costs a cover read per verdict where it held a number. You flagged runtime on PR 6329; the ball is the cheap form.

**Confidence.**
- No fourth source: likely.
- Bits are not a reason: sure.
- The door is a cast: sure.
- The 7/46/13 split: likely, ±5 between the last two.
- The `rim_wedge` arm: likely.
- `plane_misses` depends on the stored origin: sure.

## For the orchestrator

**How I checked.**
- `rg 'Unsourced lever' crates` gives 66 markers, matching the 66 pins.
- I read every marker comment, and the code at about 25 sites: `dihedral` (2), `join` `off_conic_slack`, `validate`, `pcurves::decision`, `intersect` (5), `fate`, `overlap`, `pcurve_cache` (focal beta and the Villarceau block), `offset_axial`, `ring_path` (2), `profile` (`seg` and `path`), `props::curved`, `vtxfac`, `trace` (2), editor `trace`, `rim_wedge`, `measure` `arm`.
- The other 40 or so are classified from their comments only.
- The test-module check on the 7 fixtures was a scan for the nearest `#[cfg(test)]` above each line. `link/chord_rows.rs` I inferred from its name and its `mod` line in `link.rs`, not from a cfg read.

**Errors and staleness in the brief and the filing row.**
- **The row's site list is stale.** Its title says 74 and its body 63; the tree has 66. Gone from the tree: `splitting::containment` `extent_from`, `boolean::recl`'s corner ball, `checks::at_rest` `constant_pairing`, `unglued_flat_sector`. New and not on the row: `star::order` (2), `star::cut`, `coincide::replay`, `link::chord_rows`, `boolean::fan` (2).
- **Seven pinned sites are test code.** The row's five families do not have a "fixture" family, so these are miscounted as open questions. They can move to the fixture door without Ev.
- **"Sums of radii … whose union read is their `max`" is wrong as a general statement** (see the premise check). `Lever::max`'s doc has the same imprecision.
- **Family 4's "a rate with no span" (`unit_sign`) reads to me as an extent lever**: tangent·axis times the circle's radius is the circle's axial rise. I did not derive it fully; unsure.

**What I assumed.**
- The `oneround/lever-doors` unit lands "constructors take geometry, not numbers". My `ExtentBall`-about-a-`Lever` suggestion for `vtxfac` is consistent with that; a home function taking two radii would not be.
- `Band`'s existing `tolerance_deciding`, `offer_deciding` and `padded` can carry the `dihedral` and `join` questions. I read their signatures, not their bodies.
- The shallow checkout did not block provenance: `git log --all -S` found the README sentences at `31576a6c6` (2026-11-02), plus one unrelated later commit, `66d3c8c78`.

**Not checked.**
- No builds were run and nothing was measured. The count of about 30 sites that move beyond bits is an estimate from the arm formulas.
- The runtime cost of records holding a `Reach` is unmeasured.
- I did not open the branch `analysis/design-fork/unsourced-levers` (local and on origin), in case it is the other lane's.

**Off the question (file or route as you see fit).**
- `Lever::curvature(kappa)` has no callers anywhere under `crates` (grep), against 51 `curvature_radius` and 3 `Margin::levered_inv`. It is dead, and it is a second spelling of `levered_inv`'s curvature case.
- `trace` `plane_misses` reads a verdict off a plane's stored origin, which `[tolerance.residuals]` ("whatever … a meter reads is a function of the geometry") rules out. This is a defect whatever Ev decides; it belongs with "Pivot not the centre" on `extent-reads-made-outside-the-extent-home`.
- `intersect`'s apex block still charges rounding (`gap_err`, bare `f64::EPSILON` multiples) inside the arms pinned here. Check it is listed on `site-rounding-charges-owed-a-re-posing`; I did not.
- editor-core `mate::coset::Arm` is a second lever type with its own "one door" doc. It is the same class as `consumed-extent-wraps-extent-ball` and could be filed beside it.
- The README's lever bullet does not state the bound-direction rule that `extent.rs` documents (inf for a definite claim, sup for an acted-on Zero, both for a two-sided row). Item 5 proposes carrying it; if Ev declines item 5 it still wants a row.

**Sequencing, not design.** The fixtures (7) and the pure bit-movers can go first, without waiting for Ev: the `[simple.conditioned]` ruling already covers them. That includes the unit-vector-times-extent arms, the line parameter spans in `offset_axial`, `reduce` and `census`, and `intersect`'s `levered_inv` forms. The verdict-moving sites and `profile` wait for the answer.