## For Ev

**What changed.** The fast way of reading a conic stored at its centre turned out to be wrong. Last round all three of us agreed that a centre-held conic would be read at f64 by short chords from its seam vertex, rounded once (`fl(centre + u·a)`, where `u` is the seam direction and `a` the semi-axis). That rounding moves the whole curve slightly, by up to about 2e-12 m on a 10 km circle. At near-tangent crossings that tiny move shifts a root by up to 2.7e-9 m, more than the tolerance ε = 1e-9.
- So that reading answers for a slightly different circle from the one stored, and the lane found 12 wrong definite answers.
- Three of them are at an ordinary run setting (K = 10, the ratio between "definitely apart" and "the same point"), on a 10 km circle. That is far inside any length limit we discussed, so no length limit would have saved this reading.
- Main's way of reading (sum every point from the centre) gave no wrong answers, only more refusals.

**What I now recommend** (likely):
1. **A conic stored at its centre is the curve about that centre.** Every reading, and every test of one, is judged against that curve, never a rounded copy of it.
2. **At f64, read it by chords from its seam vertex, with the vertex held exactly.** Compute `centre + u·a` as an exact pair of numbers: the rounded value plus its rounding error, recovered by the standard error-free product and sum. Then sum the chord on top of that pair.
   - The reading then stays on the stored curve and keeps the accuracy of the arc's own scale.
   - This is the error-free transformation "Float rounding" already permits.
   - It is unmeasured, so a probe is owed first (below). If it fails, the centre-held conic keeps main's centre sum: sound, with more refusals.
3. **At `Interval` and the symbolic scalar, read about the centre, as main does.** There a large centre-held conic is wide by about u times its size near the arc, and no reading fixes that. Its directions are read at `Interval` as the nearest exact rotation (PR 7131), about one rounding wide per entry, and that width is multiplied by the distance from the stored point.
4. **Everything else from last round stands:**
   - one placement inside each conic, centre or arc point, chosen by how the conic was built, never by a size;
   - a kernel cut whose centre can run off to infinity always stores an arc point, and storing a centre there is a kernel bug, so a panic;
   - no length limit, which the new numbers make stronger.

**Does which point is stored matter after all?** Yes, for large conics.
- At `Interval` it matters for good, by the width above.
- At f64 it matters unless the exact vertex works.
- An arc-held conic's stored point is exact, so it is its own truth and certifies near-grazes from 500 m up. A centre-held one refuses many of them.
- This is why the kernel's own cuts must store the arc point they hold. It is not a reason for a size rule: no size marks where the loss starts, and converting a centre you wrote into an arc point would store a different curve, which is exactly the mistake just measured.

**What a person who authors a large circle by its centre gets:** never a wrong answer. Near-grazing crossings may be refused with a typed error, more of them at `Interval`, and the recourse is to loosen the tolerance or reshape the part. With the exact vertex, f64 should certify most of them; that is the probe below.

**Proposed sentence** ("Float rounding", `crates/geom-core/README.md`):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family of placements behind the one `Conic` reading, held about its centre or about a point of the arc it serves with its parameter zero there. Its construction fixes which, the same at every size, and a construction whose centre runs off at a pose it can reach holds the arc. A conic is the curve its stored point states: at `f64` a centre-held conic is summed by chords from its seam vertex held as an exact sum, and at the enclosure scalars about its centre; no reading replaces a conic with a rounded copy of it. Every reading of where a conic is goes through the conic."

If the probe fails, the clause on the exact sum becomes "a centre-held conic is summed from its centre".

**Measurements before deciding.** Each one is judged against the exact roots of the stored data's own curve: the stored centre, the stored directions as their f64 entries, the stored sizes. For an arc-held conic, its stored point.
1. **The exact-vertex chord reading,** on the far-centre probe (D = 3e3, 1e4, 1e5) and on the size table (10 m to 10 km), at ε 1e-9 and 1e-12.
   - No wrong answers and counts close to arc-held storage: adopt it.
   - Any wrong answer: the reading still replaces the curve somewhere, so fall back to main's centre sum.
   - Counts close to the centre sum's: it gains nothing, so use the centre sum.
2. **Chords with the vertex rounding charged** into the root door's slack, which is the lane's open item. I expect no wrong answers but counts near the centre sum's: that charge is again about u·a, times the same near-tangent sensitivity. That is the "about 5×" PR 7194 already found. If it certifies near arc-held counts instead, it is the simpler choice, and the exact vertex is unnecessary.
3. **The re-tally of the size table's reader-chord column** against the stored centre's circle. It is expected to show wrong answers and confirm the correction; it decides nothing new.

**Confidence:**
- that a centre-held conic is its stored curve and must be judged against it: sure;
- that the uncharged chord reading must go: sure (measured);
- that at `Interval` a large conic's storage matters for good: likely, by arithmetic;
- the exact-vertex reading: likely sound by construction, its gain unmeasured;
- no length limit: likely, now stronger.

## For the orchestrator

**What moved me**
- The correction refutes the f64 chord reading I signed in round 4. It also shows a single rounded vertex is not a "constant per carrier, harmless" quantity. A near-tangent root has a lever of about 1000 on a rigid shift: 1.8e-12 m became 2e-9 m.
- I now treat the "anchor recomputed from the centre" variant as dead unless it is exact or charged. PR 7194's "5×" finding is consistent with option 2 above.
- New in my answer: an error-free seam vertex.
  - `TwoProd(u_i, a)` by `f64::mul_add` gives `p + e` exactly. `TwoSum(c_i, p)` gives `s + t` exactly. The vertex is then `s + (t + e)`, accurate to about u²·a.
  - A point is `s + ((t + e) + offset(θ))`, where `offset` is the chord term `u·a·(cos θ − 1) + v·b·sin θ`.
  - `mul_add` is correctly rounded IEEE, so D9's determinism holds.
  - The tree has `exact::Expansion`, which PR 7131's polar read uses.
  - PR 7194 dismissed "a centre form with exact sums" as "a new scalar operation, nothing at `Interval`". That dismissal assumed the centre stayed the truth for every conic. Here it serves only centre-held conics at f64. Ev should hear that this revives that option on new evidence.
- **Interval, made explicit:** a frame's `Interval` read is the polar factor (PR 7131, decision 1), so a centre-held conic's enclosure near its arc is about w·a wide, inherently. The chord-`Interval` column (0 wrong, many refusals) fits this.

**Round 4's points: what I hold** (as asked in (d))
- Placement enum fixed by construction, no size: held.
- Runaway-pose mints hold the arc, with an assertion: held.
- f64 chords from `fl(centre + u·a)`: withdrawn. Replaced by the exact vertex, or else the centre sum.
- Enclosure scalars read about the centre: held.
- No length rule: held, stronger. The R rule fails at K = 10, D = 1e4.
- "Parameter zero at the anchor" for arc-held conics: held.
- Covers and readers go through the conic: held.
- The 1e7 cover is main's own, and the symbolic loss is node identity: held.

**Owed, and assumed**
- That the oracle in measurement 1 reads the stored f64 entries as given, not their polar factor. The f64 curve is defined with those entries as stored; the `Interval` curve uses the polar factor, and the two differ by about u·a near a far arc. Make sure the lane judges each lane against its own definition.
- The lane's open item "the root's displacement per metre of rigid shift": it sizes measurement 2's charge. Ask for it with measurement 2.
- I did not read `far-centre-through-real-doors/README.md`; I relied on the message's verbatim tallies. Nothing was edited or run.
