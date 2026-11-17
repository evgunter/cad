# Design fork `embed-sliver` — draw

Dispatched 2026-11-17 by the EMBED orchestrator, on `work/embed/the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause.md` (P1, M, design). Protocol `d327ffb52`.

- Arm byte **76** (< 86): three designers.
- Label byte **196** (mod 3 = 1): **Fable is B; the two Opus designers are A and C**.

The problem statement every designer was handed is `problem.md`.

## First reports (2026-11-17)

Each designer saw only the problem statement (`problem.md`).

- **A:** the sliver is not a reading inside the band: it fires only on `Zero`, a width of at most ε. Keep it, and say what it is. `Presentation::Vacuous` means two things today ("no point here" and "this face's part here is the shared edge, to within ε") and splits into `Empty` and `OnShared(line)`. `OnShared` is read only where the strip's far side is the cell's own cut, and the departure arm certifies on it only when the other face's part in the cell is its own sheet at that line (empty, on the same line, or presented with it as a side). `[topology.tiers]` gains one sentence on `Zero`. Confidence: likely. Rejected the framing (the premise). Proposes changing ratified text (one added sentence).
- **B:** retire the sliver, and do not refuse in its place: it is evidence that the departure reading meters "how two faces leave their shared edge" at the width of its own subdivision cell (`sign_on` decides rate × the rectangle's width), so a thin cell can never pass and is waved through unread. Meter the rate at the reach of the faces (a `Lever`, D4's θ = ε/r); a thin rectangle is then read like any other, no `Zero` comes from a cell bound (PR 6821's answer), and `Vacuous` means only that the cell lies off the face. Confidence: likely (that the sliver-held bodies pass under it: unsure, unmeasured). Rejected the framing. Proposes changing ratified text (the clause names the arm of the local reading).
- **C:** the sliver is a `Zero` reading, at most ε, never a reading inside the band, so it is consistent with the clause under a reading the clause never states. Name that reading in DESIGN (a face's part within ε of a shared edge or vertex is read as that edge or vertex), give it its own `Presentation` variant distinct from `Vacuous`, built only from an already proven presentation, and correct the module docs. Refusing the sliver is worse. Confidence: likely. Rejected the framing (the premise). Proposes changing ratified text (one sentence, and "the band" made precise).

All three correct the brief and the row: the sliver's threshold is ε (`Ok(Sign::Zero)`), not K·ε. Checked by the orchestrator against `present_chart` and `Interval::sign_within`: true. All three reject refusing at the sliver. A and C keep the reading and type it (A adds two conditions on when it certifies); B retires it by changing what the departure arm meters. A reconciliation round is owed.

Harness token counts (first report):
- A (Opus): 131,692
- B (Fable): 271,038
- C (Opus): 133,731

## Round 2 (2026-11-17)

Each designer was shown the other two round-1 reports. A and C moved to B's answer; B held it.
- **A** moved off keep-and-type. What moved it: PR 6821's ruling, which B cited ("the engine never establishes an equality from a cell bound"), names `self_intersection_on_shared`, the site A's analogy rested on, as a cell Zero to remove; and B's reading of `sign_on` (the departure margin is the rate times the presentation's own span, and a sweep cell is not a lever the kernel admits). A now recommends: retire the sliver and re-lever the departure reading at the faces' reach. Its two holes (a rectangle collapsed by a wrong clip; a foreign sheet touching the shared edge) close under it without rules of their own. C over C′ (the rate's sign read exactly, no band): unsure, "the one choice that is genuinely yours".
- **C** moved off keep-and-type for the same two reasons, and because A's foreign-sheet case refutes its own round-1 sketch that a dropped defect still surfaces through a distance pair. It now recommends the same: retire, meter the rate at the faces' reach. It frames C against C′ as a question of who owns "how the faces leave their shared edge" over the whole edge, tier 3 or the engine. C over C′: unsure.
- **B** held: retire, and meter the rate at the body cell's own extent. It took A's partner rule and C's constructor as the best form of keeping, and argues that form likely refuses `chute` (the cap is not one of the admitted partners there). `[topology.census]` moved it from unsure to likely on C over C′: the clause names the number and its arm ("its length times the sine of the angle between them", the cell's own extent as the arm).

All three now recommend one final state, with C′ as a stated alternative each leans against. All three name the same unmeasured premise: that the bodies the sliver holds today certify once the departure margin is not scaled by the cell. B says which result would move it back to keeping (the rate's enclosure straddling zero on the thin rectangle). The measurement lane has that run; a closing round follows its numbers.

Totals after round 2: A 151,717; B 315,428; C 153,732.

## Round 3 (2026-11-17), closing

Each designer was shown the other two round-2 reports and the measurement (`work/embed/the-sliver-concession-…`, `## Measured`, sections 1 to 8). All three hold the round-2 recommendation: retire the sliver, with no refusal in its place, and meter the departure reading at the faces' reach.
- **What the numbers did.** All 96 `Zero` decisions in a sweep are at rounding scale, on a cell across the line from its face, with the partner unpresented: A's round-1 partner rule would have refused all 96, and A says so. With the sliver refused and the margin un-scaled, `chute`, the stepped ring and `tiltedcut_above` hold.
- **The drum at Interval** still refuses on a straddling rate. B had said in round 2 that this result would send it back to keeping the sliver; B does not move, and says why (keep-and-type has nothing to keep; the straddle is on both faces under the one separator that works at a tangent seam). C withdraws a like clause. All three call the drum a missing engine reading and give three causes: **A** its line placed at Interval as an enclosure where construction fixes it; **B** the graph separator's direction read at the rectangle's middle (`heading`); **C** no separator crosses a tangent seam. A trace was dispatched to tell them apart.
- **Choices they leave to Ev.** A band on the re-metered rate against its exact sign: all three lean the band (A and C unsure, B likely); only the exact sign was measured. Retiring the last reader's sliver with the drum's fix against now: A and C lean together; B sequences it the same way. B adds a fallback: one named concession (the touch, in metres at ε) for a cell across the line, if the drum's fix does not close it.

Agreed. The `[ev]` PR is opened from `embed/ev-sliver`; `docs/DESIGN-FORK-LOG.md` row 285 carries the recommendation half.

Totals: A 174,824; B 375,792; C 179,915.
