# PROPS-B3 — implementer block record (branch-side; reviewers do not read this)

Protocol: `docs/MODEL-AB-LOG.md` — three slots {opus, opus, fable}
(the 2026-09-04 ratio amendment), fable's position = byte mod 3, byte
rejected at ≥ 252. Recorded here rather than in `work/props/log.md`
because a block record naming unstarted slots is a reviewer-visible
leak (`memories/model-ab-experiment.md`); this branch merges to main
when the block concludes (PROPS-B1's shape, #1978).

| drawn | byte | fable slot |
|---|---|---|
| 2026-09-08, by the PROPS orchestrator | 32 | 2 |

| slot | unit | pre-draw difficulty (logged at spec) | arm |
|---|---|---|---|
| 0 | sphere-pole-side — `docs/PROPS-SPHERE-POLE-SIDE-SPEC.md` (`rimless-polar-cap-refuses-degenerateface` + `two-face-sphere-split-measures-zero-volume`) | H / NUMERIC | OPUS — spec 2026-09-08, dispatch waits on disk |
| 1 | escalation-channel — `docs/PROPS-ESCALATION-CHANNEL-SPEC.md` (`escalation-channel-misses-op-minted-indeterminates` + `indeterminate-error-arms-sweep`) | H / STRUCTURAL | OPUS — CONCLUDED 2026-09-20 at merge 3502371ec, ordinal 2407, sample #225 (both arms APPROVE-WITH-FIXES; no tally candidate — both MAJOR-class findings bilateral, the one severity divergence R1 MAJOR / R2 MINOR on the same proven fact) |
| 2 | (still owed to FABLE — see the deviation below) | — | FABLE |

Dual reviews draw their R1/R2 parity byte at review dispatch and record
it in the row; ordinals claim at review dispatch on main.

## Deviation, disclosed 2026-09-20 (orchestrator error)

The orchestrator dispatched **two** implementer lanes together after the
program cut — escalation-channel and curved-residues — and assigned
neither to a slot at dispatch. Both ran on **OPUS**. escalation-channel
is slot 1 and correct. **curved-residues is not slot 2**: slot 2 is
FABLE by the 2026-09-08 draw (byte 32), and drawing a fresh block now
to fit an arm already in flight is exactly the rigging the
draw-before-dispatch rule exists to prevent.

So curved-residues records its row with the arm that actually ran
(OPUS), marked **not a drawn assignment**, and does not fill any slot.
PROPS-B3 slot 2 stays owed to FABLE, and the **next two** PROPS
implementer units are FABLE — one for the owed slot, one to repay the
extra opus row — so the 2:1 ratio is restored rather than merely noted.
Recorded here and in the ordinal claim on main so a tally reader sees
it without reading this branch.
