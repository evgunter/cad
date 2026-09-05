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
| 0 | PROPS-1 — `docs/PROPS-1-SPEC.md` (mirror + reject_from respells, the one re-baseline pass) | M | OPUS |
| 1 | banked — the next kernel-logic unit cut | — | OPUS |
| 2 | banked | — | FABLE |

Dual reviews draw their R1/R2 parity byte at review dispatch and record
it in the row; ordinals claim from 2400 at review dispatch on main.
