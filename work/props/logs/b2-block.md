# PROPS-B2 — implementer block record (branch-side; reviewers do not read this)

Protocol: `docs/MODEL-AB-LOG.md` — three slots {opus, opus, fable}
(the 2026-09-04 ratio amendment), fable's position = byte mod 3, byte
rejected at ≥ 252. Recorded here rather than in `work/props/log.md`
because a block record naming unstarted slots is a reviewer-visible
leak (`memories/model-ab-experiment.md`); this branch merges to main
when the block concludes (PROPS-B1's shape, #1978).

| drawn | byte | fable slot |
|---|---|---|
| 2026-09-05, by the PROPS orchestrator | 87 | 0 |

| slot | unit | pre-draw difficulty (logged at spec) | arm |
|---|---|---|---|
| 0 | coeffs-window — `docs/PROPS-COEFFS-SPEC.md` (`coefficients-carry-their-knot-vector`, ruling A one level down) | L / STRUCTURAL | FABLE — MERGED 2026-09-05 at 55d541ae5 (#1985) BEFORE the dual, by the orchestrator's spec defect (§Landing said close, not review); dual dispatched on the merged head, ordinal 2403, sample #144; fix pass to follow as its own PR |
| 1 | (next kernel unit in dispatch order) | — | OPUS |
| 2 | (next kernel unit in dispatch order) | — | OPUS |

Dual reviews draw their R1/R2 parity byte at review dispatch and record
it in the row; ordinals claim from 2403 at review dispatch on main.
