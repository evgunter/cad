## For Ev

**My recommendation is unchanged, and I adopt the other designer's wording with one exception.** I would drop "an edge that left it and has turned back" from the sentence. Of the parenthesis, I keep "a part that does not touch what they share" and "the face's own end".

**Terms.**
- *Band*: K·ε.
- *Far run* (what is built today): the farthest any point of a face stands from the carrier of the edge it shares.
- *Near run*: how far a face runs out from the shared edge before its boundary closes back in on it.
- *Neighbouring edge*: an edge of the face that touches the shared edge at one of its ends.

**Recommendation (likely).** The two designers agree on all of the following.
- **One reading per cell beside a shared edge.** It is the sine of the wedge between the two faces, never more than one, levered at the narrower face's near run.
- **The near run is the lesser of two lengths.** The first is the least distance from the shared edge to any part of the face's boundary that does not touch it (a far edge, a hole's rim, an L's inner edge). The second is the face's own end.
- **The face's own end** is both the fallback and the cap. A cylinder's end cap, a half-disc, a sphere lune or a lens has nothing that cuts in, so it reads its far end. A part of the boundary standing past the edge's end can never read longer than the face itself.
- **Rate:** this replaces "the faces' rates summed". The sum reads the chord between the faces' far sides and certifies a strip narrower than the band.
- **Promise:** a certified solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there. Each pair of faces keeps this for its own boundary.
- **Tier 3 keeps one length per face and edge**, and per vertex shared alone, in the slots it has today. This is a kept length, not a rule in the walk, so the verdict does not depend on how the walk subdivides.
- **The far run stays tier 3's lever** for whether the wedge is a real angle. The question does not read it again, so "decides none of it" holds. A pass at the near run implies a pass at the far run, because near ≤ far (sure).
- **The question owns "pieces that do not meet are more than K·ε apart"** for a face's own boundary beside a shared cell, on every surface kind. The census's plane-only passes that read the same number within one solid are a follow-up row.
- **The derivation goes in the PR body, not the ratified text.** It is: "every body within the band of this one, with the same topology, stays embedded". A face that is ε off can tilt by ε over the far run, which is tier 3's class. It can move the edge sideways by ε over the sine, which must stay inside the near run, which is this reading. "The same topology" and "within the band" are not yet defined, and nobody has checked the rule against the question's other arms.

**Why I would drop "an edge that left it and has turned back" (likely).** The words have two readings. Neither works.
- **Reading 1: the least distance of the edge after it stops moving away.** Every arc that leaves one end of the shared edge and arrives at the other end "turns back" and ends on the shared edge, so it reads zero. That covers:
  - a D-shaped face's arc;
  - a half-disc's rim;
  - a sphere lune's second meridian.

  So this reading refuses ordinary faces, unless the return to the shared edge's end is excluded. That exclusion needs a distance cutoff, which is the very neighbourhood the reading exists to measure.
- **Reading 2: the edge's distance where it first stops moving away** (the other designer's recipe for the implementer). A D-shaped face then reads its bulge, which is the same as excluding the arc. But the edge that dips back beside the shared edge reads its first peak, not its dip, so the gap the clause was added for stays open.
- **Either way the clause adds a new decision with its own tolerance**: where a spline or arc "stops moving away" is a tangency read from enclosures.
  - A straight neighbouring edge collinear with the shared edge (a plate's top face whose side is split between two faces, an ordinary step) never moves away from the carrier. Measured from the carrier, it reads zero at once.
  - Measured from the shared edge's span instead, a straight edge never turns back, so the clause does nothing there.
- **The lens is not this case.** The other designer holds that the turned-back edge is the same thing as the lens's second edge. In a lens, the second edge is shared by the same two faces, so it is "what they share", and the lens reads its own end.
- **It is not the same term as the cap.** My cap is the face's own end; the turned-back edge would be a third term.

**What the dropped clause leaves open (unsure, a rare shape).** One shape is not caught: a single curved neighbouring edge that leaves the shared edge's end, swings out, and comes back within the band of the other face beside the shared edge without touching it.
- The pair across that edge reads the dip by distance wherever that pair's cell is clear of the shared vertex.
- It does not when that pair is itself certified whole by its own departure reading at the vertex.

I would file it as a row on the vertex arm, not write it into the ratified sentence. It needs a well-defined measure, which no one has yet.

**Which bodies read differently under the two sentences.**
- **D-shaped face, half-disc, lune:** the same under both if "turned back" means reading 2. Refused under theirs if it means reading 1.
- **The dip-back face:** certified at the pair under both if reading 2; refused at the pair under theirs if reading 1. Under mine it is left to the pair across the dipping edge.
- **A collinear neighbouring edge measured from the carrier:** refused under theirs, read by its far vertex under mine.

**Cases under the recommendation** (ε = 1e-9, K = 10)

| case | reading | verdict |
|---|---|---|
| 1 flat strip | 1e-7 × 5e-5 = 5e-12 | refuses, as today |
| 2 strip with an ear | the far edge closes in at 1e-7: 5e-12 | **refuses** (certified today) |
| 3 sector sliver, 10 m and 1 m | about 480ε, about 50ε | certified, as today |
| 3 sliver 20ε apart at its far end | about 2 bands | should certify; today's budget refusal is the walk's |
| 4 strips 0.6·K·ε at 2.6 rad, 0.8·K·ε at a right angle | 0.6 and 0.8 of a band | **refuse** under every separator (certified today) |
| 5 L-shaped face, hole beside the edge | the arm's width, the distance to the rim | by that |
| 5 face touching the edge's line again far away | not read by the lever; read by distance clear of the edge | by that |
| 6 spline shared edge | from the edge's own span | unmeasured; a least distance cannot count travel along the edge |
| lens 5e-12 thick | its own end | refuses |
| end cap, half-disc, D-shaped face, lune | their own end | certify, as today |
| 7 check 4 | keeps its far-run lever for its own fact | that row's question |

**The sentence I put to you.** In `[topology.tiers]`, replace "Over each cell it reads the rate … passes by more than the band" with:

> Over each cell it reads the sine of the wedge between the two faces, never more than one, levered at how far the narrower of them runs out from what they share before its boundary first closes back in on it (a part that does not touch what they share, or the face's own end) ([tolerance.epsilon]), never at a cell of the question's own subdivision, and that passes by more than the band. So a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

Provenance: designers wrote the sentence this replaces (commit `b130a7bbd5`). You approved it by comment on PR 7456, with a doubt on this lever.

**Remaining difference.** One clause. If the other designer keeps "an edge that left it and has turned back", the question for you is whether the sentence names that edge now, or leaves it to a filed row until it has a measure that reads a D-shaped face at its bulge and a dip at its dip. I lean to leaving it out.

**Confidence.**
- Lever, rate, promise: likely.
- Case 2 is within the band whatever the ear: sure.
- The face's-own-end term is needed: sure.
- The turned-back clause is ill-defined as written: likely.
- No valid ordinary body is newly refused: unsure, nothing was run.

## For the orchestrator

**What changed this round.**
- I adopt the other designer's "or the face's own end" in place of my cap wording. It is the same term (min with the far end), better said.
- I dispute only the middle item of the parenthesis, on the two-readings argument above.
- The other designer's round 3 says I would ratify the rule. My round 3 already put it in the PR body, so there is no difference there.

**Checked.** The other designer's citations: `extent_of` returns `None`, so `parts` never passes and a recipe with no strangers refuses. `BoundaryStations` reads each edge's middle for the half-disc, and `injective::spanned` gives a disc its radius. These are consistent with what I read in round 1. I did not re-open them this round.

**Not weighed.**
- The near run from a vertex shared alone. I assume the same terms, measured from the vertex.
- Whether a spline shared edge's "own span" needs its end tangents for the far-end term.

**Rows to file under either sentence.**
- The dip-back neighbouring edge, on the vertex arm.
- The census plane passes duplicating the question within one solid.
- A typed in-band refusal carrying sine × run. Today a failed lever reading surfaces as `Uncertified` or `Budget`, with no length.

**Fixes owed under either sentence.**
- `parts`' doc comment.
- The pinned test of the summed rate.
- The flat-strip fixture's "right verdict at the wrong door".

**What the measurement lane could overturn.**
- That chamfer ends, fillet ends and hexagonal faces near the band are not newly refused.
- That the dip-back shape is rare enough to leave to a row. One ordinary body with it would favour writing a measure for it now.
