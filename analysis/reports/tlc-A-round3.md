## For Ev

Nothing here has been run.

Terms used below:
- **Bound from below:** a length the region certainly reaches.
- **Bound from above:** a length the region certainly does not pass.
- **Straddle:** one decision whose readings at the two bounds differ.
- **Site:** the point or line a lever's distance is measured from.
- **Oracle:** a test-only reader that samples a face densely inside its trim and returns its true reach as an interval.

### What you are asked now: one sentence

**The lever bullet in `crates/geom-core/README.md`, "Float rounding", restated as the rule it comes from.** The sentence it replaces was written by an agent and approved on PR 7141. That sentence says a definite sign is read from below. That is true only of a reading multiplied by its lever.

A "datum plus swing" row proves its definite sign from above. The cone–cylinder coaxial row is one: a longer lever means more swing and less clearance.

Replace the text after "no longer than the site's own measure of it:" with:

> a lever read off a consumed extent is known from both sides, from below off points of the region and from above off its cover, and a verdict is served only where it holds at every length between the two. For a reading multiplied by its lever that is a definite sign read from below and a Zero from above, whoever acts on it. Where the two disagree the reading is in band, refused once under the predicate's straddle row. Where the region has no cover nothing proves a Zero. No consumer takes one bound.

What this changes:
- No code chooses a bound any more.
- The 41-caller classification goes away. Four reviewed units got that classification wrong.
- The first unit below follows the sentence as it stands and can land without this change.

Recommend yes (likely).

### What comes back to you after a measurement

- **The site, together with the open row `honeone/the-wedge-reach-counts-a-faces-run-along-its-edge`, after Q4.** The candidates are:
  - the edge's middle, as today;
  - the datum's point;
  - the line the datum turns about: the edge's tangent line at a station, or the chart's axis at a pole.

  The line looks right on two planes. A tilt about a line moves each point by the tilt times its distance from that line, so measuring from a point counts the faces' run along the edge. That over-states the lever a definite sign needs.

  Curved pairs and the joint turn have no truth that assumes no site. This question therefore comes to you with its measured classes, not before. The joint turn's region comes with it: the walked face alone, or its two edges' neighbours (unsure).
- **Boundary edges in the bound from below, after Q5.** This moves definite verdicts toward deciding.
- **Any coarser question gate**, such as "is it definitely Negative?". It comes only if Q3 finds a case. It moves exact-lever readings toward deciding.
- **Every class of valid bodies the first unit newly refuses**, from Q2, before that unit lands.

### What an agent lands without you

This is one unit, faithful to the ratified text.

**The lever stays two-sided down to each scalar decision.**
- The two-sided margins are built term by term in the `Margin` doors, with both ends evaluated. There is no free closure from a lever to a margin.
- Their home is in `geom-core`.
- `below()` and `above()` cannot be read outside that home.
- A predicate's row and its straddle row are one value.

**Disagreements and missing bounds.**
- A disagreement is one decision's in-band reading. It is minted once as a straddle and leaves each composite verdict through the in-band arm the verdict already has. No verdict gains a variant.
- A missing bound from above is a typed case that carries its reason. Definite signs stand. Nothing proves a Zero.

**The bounds' constructor.** A point of the region is known to lie on it only within the run's ε.
- An inversion of at most ε is rounding, and the bounds are ordered.
- An inversion past ε becomes the typed "no bound from above" case, with the reason that the cover does not hold a point of its region. It is counted (Q6) and filed at its source.

**The doors.**
- The doors return the region, not a lever.
- The doors read from the middle as today, and the bound from below is unchanged.
- The bound from above reads the face as the door holds it, which gives the site mint a cover.
- The lift-free cover stays.

Its differential can only move toward refusing.

### The measurements

The oracle gives the truth of a lever only, never of a body's validity. "Built and passed tier 3 at the base" is never used as valid. Where there is no truth, the count is reported under that label.

- **Q0, calibrate the oracle.**
  - The oracle counts only samples inside the face by a definite margin, and takes the boundary from edge samples.
  - Its interval must hold the closed form on two planes, a sphere cap, a cylinder strip and a torus band.
  - Otherwise nothing that uses it is reported.
- **Q1, why faces read no cover.**
  - A tally of the typed reason, by surface kind and by at rest or mid-door.
  - Any face at rest with no cover is a defect.
- **Q2, the first unit's differential.**
  - Each change is judged against the base's verdict on the same read, and against the datum decided over the oracle's interval.
  - A definite verdict that changes is a conversion bug.
  - A Zero that refuses because the oracle reads it non-Zero means the base over-claimed.
  - A Zero that refuses although the oracle reads Zero is a completeness loss, and comes to you.
  - Mid-door faces fall under "no truth".
- **Q3, sign agreement or a coarser gate.**
  - The consumer's own returned action at each bound, and at exact-lever readings in band, on bodies valid by the oracle or by closed form.
- **Q4, the site.**
  - The truth assumes no site: on two planes, the largest distance of one face's points from the other face's plane; otherwise, the open row's declared profiles.
  - A definite claim past that truth is unsound and counts against its site.
  - On curved pairs and at the joint turn, the three sites' verdicts are reported side by side as disagreements.
- **Q5, edges in the bound from below.**
  - The truth is the oracle's reach from the same site.
  - Adopt only if every definite verdict it newly serves is definite at the oracle. The cost on the symbolic lane is reported beside it.
- **Q6, bounds out of order.**
  - Inversions past ε, and oracle samples outside the cover, each with its distance off the surface.
  - On the surface means the cover is defective; off it means the record is not the face. Target zero.
- **Q7, door against rest.**
  - A consistency count of edges a door decided that tier 3 decides otherwise. Target zero.

All of these run on the whole corpus, at ε of 1e-6, 1e-9 and 1e-12, and on the interval lane.

Confidence:
- the landable unit: likely;
- the sentence: likely;
- the site as a line: likely on planes, unsure beyond them;
- the turn's region: unsure.

## For the orchestrator

I sign the other report's position on all four points.

- **(1) Wording: signed.** The swing rows prove definite signs from above, so my narrower sentence was wrong for them. The general rule, with the multiplied reading as its instance, is right.
- **(2) Site after the measurement: signed.** The inequality is a proof on planes. What Ev would be ruling on also covers curved pairs and the joint turn, which have no truth that assumes no site, and the honeone row. Asking first would invite the rewrite-after-measurement pattern you describe.
- **(3) Constructor: signed.** Refusing loudly would raise false alarms on exact carriers. The ε rule uses the run's band, adds no hand-written slack, and keeps the defect visible through Q6.
  - Open: whether a vertex known only within a looser import tolerance needs that band rather than ε. I did not read the import door's bands.
- **(4) Truth for Q4: signed.** My T1 on the plane-plane subpopulation (the farthest vertex's distance from the edge's line) assumed the hinge, so it was circular in the same way.

Precondition before the unit is dispatched, still unchecked by either report: grep the `CertifyError` matches for any caller that acts on the sign a certification refusal decided (for example, retrying as a tangent). If one does, the unit changes a flow there, and that goes in its PR body.

Also still unchecked:
- whether `Cover::distance_range` from a line is tight on every piece kind;
- the symbolic lane's cost of a line site;
- `LeverBounds`' own doc comment, which describes only multiplied readings, should be reworded when the type moves.

Not filed (read-only):
- the cover inside its own vertices;
- the middle-of-span site;
- the stopped tree's `.below()` reads for every-sign decisions in `edge_wedge.rs` and `singular_turn`.
