## For Ev

**Recommendation (likely).** Keep the run as it was built: the farthest any point of the face stands from the carrier of what the two faces share, with the lesser of the two faces' runs as the lever. Change the rate it is multiplied by. Summing the two faces' sines over-reads at wide wedges, and that is the defect here. Read instead the sine of the angle between the two faces, where each face's angle off the separator is added as an angle (not as a sine), and the angle is taken as no more than a right angle. Then state in the sentence what a passing reading promises:

> Neither face lies wholly within the band of the other.

Near what the faces share, a band-wide neighbourhood is unavoidable. That neighbourhood is not read here (as `[topology.census]` already says). Any boundary of either face that lies inside it (a far edge, a hole's rim) belongs to other pairs, which read it.

The doubt you raised about "the narrower of them" turns out to rest on a wrong premise (below). The real issue later is the summed rate.

### Premise check
- **The problem statement frames the ear, L and hole cases as a fault in the run. They are not (likely).** In each, the narrow part's far boundary (an edge, a hole's rim) lies within ε of the other face. That boundary belongs to a third face: the face closing the strip, or the hole's wall. That third face shares nothing with the plate, so the existing rule applies: pieces that do not meet must be more than K·ε apart. A face that shares something with the plate fares the same way. A cell of it that holds no shared edge is read as a plain distance against the band. So in a finished body these cases refuse at the pair that actually owns the near-touch. Asking the A–B pair to catch them reads one fact twice.
- **Changing the run to "width across the edge" has no coherent version (sure).** Measured pointwise or as a minimum along the edge, the width is zero at both ends of every edge, because the face's next edges start there. Every face would refuse unless the edge's end neighbourhoods were cut out ad hoc. "Across" therefore means *perpendicular to the edge's carrier, not along the edge*, which is what was built for lines and circles.
- **The summed rate is not a fact of the pair (sure).** By its own comment in `injective::parts`, the sum lies between the wedge's sine (when the other face's carrier is the separator) and 2·sin(θ/2) (when the bisecting plane is the separator). So one pair reads differently depending on which separator ran. One reviewer showed this: a strip 0.8·K·ε wide at a right angle refuses under the carrier separator and is certified under the plane separator. The comment's "which no choice of separator moves" is false beyond small wedges.

### What the number means
Take a narrower face of run R (its farthest point from the shared carrier) and a wedge θ to the other face. Its farthest point stands at least R·sin θ from the other face when θ ≤ 90°, and at least R when θ > 90° (the nearest point of the other face is then on the hinge).

So the honest rate is sin(min(θ, 90°)), and the reading R·rate is a lower bound of `[topology.census]`'s own number for two faces sharing an edge: "their largest distance over the shorter cell, beyond what they share". The census picks the shorter cell, and that is where "narrower" comes from. It is the right choice, because a narrow face lying on a wide one is the sliver to refuse, whatever the wide face does (sure).

One consequence follows with no special case: **a face whose run is inside the band is never certified beside what it shares** (rate ≤ 1). Under the summed rate this fails, since the sum can reach 2.

### Each case under this answer
1. **Flat strip (run 1e-7, sine 5e-5): refused.** 5e-12 is inside the band. (sure)
2. **Strip plus an ear a metre out: the A–B pair is certified** (run 1 m, reading 5e-5 m), and that is correct for this pair: the strip is not one surface with the plate. The strip's far edge, 5e-12 off the plate, is read by the pair that holds that edge's other face against the plate. That pair refuses, as does the census's edge-against-face pass for planar cells. As noted, no finished body holds this shape; tier 3 refuses its sub-ε closing faces first. (likely)
3. **Sliver 10 m by 1 mm at 5e-8 rad: certified**, both runs 9.6 m. The 20 mm where the two faces are within ε exists beside every crease. `[topology.census]` already rules on it: "Near what two cells share, every pair is within the band whatever its angle. That neighbourhood is not a coincidence, and its extent is not a number the census reads." Tier 3 classes the edge a convex crease, so the two doors agree. (likely→sure)
4. **Strip 0.6·K·ε wide at 2.6 rad: refused** (rate 1, reading 0.6·K·ε). It is certified today, which is wrong: every point of it lies within 0.6·K·ε of the other face. The 0.8·K·ε right-angle strip also refuses, under either separator. (sure)
5. **L-shape, hole beside the edge, face meeting the edge's line again far away:** the lever reads the far arm, which is correct. The narrow arm's inner edge, the hole's rim (owned by the hole's wall), and the far re-touch (a cell holding no shared zero, read as a distance) are each refused at their own site when they lie within the band. (likely; no body was built)
6. **Spline shared edge:** the run must not count travel along the edge. Measure from the edge's arc continued along its end tangents, which for a line edge is exactly its line. Today's bare-arc reading over-reads, and could certify a narrow face beside a short spline edge. (principle likely; the exact continuation unsure; unmeasured)
7. **Check 4:** it should read the same number, meaning the lesser run and the angle rather than a sum, so that tier 3 and the question never disagree on one edge. Beyond that, it is the other program's row.

### Alternatives weighed
- **(A) Recommended: as above.** Gives one number per pair, independent of the separator, and a lower bound of the census's number. Each near-touch is read once, at the pair that owns it. Easy to reverse: the change is one rate expression and one sentence.
- **(B) As built (summed sines).** Certifies case 4 and depends on the separator. Not defensible past small wedges.
- **(C) Narrowest width across the edge.** Catches case 2 at A–B, but reads zero at every edge's ends (see the premise check). It double-reads what other pairs own and needs carve-outs. Rejected.
- **(D) The census's distance read directly** (the narrower face's largest distance from the other face, computed in tier 3). Exact where (A) is a lower bound, but it needs a face-to-face distance kept per edge. (A) is its cheap reading, and (D) is the target if (A) ever refuses a valid body. (unsure whether it ever would)

### Ratified text I would change
- **`[topology.tiers]`:** "the faces' rates summed" → "the angle between the two faces, each face's angle off the separator added as an angle, taken no wider than a right angle". After "measured across it", add: "(the farthest any point of it stands from the carrier of what they share: a line or circle edge's whole carrier, a spline edge's arc continued along its end tangents, a shared vertex)". Then add the promise: "A pair that passes is not a coincidence: neither face lies wholly within the band of the other. The neighbourhood of what they share is not read here ([topology.census]); a boundary of either face inside it is read by the pairs that hold it."
- **Provenance:** the sentence was written by designers (commit `b130a7bbd5`, fork-log row 304). You approved it on PR 7456 in passing, with the doubt you stated about the lever. The summed rate was a designer's compromise from round 3 that you were never asked about separately. The census sentence I lean on (`ea68daa237`) was also written by designers, in the fork answering your symbolic-tier question.

### What tier 3 keeps
Nothing new. It keeps exactly what was built: per face, its run from each shared edge and from each vertex shared alone; per edge, its class and turn; poles and apexes by key. Only the spline run's locus changes.

## For the orchestrator

- **The fix in code.** `injective::parts` folds the sines by `+`. It should combine them as sin(α+β), formed from the sines and cosines of each face's enclosure, capped at 1 for α+β ≥ π/2. Under the carrier separator the face's own sine already is sin θ. For θ > π/2 the carrier separator under-reads, which is conservative, and that separator is barred at seams anyway. Fix the doc comment on `parts`, which claims "the wedge's square over eight". The 0.8·K·ε right-angle probe and the 0.6·K·ε at 2.6 rad probe from reader 2 should become pinned rows that refuse under both separators.
- **Not checked:** whether the pair holding the strip's far edge always refuses in a finished body built on curved surfaces. The census's edge-against-face pass is exact only for planar cells. The general case rests on the question's other pairs: the "do not meet → more than K·ε" rule, and cells with no shared zero being read as distance (reader 2's account). A curved L-shape body would be the test. It is worth one probe before the sentence changes.
- **Not traced:** PR 7456's comment thread itself (`gh` unavailable). Ev's words are taken from the row and from fork-log row 304.
- **Spline row:** case 6 belongs on `work/honeone/the-wedge-reach-counts-a-faces-run-along-its-edge.md`. The end-tangent continuation is a suggestion, not a measured design.
- **Brief:** framing case 2 as "the definition certifies what the ruling meant to refuse" holds only if the pair has to own the near-touch of its own far boundary. I hold that it does not; state that premise to Ev explicitly if the other lanes disagree.
- A `git log -S` over the whole history in `/root/embed-vr` timed out and was left running in the background, read-only. The short-phrase runs completed.
