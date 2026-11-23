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


## Reconciliation

Each round: every designer shown the others' latest replies; verbatim replies beside this file (`-round1-*.md`, `-round2-*.md`, `-round3-*.md`). Token counts are the harness's cumulative figure for the subagent at the end of each reply.

**Round 1** (2026-11-23, 17:30Z to 17:32Z).
- B moved on two of three: drops its every-inside-side rule for C's pair trigger; moves the sign refusal out of `tessellate` to the export door, per shell (A's point: the viewer tessellates every body at δ = 1e9). 149,977.
- C moved on one: concedes the sign refusal inside `tessellate`, puts it at the export door; keeps the pair trigger. 142,298.
- A: core unchanged (distance and a closed manifold; flat is `Ok`; the sign at the door; the foot); withdraws its own alternative trigger for C's, if the owner wants thin bodies kept open; "no separation rule in the unit" down from likely to unsure. 236,398.

**Round 2** (17:32Z to 17:35Z). The orchestrator stated what was left: one point, whether the pair trigger lands in the unit with no promise behind it.
- B and C: yes (likely), each with a measurement rule (no body refused that the base meshes; no cell flat where the base was positive; nothing added on ordinary bodies). 156,970 and 149,372.
- A moved: yes, on a measured condition (likely; was no, unsure), adding that no body may be refused that the head with the foot alone returns, at any δ. 249,252.

**Measurement A1**, taken by the implementer between rounds 2 and 3 (`-measurement-A1.md`): the base is not flat at δ = 1e9; chord-count floors make δ 1 and δ 1e9 one mesh; the base returns the thin cap at 0.726 of its volume there and the head returns it flat at one latitude.

**Round 3** (17:35Z to 17:37Z), a confirmation: one four-clause statement, signed by all three; all three withdraw the probe δ's exemption from condition (b). 255,761 (A), 163,490 (B), 155,914 (C).

## Agreed recommendation (all three, round 3)

1. `tessellate` owes distance and a closed manifold, not volume; a flat mesh of a body thinner than δ is a correct `Ok`. The sign is read per shell by a check that takes the body, which the export path calls and refuses typed on, naming δ. That check, a sentence in `crates/mesh/README.md` and a clarification of `[mesh.winding]` are the parked row's and wait for the owner.
2. A split's point is the foot, on the face's own surface, of the side's straight midpoint, on every analytic surface; spline and spindle faces keep the chart midpoint.
3. The pair trigger (two faces that each have a triangle built only from points they share both split those triangles' inside sides at the join; such a triangle with no inside side gets a point at the foot of its centroid, only if cells of the class need it) lands in the unit if, measured: (a) no body is refused that the base meshes or that the head with the foot alone returns `Ok`, at any δ including the viewer's probe; (b) no cell of the class is flat or negative where the base was positive, the probe δ included; (c) it adds nothing on bodies with no two faces closer than δ (under about 1% of triangles over the two suites).
4. If (b) fails on more than a handful of cells, or (a) or (c) fails and cannot be fixed inside the trigger, the trigger stays out, the unit merges with the foot and corrected wording, the flat cells are a correct `Ok` listed on the parked row, and "refuse or leave flat when a pair split fails" goes to the owner with that row.

Tokens, first report / total: A 218,334 / 255,761; B 136,402 / 163,490; C 128,088 / 155,914.
