## For Ev

**I sign points 1, 3, 4, 5 and 6 as the coordinator states them, and amend 2 and 7.** Nothing here was run.

Terms:
- *Bound from below*: a length the region certainly reaches.
- *Bound from above*: a length it certainly does not pass.
- *Straddle*: one decision whose two bounds read differently.
- *Site*: the point or line a lever's distance is measured from.
- *Oracle*: a test-only reader that samples a face densely inside its trimmed boundary and returns its true reach as a small interval.

### Yours now, before any measurement

**The lever sentence, restated as the rule it comes from (point 7, amended).** The sentence both reports proposed is too narrow. It says a definite sign is read from below, but a "datum plus swing" row (the cone–cylinder coaxial row, for one) proves its definite sign from above, because a longer lever means more swing and less clearance. The other report found the same thing. The sentence should state the general rule and give the multiplied case as its instance.

Wording I would put in `crates/geom-core/README.md`, replacing the text after "no longer than the site's own measure of it:" in the lever bullet:

> a lever read off a consumed extent is known from both sides, from below off points of the region and from above off its cover, and a verdict is served only where it holds at every length between the two. For a reading multiplied by its lever that is a definite sign read from below and a Zero from above, whoever acts on it. Where the two disagree the reading is in band, refused once under the predicate's straddle row. Where the region has no cover nothing proves a Zero. No consumer takes one bound.

- Provenance of the sentence it replaces: agent-written, approved on PR 7141.
- This is the only decision that needs you before measuring. The first unit is faithful to the sentence as it stands and can land without it.

### Yours after a measurement

- **The site** (point 4), as one question with the open row `the-wedge-reach-counts-a-faces-run-along-its-edge`, after Q4. With it comes the joint turn's region: the walked face alone, or its two edges' neighbours. I lean to the face (unsure).
- **Boundary edges in the bound from below**, after Q5. It moves definite verdicts toward deciding.
- **Any coarser gate** (point 5), only if Q3 finds a case.
- **Each class of valid bodies newly refusing**, from Q2, before the first unit lands.

### Point 2, amended: what the bounds' constructor does

The two reports differ here. One orders the bounds silently; the other refuses loudly whenever the bound from below exceeds the one from above. Neither works alone.

- On a line or an arc the two reads are the same length computed two ways, so they can differ by rounding. A loud refusal there is a false alarm.
- A silent fold hides a cover that misses its own face.

The amendment: a point of the region is only known to lie on it within the run's tolerance ε. So:
- an inversion of at most ε is rounding, and the bounds are ordered;
- an inversion past ε is a contradiction. It becomes the typed "no bound from above" case, carrying the reason that the cover does not hold a point of its region. Definite signs still stand and every Zero refuses by name.
- The defect is counted (Q6) and filed at its source.

This uses the run's own band and no hand-written slack. The rest of point 2 I sign.

### The measurements, as one list

The oracle gives the truth of a lever, never of a body's validity, and only where it has first been checked. Where there is no truth, the count is reported under that label and added to neither side.

- **Q0. Calibrate the oracle.**
  - Truth: closed forms on fixtures (two planes on a line edge, a sphere cap, a cylinder strip, a torus band).
  - Judged: its interval must hold the closed form on every fixture. If it does not, nothing that uses it is reported.
- **Q1. Why faces read no cover.**
  - Instrument: tally the typed reason by surface kind and by caller, at rest or mid-door.
  - Truth: none needed; it is a census.
  - Reading: all placeholders mid-door means item 2 is a face not yet built. Any at rest is a defect to file.
- **Q2. The first unit's differential.**
  - Instrument: per decision, the reading at each bound and what the base returned.
  - "Moved" is judged against the base's verdict on the same read.
  - "Right" is judged against the same datum decided over the oracle's interval. "Built and passed tier 3 at the base" is never used as valid.
  - Reading:
    - A definite verdict changed means a conversion bug.
    - A newly refused Zero that the oracle reads definite or in band means the base over-claimed. It is listed for you as moved toward refusing.
    - A newly refused Zero that the oracle reads Zero means the cover is too loose. That is a completeness loss, to you before landing.
    - No truth where the oracle's own two ends disagree, or for mid-door faces not in a body.
- **Q3. Sign or coarser gate.**
  - Instrument: the consumer's returned action at each bound, and at exact-lever readings in band.
  - Truth for validity: the oracle verdict, or a closed-form fixture.
  - Reading: none found means sign agreement stays. Any found with a truth behind it goes to you.
- **Q4. The site.** The three candidates, both bounds, on the same reads.
  - The other report judges this against the oracle's distance from the hinge line. That is circular: it assumes the hinge is right.
  - Truth that assumes no site: on two planes, the largest distance of one face's points from the other face's plane (closed form, and the oracle gap). Also the open row's table, where the profile's declaration is the truth by construction.
  - On curved pairs and at the joint turn there is no site-free truth. Report the three sites' verdicts side by side as disagreements, labelled as such.
  - Reading: a bound from below whose margin exceeds the true gap is an unsound definite claim, and counts against that site. A bound from above whose margin falls short of it is an unsound Zero.
- **Q5. Edges in the bound from below.**
  - Truth: the oracle's reach from the same site.
  - Reading: adopt only if every newly served definite verdict is definite at the oracle and it cures a real share of Q2's straddles. The time on the symbolic lane is reported beside it.
- **Q6. Bounds out of order.**
  - Instrument: inversions past ε, and oracle samples outside the cover, each with the offending point's distance off the face's surface.
  - Reading: a point on the surface means a cover defect. A point off it means the door's record is not the face it governs. Target zero.
- **Q7. Door against rest.**
  - Instrument: edges a door decided that tier 3 decides otherwise.
  - Truth: none; it is a consistency count. Target zero.

All of it runs on the whole corpus, at 1e-6, 1e-9 and 1e-12, and on the interval lane.

Confidence: the signed state likely; the amended sentence likely; the Q4 circularity sure; the turn's region unsure.

## For the orchestrator

**Signatures**
- Points 1, 3, 4, 5 and 6: signed as written. On 1, I take the other report's term-by-term enclosure in the `Margin` doors as the form. It is what makes the swing rows right by construction, and it is why point 7's sentence needed widening.
- Point 2: signed with the constructor amendment above. I would not sign "refuses loudly when below > above" as the other report words it, nor my own earlier silent ordering.
- Point 7: signed as Ev's, with the wording amended as above.
- On point 6(i), one precondition before dispatch: grep whether any caller acts on a certification refusal's decided sign. Neither of us checked. If one does, unit (i) changes a flow and that goes in its PR body.

**On the two truths you asked us to reconcile**
- I accept the oracle for levers, on four conditions:
  - it is calibrated on closed forms first (Q0);
  - it returns an interval, not a number;
  - it is never the truth for the site question, where the other report's use is circular;
  - "no truth" is its own reported count.
- The other report should be able to sign that. Its own note says the oracle "must not reuse the cover or the points under test".
- The oracle is not fully independent of the code under test. Sampling inside a trim needs face containment, which reads lifts, and those read the joint turn's lever. It should count only samples that are inside by a definite margin and take the boundary from edge samples. That dependence goes in whatever report quotes it.
- Mid-door faces (the site mint; faces read before a door finishes) have no oracle. Their Q2 rows will sit in the "no truth" count. That is expected and is not a failure of the measurement.

**Unchecked, still**
- That `Cover::distance_range` from a line is tight on every piece kind.
- The symbolic lane's cost of a line site.
- Whether the ε-inversion rule in the constructor needs the padded band where a vertex is known only within a looser import tolerance. I did not read the import door's bands.

**Not filed (read-only)**
- The same three the other report lists: the cover inside its own vertices, the middle-of-span site, and the stopped tree's `.below()` reads.
- In addition, the sum-row case means `LeverBounds`' own doc comment ("an arm short of the length turns a reading that is Positive at it into a definite Zero") describes only multiplied readings. It should be reworded when the type moves to `geom-core`.