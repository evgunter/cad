## For Ev

**My recommendation changed in one clause: I sign the other designer's sentence and withdraw "an edge that left it and has turned back".** The two designers now recommend the same lever, rate, promise, form and sentence. No difference is left for you to rule on. One uncovered shape is recorded below.

**Terms.**
- *Band*: K·ε.
- *Pair*: two faces that share an edge or vertex.
- *Far run* (what is built today): the farthest any point of a face stands from the line or circle of the edge it shares.
- *Near run*: how far the face runs out from the shared edge before a part of its boundary that does not meet that edge cuts in. "Does not meet" is judged by key: an edge that shares a vertex with the shared edge meets it; a far edge, a hole's rim or an L's inner edge does not.
- *The lever*: the near run, and never more than the far run. The pair takes the lesser of its two faces'.

**Recommendation (likely).**
- **One reading** beside a shared edge: the sine of the wedge between the two faces, never more than one, times the lever above. It replaces "the faces' rates summed" at the far run.
- **Promise:** a certified solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there. Each pair keeps it for its own boundaries.
- **Form:** a length tier 3 keeps, in the slots it has now. It is not a rule inside the walk.
- **The far run alone** stays tier 3's lever for whether the wedge is a real angle. The question does not read that fact separately; a pass at the lever implies it.

**Why the lever is the near run.**
- A face whose points are ε off moves the shared edge sideways by ε over the sine. In the flat strip that is 2e-5 m, 200 strip widths. An ear on the strip pins its tilt and does nothing about this (sure). This is your PR 7095 question, whether the margin tolerates geometry being ε off.
- A cell that passes is discharged whole (`clearance.rs`, `pair`). At the far run, a strip's far edge 5e-12 m from the plate is certified inside that cell and read nowhere else in the pair (sure).
- The sentence before the disputed one says pieces that do not meet are more than K·ε apart. The near run makes each pair keep that for its own boundary, on every surface kind.

**Why "never more than the far run".**
- Some ordinary faces have nothing that cuts in: a cylinder's end cap on its rim, a half-disc face, a sphere face between two meridians. Their lever is their own far end, as today.
- A thin lens between two shared edges also has nothing that cuts in, and refuses at its far end (sure).
- The other designer adds a point I accept: a boundary vertex past the shared edge's end can stand farther from the edge than the face's real reach. The cap stops the near run reading long.

**The clause I withdraw, and why.** Round 3's sentence also counted "an edge that left it and has turned back".
- For every body I named for it, the cap already gives the same reading. On a half-disc, a lens, or a disc with a flat, the neighbouring edge turns back exactly at the face's far end.
- The clause is a third term only for a neighbouring edge that leaves the shared edge's end, swings away, and comes back beside the edge without ending. That shape is a *pinch*.
- The term is well defined: the least a neighbouring edge stands from the shared edge's own span, anywhere it is not moving away from it or closing onto its far end. For a line neighbour it never fires. For an arc or spline it fires at a turn.
- I cannot show a recipe that reads it from below without under-reading ordinary corners. A spline neighbour's first piece at the vertex may not prove it is moving away, and it would then read near zero (unsure). A lever that refuses rounded corners is worse than a gap in an unbuilt shape.
- For a pinch under a steep wedge, the other designer's route catches it: the two walls through the two edges share only their corner edge, and stand in the band of each other far from it. That is read by distance and refuses (likely).
- Under a shallow wedge three faces pinch at once, and no pair reads it under either sentence (unsure; nobody has built one).

**The one uncovered shape.** A pinch under a shallow wedge: three edges leave one vertex and come back within the band of one another far from it. Neither designer's lever reads it. It needs a row and a built body. It is not a reason to hold the sentence.

**Cases** (ε = 1e-9, K = 10).

| case | reading | verdict |
|---|---|---|
| 1 flat strip | 5e-5 × 1e-7 = 5e-12 | refuses, as today |
| 2 strip with an ear | the far edge cuts in at 1e-7; the same 5e-12 | **refuses** (certified today) |
| 3 sector sliver, 10 m and 1 m | about 480ε and 50ε | certified, as today |
| 3 sliver, 20ε at its far end | about 2 bands | should certify; today's budget refusal is the walk's |
| 4 strips 0.6·K·ε at 2.6 rad and 0.8·K·ε at a right angle | 0.6 and 0.8 of the band | **refuse** under every separator (certified today) |
| 5 L-shaped face, hole beside the edge | the arm's width, or the distance to the rim, × sine | by that |
| 5 face touching the edge's line again far away | not read by the lever | n/a |
| 6 spline shared edge | near run from the edge's own span; cap from the edge continued past its ends | unmeasured |
| end cap, half-disc, lens | the face's far end | cap and half-disc certify as today; a lens in the band refuses |

- The 20 mm in-band stretch of case 3 holds only the two faces and the side edges, which meet the tip edge at its ends. `[topology.census]` already says that neighbourhood "is not a coincidence".
- Case 7: check 4 keeps its own far-run lever for its own fact. The question refusing a body tier 3 passed is the right verdict at the right door. Which face's far arm check 4 takes stays the other program's row.

**What tier 3 keeps.** The same slots: one length per face and edge, and per vertex shared alone. The recipe changes to the least distance to each boundary edge and vertex that does not meet the shared edge, read from below, and never more than today's far run.

**What a refusal tells its reader.** This face runs only *d* from the shared edge before its boundary cuts in, which at this wedge is inside the band. Open the wedge or move that boundary away.

**The sentence.** In `[topology.tiers]`, replace "Over each cell it reads the rate … passes by more than the band" with the other designer's sentence, which I sign as written:

> Over each cell beside what two faces share it reads the sine of the wedge between them, never more than one, levered at how far the narrower of them runs out from what they share before any part of its boundary that does not meet what they share cuts in, and never farther than it stands from that at all ([tolerance.epsilon]), never at a cell of the question's own subdivision, and that passes by more than the band. So the two faces are distinct there, and a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

- Provenance: designers wrote the sentence it replaces (commit `b130a7bbd5`). You approved that by comment on PR 7456, with a doubt on exactly this lever.
- Until the pinch is read, the last words promise slightly more than the lever delivers. I would leave them as the intent and let the row carry the gap.

**Also agreed, and not for you to rule on.**
- The rule both readings derive from ("every body that moves by less than the band, keeping its topology, stays embedded") goes in the PR body as the argument, not in ratified text.
- The question owns "pieces that do not meet" for a face's own boundary beside a shared cell. The census's plane-only passes then read the same number within one solid; that is a follow-up row.

**Confidence.**
- Lever, rate, promise, form: likely.
- Case 2 is in the band whatever the ear: sure.
- The cap is needed: sure.
- No valid ordinary body is newly refused: unsure, nothing was run.
- The shallow pinch is uncovered: likely.

## For the orchestrator

**1. Do I sign the other designer's sentence?** Yes, as written.
- The words of mine that go: "an edge that left it and has turned back". "The face's own end" is their "never farther than it stands from that at all".
- Bodies that read the same under both sentences: half-disc, lens, end cap, disc with a flat.
- The only body that reads differently is the pinch: refused at this pair under my round-3 sentence, certified at this pair under theirs.

**2. The turned-back term.**
- **Is it well defined?** Yes, if measured from the shared edge's own span, not its carrier.
  - Measured from the carrier, a neighbouring edge that continues the shared edge's line (a T-junction split) reads zero and every such face refuses.
  - From the span, a line neighbour's distance is strictly increasing. So is a tangent arc leaving past the end: the distance is from the end vertex.
  - An arc or spline can turn. On a curved face the same definition holds in space.
- **Is it the cap, or a third term?** A third term. It coincides with the far-run cap wherever the first turn is the face's far end, which is every convex case. It differs only at an interior return, the pinch.
- **Does it under-read anything ordinary?** By the definition, no.
  - A D-shaped face reads its bulge.
  - A fillet's end arcs and long lines are monotone.
  - A chamfer's lines are monotone.
  - The sector sliver's side edges are monotone.
- **The risk is in the recipe.** The first cover piece at the vertex must prove the neighbour is moving away. At `Interval`, or for a spline, that enclosure can hold zero, and the safe reading is then near zero. I did not test this; it is why I withdraw the clause.
- **The other's sector-sliver point** (a per-cell judgement would refuse it) applies to the rule-in-the-walk form, which both of us already rank second. I did not run it.
- **Checked:** "the injectivity question ends at its first FINDING" is in `clearance.rs`'s module docs. My round-2 cost "one near-touch reported by two pairs" is void.

**3. Does a difference remain?** None for Ev. Recommended follow-up rows; I filed nothing:
- **The shallow pinch** (`work/embed/`, design: true). Two bodies for the measurement lane:
  - a profile whose spline side leaves a corner and returns within 5ε of the adjacent straight side, extruded. Expect a refusal by distance between the two side walls.
  - the same return under a wedge of sine about 5e-5. Expect it to certify under either sentence.
- **Census duplication:** `sweep_vertex_face` and `sweep_edge_face` duplicate the question's number within one solid for plane faces.
- **A typed in-band refusal** carrying sine × lever. Today a failed lever reading is `Outcome::Split` and surfaces as `Uncertified` or `Budget` with no length.

**For the implementer.**
- Lever = min(near, far).
- "Meets" is by key.
- Stranger edges are read from below over their whole cover; three stations bound from the unsafe side.
- Keep `BoundaryStations` and `injective::spanned` as the cap.
- Vertex runs take the same recipe from the vertex. I did not weigh that separately.
- Re-pin: `the_pairs_sines_are_summed_and_each_side_is_its_enclosures_sign`, the eared-strip probe, and the wide-wedge probes.
- Reword: `parts`' doc, the fixture doc's "right verdict at the wrong door", `rest_facts.rs`'s module doc, and the unit row's spec item 5.

**What the measurement lane could overturn.**
- A valid ordinary body refused at the near run. Candidates: a short chamfer beside a moderate wedge, a hexagon-like face near the band, a notched shallow wedge.
- A body where the near run passes with a stranger part in the band, which would show the lower bound failing on curved faces.
- The shallow pinch certifying whole. That would turn the recorded gap into a defect with a body.

**Still unread.** PR 7456's thread (`gh` unavailable) and `sign_on`'s body. Nothing was run.
