# Design fork: a thin face laid on its neighbour's points (TESSLANE, PR 7573)

- Protocol: `docs/DESIGN-FORK-PROTOCOL.md` at `ddd201f411f`.
- Dispatched 2026-11-23 (time by `date -u` in the commit).
- Bytes (`od -An -N2 -tu1 /dev/urandom`): **82, 225**.
  - Arm byte 82 (< 86): THREE designers.
  - Label byte 225: 225 mod 3 = 0, so the Fable designer is **A**; the two Opus designers are **B** and **C**.
- Designers: local background subagents of the orchestrator's session, read-only, on a checkout of PR 7573's frozen head `24ee9a28fb`; one identical problem statement (`tesslane-a-thin-face-laid-on-its-neighbours-points-problem.md`), no candidate solutions.
- Models: A dispatched with the harness's `fable` model (Fable 5.2 per the session's model list), B and C with `opus` (Opus 5.5 per the same list); the task notifications report tokens, tool uses and seconds, not a model id.

## First reports

Recorded as delivered, before any reconciliation (2026-11-23; A at 17:29Z, B at 17:22Z, C at 17:21Z by `date -u`). Reports verbatim beside this file (`-report-A.md`, `-report-B.md`, `-report-C.md`).

| | answer | one line of argument | confidence | rejected the framing? | proposes changing ratified text? | tokens / tool uses / seconds |
|---|---|---|---|---|---|---|
| A | (1) a body thinner than δ is owed distance and a closed manifold, not volume: a flat mesh is a correct answer, and the door that needs a solid (STL export) checks the sign and refuses typed; (2) the split point is the foot on the face's surface of the side's straight midpoint; (3) the unit merges with (2) and corrected wording, the flat cells do not block it, the check belongs to the parked row | no split rule can prevent a flat mesh in general (two faces closer than δ that share no point), distance-only is what the pages say, and the viewer asks for δ = 1e9 on every body on purpose | likely (1), sure/likely (2) | yes: "thinner than δ" has a sharp form (δ × area ≥ volume); the base does not have the property either; the split was never the cure for flatness; the point should not live in the chart | yes: a README sentence that a mesh within δ may enclose no volume; a clarification of `[mesh.winding]` | 218,334 / 36 / 731.6 |
| B | (1) every curved face splits every inside side that joins two of its own boundary points, at its own surface (an alternative trigger, the base's test, if the overhead is large: a measurement); (2) the foot; (3) `tessellate` refuses typed a mesh whose signed volume is not positive above rounding; all three land in the unit and the parked row closes into it | the retired rule did a second job, separation: the defect is a curved patch whose inside sides carry none of its own surface | likely | yes: the retired rule was not only a wrong prediction | yes: add to `[mesh.winding]`; a new clause `[mesh.own-points]`; a sentence in `[mesh.sizing]` | 136,402 / 14 / 297.3 |
| C | (1) widen the census: two faces that each have a triangle whose three corners are points they share both split those triangles' inside sides; (2) the nearest surface point to the 3-D midpoint (spline keeps the chart midpoint); (3) a per-shell sign promise checked at the join, typed refusal naming δ: the parked row's work, decided now | the defect is two patches covering one region with triangles built only from shared points; a side made twice is one form of it | likely | yes: the cap is outside what the census looks for; the base rule was not wrong on this body | yes: extend `[mesh.winding]` from one triangle to whole shells | 128,088 / 13 / 265.6 |

**Agreement on first reports:** all three on the split point (the foot of the straight midpoint, in space and not in the chart) and on a sign read with a typed refusal existing somewhere. **Split** on what `tessellate` owes: A says a flat mesh is a correct answer at such a δ and the check belongs at the door that needs a solid; B and C say `tessellate` keeps the sheets apart by a wider trigger (two different triggers) and itself refuses a mesh without the body's sign. Reconciliation follows.
