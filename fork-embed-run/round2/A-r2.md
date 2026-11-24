## For Ev

**My recommendation changed: lever at the near run, not the far run as built.** The rate change stands, and the two designers now agree on both.

**Definitions used below.**
- The **far run** (what was built) is the farthest any point of a face stands from the carrier of the edge it shares.
- The **near run** is how far the face extends from the shared edge before any part of its own boundary that does not touch that edge cuts in.
- The **band** is K·ε. The **pair** is the two faces that share the edge.

**Recommendation (likely).**
- **Lever:** the near run of the narrower face. For each face, the run is read as the least distance from the shared edge's own span to each of its boundary edges that shares no vertex with it, and to each of its vertices that is not on the edge. The pair takes the lesser of its two faces' runs.
- **Rate:** the sine of the wedge, composed from the two faces' angles off the separator (sin(α+β)), and never more than one. It replaces "the faces' rates summed".
- **Promise:** a certified solid comes within the band of itself only beside an edge or vertex its faces share, and only between the faces and edges that meet there. Each pair keeps this promise for its own boundary.
- **Where the far run belongs:** it stays the lever for whether the wedge is a real angle. That is tier 3's crease class, which the verdict already keeps.

**Why I moved.** Each point below is the other designer's argument, checked against the tree.
- **The far run makes the question decide tier 3's fact a second time (sure).** With the far run as lever, the question's band test at a crease asks whether the wedge is a real angle at the narrower face's far arm. That is check 4's question. My first report even asked check 4 to read the same number, which concedes the duplication. The ratified sentence says the question "decides none of it". With the near run, the question decides a different fact that no other door decides for every surface kind: whether the in-band strip beside the edge is empty of other boundary.
- **Geometry ε off moves the edge sideways, and an ear does nothing about that (sure).** If a face's points are ε off, the line where the two faces meet moves sideways by ε/sine, which is 2e-5 m in the flat-strip fixture, 200 strip widths. The far run only bounds the tilt (ε divided by the arm). The condition that survives a shift of ε is: near run × sine > band. That is the near run's reading. This is the PR 7095 question, whether the margin tolerates geometry being ε off.
- **The far run certifies a pair in which an edge of one face sits 5e-12 m from the other without meeting it (sure; checked in the tree).** When the cell holding the shared edge passes, that cell's whole rectangle is discharged, and the strip's far edge lies inside it. Nothing else in the same pair reads that edge again. The sentence just before ("pieces that do not meet are more than K·ε apart") is then not kept by the pair.
- **What did not move me: in a finished body, another pair usually catches it (sure for the path, unsure for every body).** The strip's far edge also belongs to a closing face. The question asks every face pair of a solid, and it reads by distance any cell pair that shares nothing, on every surface kind. If the closing face shares nothing with the plate, that pair refuses. But if the closing face shares a vertex or edge with the plate near the far edge, its own pair is another departure reading with the same far-run escape. So the far run's promise holds only through an unstated, recursive argument across pairs. The near run makes each pair hold it.

**What each case reads under the recommendation** (ε = 1e-9, K = 10)

| case | verdict | why |
|---|---|---|
| 1. Flat strip, 1e-7 wide, sine 5e-5 | refuses | near run 1e-7, reading 5e-12 (sure) |
| 2. The same strip with an ear | **refuses** (certified today) | the ear lies past the far edge, which cuts in first, so the reading is unchanged (sure) |
| 3. Sector sliver, 10 m and 1 m | certified | both near runs equal the face length; the side edges touch the tip edge at its ends; `[topology.census]` already says the in-band neighbourhood of what is shared "is not a coincidence" (likely→sure) |
| 3. The sliver 20ε apart at its far end | should certify (about 2 bands) | today's refusal is the walk running out of budget, not the reading (likely) |
| 4. Strip 0.6·K·ε wide at 2.6 rad, and 0.8·K·ε at a right angle | **refuse** under every separator | sine capped at one, times the width (sure) |
| 5. L-shape, hole beside the edge | read at the arm's width, or at the distance to the hole's rim | that boundary cuts in first (likely) |
| 5. A face that touches the edge's line again far away | lever unchanged | that cell holds no shared zero, so it is read by distance (likely) |
| 6. Spline edge | measured from the edge's own span | a least distance never counts travel along the edge, so today's over-read cannot certify (likely) |
| 7. Check 4 | keeps a far-run lever for its own fact | which face's far arm it takes stays that row's question; the question refusing a body check 4 passed is then the right door |

**At the shared edge's ends.**
- The far run ignores the ends, since it is a farthest distance.
- The near run excludes, by key, the boundary edges that touch the shared edge (the next edges at each end). Their own meeting with the other face is a wedge at a shared vertex, read at that vertex or by the census. The vertices at their far ends do count.
- A rectangle reads its width, and a triangle its apex.
- One case is not covered: a single curved neighbouring edge that leaves the end and dips back beside the edge has no vertex to catch it (unsure).

**The question underneath, which neither first report asked.** One rule should be stated, and the lever derived from it rather than chosen:

> "Passes by more than the band" means every body within ε of this one, with the same topology, is embedded.

Under that rule:
- the tilt uncertainty (ε over the far run) is tier 3's class;
- the sideways uncertainty (ε over the sine), compared against the near run, is the question's.

A second fork follows from it: which door owns "pieces that do not meet are more than K·ε apart" for a face's own boundary edges against a neighbour face.
- The census reads exactly this number per cell pair, but only for planar cells. Its module docs list same-solid curved pairs as undetected.
- The near run makes the question read it for every surface kind, so for planar bodies two doors decide it.

I lean to the question owning it (unsure). That is worth a line from you.

**Ratified text I would change.** In `[topology.tiers]`, replace "Over each cell it reads the rate … passes by more than the band" with:

> Over each cell it reads the sine of the wedge between the two faces, never more than one, levered at how far the narrower of them runs out from what they share before any part of its boundary that does not touch it cuts in ([tolerance.epsilon]), never at a cell of the question's own subdivision, and that passes by more than the band; so a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

**Provenance.** The sentence was written by designers (commit `b130a7bbd5`, fork-log row 304). You approved it in passing on PR 7456, with a doubt about exactly this lever. "Measured across it" admits both runs, which is why the readers split.

**Alternatives.**
- **Far run with the honest rate** (my first answer): one number that matches the census's whole-face number. But it duplicates check 4, and its promise is held only across pairs.
- **Sign only beside a crease:** the same promise, but every narrow face is walked down to its width and refuses with no length to report.
- Both are reversible. The near run changes one recipe behind the number tier 3 already keeps.

**Confidence.**
- Recommendation: likely.
- Case 2 lies within the band whatever the ear: sure.
- The sum over-reads at wide wedges: sure.
- No valid ordinary body is newly refused: unsure, nothing was run.

## For the orchestrator

**Facts checked at `273f765de5`, read-only.**
1. **The fixture.** `injective.rs`, `a_flat_strip_inside_the_band_of_its_plate_refuses`. Plate vertices: (0,0,0), (1,0,0), (1,1,0), (0,1,0). Strip vertices: (1, w·cos φ, w·sin φ) and (0, w·cos φ, w·sin φ). Both lie on the +y side of the edge, so the strip is folded over the plate, its far edge 5e-12 above it, classed `Crease(Concave)`. Your reading is right.
2. **What a passing edge cell discharges.** `certify_pair_in` → `departure` → `parts`. The strip's cell is `(0.4,0.6)×(0,width)`, which holds the far edge. A pass discharges the whole cell pair. Nothing else in the same pair reads the far edge after that: no edge-against-face arm runs inside a touching pair. The walk splits only on a failure.
3. **Who reads a far boundary in a finished body.** `clearance.rs`, `Sweep::run` doc: in the same-body injectivity walk, "a cell pair is decided by distance only where it does not touch". That covers every face pair of the solid and every carrier kind the engine admits. So the closing face against the plate, if the two share nothing, refuses by distance, curved faces included. The census path is not needed for that. `census.rs`'s module docs do list "the other SAME-solid distinct-key curved pairs" as undetected, but that gap matters only where the face holding the far boundary shares a cell with the neighbour.
4. **Ends:** as stated in For Ev.

**Claims the measurement lane could overturn.**
- If it finds no finished body, of any kind, that the far run certifies with an in-band far boundary, the far run is harmless in practice. My recommendation rests on the duplication argument and the ε-shift argument, not on that count.
- If the near run refuses an ordinary valid body (a short chamfer edge at a moderate wedge, a fillet's end), the "no new refusals" claim falls. Planar cases of that kind should already be refused by the census edge-against-face pass, which reads the same number; check that.
- If it builds a curved neighbouring edge that dips back beside the shared edge, that is the near run's open gap.

**For the implementer.**
- Three stations per edge bound a least distance from above, which is the unsafe side. The near run needs each stranger edge's whole cover read from below.
- The `parts` doc comment ("square over eight", "no choice of separator moves") is wrong under either answer.
- The row's spec item 5, "right verdict at the wrong door", needs rewording.
- The unit test `the_pairs_sines_are_summed_and_each_side_is_its_enclosures_sign` pins the sum and will need re-pinning.

**Not done.** I did not run anything, and the gap for a curved neighbouring edge that dips back is unresolved.
