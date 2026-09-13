# PROPS-B1 — implementer block record (branch-side; reviewers do not read this)

Protocol: `docs/MODEL-AB-LOG.md` — three slots {opus, opus, fable}
(the 2026-09-04 ratio amendment), fable's position = byte mod 3, byte
rejected at ≥ 252. Recorded here rather than in `work/props/log.md`
because a block record naming unstarted slots is a reviewer-visible
leak (`memories/model-ab-experiment.md`); this branch merges to main
when the block concludes.

| drawn | byte | fable slot |
|---|---|---|
| 2026-09-05, by the PROPS orchestrator | 146 | 2 |

| slot | unit | pre-draw difficulty (logged at spec) | arm |
|---|---|---|---|
| 0 | PROPS-1 — `docs/PROPS-1-SPEC.md` (mirror + reject_from respells, the one re-baseline pass) | M | OPUS — concluded 2026-09-05 at merge 93baf9ce0, ordinal 2400, sample #137 — renumbered at the sync, #136 was FILLET-H7's by merge order (no tally candidate; pair FAIR — both MAJORs converged with a severity divergence) |
| 1 | Span sweep — `docs/PROPS-SPAN-SPEC.md` (`span-carries-its-knot-vector`, ruling A) | L / STRUCTURAL | OPUS — concluded 2026-09-05 at merge 46020d6b9, ordinal 2401, sample #140 (one tally candidate R1/OPUS — the executed panic at the curve doors; both arms interrupted by the session limit and resumed equally) |
| 2 | k-stats bracket + escalation channel — `docs/PROPS-KSTATS-SPEC.md` (`k-stats-escalation-channel-and-redo`) | L / STRUCTURAL | FABLE — concluded 2026-09-05 at merge 3f8a91ff3, ordinal 2402, sample #142 (no tally candidate; the one MAJOR converged with the other arm's MINOR) |

Dual reviews draw their R1/R2 parity byte at review dispatch and record
it in the row; ordinals claim from 2400 at review dispatch on main.


**Block PROPS-B1 concluded (2026-09-05)**: three slots run and merged — OPUS #137, OPUS #140, FABLE #142; one tally candidate over the block (slot 1, R1/OPUS). This record merges to main with the block; the next kernel unit draws PROPS-B2.
