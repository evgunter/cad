## For Ev

**What changed.** Last round all three designers said a conic stored at its centre could be read "by chords from its seam vertex" and would then be as good as one stored on its arc. That rested on a measurement judged against the wrong curve. Judged correctly, that reading gives wrong answers: 12 of 768 decisions at a 10 km radius, roots placed 1.4 to 2.7 nm from the true ones at a tolerance of 1 nm.

**What I now recommend.** A centre-held conic is read exactly as main reads it today, from its centre. The only new thing in the kernel is the arc-held placement, for constructions that hold a point of the arc. I also withdraw the length rule I proposed.

Your four questions, with the answers as they now stand:
- **How ugly?** Less than last round. A centre-held circle or ellipse is main's, unchanged. The family adds one member.
- **Family pattern?** Yes: `at: Centre(point)` or `OnArc(point)` inside `Circle` and `Ellipse`, each holding only its own numbers, behind the one `Conic` reading.
- **No cutoff.** Unchanged. The construction fixes the placement. No size does.
- **The panic.** There is no number to assert (below). The kernel bug is a mint that stores a centre when its construction's centre can run off to infinity. It is closed by which door that mint calls, not by a run-time check.

**(a) What a centre-held conic is**

- It is the curve its stored numbers state: the circle or ellipse about the stored centre. Every reading of it is judged against that curve, and against no other.
- Those numbers fix a point of the arc only to the rounding of the centre's distance: about 2e-12 m at 10 km.
- At a near-tangent crossing that small uncertainty is multiplied about a thousandfold along the arc. That is the 1.4 to 2.7 nm.
- The chord reading answered confidently for the circle through the *rounded* seam vertex, which is a different circle. Its meter never saw the rounding, so it certified.
- The centre sum's meter does see it, so it refuses those poses. In every measurement so far the centre sum has 0 wrong answers.

So the reading is the centre sum, as main has it, at every scalar. (likely)

The chord reading could be kept with the seam vertex's rounding charged into the meter. The most that can gain is a constant factor of about five in the meter's bound, and a measurement for that is pending. I would not build it on what is known now.

**(b) Which point is stored matters for a large conic**

This is the original finding, and it stands.
- Arc-held: certifies near-graze poses at every size measured, 10 m to 10 km. Its stored point is exact, so its own curve is the truth.
- Centre-held: certifies 40 of 40 up to 500 m, 33 at 900 m, 28 at 1 km, 7 at 1.5 km, 0 at 10 km. The rest are typed refusals, never wrong.

What follows, under your two constraints:
- A construction that holds a point of the arc, and whose centre nobody wrote, stores that point, at every size. This covers the sections whose centre can run away. It should also cover sketch arcs written by points (`Bulge`, `Via`, `Sweep`, `ArcLen`), which today keep only a centre one level up, in the profile.
- A construction whose datum is a centre stores it, at every size.

**What a person gets who authors a large centre-held conic** (a `Center` arc of 10 km, or a revolve about an axis 10 km away):
- The body builds. At ε = 1e-9 it built through three real doors with the centre up to 100 km out, and was refused, typed, from 1,000 km.
- Operations that cross it nearly tangentially refuse more often as it grows, from about 500 m.
- No wrong answer in anything measured.
- The recourse is to write the arc by points on it, or loosen the tolerance.
- Nothing switches its form behind the person's back.

**(c) The length rule: withdrawn.** (likely)

- I proposed that a centre-held conic have no semi-axis past the rounding reach R, because the chord reading's shift "fits the headroom" up to R. Three of the wrong answers are at 10 km, with R at 5,000 km. A near-tangent root is not a well-conditioned reading, so R does not bound it.
- Last round I said: if the probe shows only typed refusals with the centre read, certification alone would do. That is what it shows: 0 wrong in the centre sum. Certification itself refuses the bodies it cannot hold: from 1,000 km at ε = 1e-9, and every far centre tried at ε = 1e-12.
- One exposure remains, and it is on main today: nine filed readers that decide on a far-centred conic's terms without any meter. They are fixed by reading through the conic, not by a length.

**(d) Round 4's points**
- Placement inside `Circle`/`Ellipse`: held.
- The construction fixes it, no size: held.
- A runaway-centre construction holds the arc: held.
- Centre-held read by chords at f64: withdrawn.
- Centre-held read about its centre at `Interval` and the symbolic scalar: held, and now the same at f64.
- A length rule at R: withdrawn.
- Readers state the curve only through the conic; the 1e7 cover is main's; the symbolic loss is node identity; parameter zero at the stored point for arc-held: all held.

**(e) The sentence** ("Float rounding")

"A stored form that loses digits is re-minted in a well-conditioned form: a conic stores the point its construction holds, its centre or a point of the arc it serves with its parameter zero there, as one of a family of placements behind the one `Conic` reading, and no size chooses between them. A conic is the curve its stored point states; one held at its centre is read from its centre, and so fixes a point of its arc only to the rounding of the centre's distance, and a construction that needs the arc held closer stores a point of it. Nothing states where a conic is except through that reading."

**(f) Measurements still needed**

Each must be judged against the curve the stored numbers state: the stored centre's for a centre-held conic, its own for an arc-held one.
1. *The chord column with the seam vertex's rounding charged.*
   - Any wrong answer: the chord reading is dead.
   - 0 wrong and nearly all of the 900 m to 1.5 km cells recovered: it earns a place as the centre-held f64 reading.
   - Within a few poses of the centre sum: keep the centre sum.
2. *The size table's uncharged chord column, re-judged.* I expect wrong cells at the large sizes. A clean result at small sizes would not rescue it: it would only mean the shift is under ε there.
3. *The nine unmetered readers, on bodies stored through real doors, centre read.*
   - Any wrong answer at a size the doors actually store: those rows become urgent, or a length rule comes back.
   - None: no length rule, confirmed.

**Confidence**
- The centre sum as the reading: likely.
- No length rule: likely.
- The chord reading's wrong answers: sure (measured, and the arithmetic agrees).

## For the orchestrator

**What I got wrong, and where it came from**
- **Round 3.** I moved the chord reading into the centre-held evaluator on the size table's "digit for digit" row, and wrote that a centre-held conic "can read as well as the arc kind at any size".
  - The row's own README says the reader-chord form was "built as an anchored conic whose anchor is that sum". I read that and did not ask which conic the oracle then judged.
  - That was the place to catch it.
- **Round 4.** I justified R by "the seam-vertex sum is one short chain at the centre's distance, and R is where that leaves the headroom".
  - That treats the reading as lever 1. The root door's lever at these poses is about a thousand.
  - "Float rounding" has the pattern for this ("the pose's band also reaches as far as the quotient's lever passes R"), and I did not apply it.
- **My own arithmetic check now.** At depth d = 1e-6 against a wall of radius 1, the root sits about √(2d) ≈ 1.4e-3 m along the arc. Its displacement per metre of normal shift is about 1/1.4e-3 ≈ 700. With ulp(1e4) = 1.8e-12 that gives about 1.3e-9 m, matching the reported 1.4e-9 to 1.9e-9. By hand, not run.

**One further point on what a centre-held conic is, which the lane should fix before re-tallying**
- "The circle about the stored centre" also needs a frame.
- PR 7131 (open, and the unit is stacked on it) reads a stored frame at `Interval` as the polar factor of its entries, about one ulp per entry from the stored numbers.
- On a centre-held conic one ulp of direction times the semi-axis is the same u·a as the centre's rounding. For a frame that is not axis-aligned there are therefore two candidate truths u·a apart: the stored entries read as reals, and the polar factor.
- The probes' doors may all be axis-aligned; I did not check. The re-tallies should say which frame the oracle used, and for tilted frames judge against both.
- This is also why an exact (error-free) seam-vertex sum does not rescue the chord reading. It is exact for the stored entries and still u·a from the frame the `Interval` lane certifies. The PR 7194 designers said this ("not exact under PR 7131's reading of a frame"), and it holds.

**Phase 1 (a) needs the same re-judging**
- The far-centre row records: "Reading the point from the seam vertex … certifies 40 of 40 in all five families … every root within ε of the exact oracle's", with the bound 6.4e-13 → 1.28e-13.
- If that oracle ran on the seam-vertex twin, the row's premise (the vertex evaluation recovers the near-gap families with no stored form) is unverified too.
- The 1.28e-13 figure (0.77 u·ρ) suggests the vertex's rounding was charged there. If so, phase 1 (a) is already measurement (f) 1 at ρ = 1500. Worth one look before commissioning a new run.

**Checks on the message**
- "No length rule (two of you) or a rule at R (one)": the one was me. Withdrawn on my own stated test.
- "The centre-held one refuses near-graze poses from about 500 m": the size table's centre column is 40 of 40 at 500 m and first loses at 900 m (33). So it is "from somewhere between 500 and 900 m". At ε = 1e-12 the centre form certifies almost nothing at any size at those depths.
- The far-centre probe's README gives the centre sum 0 wrong over all cells. That is one metered door. It says nothing about readers without a meter. Phase 1 (c) found eleven of thirteen such sites misreading on main once terms pass the box, one of them at the default ε from 1e4 (the section-area excess). That is why (f) 3 is on my list.

**What the final state now needs from other rows**
- **Sketch arcs written by points.** The profile's `Arc2` holds a centre for every spec, so a flat `Bulge` arc is centre-held and meets `ArcBelowSceneResolution` or a certificate refusal (the probe's build table). The arc-held placement only reaches these when the profile's arc can hold its start point. Not filed. It is now the main route by which a person gets a large conic that certifies.
- **The four runaway-centre mints** that still store a centre: the cylinder's tilted ellipse, `bisector_ellipses`, the oblique end face, `recognize_curve`'s circle.
- **Circles about a far axis** (the quadric row). The probe shows nothing refuses a revolve about a far axis before the conic is minted. The edge certificate does it afterwards.

**The panic**
- With the centre read, a wrongly centre-held conic produces refusals, not wrong answers. So no run-time assertion is needed for soundness, and none has a number to test.
- If Ev still wants one, the only honest site is the edge certificate's refusal on a carrier minted by a kernel section door (as against a person-sized door). Reaching it there means the mint chose wrong. That needs the certificate to know which door minted the carrier, and I would not add that for a check's sake (`[simple.checks]`).

**Unchecked**
- Nothing was run. This round I read the message and `far-centre-through-real-doors/README.md` on `origin/analysis/oneround/conic-vertex-probes`.
- I did not open the `decisions-*` raw files.
- The "factor of about five" for a charged chord reading is fork 294's figure for the meter's rounding term (3.8 u·ρ against 0.77 u·ρ). The size table's slack column includes the pose's conditioning and is not comparable.