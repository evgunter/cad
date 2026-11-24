## For Ev

**My recommendation changed in form, not in substance.** I still hold that the question must answer for the two faces' own boundaries beside what they share. I now state that as a rule about what a certified cell may hold, and keep the far run for the part the other designer is right about.

**The fork for you.** When two faces share an edge and a cell holding that edge is certified, does the certificate also cover the parts of either face's boundary in that cell that do not meet the edge (a strip's far edge, a hole's rim, an L's inner edge)?
- **The other designer: no.** The pair promises only "neither face lies wholly within the band (K·ε) of the other". A far boundary lying in the band of the neighbour is left to whichever other pair holds it.
- **Me: yes.** The promise is: a certified solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

**Terms.**
- *Far run*: the farthest any point of a face stands from the shared edge's line or circle. This is what was built.
- *Near run*: how far the face runs out from the shared edge before a part of its boundary that does not touch that edge cuts in. It equals the far run where nothing cuts in.

**Where the two designers agree (sure).**
- The rate is the sine of the wedge between the faces, never more than one, in place of the two rates summed. The sum reads the chord between the faces' far sides and certifies a strip narrower than the band.
- The 10 m sliver is certified.
- The 0.6·K·ε strip at 2.6 rad refuses.
- The lesser of the two faces' runs is the lever.

**What moved me.**
- "Neither face lies wholly within the band of the other" is a real fact of the pair, and the far run is its lever. It is `[topology.census]`'s own number for two faces sharing an edge ("largest distance over the shorter cell"). My first report gave the far run to tier 3 alone; that was too narrow.
- A far edge in the band of the neighbour is also another pair's business: the face across that edge. Where that face shares nothing with the neighbour, the question reads the two as a plain distance and refuses.

**What did not move me (checked in the tree).**
- **A certified cell is discharged whole.** In `clearance.rs`, the sweep's `pair` discharges on `Outcome::Certified` and returns; children are made only on a split. So once a cell holding the shared edge passes, nothing in that pair reads the strip's far edge against the plate again (sure).
- **The "other pair" is guaranteed only when it shares nothing with the neighbour.** If the third face shares any edge or vertex with the neighbour, its own cell holding that shared cell and the near-touch is subject to the same whole-cell discharge at its own far run (likely; read, not built).
- **The census does not cover curved faces.** Its vertex-against-face pass (`sweep_vertex_face`, `pair_vertex_face`) reads plane faces only. Its module docs list same-solid curved pairs as "genuinely undetected" (sure).
- **"Width across the edge is zero at every edge's ends" does not reach the near run.** The near run is one length per face and edge, not a pointwise width. Edges through the shared edge's end vertices meet it and are not counted, so nothing is cut out by distance (sure).
- The sentence just before the disputed one says pieces that do not meet are more than K·ε apart. A far-run certificate discharges a cell that holds such pieces 5e-12 apart, and reads them nowhere in that pair.

**The question beneath both reports.** Both argued over a length. The open matter is what a certificate over a cell discharges. The census answers for every pair of cells, including edge against face and vertex against face. The question answers per pair of faces and discharges whole cells. So there are two facts beside a shared edge:
1. *The faces are distinct there.* Capped sine × the narrower face's far run passes the band. This is the other designer's promise.
2. *Nothing else of either boundary lies in the band there.* Every part of either face's boundary in the cell that does not meet what they share stands more than the band from the other face. No reading in the built code decides this.

Fact 2 does not replace fact 1: a lens between two shared edges has no boundary that cuts in, and only fact 1 refuses it. The near run is one kept number that gives both at once.

**Ratified text I would change.** `[topology.tiers]`, in place of "Over each cell it reads the rate … passes by more than the band":

> Over each cell beside what two faces share it reads two things. The faces are distinct: the sine of the wedge between them, never more than one, levered at how far the narrower of them runs from what they share ([tolerance.epsilon]), never at a cell of the question's own subdivision, passes by more than the band. And nothing else of either lies in the band there: each part of either face's boundary that does not meet what they share stands more than the band from the other face. So a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there.

Provenance is as before: designers wrote the sentence (commit `b130a7bbd5`), and you approved it by comment with a stated doubt on the lever.

**The two final states.**
- **Mine (both facts).**
  - Makes true the promise above, in the pair that certifies.
  - Leaves possible a refusal where the near run under-reads, such as a narrow slot at the edge's end under a very shallow wedge. That is the safe direction.
  - One near-touch can be reported by two pairs.
  - Reversible: one recipe behind one kept number.
- **The other designer's (fact 1 only).**
  - Makes true "the faces are distinct", with one number per pair and nothing new kept.
  - Leaves possible a certified solid with a face's edge in the band of a neighbour, wherever no other pair or census pass happens to refuse it. For curved faces no path guarantees that.
  - Whether any finished body gets through is unmeasured (unsure).

**Cases under my answer** (ε = 1e-9, K = 10).

| case | reads | verdict |
|---|---|---|
| 1 flat strip | 5e-5 × 1e-7 = 5e-12 | refuses, as today |
| 2 eared strip | fact 1 passes at 1 m; fact 2: far edge 5e-12 from the plate | **refuses** (today certified) |
| 3 sliver, 10 m | near run 9.6 m both faces, about 480ε | certified, as today |
| 3 sliver, 1 m | about 50ε | certified, as today |
| 3 sliver, 20ε at far end | about 2 bands | should certify; today's budget refusal is the walk's |
| 4 strip 0.6·K·ε at 2.6 rad | sine capped: 0.6 band | **refuses** (today certified) |
| 4 strip 0.8·K·ε at a right angle | 0.8 band | **refuses** under every separator |
| 5 L, edge along the thin arm | arm's width × sine | by that |
| 5 hole beside the edge | distance to the rim × sine | by that |
| 5 line touched again far away | the far touch is not read | n/a |
| 6 spline edge | fact 1 from the edge continued past its ends (the other designer's point, which I accept); fact 2 from the edge's own arc | unmeasured |

The 20 mm in-band stretch of case 3 holds only the two faces and the side edges, which meet the tip edge at its ends, so it passes fact 2.

**What tier 3 keeps.** The same slots as now. Either the near run alone, or the far run with fact 2 read by distance in the walk.

**Check 4.** Fact 1 is tier 3's wedge fact. If check 4 read it at the lesser far run with the capped sine, the question would not need to decide it again. That is the other program's row.

**Confidence.**
- The question must own fact 2: likely.
- Whole-cell discharge and the census's plane-only pass: sure.
- No valid ordinary body is newly refused: unsure.

**What the measurement lane could overturn.**
- If every finished body with such a near-touch, including curved ones and ones whose third face shares a cell with the neighbour, refuses under the far run alone, fact 2 adds nothing in practice and the other answer suffices.
- One body certified whole settles it the other way.

## For the orchestrator

**Your summary of my first report is right.** One correction: I also said the near run falls back to the far run where nothing cuts in.

**1. What moved me, and what did not.** As in `For Ev`. Moved: the far run is the lever of a real pair fact, and the census precedent `ea68daa237` supports it. Not moved: that the pair may leave its own boundary's near-touch unread.

**2. The other report's three claims.**
- **(a) Answered in part.** It is true where the third face shares nothing with the neighbour (`sh.is_empty()` in `pair`, so the distance arm runs). It is not guaranteed otherwise. "Reads one fact twice" is fair as far as it goes: under my answer two pairs can report one near-touch.
- **(b) Not an answer to the near run.** It answers a pointwise width, which I also reject. The near run has its own weak points, listed under 3.
- **(c) Accepted for fact 1.** But the census has separate numbers for edge against face and vertex against face. The question has no such pairs, so its face pairs must carry them.

**3. Facts checked at `273f765de5`.**
- **Fixture `a_flat_strip_inside_the_band_of_its_plate_refuses` (`injective.rs`): your reading is right.**
  - Plate vertices are (0,0,0), (1,0,0), (1,1,0), (0,1,0).
  - Strip vertices 5 and 6 are at y = w·cos φ, z = w·sin φ.
  - Both lie on +y of the shared edge, with the strip's far edge 5e-12 above the plate's face. The class is `Crease(Concave)`.
- **Discharge.** In `clearance.rs`, `pair`: `Outcome::Certified` runs `discharge(...)` and returns `ControlFlow::Continue`. `walk.split` is reached only on `Outcome::Split`.
  - `Outcome::Certified`'s doc says "the pair's union is injective". That is a sign proof, not a clearance.
  - So nothing else in the same pair reads the far edge against the plate afterwards.
- **Which path reads a far boundary against its neighbour in a finished body.**
  - The question's pair (third face, neighbour), by distance, when they share nothing. `injective.rs` module docs: "pieces of the boundary that share no cell are more than the band apart, or the pair is `Violated`".
  - The census's pass 3 for plane faces (`pair_vertex_face`: `geom::plane_offset(f.origin, f.normal, q)`, `f.plane()`), and pass 4 edge × face.
  - For curved faces there is `sweep_edge_in_curved_face` (a line edge in a cylinder or cone face) and nothing general.
  - I did not read passes 4 and 4′ in full.
- **The shared edge's ends.**
  - Both levers are one number per face and edge, read through `extent_of`, and neither goes to zero at the ends.
  - The near run counts the far vertices of the two adjacent edges. A short adjacent edge such as a small chamfer shortens it; I hold that is right, because that vertex is in the band of the neighbour.
  - The case my stand-in misses is an adjacent curved edge that leaves the end vertex and dips back beside the shared edge. The far run misses it too. Only fact 2 read by distance per cell catches it, in a cell that does not hold the end vertex.
  - That makes the per-cell distance reading the general form and the kept near run its cheap, incomplete summary.

**4. The question beneath.** As in `For Ev`: what a departure certificate over a cell discharges, and that the question has no edge-against-face pairs. Either implementation of fact 2 serves:
- lever fact 1 at the near run, or
- keep the far run and bar whole-cell discharge of boundary parts that do not meet the shared cell, reading them by distance.

**Claims the measurement lane could overturn.**
- That a third-face pair sharing a cell with the neighbour is discharged whole over the near-touch. I reasoned this from `pair` and from `sign_on`'s doc (a side within the band's zero of the carrier reads as on it). I built nothing.
- That no valid body in the suite or tour is newly refused at the near run. Hexagon-like faces and notched shallow wedges are the ones to look at.
- That a finished body with an in-band far boundary exists at all without a sub-ε face or edge. My own attempt, an undercut slot under a step, refuses at the strip's own pair when the strip has no ear, and went non-manifold when I added one.

**Still unread.** PR 7456's thread (`gh` unavailable), and `sign_on`'s body.

**Off the question.** The fixture's doc comment ("the right verdict at the wrong door") and `parts`' "square over eight" need rewording under either answer.
