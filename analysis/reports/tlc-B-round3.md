## For Ev

**You are asked one thing now: the wording of one sentence. There is also a one-line ruling on whether to take the site now or after its measurement.** Nothing here was run.

Terms:
- *Bound from below / from above*: a length the region certainly reaches / certainly does not pass.
- *Straddle*: one decision whose two bounds read differently.
- *Site*: the point or line a lever's distance is measured from.
- *Hinge*: the line a datum turns about.

### Asked now: the lever sentence

In `crates/geom-core/README.md`, "Float rounding", the lever bullet, replace the text after "no longer than the site's own measure of it:" with:

> a lever read off a consumed extent is known from both sides, from below off points of the region and from above off its cover, and a verdict is served only where it holds at every length between the two: for a reading multiplied by its lever, a definite sign at the bound from below and a Zero at the bound from above, whatever acts on the verdict. Where the two disagree the reading is in band and is refused as one. Where the region has no cover nothing proves a Zero. No consumer takes one bound.

- **What it changes.** No code chooses a bound, so the classification of about 41 callers, which four reviewed units got wrong, disappears.
- **Why not the shorter wording** ("a definite sign is read at the bound from below and a Zero at the bound from above"). It is false for rows already in the tree. A "datum plus swing" row, such as the cone–cylinder coaxial row, proves its definite sign from above, because a longer lever means more swing and less clearance. A reviewer holding the short sentence would read that correct row as a violation, which is this track's failure. The wording above states the rule and gives the multiplied case as its instance. It takes the other wording's "whatever acts on the verdict" and "refused as one".
- **Provenance** of the sentence replaced: agent-written, approved on PR 7141.
- Recommend yes (likely on the wording, sure on the rule).

### The site: rule now, or after its measurement

- **Now.** "Its lever is the extent's far distance from that point" becomes "from the point, or the line, its datum turns about". The code lands after the measurement either way.
- **After (my recommendation).** One question with the open row `the-wedge-reach-counts-a-faces-run-along-its-edge`, once the measurement shows which classes move and what a curved edge needs.

Why after:
- The proof covers flat pairs: a tilt about a line moves a point by the tilt times its distance from that line.
- On a curved edge the tangent line at a station still counts the edge's own run. A point of a circle edge of radius r stands up to 2r from the tangent line at another of its points, and it is on both faces.
- The open row's own fix names the edge's carrier, not a tangent line. So the wording for curved edges is not settled, and a sentence ruled on now may be rewritten.
- Ruling now buys nothing, since the code waits for the measurement in both options.

### Comes back to you after a measurement

- Every class of valid bodies the first unit newly refuses.
- The site and the classes it moves. Corners on long thin faces move toward refusing and smooth joins toward deciding. With it comes the joint turn's region: the walked face alone, or its two edges' neighbours (unsure).
- Boundary edges in the bound from below, which moves definite verdicts toward deciding.
- Any coarser gate for a consumer that treats two signs alike. It is built only if the measurement finds such a case on a body valid by a truth that is not the base's own verdict.

### What an agent lands without you

All of this is faithful to the text as it stands.

- The lever stays two-sided down to each scalar decision, in one home in `geom-core`.
- A disagreement is that decision's in-band reading: minted once as a straddle, and leaving each composite verdict through the in-band arm it already has. No verdict gains a variant.
- A missing bound from above is a typed case carrying its reason. Definite signs stand and nothing proves a Zero.
- Two bounds that contradict each other by more than the run's tolerance refuse loudly on a row of their own.
- The doors return the region, not a lever.
- The bound from above reads the face as the door holds it, which gives the site mint a cover. The lift-free cover stays.

This first unit reads from the middle as today and leaves the bound from below unchanged. Its differential can only move toward refusing.

## For the orchestrator

**(1) Wording: I hold, with one merged text.** The reason is above. The other report itself agrees the sum rows prove both verdicts from above (its point-1 note), so the short sentence contradicts what we both signed. The merged text takes its phrasing where the two differ only in words. If it has to be shorter, drop the last two sentences before the first two.

**(2) Site now or after: I hold "after".** The split is in For Ev. In round 2 I signed point 4 as "the edge's tangent line at a station". I now narrow that signature:
- It is proved for flat pairs and line edges only.
- For a curved edge the candidates are the tangent line at the station and the edge's carrier itself (distance from the curve). On a line edge the two are the same.
- So Q4 gains a fourth candidate site: the edge's carrier.
- This is the concrete way the README wording could change after it went up. With your note on today's rewritten fork, it is why I would not ask now.

**(3) Constructor: I move to the other report's loud refusal, keeping my threshold.**
- What moved me: an inversion past ε does not say which read is wrong. The lane's measured case (item 5) was the point standing off its carrier, so the bound from below is the suspect one. My "no bound from above, definite signs stand" trusted the wrong side.
- The form I sign:
  - An inversion of at most the run's ε is rounding, and the bounds are ordered. A point of a region is only known to lie on it within ε.
  - Past ε, neither bound is trusted. Every decision levered at that region refuses, on a row of its own naming the contradiction. The hits are counted (Q6) and filed at the source.
  - No hand-written slack.
- Consequence for sequencing: Q6 runs before unit 1 is dispatched. Every mid-door hit is a door that hands a face record that is not the face it governs. With the loud refusal that door stops certifying. So each hit is either fixed in unit 1 or reported as a class before it lands. This also bends "definite verdicts stay bit-identical" for unit 1, and its PR body has to say so.

**(4) Truth for the site: I sign the other report's T1/T2 split with one condition.**
- T1: closed form on plane–plane edges and the open row's table. Its "farthest vertex from the edge's line" and my "largest distance from the other face's plane" are the same number on two planes.
- T2 is admitted for the site only in a site-free form: the largest distance of one face's dense samples from the other face's carrier. It is never the oracle's distance from a chosen site, which assumes the answer.
- Site-free T2 judges definite claims only. A corner claimed with a margin larger than the faces' real largest separation is an over-read.
  - It is a flag to be read, not a count of truth. A sample can lie near the other carrier somewhere else (a plane meets a cylinder along two lines), which reads the gap short.
- No truth for Zero claims on curved pairs, where curvature separates tangent faces, or at the joint turn. There the candidate sites' verdicts are reported side by side as disagreements.
- Calibration on closed forms first, an interval result, and counting only samples inside by a definite margin all stand. The other report's T2 note ("still kernel code, reported as a second instrument") is compatible with this.

**Unchecked, still**
- The `CertifyError` grep: whether any caller acts on a refusal's decided sign.
- Whether `Cover::distance_range` from a line is tight on every piece kind. The cover has no distance from a curve at all, so the carrier site needs a new read.
- The symbolic lane's cost of either site.
- Whether the ε threshold in (3) should be the padded band at an import door.