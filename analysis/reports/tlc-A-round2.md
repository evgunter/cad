## For Ev

**Two decisions are yours now, before any measurement. Both are wording in `crates/geom-core/README.md`, "Float rounding". Both are agent-written text you approved on PR 7141.**

1. **The lever bullet's last sentence, restated as the rule a proof follows.** Replace "a verdict that claims a definite sign reads the extent from below, one that acts on its Zero reads it from above, and one that acts on every sign reads both and acts only where they agree" with:

   > "A definite sign is read at the extent's bound from below and a Zero at its bound from above, whatever acts on the verdict; where the two readings disagree, the reading is in band and is refused as one."

   What changes: no code chooses a bound any more, so the ~41-caller classification (which four reviewed units got wrong) disappears. Recommend yes (sure).

2. **The point a lever is measured from, for a reading that turns about a line.** In the consumed-extent bullet, replace "Its lever is the extent's far distance from that point" with:

   > "Its lever is the extent's far distance from the point, or the line, its datum turns about: a tilt about a line moves each point by the tilt times its distance from that line."

   Why: a dihedral at a station turns about the edge's tangent line there, and an azimuth at a pole turns about the chart's axis. Distance from a point counts the faces' run along the edge.
   - That still proves a Zero soundly.
   - It over-states the lever a definite sign needs, which is unsound. The open row `honeone/the-wedge-reach-counts-a-faces-run-along-its-edge` is this defect.
   - Example: a strip 4 long and 0.05 wide, at 15ε to its neighbour. From the edge's middle (today) it reads 30ε, a corner. From the hinge line it reads 0.75ε, the tangent joint that was declared.
   - The inequality is sure; the choice is likely. This settles that row too.
   - It moves definite verdicts: corners on long thin faces move toward refusing, and smooth joins toward deciding. So the code change lands only after measurement M4 reports which classes move. The principle can be ruled on now.

**Decisions that come back to you after a measurement:**
- every class of valid bodies the first unit newly refuses (M2);
- the classes the line site moves (M4);
- boundary edges in the bound from below, which moves verdicts toward deciding (M5);
- any coarser question gate, which moves exact-lever readings toward deciding (M3, built only if M3 finds cases);
- the joint turn's region: the walked face alone, or its two edges' neighbours (unsure; we read it before asking).

**What an agent may land without you** (faithful to the ratified text):
- The lever stays two-sided down to each scalar decision.
- A disagreement is that decision's in-band reading: minted once as a straddle under the predicate's own straddle row, and leaving each composite verdict through the in-band arm it already has. No verdict gains a variant.
- One home for this in `geom-core`.
- A missing bound from above is a typed case: definite signs stand, nothing proves a Zero.
- The bounds are ordered by construction.
- The doors return the region, not a lever.
- The bound from above reads the face as the door holds it, which gives the site mint a cover.
- The lift-free cover stays.

This first unit reads from the middle as today and leaves the bound from below unchanged. Its differential can only move toward refusing, and any class of valid bodies it newly refuses comes to you before it lands.

Terms:
- *Bound from below / from above*: a length the region certainly reaches / certainly does not pass.
- *Straddle*: one decision whose two bounds read differently.
- *Hinge*: the line a datum turns about.

## For the orchestrator

**(a) The seven points.** I sign all seven, with three amendments of wording:

- **Point 1, signed.** "No free closure from a lever to a margin" is the form I sign: margins built term by term from the two-sided `Margin` doors, with both ends always evaluated. "Both ends evaluated" alone is not enough. A closure that dips between its ends is not bounded by them, and the sum rows are such a case.
- **Point 4, signed.** I moved to the line in round 1. The other report's correction stands: my round-1 M2 took the station point as truth, and I withdraw it.
- **Point 5, signed in the other report's form.** It is better than my per-call map: a map at the bounds door alone would decide more than an exact lever does, so knowing less would decide more. One rule covers every gate: an outcome is served only where it holds at every length between the bounds. Each such gate is Ev's.
- **Points 6 and 7, signed.** One amendment to 6(ii): Ev can rule on the site's principle now (the inequality is a proof). M4 decides when and how it lands and reports the moved classes. M4 is not what makes the principle true. I have written it that way in For Ev.
- **Points 2 and 3, signed.** One addition: the ordered-bounds constructor refuses loudly, recorded on its own row, when the cover does not hold a point known to lie on the region. Taking the max hides the defect; the hits are M6's.

**Still to check before unit 1:** whether any caller acts on the decided sign inside a certification refusal (for example, retrying as a tangent). Neither of us has grepped the `CertifyError` matches. If one does, unit 1 changes a flow there, and the Zero it acts on must be proved from above.

**(b) One list of measurements, with the truth each uses.**

Truths, in order of strength:
- **T1, closed form:** fixtures whose lever and dihedral are known exactly. Plane-plane edges with straight-edged faces, where the farthest vertex's distance from the edge's line is exact; also coaxial rims, and the honeone strip table.
- **T2, sampled oracle:** dense points of each face's carrier inside its trim, at a spacing whose error is bounded. This gives an interval on the true lever, independent of covers and vertices. It is still kernel code (its in-face test uses the trim classifier), so it is reported as a second instrument, never as T1.
- **T3, no truth:** a count against the base's own verdict. It is reported as "how far the base over-claimed", never as "valid bodies lost".

Each measurement's result is reported separately under every truth that applies, and each count says which truth it is taken against.

- **M1. Why faces read no cover.**
  - Instrument: the typed reason per read, by surface kind and by at rest or mid-door.
  - Truth: none needed (a census of a typed value).
  - Reading: placeholder nets mid-door means the site-mint fix covers it. Any at rest is a cover defect row. A finite net refusing means the instrument is broken.
- **M2. Unit 1's differential.**
  - Instrument: per decision, the row, the caller, the reading at each bound (in band included) and the base's verdict.
  - Truths: T3 for "moved"; T1 for "valid", with T2 for the rest.
  - Gates: a definite verdict that changes means a conversion bug. A reading definite at the bound from below but not at the bound from above means that margin does not grow with its lever (this check also catches free closures).
  - Reading: a refused join tangent by T1/T2 is a completeness loss, to Ev as a class. One near-tangent by T1/T2 is the intended refusal.
- **M3. Sign or a coarser gate.**
  - Instrument: among the straddles and the exact-lever in-band readings, count those where the consumer's own returned action is the same at every possible sign.
  - Truths: the consumer's return value, on bodies valid by T1/T2, never by T3.
  - Reading: zero means no gate. Nonzero means that gate goes to Ev.
- **M4. The site.**
  - Instrument: middle, station point and hinge line, per station, for both bounds.
  - Truth: T1, on the plane-plane subpopulation and the honeone table. T2 elsewhere, reported separately.
  - Count: a bound from below above the truth, a bound from above below it, and verdict flips each way.
  - Reading: hinge-line flips are the moved classes, to Ev. Any over-read at the station point confirms rejecting it.
  - The joint turn has no truth beyond hand-built fixtures, and is reported as such.
- **M5. Boundary edges in the bound from below.**
  - Instrument: M2's straddles cured, and the definite verdicts newly served, at the same site; plus the symbolic lane's time.
  - Truth: each newly served verdict must be definite by T1/T2. Where neither applies, only "each sample is a point of the region" holds, and that is reported as no truth.
  - Reading: a cure without a truth is not counted toward adoption.
- **M6. Covers inside their own boundary.**
  - Instrument: per face, the cover's far distance against the far boundary point, and that point's distance off the face's surface.
  - Truth: none needed.
  - Reading: the point on the surface means a cover defect. Off it means the door's record is not the face it governs. Target zero beyond rounding.
- **M7. Door against rest.**
  - Instrument: edges a door decided that tier 3 decides otherwise on the finished body.
  - Truth: none; a consistency count. Target zero.

All of these run on the whole corpus, at ε = 1e-6, 1e-9 and 1e-12, and on the interval lane. The lane's figures (one ε, a partial f64 run, measured from the middle) are re-taken, not reused.

**(c)** Done in For Ev: the two decisions for now, each with its README wording, set apart from the five that come back after a measurement.

**Still unchecked by me:**
- that `Cover::distance_range` from a line is tight on every piece kind;
- the symbolic lane's cost of a line site;
- the `CertifyError` grep above.
