## For Ev

**Where this stands after reading the other report.** We can both sign one final state for everything except the site. On the site I hold my position with a sharper argument, and it is yours to rule on either way. Nothing below was run.

Terms: *bound from below* is a length the region certainly reaches; *bound from above* is one it certainly does not pass. A *straddle* is one decision whose two bounds read differently. The *site* is the point or line a lever's distance is measured from. The *hinge* is the line a datum turns about: the edge's tangent line at a station for a dihedral, the chart's axis at a pole for an azimuth.

### What moved me, and to what

- **A missing bound from above becomes a typed case** (`LeverBounds::AtLeast(below)`), not a poison lever. Definite signs stand, nothing proves a Zero, and the refusal is a straddle with no upper end, not an anonymous invalid margin.
- **A predicate row and its straddle row become one value**, so a levered decision with no straddle row does not compile.
- **A cover that reads inside its own face's vertices is a defect at its source.** I had only asked that the two bounds be ordered by construction. I keep that, because rounding needs it, but it is not the cure.
- **The bound from above reads the face as the door holds it** (its surface and its loops' edge covers), not a face key in a body. This is my site-mint answer made general.
- **On "agree on what": partly moved.** The other report is right that refusing a straddle whose two signs do the same thing loses an honest verdict. I do not accept a per-call map from sign to action at the two-bounds door, for the reason in the next section.

### (1) Sign or action: one state we can both sign

- **The flaw in a map at the bounds door alone.** It serves "Zero below, Positive above" as "not opposed". The same consumer with an exactly known lever and a reading in the band between Zero and Positive still refuses today. Knowing less would then decide more.
- **The coherent form.** The consumer asks the question it acts on, through a gate whose outcomes are the ones it tells apart. The funnel already has such gates (`decide_nonzero`, `decide_positive`, `decide_magnitude`). One rule then covers all of them: an outcome is served only where it holds at every length between the bounds.
  - With three outcomes this is sign agreement.
  - With "Negative or not" it serves the other report's case, and also the in-band reading at an exact lever.
- **What lands first.** Sign agreement lands as the faithful elaboration. A coarser gate is built only where measurement Q3 finds real straddles of that kind, and each one is yours, because it moves exact-lever readings toward deciding.
- **The must-carry bend needs no gate.** The other report's own example concludes that it should be asked only inside the certificate lane, where its answer matters. I agree.
- **A refusal that names a decided sign is a verdict too.** If a caller acts on "certification decided Zero" (for instance by retrying as a tangent), that Zero must be proved from above. A straddle must reach it as an escalation, not as Zero.

If we do not both sign that, the split is:
- **Sign:** every straddle refuses. Simplest. `stands_on` and `sheets_part` refuse "Zero below, Positive above".
- **Action map:** those are served, at the price that an exact lever in band still refuses.
- **Outcome gate:** both are served. It changes exact-lever behaviour and is built only on evidence.

### (2) The site: not agreed, and my earlier argument is not answered

The other report reads each lever from its datum's point (each station; the joint vertex) and calls that sound. It is sound for a Zero. It is not sound for a definite sign, by the README's own inequality.

- The tilt bound says a face point `x` moves by at most `θ·|x − pivot|`. An upper bound on movement proves a Zero.
- A definite sign needs some face point to move by at least Kε. A point's distance from the pivot does not give that. Its distance from the hinge does: the point moves `sin θ` times that distance.
- So the bound from below is only a bound when it is read from the hinge. Read from a point, it counts the faces' run along the edge.
  - Today, from the middle, it counts half that run. This is the open row `the-wedge-reach-counts-a-faces-run-along-its-edge`.
  - From an end station it counts nearly all of it.
- **Example**, from that row's own table: a strip 4 long and 0.05 wide at `φ = 15ε`. The hinge reads 0.75ε, the tangent joint the profile declared. The middle reads 30ε and an end station about 60ε, both a sharp corner.

Moving to the station point therefore makes definite claims worse. Certification is entirely definite claims and is the most numerous reader.

One line per option:
- **Middle, as today:** nothing moves. A Zero is under-read by up to 2× at end stations, and a definite sign is over-read as that open row says.
- **Datum's point:** README-literal. Zeros become sound. Definite signs are over-read up to twice as far as today.
- **Hinge line, my recommendation (likely):** both bounds are sound, and exact for two planes. The README's "far distance from that point" becomes "from the point or line its datum turns about". Corners on long thin faces move toward refusing, and smooth joins toward deciding.

Both of us agree the doors return the region, so each option is a change of reader, not of door. I would put this and that open row to you as one question, not in sequence.

### (3) Where the funnel lives: not a fork

We agree on the constraints, and the rest is the implementer's choice:
- one home in `geom-core`;
- `below()` and `above()` not readable outside it;
- the row pair as one value;
- no free closure from a lever to a margin, or else both ends always evaluated.

I lean to the margin carrying both readings into the existing gates, because the gate family and `Period::decide`'s meter then need no second copy.

### Faithful elaborations an agent may land

- The two-sided funnel with sign agreement, the row pair, and the closed accessors.
- `AtLeast` for a missing bound.
- Composites taking the region and passing it unopened; a straddle leaving through each verdict's existing in-band arm.
- Doors returning the region, read from the middle as today, with the bound from below unchanged.
- The bound from above read off the face as the door holds it. This gives the site mint a cover.
- The lift-free cover kept; ordered bounds; `classify_seam`'s spline bracket folded into the general case.

This unit's differential can only move toward refusing. Any class of valid bodies it newly refuses comes to you before it lands.

### Yours

- The ratified sentence's text: "a definite sign is read from below and a Zero from above, whoever acts on it; where the two disagree the reading is in band".
- The site, above. Both alternatives to today move definite verdicts.
- Boundary edges in the bound from below (toward deciding).
- Any coarser gate (toward deciding at exact levers).
- The joint turn's region: the walked face alone, or its two edges' neighbours (I lean to the face; unsure).
- Every newly refusing class of valid bodies.

### Measurements

Each is listed with its instrument, what it counts against, and which result means which answer.

- **Q1. Why faces read no cover.**
  - Instrument: at both doors, tally the typed refusal reason by surface kind and by caller (at rest or mid-door).
  - Truth: none needed; it is a census of a typed value.
  - Reading: all poisoned nets mid-door means item 2 is a placeholder face and only Zeros at those doors refuse. Any at rest is a defect row on the cover. A finite spline net refusing means the instrument is wrong.
- **Q2. The two-sided funnel alone.**
  - Instrument: per decision, record the row, the caller, the reading at each bound (in band included) and what the base returned.
  - Truth: for "moved", the base's verdict on the same read. For "valid", the base is NOT truth, because it read from below. Use only fixtures whose dihedral is known in closed form.
  - Reading:
    - A reading definite at below and anything else at above means that row's margin does not grow with its lever, and the design's assumption fails there.
    - No body newly refusing means it lands.
    - A newly refused join that is tangent by construction is a completeness loss and comes to you as a class.
    - A newly refused join that is near-tangent by construction is the intended refusal.
- **Q3. Sign or outcome.**
  - Instrument: among Q2's straddles, and separately among exact-lever in-band readings, count those where the consumer's own returned action is the same at every sign the reading could be.
  - Truth: the consumer's return value, computed at each end.
  - Reading: zero of both means sign agreement and no coarser gate. Nonzero on valid bodies means the outcome gate, to you.
- **Q4. The site.**
  - Instrument: on plane–plane line edges with straight-edged faces, compute the three candidate levers at each station, for both bounds.
  - Truth: the farthest face vertex's distance from the edge's line, which is exact there.
  - Count: a bound from below above the truth, a bound from above below it, and verdicts that differ from the truth's.
  - Reading: any over-read from the station point with a flipped verdict rejects the point site for the bound from below. None still leaves it unsound in principle, and that open row's fixture already shows one.
  - For the joint turn I have no truth outside hand-built fixtures.
- **Q5. Boundary edges in the bound from below.**
  - Instrument: on the same reads at the same site, the straddles from Q2 that it cures, and the definite verdicts it newly serves.
  - Truth: Q4's, on that subpopulation only. Elsewhere there is none beyond "each sample is a point of the region".
  - Reading: few cures against the symbolic lane's cost means do not build it.
- **Q6. Covers inside their own vertices.**
  - Instrument: per face, the cover's far distance against the far boundary point, plus that point's distance off the face's surface.
  - Reading: the point is on the surface means a cover defect. Off it means the door's record is not the face it governs. Target zero beyond rounding.
- **Q7. Door against rest.**
  - Instrument: edges a door decided that tier 3 decides otherwise on the finished body.
  - Truth: none; this is a consistency count. Target zero.

All of it runs on the whole corpus, at 1e-6 and 1e-12, and on the interval lane.

Confidence: the signed state likely; the one-directional tilt argument sure; the hinge as the site likely; the turn's region unsure.

## For the orchestrator

**Your reading of where we stand is right.** Three corrections of detail:
- The other report already hedges its own point 4 ("if there are none, take sign agreement and drop it"). (1) is closer to agreed than it looks.
- The other report treats moving the pivot as landable by an agent. By your own test (anything that moves a definite verdict toward deciding is Ev's) it is not: from an end station the bound from below grows.
- The other report's M2 judges "a definite verdict that becomes Zero or a straddle under the datum pivot was unsound before". That takes the station-point lever as truth, which is the thing in question. Q4 replaces it with a closed-form truth on the one subpopulation that has one.

**On the points you asked me to weigh**
- *A closure handed to the door, and the sum rows*: these were mine. The other report's M7 is the right check, and Q2 carries it. I would still close the door to free closures instead of relying on a count.
- *The ordered-bounds invariant against "defect at source"*: both hold. The constructor orders the pair, because on an exact carrier the two reads differ by rounding. Q6 finds the real hits, and the row goes to whichever slate owns `topo::cover`, as the other report says.
- *A missing bound as an anonymous margin*: `AtLeast` answers the typing. The cover's refusal reason still does not reach the person. I leave that as a note, not a blocker.
- *Bit-identical to the old lever*: holds for the first unit as I scope it (site and bound from below unchanged). It ends with the site or with edges in the bound from below. Both are Ev's above.

**Sequencing I would sign**
1. The two-sided funnel with the doors returning the region, read as today.
2. The site, as one `[ev]` question together with the honeone row, after Q4.
3. Edges in the bound from below, after Q5.
4. Coarser gates, only if Q3 is nonzero.

**Still unchecked by me**
- Whether any caller acts on a certification refusal's decided sign. The other report raised it and I did not grep. It decides whether unit 1 changes a flow.
- That `Cover::distance_range` from a line is tight on every piece kind.
- The symbolic lane's cost of a line site.
- `run_checks` whole, `must_carry_description`, `classify_shared_rim`'s body and `walls_inside`'s decisions. I did confirm `classify_shared_rim` returns `Result<RimRouting, Indeterminate>`, which answers the other report's open check on `RimRouting`. `SingularMiss` sits beside `PinMiss::Escalated`.

**One caution on Q2's truth.** The suites' "valid body" is mostly "built and passed tier 3 at the base". That is the instrument under test for every Zero claim. A count of newly refused bodies taken against it measures how far the base over-claimed, not how many valid bodies were lost. Only the closed-form fixtures separate the two, so the report to Ev should give both counts and say which is which.