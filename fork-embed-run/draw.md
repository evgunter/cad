# Design fork `embed-run` — draw

Dispatched 2026-11-24 by the EMBED orchestrator, on the two reviews of PR 7666 (`embed/verdict-read`) and the row `work/embed/the-embedding-question-reads-tier-3s-class-and-the-pairs-extent-from-the-verdict.md`. Protocol `d327ffb520`.

- Arm byte **181** (>= 86): two designers.
- Label byte **64**: **Opus is A, Fable is B**.

The problem statement every designer was handed is `problem.md`; the two readers' passages it cites are `reviews/reader-1.md` and `reviews/reader-2.md` (verbatim excerpts, attribution stripped).

## Designers (as the harness reports each)

- A: `claude-opus-5-5` (Opus 5.5), agent `a2b64f99cea0fd12e`.
- B: `claude-fable-5-2` (Fable 5.2), agent `afbefa733a8d77d8c`.

## First reports (recorded before any reconciliation)

Both delivered 2026-11-24 (A about 02:30Z, B about 02:45Z); neither saw the other's. Verbatim in `round1/A-r1.md`, `round1/B-r1.md`.

| | A | B |
|---|---|---|
| Answer | Keep the run as built (the farthest the face stands from the shared locus's whole carrier; the lesser of the two). Change the RATE: the sine of the angle between the faces, each face's angle off the separator added as an angle, capped at a right angle. State the promise: "neither face lies wholly within the band of the other". | Lever at each face's NEAR run: how far the face runs out from the shared edge before any part of its boundary that does not touch that edge cuts in, read from below; the lesser of the two. Rate: the wedge's sine, never more than one. Promise: a solid's boundary comes within the band of itself only beside a cell its faces share, between the faces and edges that meet there. |
| One line | The ear, L and hole near-touches belong to other pairs, which read them; a "width across" is zero at every edge's ends; the summed rate is the defect (it depends on the separator and reaches 2). | Two facts, not one lever: whether the angle is real (far run; tier 3's) and whether the in-band strip beside the edge closes before either face ends (near run; nobody's today). The built lever gives the question tier 3's fact a second time. |
| Confidence | likely | likely |
| Rejected the framing | yes (the ear case is not a fault in the run) | yes (the readers hold two facts, not one disputed lever) |
| Proposed changing ratified text | yes (`[topology.tiers]`: the rate, a gloss of the run, the promise) | yes (`[topology.tiers]`: the lever, the rate, the promise) |
| Tokens, first report | 124,245 | 253,477 |

Agreement on the final state: **no** on the lever (far run against near run) and on the promise; **yes** on the rate (the wedge's sine capped at one, not the two rates summed), on the sector sliver (certified), and on the 0.6·K·ε strip at 2.6 rad (refuses).

## Round 2 (2026-11-24, about 02:37Z to 02:45Z)

Dispatched: both designers, by message to the same agents. Shown: each the other's first report in full (a copy at a neutral path, without this file), my reading of where the two stood, the other's three main claims against its own, four points of FACT about the tree to check at `273f765de5` (the fixture's geometry; what a passing edge cell discharges; which pair and path reads a far boundary in a finished body, curved faces included; each lever at the shared edge's ends), and the question whether something lies beneath both positions. Verbatim: `round2/A-r2.md`, `round2/B-r2.md`.

What moved:
- **A moved to the near run** (B's first answer): lever = the near run of the narrower face; rate = the wedge's sine capped at one; B's promise. The far run stays tier 3's lever for whether the wedge is real. Its reasons: the far run has the question decide check 4's fact a second time; a shift of ε moves the edge sideways by ε over the sine, which only the near run bounds; a passing edge cell is discharged whole and nothing else in the pair reads the far edge (checked in the tree). It adds a rule to state ("passes by more than the band" means every body within ε of this one, with the same topology, is embedded) and a second fork (which door owns "pieces that do not meet" for a face's own boundary against a neighbour: the census, plane cells only, or the question; it leans the question).
- **B moved in form**: TWO facts read by the question beside a shared cell. Fact 1, the faces are distinct: capped sine × the narrower face's FAR run passes the band (it accepts A's first promise as a real fact of the pair, and the census precedent). Fact 2, nothing else of either boundary lies in the band there: each part of either face's boundary that does not meet what they share stands more than the band from the other face. The near run is "one kept number that gives both at once", or the far run is kept and whole-cell discharge is barred for boundary parts that do not meet the shared cell, read by distance; it names the per-cell distance reading the general form and the kept near run its cheap, incomplete summary.
- Both checked the same facts and agree on them: the strip is folded over the plate; `clearance.rs`'s `pair` discharges a certified cell whole; a third face that shares nothing with the neighbour is read by distance, on every surface kind; the census's vertex-against-face pass is plane-only.

Position after round 2: agreed on the promise, the rate, the cases' verdicts (1, 2, 4 refuse; 3 certified). Not yet agreed on the reading's form: one lever at the near run (A), or two readings (B: distinctness at the far run, and the boundary parts' clearance), and on whether the wedge's reality is the question's to read at all or tier 3's alone. Positions moved and did not meet: round 3.

Tokens, cumulative as the harness reports: A 158,704; B 293,366.
