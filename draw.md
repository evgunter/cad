# Design fork `clear-enclosure` — draw

Dispatched 2026-11-11 by the CLEAR orchestrator, on `work/clear/clearance-cell-enclosures-are-typed-as-the-interval-scalar.md` (P0, M). Protocol `2126b860b63`.

- Arm byte **154** (≥ 86): two designers.
- Label byte **20** (even): **Opus is A**, **Fable is B**.

## First reports (2026-11-11)

Each designer saw only the problem statement (`problem.md`).

- **A:** keep `Interval` as the engine's arithmetic, with no new type. An f64 enters as `[x, x]`, exact in meaning. The faults are elsewhere: (a) four Zero places re-decide facts tier 3 has already certified, and (b) a refusal does not say whether the family's width or the cell's blocked it. Changes: one crossing owned by the lane, tier 3's verdicts read at the Zero places, and each refusal names its lever. Confidence: likely. Did not reject the framing. Proposes an optional clarification of `[topology.tiers]`.
- **B:** keep one interval arithmetic, and an f64 enters as itself. But give the cell bound its own type, `CellBound`, which wraps `Interval`, has no `Decide`, and never writes the driver's log. The fault is that a cell reading's undecided arm is addressed to the drive (F28–F30's hand-kept `unlogged` scopes; `cavity_embedded` misses one). Point readings of a finding are made at the body's own `T` and logged. Confidence: likely. Rejected the row's framing of the fault. Proposes rewording `[topology.tiers]` ("instead").

They agree on: f64 entering as itself; the band is not in the crossing; one crossing door, with the shell's `lift` parameter removed; `Sym` fits but is impractical in the engine; and the same `[topology.tiers]` wording. They disagree on whether the cell bound gets its own type, and on which fault comes first. A reconciliation round is owed.

Harness token counts (first report):
- A (Opus): 213,136
- B (Fable): 204,182

## Round 2 (2026-11-11)

Each designer was shown the other's round-1 report. The positions crossed:
- **A** moved to B's `CellBound` type, for F28–F30 and the shell's missing scope. A added a rule: a cell bound never establishes an equality, which comes instead from topology, tier 3 or a point reading. A adopted B's point readings at `T`, and dropped its own "every refusal names its lever".
- **B** dropped the type, because `Vec3<T: Real>` forces a full `Real` by delegation, the shape PR 2701 retired. B took A's changes 1–3, and added change 4: the engine's door owns the unlogged disposition of its cell readings.

The positions moved but did not meet. Round 3 asks the question beneath them: is the log split held by a type or by a door, and does a delegating `Real` without `Decide` reopen PR 2701's ruling?

Totals after round 2: A 229,240; B 218,397.
