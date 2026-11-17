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
