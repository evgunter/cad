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
| 0 | coeffs-window — `docs/PROPS-COEFFS-SPEC.md` (`coefficients-carry-their-knot-vector`, ruling A one level down) | L / STRUCTURAL | FABLE — MERGED 2026-09-05 at 55d541ae5 (#1985) BEFORE the dual (orchestrator spec defect); dual concluded on the merged head — ordinal 2403, sample #144, no tally candidate (the one MAJOR converged with the other arm's MINOR); fix pass as its own PR |
| 1 | sign-hull — `docs/PROPS-SIGN-HULL-SPEC.md` (`interval-orthonormal-basis-sign-hull`, Ev's option-1 ruling on #1944; the (c′) dispatch of 2026-09-06 was withdrawn within the hour on the retraction, before any push) | M / NUMERIC | OPUS — dispatched 2026-09-06 |
| 2 | (next kernel unit in dispatch order) | — | OPUS |

Dual reviews draw their R1/R2 parity byte at review dispatch and record
it in the row; ordinals claim from 2403 at review dispatch on main.
