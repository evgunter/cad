## For Ev

**My recommendation changed back to one reading.** The question reads one number beside a shared edge, levered at the near run. I withdraw round 2's separate "the faces are distinct at the far run" reading. The two designers now recommend the same lever, rate and promise. One small question is left for you, at the end.

**Terms.**
- *Band*: K·ε.
- *Far run* (what was built): the farthest any point of a face stands from the shared edge's line or circle.
- *Near run*: how far the face runs out from the shared edge before its boundary first closes back in on it. "Closes back in" means the first of three things:
  - a part of the boundary that does not touch the shared edge (a far edge, a hole's rim, an L's inner edge);
  - a neighbouring edge that left the shared edge's end and has turned back toward it;
  - the face simply ending (a disc on its rim, a half-disc, a lens).

**Recommendation (likely).**
- **Lever:** the lesser of the two faces' near runs.
- **Rate:** the sine of the wedge between the faces, never more than one, in place of the two rates summed.
- **Promise:** a certified solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there. Each pair of faces keeps this for its own boundaries.
- **Far run:** it stays tier 3's lever for whether the wedge is a real angle. The question does not read it.

**One reading, not two.**
- The near run is never longer than the far run, so a pass at the near run implies the faces are distinct at the far run (sure). Reading that separately would make the question decide tier 3's fact again, which the sentence forbids ("decides none of it").
- This holds only if "the face simply ends" is part of the near run. The other designer's recipe (least distance to boundary edges sharing no vertex with the shared edge, and to vertices off it) finds nothing on some ordinary faces:
  - a cylinder's end cap on its rim;
  - a half-disc face;
  - a sphere face between two meridians.
- With no length, the code decides nothing and the body refuses (`extent_of` returns none, and `parts` then never passes). Those faces need their far end as the run. So does a lens 5e-12 thick between two shared edges, which must refuse (sure).
- The third clause also closes the gap both reports left open, the neighbouring curved edge that dips back beside the shared edge. It is the same thing as the lens's second edge: a neighbouring edge counts from where it stops moving away.

**A kept length, not a rule in the walk.** Round 2 offered both forms; I recommend the kept near run.
- **Kept near run.**
  - Tier 3 keeps one length per face and edge, in the slots it has now.
  - It is a fact of the shape, the same whichever way the walk subdivides, and it rides an isometry with the verdict.
  - A refusal can carry a length against the band and the lever to move (widen the face or open the angle).
  - It leaves possible a refusal where the sine times the run under-reads the true gap, such as a narrow slot at the edge's end under a very shallow wedge. That is the safe direction.
  - One near-touch can be reported by two pairs.
- **Rule in the walk** (a certified cell may not hold boundary that does not meet the shared cell; such parts are read by distance).
  - It is exact on every surface.
  - Its verdict depends on the walk reaching cells as small as each narrow face, which costs depth on every fillet and chamfer.
  - It needs its own exemption for boundary near an end vertex.
  - It still needs "the faces are distinct" from somewhere: either tier 3's check 4 (which today levers at the farther face) or a second reading. A thin lens certifies otherwise.
  - Its refusal names two cells and a gap. That is informative, but it arrives only after the walk gets there.
- I ranked the rule as "the general form" in round 2. As a final state it is the worse one: it ties the verdict to the subdivision, which the sentence rules out in spirit ("never at a cell of the question's own subdivision").

**The other designer's rule.** "Passes by more than the band means every body within ε of this one, with the same topology, is embedded."
- It is the right derivation (likely). A face ε off can tilt by ε over the far run, which is tier 3's class. It can move the edge sideways by ε over the sine, which must stay inside the near run. That is the question's reading, and your PR 7095 question.
- I would not put it in the ratified text now.
  - "Within ε of this one" and "the same topology" are not defined anywhere.
  - Nobody has shown it holds for the question's other readings: a vertex shared alone, a face against itself, a face that curls back.
  - It would bind all of those.
- It belongs in the PR body as the reason. Ratifying it as the definition is its own decision.

**Which door owns "pieces that do not meet are more than K·ε apart" for a face's own boundary.** This is part of this answer. Choosing the near run is choosing that the question owns it, for every surface kind. The sentence already sits in the question's paragraph (commit `1734a9746b`). What follows from it is that the census's vertex-against-face and edge-against-face passes read the same number within one solid for plane faces. That duplication is a follow-up row, not a blocker.

**Cases** (ε = 1e-9, K = 10).

| case | near run × capped sine | verdict |
|---|---|---|
| 1 flat strip | 1e-7 × 5e-5 = 5e-12 | refuses, as today |
| 2 eared strip | the same 5e-12 (the far edge closes in first) | **refuses** (today certified) |
| 3 sliver, 10 m and 1 m | 9.6 m → about 480ε; about 50ε | certified, as today |
| 3 sliver, 20ε at far end | about 2 bands | should certify; today's budget refusal is the walk's |
| 4 strips 0.6·K·ε at 2.6 rad and 0.8·K·ε at a right angle | 0.6 and 0.8 band | **refuse** under every separator (today certified) |
| 5 L, hole | arm's width, distance to the rim | by that |
| 5 line touched again far away | not read by the lever | n/a |
| 6 spline edge | from the edge's own span | unmeasured; a least distance cannot count travel along the edge |
| lens, end cap, half-disc | the face's own far end | lens refuses; cap and half-disc certify as today |

**Check 4 (case 7).** It keeps its own far-run lever for its own fact. The question refusing a body tier 3 passed is then the right verdict at the right door. Which face's far arm check 4 takes stays that row's question.

**The sentence I would put to you.** In `[topology.tiers]`, in place of "Over each cell it reads the rate … passes by more than the band":

> Over each cell it reads the sine of the wedge between the two faces, never more than one, levered at how far the narrower of them runs out from what they share before its boundary first closes back in on it (a part that does not touch what they share, an edge that left it and has turned back, or the face's own end) ([tolerance.epsilon]), never at a cell of the question's own subdivision, and that passes by more than the band. So a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

- I believe the other designer would sign this. It is their round-2 sentence with the parenthesis added, and the parenthesis closes the gap their report names as open.
- Provenance: designers wrote the sentence it replaces (commit `b130a7bbd5`). You approved that by comment on PR 7456, with a doubt on this lever.

**The one question left for you.** Should the rule above ("every body within ε of this one, with the same topology, is embedded") be ratified as what "passes by more than the band" means? The other designer would state it; I would hold it for the PR body until it is defined and checked against the question's other readings.

**Confidence.**
- Lever, rate and promise: likely.
- Case 2 is in the band whatever the ear: sure.
- The end-of-face clause is needed: sure.
- No valid ordinary body is newly refused: unsure, nothing was run.

## For the orchestrator

**Your summary of my round 2 is right.** This report drops its two-fact structure.

**1. What moved me, and what did not.**
- Moved:
  - The other's point that reading "distinct at the far run" in the question concedes the duplication of check 4.
  - Its point that the far run's promise holds only through an unstated argument across pairs.
  - Its rule, as the derivation.
- Not moved:
  - Its recipe as written (item 2).
  - Putting the rule into ratified text.

**2. One reading — checked in the tree.**
- `injective::extent_of` returns `None` where a face holds no run, and `parts` returns false with no extent. A recipe with no strangers therefore refuses.
- The built code already knows these faces:
  - `BoundaryStations::of_boundary` reads each edge's middle because "a half-disc's two vertices lie on its arc's circle";
  - `injective::spanned` with `geom_brep::centre_line` gives a disc its radius.
- Both must survive as the "face's own end" term.
- Named bodies: a cylinder's end cap, a D-shaped face, a sphere lune, and the thin lens, which must refuse.

**3. Form.** Recommended: the kept near run.
- **What reading settled.**
  - Today a failed lever reading is `Outcome::Split`. It reaches the user as `Uncertified { arm: Departure }` or `Budget` from `clearance.rs`'s `pair`, with no length. Reader 1's `Refused(Budget Depth 40)` is this path.
  - A typed in-band refusal carrying sine × run is therefore new work under either form. The kept length makes it possible at the first cell whose reading is wholly inside the band.
  - The rule form is readable from what the walk holds (`Local`'s edge lists and `Near::of`), so it is feasible. I rank it lower for the reasons in `For Ev`.
- **What tier 3 keeps.** The same `Tier3Facts::edge_runs` and `vertex_runs` slots. The recipe has three terms:
  - each boundary edge sharing no vertex with the shared edge, and each vertex off it: the least distance, from covers, read from below;
  - each neighbouring edge: its distance where it first stops moving away. A straight one never does; its far vertex is the stranger;
  - the face's far end as now (stations and `spanned`), where nothing else closes in.
- Where a term cannot be bounded from below, it must under-read, never be skipped.

**4. Far run in the question.** None. `departure` keeps reading the class for which separators run, the turn check and the `End` refusal. `WedgeReach::reach`, check 4's lever, is untouched. The near-run reading does not depend on check 4's lever being repaired, which the rule form would.

**5. The rule.** I could not check it against the vertex arm (`one_shared_vertex`), `certify_self` or the monotone arm. I did not weigh those arms.

**6. Ownership.** Follow-up row to file: within one solid, the census's `sweep_vertex_face` and `sweep_edge_face` read, for plane faces, the number the question will now read for every kind. That is two doors on one fact.

**7. The sentence.** As in `For Ev`. The remaining difference is the rule's ratification only.

**Not weighed.**
- The near run from a vertex shared alone. I assume the same three terms measured from the vertex; I have no case.
- Whether "turned back" should be read against the shared edge's own span or its carrier for a spline edge.

**Claims the measurement lane could overturn.**
- That ordinary bodies are not newly refused. The ones to look at:
  - short chamfer edges beside a moderate wedge;
  - fillet ends;
  - hexagon-like faces near the band;
  - faces whose neighbouring arcs turn back early, such as a D-shaped face with a shallow arc. These read the arc's bulge, which is the same as their far run today.
- That a finished body exists which the far run certifies whole with an in-band far boundary. The recommendation does not rest on it.

**Still unread.** PR 7456's thread (`gh` unavailable) and `sign_on`'s body. Nothing was run.
