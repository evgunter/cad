**Reconciliation round 2: the crossed points.** Your read is right. On every crossed point I now think one position, or a meeting of the two, should be forwarded; none of them needs Ev's attention as a choice.

**(1) The README text: forward three sentences, with no separate definition of a lever.**
- *The definition item* should not be forwarded. The README's extent bullet already says "Its lever is the extent's far distance from that point", and `[tolerance.epsilon]` says "the reach at which the angle displaces the geometry it governs". A third statement puts one rule in two places in the same section. Ev's own comment on PR 6329 objected to exactly that ("mentions the limits on eps and K multiple times").
- *The records item* should be forwarded. I dropped it as implied by the same-point sentence, but a careful reader of that sentence still proposed a `Lever` field for `SsiDomain`, so the consequence needs stating.
- *The tolerance/measurand sentence and the which-bound sentence* are met already.
- Whether the definition is restated is a detail, not worth Ev's attention.

**(2) How absolute "never carried" is: forward a line drawn by what the record is, not either earlier position.**
- A rough scan finds about 34 struct fields of type `Lever` or `LeverBounds` outside the extent's home. Many are reading records that pair a datum with the arm it was read at (`dihedral`'s `Reading`, `vtxfac`'s `HeldRead`, `carrier_pose`'s `PlanePair`). "No record holds a lever" would outlaw those, and they are sound: the arm travels with its own pivot.
- My exception ("one region, one pivot may keep it") is unenforceable by type. It also leaves the sector's stored `arm` free to disagree with its edges; `boolean::fan`'s fixtures build sectors with an arm of 1 and no edge.
- Forwarded line: a lever belongs to a reading. A record that describes geometry for verdicts still to come (a domain, a sector, a path piece, a section oval, a mate bundle) holds its region. Every record on B's list is of that kind, so B's list stands and my sector exception is withdrawn.
- A unit settles which of the 34 are which.

**(3) `validate::resolved_as`: forward the routing to the site-rounding row.** In the final state the function does not exist; that row owns its removal. The band form is only what it becomes if it outlives the door. A detail.

**(4) `pcurves` joint room: both are defensible; a unit settles the door.** What is common is what matters: `d` is a point's distance from the axis (a point read), and the `− 2ε` leaves the arm for the band. The margin is a quarter chord, `d·sin(step/4)/2`. The metric door would give the arc, which over-states a separation claimed from below, so I lean to the lever at `d`. Not worth Ev's attention.

**(5) My two corrections to B** stand, in the orchestrator section below.

**`profile` should not be put to Ev as a question.** Both reports reject a second home in `profile` and reject `profile` depending on `geom_brep`. B's reason against a second home is the decisive one: `profile` would hold the bare-length door, and its natural call reads a stored `f64`. One answer is plainly right. What Ev signs is the README clause that places the point-level reads, with the rejected alternatives in one line.

## For Ev

**Recommendation (likely): no fourth source. Keep the three, delete the `unsourced` doors and their pin, and add three sentences to the contract.** No choice between defensible end states is left; the third sentence can be declined on its own.

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
  - *Sums of radii or reaches*: the larger where both are read from one pivot (`rim_wedge`). Where one pivot stands off the other, the sum is honest and the home already has that read (`lever_from_within`): `eval::measure`, the mate bundle.
  - *Arc lengths and parameter spans* (`offset_axial`, `ring_path`, `pose`, `reduce`): the span's read from the corner. `offset_axial` chose arc length because a closed edge's chord is zero; the home's read now handles closure.
  - *A carrier's term* (Villarceau's `R`, the spiric chart's `R`): the consumed circle's or oval's own extent.
  - *`vtxfac`'s ball*: a ball about the point out to a sourced `Lever`, not a number.
  - *A ray, a marched polyline, a mitre*: the segment, the points, the edges they consume.
  - *`plane_misses`* levers at the plane's stored origin, so one plane stored with two origins gives two verdicts. It re-anchors (`anchored_at`) and reads the scene's extent.
  - *A unit arm on a pure number* (`fate` `sign`, `overlap` `one_sense`): 1 m is the hidden length scale `[tolerance.epsilon]` rules out. Both sites hold geometry (the runs' reach, the faces' extent).
- **A sourced arm times a pure number, or less an offset:** the number moves into the reading, the offset into the band.
  - `star::order` `leaned_lever`: the lean multiplies the reading.
  - `pose` `route` (`length + 2e`): the home's widen-by-tolerance read.
  - `pcurves` joint room (`d − 2ε`): `d` is the vertex's distance from the axis, a point read; the ε goes to the band.
- **The length is what is measured:** `Margin::of` or the quotient door. `props` `off_axis` (an offset times `cos α`); `intersect`'s apex reaches (5, a stand-off over a slope).
- **The length is a tolerance:** the question is about the band and is answered with a band, with no margin built.
  - `dihedral` (2) compares `sin θ` with `1/K`: the true arm, which every caller holds, in the band of the tolerance offered.
  - `join` `off_conic_slack`: `Band::padded`.
  - `validate`'s `K · rounding` is a rounding charge. It goes with the open row on site rounding charges.
- **A rate** (`pcurve_cache` focal β; unsure): the metric doors.
- **A zero meaning "no arm"** (`ring_path`'s lead piece): that piece has no lever field.

**A lever belongs to a reading; a record that describes geometry holds its region.** A lever means something only from the point its reading's datum is pinned at. The contract already says so: "any point is sound provided the datum and its lever are read at the same one".
- **`SsiDomain::extent`** is a bare number that four verdicts read, each pinned at a different point: the boundary strip, the tube's transversality, the march at its current point, the section's crossing. One carried length can be sound from every pivot only as a diameter, and that over-states the arm by up to 2×.
  - Its producers have a ball in hand. The record holds that ball (four scalars, the same cost) and each verdict reads its own lever.
  - The record already carries `center` and `half_extent` beside `extent`; holding the ball leaves one description of the domain.
- **The same change** applies to the other records that describe geometry for verdicts still to come:
  - a sector's `arm`, where tests today build sectors whose arm matches no edge;
  - editor-core's `Arm(f64)`, a second lever type;
  - `Piece::Segment`'s `lever`;
  - `SectionOval`'s `extent`.
- **A reading record** that pairs a datum with the arm it was read at keeps it: the arm travels with its own pivot.
- Confidence: likely.

**`profile`.** The point-level reads (two points' distance, a point set's reach from a pivot) move down to `geom-core`, beside `Lever`. Covers and `Reach` stay in `geom_brep`, so "It is one type" is untouched.
- `profile`'s legs return `Lever`s built from those reads: a line's chord, an arc's lesser of radius and chord.
- `profile` depends on nothing new and never holds a bare-length door.
- Rejected: `profile` depending on `geom_brep` (a sketch crate depending on the B-rep crate for a chord). Also rejected: `profile` as a second extent home. It would hold the bare-length door, and its natural call reads a stored `f64` that the gate cannot tell from an honest read.
- Confidence: likely.

**Worked example: `rim_wedge` `same_circle`.** It decides whether two rims are one circle, and levers the axes' tilt θ at `r_a + r_b`. Radius and centre are decided first, so the tilt pivots at the shared centre and moves a rim point by θ·r: the honest arm is the larger radius.
- With θ·r between K·ε/2 and K·ε, today's arm reads past K·ε and decides "not the same circle". The honest read is in band and refuses.
- With θ·r between ε/2 and ε, today's arm refuses a coincidence the honest read accepts.

**Ratified text.** All of it is the lever bullet of `crates/geom-core/README.md`, "Float rounding". Agents wrote it on PR 6329 and you approved it; the reply that explained the three sources said the arms were unclassified and "any that don't come back to you". The three sources and "rates are not levers" stay. `docs/DESIGN.md` is unchanged. Add (the wording is yours):

1. > "A length that is the reading itself (a residual, an offset, a stand-off) is the margin, and a tolerance is the band's: neither is an arm. A dimensionless factor belongs to the reading, not the arm. The point-level reads (two points' distance, a point set's reach from a pivot) live beside `Lever` in `geom-core`; covers and `Reach` stay in `geom_brep`."
2. > "A lever belongs to a reading, read from the point that reading's datum is pinned at. A record that describes geometry for verdicts still to come holds its region, never a lever."
3. > "A lever is never shorter than the region its verdict governs and no longer than the site's own measure of it: a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree."

Sentence 3 is the rule `extent.rs`'s docs hold today; the README does not carry it. About 20 of the re-read sites are arcs or splines, where the two bounds differ and the wrong one gives a wrong answer. It is separable: declining it changes nothing above.

**The door and the gate.** `Lever::unsourced` and `ExtentBall::unsourced` are deleted, and `lever-sources.counts` with them. The gate keeps the extent's door in its home, and the fixture pins.

What stays possible:
- `Margin::of(x * len)`, a lever outside the door that no gate sees. Closing the door raises the pull toward it, which is why the non-levers get honest doors rather than only a ban.
- A constructor fed a caller's number, the two-point read included. That is the constructor-doors work already under way.
- `sagitta`'s and `over_lever`'s bare arms.

Deleting the door is trivially reversible: if an arm with no source ever appears, a fourth can be ratified then.

**Options not recommended.**
- **A fourth source, "a length the site measures itself"**: any site could call its number that. It is the vouched length the rule exists to stop.
- **Keeping the door as a shrink-only list**: what you declined on PR 6380 ("keeping the text and fixing").

**Feasibility, apart from the ranking.** Perhaps 30 sites move verdicts beyond bits, and each wants its own differential. A record holding a ball costs one read per verdict where it held a number.

**Confidence.**
- No fourth source: likely (about 40 sites are known to me only by their comments).
- Bits are not a reason: sure.
- The door is a cast: sure.
- The 7/46/13 split: likely, ±5 between the last two.
- The `rim_wedge` arm: likely.
- `plane_misses` depends on the stored origin: sure.
- Least certain sites: `pcurve_cache`'s focal β and `unit_sign`, the `pcurves` joint room's door, `coincide::replay`'s power-of-two arm.

## For the orchestrator

**Remaining differences from B, as of B's round-1 report.**
- **README text.** B forwards four items, including a definition of a lever. I forward three sentences and no definition. A detail; pick the shorter unless you see a reader the existing two statements fail.
- **Reach of the records rule.** B's item 2 reads "a lever is read at the verdict, never carried". Mine lets a reading record keep the arm it was read at. B's list of records to change is the same as mine now; the difference is only whether the sentence also outlaws the reading records.
- **`profile`.** B puts it to Ev as a question with a lean. I say it is not a question, since both of us reject the alternatives.
- **`pcurves` joint room.** No real difference left: both unsure which door, both agree on `d` and on moving the ε.

**Corrections to B's report.**
- B says `geom`'s spiric torus ball "is the same problem" as `profile`. The layering is the same, but the point-level lowering does not fix it: that arm is `major_radius + minor_radius`, a carrier's ball and not a point read. The gate's exempt item stays unless the spiric reads a cover.
- B's round-0 report offered `sign_of` for `fate` `sign` "where the reading is exact". `Margin::sign_of` is for `Band::enclosure_sign` only, and a half-turn count summed from `atan2` is not exact. The corner's edges are the arm. B's round-1 text no longer says it.
- B's option (a) has `profile` call `Lever::curvature_radius(r)` "taking geometry". Today it takes a bare `T`, and `oneround/lever-doors` is closing that. An arc's radius as a point read (centre to end) avoids the dependency.

**How I checked this round.**
- The scan for lever fields is a Python regex over `crates/*/src` for `name: Lever<…>,` or `LeverBounds<…>,` lines whose enclosing block opens with `struct`, `enum` or a bare variant brace. It excludes `extent.rs` and `predicate.rs` and finds 34.
- It is a heuristic: it can miss tuple fields and misread a multi-line signature.
- I read the names, not the uses, so "many are reading records" is likely, not sure. `pair_section`'s `PairSection.lever` and `CarriedSection.lever` may be geometry records that the rule reaches beyond the 66.
- Earlier rounds: every marker comment, about 25 sites in code, `metering_arm` (9 references), the `SsiDomain` struct, and `profile`'s arm readers.

**Not checked.**
- No builds were run and nothing was measured. The count of about 30 verdict-moving sites and the count of about 20 arc or spline sites are estimates.
- That `SsiDomain`'s four verdicts pivot at different points is read from their call lines, not their derivations.
- `unit_sign` reads to me as an extent lever (the circle's axial rise at its own radius); unsure.
- I opened no `analysis/*` branch.

**Staleness in the filing row.**
- Title 74, body 63, tree 66.
- Gone from the tree: `splitting::containment` `extent_from`, `boolean::recl`'s corner ball, `checks::at_rest` `constant_pairing`, `unglued_flat_sector`.
- New and not on the row: `star::order` (2), `star::cut`, `coincide::replay`, `link::chord_rows`, `boolean::fan` (2).
- It has no fixture family.

**Off the question (file or route).**
- `Lever::curvature(kappa)` has no callers under `crates` (grep). It is dead and is a second spelling of `levered_inv`'s curvature case.
- `Lever::max`'s doc ("a read of the union of their extents") holds only from a shared pivot; a doc fix.
- `trace` `plane_misses`' stored-origin dependence is a defect whatever Ev decides. It belongs with "Pivot not the centre" on `extent-reads-made-outside-the-extent-home`.
- `Margin::sagitta` and `Margin::over_lever` bare arms: that row's "Hidden levers".
- `intersect`'s apex block still charges rounding (`gap_err`, bare `f64::EPSILON` multiples). I did not check it is on `site-rounding-charges-owed-a-re-posing`.
- The roughly 34 lever fields want one pass against sentence 2 once Ev answers; a row.

**Sequencing, not design.** The fixtures (7) and the pure bit-movers can go before Ev answers: `metering_arm`'s callers, the line parameter spans, `leaned_lever`, `intersect`'s quotient forms. The `geom-core` point reads should take geometry from the start, sequenced with `oneround/lever-doors`. The verdict-moving sites and the records wait for the answer.