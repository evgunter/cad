## For Ev

**Recommendation (likely on the shape; unsure between the last two criteria).** A conic stores one point and where that point sits on it.
- **The limit.** The point may be the centre only on a conic with no semi-axis longer than the session box (1 km). On a larger conic it is a point of the arc it serves, with the parameter zero there.
- **Within the limit, I lean to storing the point the mint itself computed.** That is the centre for a circle or ellipse made from a centre, and the vertex for a section computed at its vertex. The alternative is "always the centre within the limit". It is also defensible, and a measurement would decide between them (below).
- **Agreed already across the reviews:**
  - one evaluator, `P(θ) = O + u·a·(cos θ − k_u) + v·b·(sin θ − k_v)`. Here O is the stored point and k is where it sits in the conic's unit coordinates: (0, 0) for the centre, (cos θ₀, sin θ₀) for a point of the curve;
  - every reader states the curve from O;
  - a door handed a centre outside the box refuses;
  - the unsound 1e7 cover is a defect on main itself, which the new evaluator exposed.

**Terms.**
- *Admission floor*: the rounding a run already allows for a short sum of box-sized numbers, about 1.8e-12 m.
- *Box scalars*: `Interval` and the symbolic scalar.
- *u*: the unit roundoff of f64, about 1.1e-16.
- *Large conic*: one with a semi-axis longer than the box.

**The three criteria on the table**

*P. Where the centre is: the centre is stored if it lies in the box.*
- **Makes true:** no stored coordinate lies outside the box, which is the literal reading of `[tolerance.units]` ("no coordinate larger in magnitude").
- **Leaves possible what it should not:**
  - A rigid move can carry a centre across the box face while the arc stays inside. The move must then re-mint the conic on its arc, so moving a part changes how its carriers are stored.
  - That breaks `[intrinsic.invariance]` ("a designed rule commutes with rigid motions").
  - It also breaks the geom-core README's later sentence that the session box is "one predicate over the region a piece of geometry occupies, never a carrier's stored terms".
- **Reversal and re-seam:** unaffected.
- **Verdict:** rejected. (likely)

*S. Size alone: the centre is stored if no semi-axis exceeds the box; otherwise a point of the arc.*
- **Makes true:**
  - a pure fact of the curve, so a move, a reversal and a re-seam never change which point is stored;
  - one stored form per size of conic;
  - an authored circle keeps its centre exactly at every scalar.
- **Leaves possible what it should not:** a small section conic computed at its vertex (the cone section's mint) stores `vertex − u·a`, a centre the mint never computed.
  - At `Interval` the vertex then reads back with the semi-axis's rounding twice.
  - That is the same loss as the concentric hole-and-boss case below, now on "the vertex lies on the cone's ruling".
  - The step that stores cone sections at their vertex was measured narrower at `Interval` than the centre form. Those sections are probably box-sized, but I have not checked.
- **Under a move:** O is mapped and k is kept, so the form never changes.
- **Under a reversal:** k_v changes sign, exactly.
- **Under a re-seam:** a centre-stored conic re-seams by permuting its directions exactly. A large on-arc conic has its seam at its anchor and is never re-seamed: a different anchor is a re-mint.
- **Conflicts with nothing ratified,** once the sentence is reworded.

*L. Size as the limit, the mint's own point within it (my lean).*
- **Makes true:**
  - everything S does, plus the principle under the box-scalar evidence: the stored point is the value the mint holds independently, because the box scalars cannot cancel a derived one;
  - an authored centre is exact, and so is a section's vertex.
- **Leaves possible:**
  - two congruent small conics stored differently, centre on one and vertex on the other. k says which, and at f64 both are within the floor;
  - their stored data differ, so a twin test that compares stored data sees two carriers. That already happens between differently anchored large conics.
  - This is not the rejected "two forms, read as born": there is one evaluator, a far centre form cannot be built, and accuracy at f64 does not depend on how the conic was made.
- **Under a move and a reversal:** as S.
- **Under a re-seam:**
  - a small vertex-stored conic re-seams by permuting k exactly, as the built step does. Its parameter grid there is at most u·box, within the floor;
  - a large on-arc conic is not re-seamed.
- **Agrees with** `[intrinsic.invariance]`, because the choice is made once at the mint and every motion carries it. It also matches the README's box sentence and Ev's PR 6329 remark that another under-the-hood form is "not that bad".

**Why the centre must be storable at all** (sure).
- A hole and a boss share one centre, and each radius is a range of ±0.1 mm.
- Stored on its arc, each centre reads back as `(c + u·r) − u·r`, about ±0.2 mm, so "concentric?" cannot be decided.
- Stored at the centre, both read the same `c`, and the question is decided exactly.
- No change in the readers removes this.

**What decides L against S.**
- Measure the `Interval` widths and refusals on cone sections with a ≤ 1 km, stored at their vertex against stored at their centre.
- A material difference means L. None means S, the simpler rule.
- The pending size measurement (10 m to 1e4 m) decides the limit, not this choice. A wrong verdict on a centre-stored conic at or below 1 km would move the limit inside the box under either rule.

**"Parameter zero at the anchor".** Keep it for large conics, the only ones its reason binds. Its reason is that otherwise an edge's ends sit on a grid of u·a along the arc. Small on-arc conics may keep the built anomaly from the seam, with the anchor's place stored.

**Proposed sentence** ("Float rounding"; written by an agent, approved on PR 7194 before these measurements):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic stores one point, the one its mint computed, and where that point sits on it; the point may be its centre only where no semi-axis is longer than the session box's extent, and otherwise is a point of the arc the conic serves, its parameter zero there. Every reading of where a conic is starts from the stored point."

Under S, the words "the one its mint computed" become "its centre where it may be".

**Confidence.**
- One point plus k, with the size limit: likely.
- P rejected: likely.
- L over S: unsure.
- The box-scalar case: sure.
- A box-sized centre form stays within the floor: likely, not yet measured.

## For the orchestrator

**What moved me, and to what.**
- **Criterion: from P to L, with S defensible.**
  - The move argument against P is my own round-0 motion-invariance point. One report made it decisive, and the citations check out against the tree:
    - DESIGN line 173 `[intrinsic.invariance]`: "a designed rule commutes with rigid motions";
    - the geom-core README, "Float rounding": the box is "never a carrier's stored terms".
  - `[tolerance.units]` (DESIGN 753–756) does say "no coordinate larger" and "rejected at construction". Read with the README sentence, it is about regions, so it does not give P.
  - L's principle ("store the datum the mint holds") is the general form of my own hole-and-boss argument, and I had not applied it to sections. It persuades me as an argument. The evidence for it (step A narrower on cone sections) is not split by size, hence unsure.
- **Re-seam scope: moved.** "Parameter zero at the anchor" needs to bind only on large conics. Small on-arc conics may keep the built step's anomaly from `u_ref` with k stored.

**Whether my round-1 arguments are answered.**
- The far-centre refusal at the door: agreed by all three.
- The floor claim: shared, unmeasured. One report's hand bound is 8–10 u·box, another's is 15 against 16.
- The symbolic cause: one report, reading the test, finds the failing assertion is `sign(rim − radius)` with `rim = (q − circle.frame().origin()).norm()`. No evaluator is involved, which rules out my `sin²(θ/2)` hypothesis for that assertion. Freezing of the `c + u·r` forms is left. Under S or L that row is centre-stored and recovers either way.
- The cover attribution: settled as main's own.

**Where I still differ.**
- With the size-only report:
  - it should name the small-section cost;
  - its door "centre plus window, refuse only a far centre" is fine when both the centre and the arc are in the box (the anchor's terms are then box-sized), so there is no real disagreement left there.
- With the L report: I am less sure than "likely" until the split measurement exists.
- Its point about K is right: ε = 1e-12 is admitted only for K ≥ 2.8 (`ADMISSION_FLOOR / (K − 1)`). Have the size measurement report K, and separate wrong verdicts from typed refusals; that is the decisive distinction.

**Ask of the lane:**
- `Interval` widths on cone sections with a ≤ 1e3, stored at the vertex against stored at the centre;
- which mints that may produce a large conic lack an in-box arc point. Revolve rims, blend arms and the shallow cylinder cut are unchecked.

**Housekeeping:** read-only; I read only the two reports, plus DESIGN `[intrinsic.invariance]` and `[tolerance.units]` and the README sentence on origin/main, to check citations.
