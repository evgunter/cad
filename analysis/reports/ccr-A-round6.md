## For Ev

**What changed since you last read.** The faster way of reading a conic stored at its centre has been dropped. It summed the seam vertex once, rounded, and read short chords from it. A rounding under 2e-12 m in that vertex moved near-tangent crossings by up to 2.7e-9 m, more than the tolerance, and the lane found 12 wrong answers on a 10 km circle. Read from its centre, as main does today, the same conic gave no wrong answers; it refuses those crossings instead.

**What is asked now (I sign it).**
- **A conic holds one of two placements:** about its centre, or about a point of the arc it serves, with its parameter zero there.
  - The construction fixes which, the same at every size. No cutoff.
  - A kernel construction whose centre can run off to infinity always holds a point of the arc. That is enforced by which constructor its code calls, so getting it wrong is a kernel bug, not a refusal a person meets.
- **A conic held at its centre is main's circle or ellipse, unchanged,** read from its centre at every scalar.
- **No length limit.**
- **Every reader states the curve through the conic.**

**What a person gets who writes a large circle by its centre:** the curve they wrote, never a wrong answer, and, as it grows, more typed refusals at near-grazing crossings (measured from somewhere between 500 and 900 m). The recourse is to write the arc by points on it, once sketch arcs can hold a point (a separate row), or to loosen the tolerance.

**A faster exact reading, which I no longer recommend even as a later step.** One idea was to sum the seam vertex exactly, as a rounded value plus its error, and read chords from that. It is exact for the stored direction numbers. But a stored frame's true shape is its nearest exact rotation: the open PR 7131 reads it that way, about one rounding per entry from the stored numbers. On a large conic with tilted directions that difference times the size is about the same tiny shift that caused the wrong answers. So this reading is exact for the wrong curve again. It could come back only if a measurement judged against the true-rotation curve, on tilted frames, showed no wrong answer.

**Sentence** ("Float rounding"; the proposed wording, with one clause widened):
> "A stored form that loses digits is re-minted in a well-conditioned form: a conic is one of a family of placements behind the one `Conic` reading, held about its centre or about a point of the arc it serves with its parameter zero there; its construction fixes which, the same at every size, and a construction whose centre runs off at a pose it can reach holds the arc. A conic is the curve its stored point states, and no reading replaces it with a rounded copy: one held at its centre is read from its centre, and so fixes a point of its arc only to the rounding of the centre's distance and of its size. Every reading of where a conic is goes through the conic."

The widening is "and of its size". The frame's rounding times the semi-axis is the same order as the centre's, so the sentence should name both.

**Confidence.**
- The decision as proposed: likely.
- That the exact-vertex reading does not survive a tilted frame: likely. This is arithmetic on PR 7131's reading of a frame, not measured.

## For the orchestrator

**(a) Signed, with one amendment.**
- In (3): "only to the rounding of the centre's distance" becomes "…of the centre's distance and of its size". PR 7131's polar read of the frame costs about ulp·a, the same order as the centre's rounding, so the sentence should not imply the centre is the only source.
- In (2): I would strengthen it. Do not call the exact vertex an "improvement under measurement" without the condition attached: it is judged against the polar-factor curve, on non-axis-aligned frames, and it comes back only if no answer is wrong there.

**(b) The frame objection: I concede it.**
- **Which truth.** PR 7131's decision 1 says "the true shape of a stored body is then its carrier on that frame", meaning the polar factor. The f64 lane's "as if exact" treats the stored entries as reals. On a large conic with a tilted frame those two curves are about u·a apart.
- **The exact vertex** removes the rounding of `c + u·a` relative to the entries. It does nothing about the entries-to-polar gap, which at near tangency carries the same lever of about 700 as the vertex rounding did.
- **So it does not survive a tilted frame** unless the f64 truth is defined as the entries. That definition would contradict what PR 7131 states about the body, and it would split one body into two truths by scalar.
- **The centre sum does not escape the gap either.** Its meter's running bound is about |c|·u times the pose's conditioning, which is far wider than the gap. That is why it refuses rather than answering wrong.
- **What a probe must be judged against:** the polar-factor curve; for axis-aligned frames the two coincide. The doors used so far were probably axis-aligned. I did not check, and if they were, a clean result there proves nothing about this objection.
- My round-5 recommendation of the exact vertex is withdrawn from the final state.

**(c) The other two reports, one line each**
- **Report 1:** "the reading then stays on the stored curve". That holds only for the stored entries, not for the polar frame PR 7131 makes the truth (above).
- **Report 1:** "storing a centre there is a kernel bug, so a panic". The guard is structural (which constructor the mint calls). A run-time panic has nothing numeric to test, as report 2 says.
- **Report 1:** its measurement 2 (charged chords, "if it certifies near arc-held counts it is the simpler choice"). "Float rounding" says "re-posing, never charging"; a charged reading needs a ratified exception, not just a good count.
- **Report 2:** nothing I think wrong. Its phase 1 (a) re-judge point (that row's "40 of 40, vertex summed" may share the twin-oracle error), its unfiled sketch-arc-by-points row, and its (f) 3 measurement of the nine unmetered readers should all go to the lane.

**For the record:** this is the third time a single rounded datum treated as "constant per carrier, harmless" has been refuted at a near-tangent root. Future designer briefs on this fork should state the root door's lever (about 700 at the probed depths) as a given.

**Housekeeping:** I read only the round-6 message and the two reports; read-only; nothing run.
