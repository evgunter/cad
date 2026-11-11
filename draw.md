# Design fork `shared-vertex-pairing` — draw

Dispatched 2026-11-11 by the GAUGE orchestrator, on `work/gauge/shared-vertex-crossings-that-tie-or-interleave-are-unprobed.md` (P1, M; the lane's `## Read`, PR 6828). Protocol `2126b860b63`.

- Arm byte **75** (< 86): three designers.
- Label byte **206** (not 255; 206 mod 3 = 2): **Fable is C**; the two Opus designers are **A** and **B**.

## First reports (2026-11-11)

Each designer saw only the problem statement (`problem.md`).
- **A:** pair by the pose, consecutively along the shared (pairing) solid's In arcs, with A as the pairing solid elsewhere; match the finish's seams by pair record, per corner. Typed refusal rejected for both classes. Confidence: likely. Framing rejected: y (the op-chosen start is the fault). Ratified text challenged: n.
- **B:** pair along the shared solid's In arcs, with the sub-choice of everywhere versus shared vertices only left unsure; key the finish's correspondence per corner. Typed refusal rejected. Confidence: likely. Framing rejected: y. Ratified text challenged: n.
- **C:** pair along the lead vertex's inside arcs (the shared vertex, else A's); retire the finish's vertex-set shape check, since the zip's `split_cones` reads the cones. Measured with a probe: 5,190 of 5,190 battery lines and 6 of 6 cleave-witness ops build sound. Confidence: likely. Framing rejected: y. Ratified text challenged: n.

**Agree on the final state:** yes. The pairing is read from the pose (the In arcs of the shared or lead vertex), the op leaves the insertion, the finish no longer refuses by vertex shape, and no typed refusal remains for either class. The one difference is a detail for the spec. A and B re-key the finish's check per corner; C drops it. C's measurement answers the experiment A and B could not run: with the vertex-set check off, the 524 lines build sound. So a per-corner key would only restate a guard. No reconciliation round was run.

Harness token counts (first report = total): A (Opus) 211,859; B (Opus) 255,096; C (Fable) 189,829.

**Outcome:** no ratified text moves, and the designers were unanimous with a measured result, so the GAUGE orchestrator decides (as fork row 202 was). No `[ev]` PR.

## Round 2 (reopened after the build, PR 6832, stopped on two families)
- All three shown round2-evidence.md.
- Token counts for round 2, as the harness reported them: A 320,869; B 363,926; C 242,271.
- Moved:
  - A: per-pair clear side, Out by default, op-free.
  - B: the unshared solid's kept arcs plus a non-crossing placement, which reads the op.
  - C: main's start plus turning plus re-plan on refusal, which reads the op.
- They split. Round 3 dispatched with round3.md and each other's r2 reports.
