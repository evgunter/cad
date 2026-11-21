## For Ev

**Recommendation (likely): one final state, with one small split left for you.**

1. **Nobody chooses a bound.** A turn's lever stays two-sided until the decision funnel. A definite sign is proved from the bound from below; a Zero is proved from the bound from above.
2. **A disagreement between the bounds is an in-band reading of one scalar decision.** It is minted once, as a straddle (`k_stats::straddle`, on the predicate's straddle row). It leaves each composite verdict through the in-band arm that verdict already has. No verdict gains a variant.
3. **The doors return the region, not a lever.** The region is points known to lie on it (read from inside) and its cover (read from outside). Each decision reads its lever from the site its own datum turns about.
4. **The site is a line where the datum turns about one** (changed from my first report): the edge's tangent line at a station for the wedge, and the chart's axis at a pole for the joint turn.
5. **The two-sided lever enters only through the `Margin` doors** (changed from my first report). `Margin::levered` and `Margin::sagitta` take both bounds and build a margin that bounds the reading at every lever in between.
6. **The split: what "agree" is on** (one line per option at the end).

**Terms.**
- *Bound from below*: a length the region certainly reaches.
- *Bound from above*: a length it certainly does not pass.
- *Site*: the point or line a lever is measured from.
- *Straddle*: the two bounds give different verdicts.
- *Composite verdict*: one answer folded from several scalar decisions, such as the wedge fold or the must-carry rule.

**Premise check (sure).** Asking which bound each caller needs is the wrong question. A margin of the form x·L, read at both ends of [below, above], serves:
- a definite sign exactly when the bound from below proves it;
- Zero exactly when the bound from above proves it.

So the ratified sentence's three cases are one rule, and the ~41-caller classification is not needed.

The brief says the composite verdicts have nowhere to put a disagreement. That is not so: each already carries in-band readings (`MustCarryVerdict::InBand`, `PairRefusal::InBand`, `SeamUnread::Lever`, `PinMiss::Escalated`, an `Err(Indeterminate)` from rim classification).

"NURBS faces have no cover" contradicts the tree. A spline's cover is its control net, refused only where that net is poisoned (the placeholder) or non-finite.

**The final state, in detail.**
- **Margins.** A two-sided margin is a pair of margins that encloses the reading for every lever between the bounds. Each term is bounded on its own (its least and greatest value over [below, above]), and a sum adds the lower ends and the upper ends. So "datum plus swing" sums and terms of opposite sign are enclosed without assuming the margin grows with the lever.
  - The funnel decides both ends and serves a verdict only where they agree. Otherwise it mints the straddle.
  - The closure-taking `decide_at_lever` goes.
  - `LeverBounds` moves to `geom-core` beside `Lever`. `below()` and `above()` are readable only in the extent home and the margin doors, held by a gate whose pins only fall.
- **One constructor for the bounds,** which refuses loudly when the cover does not hold a point known to lie on the region (below > above). Today `wedge_reach` folds that case away with a max. A face cover measured inside its own vertices is a defect at its source, filed rather than folded.
- **No bound from above** is a typed variant that carries why there is none (the face's uncovered reason). Definite signs are still served from below, so certification is unchanged. A Zero refuses, and the refusal names the missing cover rather than an anonymous invalid margin.
- **Site mint and mid-door faces.** The cover is computed from the face as the door holds it (its surface and its boundary edges' covers), not looked up in the body.
- **The cycle.** The looser lift-free cover stays: any cover is a sound outside read.
- **The site.** A dihedral at a station turns about the edge's tangent line there. A face point at distance d from that line moves by sin θ·d. Distance from a point on the line is longer, so the lever read from a point over-reads, and a definite sign claimed from that over-read is unsound.
  - Worked example: a strip 4 long and 0.05 wide, at 15ε to its neighbour.
    - From the edge's middle (today) it reads 30ε, a corner. From an end station it reads about 60ε, also a corner.
    - From the hinge line it reads 0.75ε: the tangent joint the profile declared.
  - So a site at the station's point, which I recommended before, makes the open row `honeone/the-wedge-reach-counts-a-faces-run-along-its-edge` worse. The two rows are one design.
  - A datum with no position (the cosine of two plane normals in `stands_on`) keeps the edge's middle.
- **The bound from below** may read each boundary edge's own reach, not only vertices (exact on lines and arcs, sampled otherwise, as `edge_reach_bounds` does).

**What moved me, and to what.**
1. **The site.** My point-at-station answer was sound for the bound from above but not for the bound from below: it over-reads definite signs, and the strip example shows it.
2. **Where the funnel lives.** A closure handed to the levered-decision door can be non-monotone, and the sum rows prove both verdicts from above. So my claim "the margin only grows" was not true of every reading. Building the enclosure term by term in the `Margin` doors makes it true by construction.
3. **The missing bound from above.** It should reach the person with its reason. My typed variant already does this, so I keep it.

**Where I have not moved.** On what "agree" is on, I still hold that serving only signs over-refuses one honest case:
- `stands_on` and `sheets_part` act on `sign == Negative` as a bool.
- A reading that is Zero at the bound from below and Positive at the bound from above means "not opposed" at every lever in between.
- Serving only signs refuses it; serving the consumer's question answers `false`, soundly.

The other report's argument is answered on soundness (refusing is sound) but not on completeness. Whether this case occurs is measurable (M3 below), so it is a question only if M3 finds it.

**The split, for you to rule on in one line each.**
- **(S) Sign:** the funnel serves a sign only where the bound that proves it does, and a bool built from a sign inherits that. Simplest: one door, no grouped question.
- **(Q) Question:** a consumer that groups signs asks the grouped question at the decision ("is it definitely Negative?"), and the funnel serves its answer where both ends answer it alike. One more door; it answers the Zero-and-Positive case.
- **My lean:** S unless M3 finds such straddles on valid bodies; then Q. If M3 finds none, Q is a mechanism nothing needs.

**Faithful elaborations an agent may land:**
- composites take the region and pass it to every leaf;
- the two-sided `Margin` doors, term by term, and the straddle through existing in-band arms;
- the bounds' constructor with its invariant;
- the typed missing-bound-from-above variant;
- the site mint's cover from held geometry;
- the lift-free cover kept;
- the datum read at its station or joint (that part of the site rule is the README's same-point clause).

**Yours:**
1. **The ratified sentence's text.** Restate it as the proof rule: "a definite sign is read from below and a Zero from above, whoever acts on it; where they disagree the reading is in band", plus S or Q. It was agent-written and approved on PR 7141.
2. **The site as a line.** It changes "the extent's far distance from that point" to "from the point or line the datum turns about", and moves definite verdicts to Zero (corner to tangent). It should be decided together with the honeone row.
3. **Edges in the bound from below.** It moves verdicts toward deciding definite, which PR 7141 did not pre-accept.
4. **Each class of valid bodies newly refusing** (PR 7140), from M2.

**Sequencing.**
1. First unit: the two-sided funnel, with the site and the bound from below unchanged. Its differential only moves toward refusing: definite verdicts stay bit-identical, and only unproven Zeros refuse.
2. Then the site and the honeone row together.
3. Then edges in the bound from below. Each later unit has its own differential.

**Measurements, with the truth each is judged against.**

- **Oracle lever.** Several measurements need the true reading, and the covers and points are the thing under test, so none of them can be the truth. The truth used is the oracle lever: the region's real far distance from the site, from a dense sample of each face's carrier inside its trimmed boundary at a spacing whose own error is bounded (on the interval lane where it applies). For a line site, the oracle departure is the largest sin θ·distance-from-line over those samples.
- **M1. Why 1.24M reads have no cover.**
  - Instrument: the uncovered reason recorded at each read.
  - Judged against: `Surface::cover`'s contract.
  - A placeholder or mid-door face means the door should read its held face (the site-mint fix). A finished body's face with no cover is a defect to file.
- **M2. The first unit's differential.**
  - Count, per predicate row: definite verdicts changed (must be 0; any is a conversion bug), Zeros now refused, and bodies newly refused.
  - Then decide each newly refused reading at the oracle lever. If it is definite there, the refusal is right and the old Zero was wrong; that class goes to you as moved toward refusing. If it is Zero there, the cover is too loose: a completeness loss to report, with the cover fixed at its source.
- **M3. S against Q.**
  - Count straddles whose two verdicts the consumer's own code treats alike. The instrument is that comparison at each grouped site, recorded beside the straddle.
  - 0 on the whole corpus means S. Any on bodies that are valid at the oracle lever means Q, put to you.
- **M4. Site: middle against station point against hinge line, on the same reads.**
  - Truth: the oracle departure, and the honeone row's table of profile-declared joints.
  - The site whose verdicts match the oracle wins. A definite verdict from a point site that the oracle reads Zero is an unsound claim, and counts against the point.
- **M5. Edges in the bound from below.**
  - Measure the below/oracle ratio before and after, the definite verdicts newly served (each checked definite at the oracle), and the time on the symbolic lane.
  - Adopt only if every newly served verdict is definite at the oracle.
- **M6. Bounds out of order.** Count cases where below > above, or the oracle exceeds above. Judged against 0. Any is a cover defect.
- **M7. Door against rest.** Count edges a door decided that tier 3 decides otherwise. Judged against 0.
- **Conditions for all of them.**
  - Whole corpus, not the partial run.
  - At ε = 1e-6, 1e-9 and 1e-12, and on the interval lane.
  - The ratio figures re-taken from the new site. All of the lane's figures are from the span's middle at one ε, and its "real under-reads" were never compared with an oracle.

Confidence:
- The agreed core: sure on soundness, likely on the shape.
- The line site: likely.
- Building the two-sided margin term by term: likely.
- S against Q: unsure until M3.

## For the orchestrator

- **The coordinator's reading of the split is right.** I moved on (2) the site and on (3) where the funnel lives. (1), what "agree" is on, stays open and is reduced to one measurement (M3).
- **The joint statement both could sign** is the recommendation with S/Q left to M3 and then Ev. I took the other report's typed points:
  - the below ≤ above invariant;
  - the missing bound from above carrying its reason;
  - the bit-identical differential ending only once the site and the bound from below move;
  - sequencing.
- **Unchecked claims.**
  - That `Cover::distance_range` from a line is tight on every piece kind.
  - That reading from a line is affordable on the symbolic lane.
  - That the at-rest gate's "the demanded set and the stored set are one set" survives (it should: both read the same region from the same site).
- **The oracle lever does not exist in the tree.** It needs a test-support reader: dense face samples with a bounded spacing error. It must not reuse the cover or the points under test, or M2, M4 and M5 measure the instrument against itself.
- **Not filed (I was read-only).**
  - The cover measured inside its own vertices (`WedgeReach::reach`'s max fold, ratio 0.29).
  - The middle-of-span site against stations and joints (`carrier.mid_point` in both doors; `JointSides::lever` takes the lesser of two reaches from two different middles).
  - The stopped PR tree still reads `.below()` for every-sign decisions in `edge_wedge.rs` and `singular_turn`, and should not be merged as is.
