## For Ev

**What changed.** The measurement behind round 4's centre-held reading was judged against the wrong circle. That reading summed the seam vertex in ordinary floating point (`fl(centre + u·a)`) and then added short chords from it. Judged against the circle about the centre the conic actually stores, it gives wrong definite answers.
- Through real doors at radius 1e4 m and ε 1e-9: 12 wrong roots out of 1,536 poses. Each is 1.4 to 2.7e-9 m from the true root.
- The cause is one rounding of the seam vertex, under 2e-12 m. At a near-tangent crossing that small shift moves a root by more than ε.
- The plain centre sum, which is what main does today, gave no wrong answer at any distance. It refuses typed instead.

**What I now recommend.**
- **Keep everything else from round 4.**
  - Two placements inside `Circle` and `Ellipse`: "about its centre" or "about a point of its arc".
  - The construction fixes which one, at every size. No cutoff.
  - A construction whose centre can run off to infinity at a reachable pose must hold a point of its arc. Getting that wrong is a kernel bug, enforced in code.
- **Change one thing: what a centre-held conic is, and how it is read at f64.**
  - A centre-held conic is the circle (or ellipse) about its stored centre, and every reading is judged against that curve. A reading that silently replaces it with a curve through a rounded point is unsound, however small the rounding.
  - The reading I recommend: chords from the seam vertex, with the vertex computed **exactly**. The sum `centre + u·a` is kept as a rounded value plus its exact remainder, an error-free sum. The geom-core README already lists that as a sanctioned remedy beside re-posing.
    - The curve read is then the stored centre's own, to the rounding of in-box numbers.
    - The near-graze gain of chords is kept.
  - Rejected: chords with the vertex's rounding added to the meter. "Float rounding" says "re-posing, never charging", and charging only shrinks the loss by a constant.
  - Fallback if the exact vertex does not pay for itself: the centre sum, as main has it. It is sound, and it refuses near-graze poses from about 500 m.
- **At `Interval` and the symbolic scalar:** the centre sum about the stored centre, unchanged from round 4.

**What a person gets who authors a large centre-held conic** (a 5 km `Center` arc, a revolve about a far axis, a STEP circle):
- the curve about the centre they wrote, exactly;
- never a wrong answer;
- near-graze questions either certify (with the exact vertex, as an arc-held conic would; to be measured) or refuse typed (with the centre sum).

No size limit applies to them, and no panic.

**Does the stored point matter for a large conic?** Less than the uncorrected measurement suggested, but it still does.
- With an exact vertex, a centre-held conic should read as well as an arc-held one near its seam vertex.
- It cannot read well on an arc far from every vertex. That is the shallow cylinder cut, whose arc sits about 0.45 rad from any vertex.
- That is why the constructions whose centre runs off must hold the arc. It is a fact about those constructions, not a size rule.

**The length rule: still none.**
- The uncorrected reading went wrong at D = 1e4 m, five hundred times inside the run's rounding reach R (5e6 m). So no length rule would have caught it.
- The centre sum is right at every distance without one.
- Soundness comes from reading the stored curve, not from a length.
- I also withdraw my round-3 claim that certification would catch a far centre's rounding. Here a 1e-12 m shift moved a root by 2e-9 m, and no residual check reads that.

**Sentence for "Float rounding":**
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family of placements behind the one `Conic` reading, held about its centre or about a point of the arc it serves with its parameter zero there; its construction fixes which, the same at every size, and a construction whose centre can run off at a reachable pose holds a point of its arc. A conic is the curve about what it stores, and every reading is of that curve: a centre-held conic read from a point of itself reads from that point summed exactly, never from a rounding of it, and every reading of where a conic is goes through the conic."

**Confidence.**
- A centre-held conic is the curve about its stored centre: sure.
- The uncorrected chord reading is out: sure (measured).
- The exact vertex restores the chord gain: likely (reasoned; owed a probe).
- No length rule: likely.

## For the orchestrator

**(a) The reading, in detail**
- The stored data define the curve. An f64 reading is judged against the exact curve of the stored data: for a centre-held conic, the oracle on the stored centre.
- **Exact-vertex chords:**
  - Compute `w = c + u·a` (an ellipse at a general phase also adds `v·b`) as an expansion: `TwoProd` by FMA for `u_i·a`, then `TwoSum` with `c_i`. geom-core's `exact::Expansion` already serves PR 7131's polar read.
  - Round it to `(hi, lo)`, with `|lo| ≤ ½ulp(hi)` and a remainder of about u²·|w|.
  - Read `P = hi + (lo + u·a·(cos θ − 1) + v·b·sin θ)`. Rounding is then about u·|P| + u·|chord| + u²·D, which is the arc-held quality, and it is the stored centre's own curve.
  - No meter charge is needed. The running bound counts `lo` as exact data.
- **Charged chords are rejected by ratified text** ("Re-posing, never charging"; "No `Margin` door takes hand-written slack"). The three poses taken apart (a 2.3e-13 m vertex rounding moved a root 1.2e-9 m, a lever of about 5e3) suggest the charge would claw back roughly what the centre sum loses times a constant. That is the PR 4812 "5×" again.
- The 12 wrong roots at K = 10, D = 1e4 being far inside R confirms that the uncharged chord's fault is not a length.

**(b) Feasibility note.** The exact vertex runs once per reading, a handful of FMAs. Caching it is possible, but a cache is a second description kept in step, so I would compute it per read. That is the lane's call on cost.

**(d) Round 4, one line each**
- Placement enum inside each kind, `Curve3` unchanged: held.
- Construction fixes the placement, no size picks: held.
- Runaway constructions hold the arc, enforced structurally, wrong = kernel bug: held. The list is the plane–cylinder tilted ellipse, the plane–cone circle on a near-flat cone, `bisector_ellipses`, the oblique end face, STEP's recognised circle.
- Centre-held f64 reading by chords from `fl(c + u·a)`: **withdrawn**, replaced by exact-vertex chords (or the centre sum).
- Centre sum at the enclosure scalars: held. The probe's chord-`Interval` lane shows no wrong answers either way.
- No length rule: held, now on soundness grounds.
- Certification as the backstop for a far centre's rounding: **withdrawn**.
- Readers through the conic, directions read without a position, `homothety` mapping the stored point: held.
- The 1e7 cover is main's own; the symbolic loss is node identity: held.

**(f) Measurements still needed, each judged against the stored datum's exact curve** (the stored centre's circle for centre-held conics, the stored point's for arc-held)
1. **Exact-vertex chords on Probe 1** (the three doors, D 3e3, 1e4, 1e5) and on the size table (10 m to 10 km), at ε 1e-9 and 1e-12, K 10 and 1.002.
   - 0 wrong, with certified counts near the arc-held column: exact-vertex chords are the final reading.
   - 0 wrong, with counts near the centre sum: drop it and keep the centre sum (simpler; same answers).
   - Any wrong: a bug or a flaw in the reasoning; fall back to the centre sum.
2. **Phase 1 (a)'s "vertex summed in f64, 40 of 40" and its "summed exactly" meter** (`a-conics-point-is-evaluated-from-its-far-centre-not-its-vertex.md`): check which oracle judged them. If it was the twin, those counts carry the same error, and the row's text needs correcting.
3. **The size table's reader-chord re-tally and the per-metre root displacement (already asked):** informative only. The uncharged chord is out whatever they show. The displacement figure sizes how much a charge would lose, for the record against option "charged".
4. **The arc-held column stands,** since its stored point is exact. Large centre-held conics at an arc far from any vertex (a circle seamed away from its arc) want one probe of the exact-vertex reading, to confirm that only the seam's distance from the arc bounds it.

**Errors I made earlier**
- Round 3 rested on the reader-chord column, and round 4 carried it forward.
- My round-3 "certification catches the shift" was reasoning, not probe, and is refuted by the amplification at tangency.

**Housekeeping:** I read only the round-5 message; read-only; nothing run.
