## For Ev

**What changed since you last read.** Two things are withdrawn, and nothing is added.
- **The "chords from the seam vertex" reading of a centre-held conic is out.**
  - It was judged against the wrong circle.
  - Judged against the circle about the stored centre, it gave 12 wrong definite answers on a 10 km circle: roots 1.4 to 2.7 nm off, at a tolerance of 1 nm.
  - Main's reading, which sums each point from the centre, gave none. It refuses instead.
- **The length rule is out.** The wrong answers were 500 times inside the length it would have set, and the centre sum needs no length to be right.

**What is asked now.** One decision, which does not wait on any measurement:

1. **A conic is one of two placements, as a family behind the one `Conic` reading.** It is held about its centre, or about a point of the arc it serves, with its parameter zero there. Each holds only its own numbers.
2. **The construction fixes which, the same at every size.** No cutoff.
   - A construction handed a centre stores it.
   - A construction that holds a point of the arc, and whose centre nobody wrote, stores that point.
   - A construction whose centre can run off to infinity at a pose it can reach always holds the arc. That is closed in code, by which door the mint calls.
3. **A centre-held circle or ellipse is main's, unchanged.** It is read from its centre at every scalar. The one new thing in the kernel is the arc-held placement.
4. **Every reading of where a conic is goes through the conic.** The roughly 110 readers that rebuild the curve from its centre are fixed under any answer.

**What a person gets who authors a large conic by its centre** (a 10 km `Center` arc, a revolve about a far axis):
- The body builds. At ε = 1e-9 it built through three real doors with the centre up to 100 km out.
- Crossings that are nearly tangent are refused, typed, more often as it grows. All 40 test poses certify at 500 m, 33 at 900 m, none at 10 km.
- No wrong answer in anything measured.
- A large conic certifies those poses only when it is arc-held. That depends on how it was constructed, never on its size.

**Named for later, not part of this decision.** Two designers propose reading a centre-held conic at f64 by chords from its seam vertex with the vertex summed exactly, so the curve read is the stored one. It is unmeasured. It has one condition I would hold it to (below), and if it earns its place it comes back as its own change.

**The sentence** ("Float rounding"). I sign the coordinator's text with one word changed, "stored point" to "stored numbers", since the directions and sizes state the curve as much as the point does:

"A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family of placements behind the one `Conic` reading, held about its centre or about a point of the arc it serves with its parameter zero there; its construction fixes which, the same at every size, and a construction whose centre runs off at a pose it can reach holds the arc. A conic is the curve its stored numbers state, and no reading replaces it with a rounded copy: one held at its centre is read from its centre, and so fixes a point of its arc only to the rounding of the centre's distance. Every reading of where a conic is goes through the conic."

**The condition on the later improvement.**
- "The stored curve" needs a frame as well as a centre.
- The open carrier-frames question (PR 7131) proposes that a stored frame's true directions are the nearest exact rotation to its stored entries, about one rounding per entry away.
- On a 10 km conic that is the same 1e-12 m as the rounding that just produced wrong answers.
- So an exact vertex is exact for one of two candidate curves. It is acceptable only once there is one stated truth for a stored frame, and the reading is shown clean against that truth on tilted frames.

**Confidence**
- The decision as asked: likely.
- The chord reading's withdrawal: sure (measured).
- The frame condition: likely (arithmetic, not run).

## For the orchestrator

**(a) I sign (1), (2) and (3).** One amendment to the sentence: "the curve its stored point states" becomes "the curve its stored numbers state". Decline it if you prefer; nothing else depends on it.

**(b) Would the exact vertex be acceptable if clean on tilted frames judged against the stored entries? Not on that alone.** It would need three things.
1. **One truth for a stored frame, across both lanes.**
   - The second report says to judge "each lane against its own definition": f64 against the stored entries, `Interval` against the polar factor.
   - That writes two curves about u·a apart into one stored body. PR 7131's own text rejects reading the entries as points because the read "would claim a rigid motion that does not exist", and says "the true shape of a stored body is then its carrier on that frame".
   - If PR 7131 lands as recommended, the f64 answer has to be right for the polar-factor curve. An exact vertex built from the stored entries is then off by (stored − polar)·a, at most about one ulp per entry times a. That is 1e-12 m at 1e4, the size that just moved roots by 2 nm.
2. **Clean against that truth on tilted frames.** There are two ways to get there.
   - The vertex is summed with the polar-corrected directions as an expansion. PR 7131's tightened read already sums `M − ½MG` exactly, so the pieces exist.
   - Or the truth is ruled to be the stored entries in both lanes, which reopens PR 7131's decision 1.
   - Either is a decision for Ev on that PR, not a detail of this one.
3. **Counts near the arc-held column on arcs away from the seam as well.** A circle has no re-seam door, so its arc can sit far from its seam vertex, and there the chords are long.

With all three I would accept it. An error-free sum is a sanctioned remedy.

**What the probe must be judged against, in consequence.**
- The lane should state its oracle's frame.
- For any frame that is not exactly orthonormal as stored, it should judge against both candidates until PR 7131 is settled.
- If the probe's doors are all axis-aligned, it cannot see this at all. It then needs one tilted door before its result means anything for the exact vertex.

**The same question touches main's centre sum, more weakly.**
- `Conic::running_point` documents the frame as "taken as exact".
- Its bound of about 3.8 u·ρ probably exceeds the frame's at most 1 u·a against the polar truth, by its constants and not by design.
- Worth one look when the lane re-tallies. I have not checked it.

**(c) What I think is wrong in the other two reports, one line each**
- *First:* "The curve read is then the stored centre's own, to the rounding of in-box numbers" holds only for the stored frame entries (point 1 above).
- *First:* charged chords are "rejected by ratified text (re-posing, never charging)". The root door already reads `running_point`'s bound, a listed charge site, so the clean ground against charging is the constant factor, not the clause.
- *First:* its list of runaway constructions includes "the plane–cone circle on a near-flat cone". That centre is far because the cone's apex is, which is the surface's far datum (the quadric row), not a pose of the section.
- *Second:* "judge each lane against its own definition" is the two-truths problem itself (point 1).
- *Second:* "storing a centre there is a kernel bug, so a panic", "with an assertion". There is no number and no site for a run-time assertion. Your summary's "closed in code" is the accurate form.
- *Second:* "an arc-held conic … certifies near-grazes from 500 m up". The size table has it equal to the centre form below 500 m and 40 of 40 from there; its eight refusals on the 1.25 ellipse at 10 m and 100 m are the same in every form. This is minor.
- *Both:* neither names the nine unmetered readers (phase 1 (c)) as the remaining exposure of a large centre-held conic on main. The probe's "0 wrong in the centre sum" is one metered door.

**Checks on your reading of round 5**
- It matches mine. I was the one who held the R rule and the one who objected on the frame.
- "Refuses from somewhere between 500 and 900 m" is the ε = 1e-9, K = 10 row. At ε = 1e-12 the centre form certifies almost nothing at any size at those depths, and the arc-held form only a third.

**What (1) leaves filed, so Ev's decision is not read as closing them**
- The profile's `Arc2` holds a centre for every sketch-arc spec. Until it can hold a start point, a flat `Bulge` arc is centre-held and gets the refusals above. Not filed.
- Four runaway mints still store a centre: the cylinder's tilted ellipse, `bisector_ellipses`, the oblique end face, `recognize_curve`'s circle.
- Circles about a far axis (the quadric row).
- The nine unmetered readers.

**Process:** read-only. This round I read the round-6 message and the two named reports; nothing was run, and no file in the tree was opened this round.