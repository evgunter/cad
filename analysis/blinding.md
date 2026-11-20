# Design fork: dl5-has-no-arm-for-a-selection-whose-result-is-the-payload

Dispatched 2026-11-19 by the SCALAR orchestrator. Arm byte 196 (>= 86): two designers. Blinding byte 250 (parity 0): Opus is A, Fable is B.
Protocol at dispatch: ddd201f411
Deviations: none at dispatch.

[ev] PR 7155; docs/DESIGN-FORK-LOG.md row 292. Models as the harness dispatched them: A = Opus 5.5 (Agent model 'opus'), B = Fable 5.2 (Agent model 'fable').
Rounds: first reports, then three reconciliation rounds (the third on the home alone). No further designers.
Tokens (harness, cumulative per agent at each return): A 90971 / 103379 / 112387 / 118149; B 101773 / 113090 / 124197 / 136633.
Deviations: the orchestrator merged origin/main into the checkout the designers were reading during round 1 of reconciliation (a tracker-only merge).
Later rounds, after Ev's comments on PR 7155 (2026-11-20). Row numbering in the fork log: its rounds 4 to 7 are these four returns. Tokens (harness, cumulative per agent at each return): A 147356 / 155811 / 215016 / 237962; B 166566 / 178514 / 235120 / 254076. A read-only explorer (not a designer; harness agent type Explore, model as the harness chose it, unknown) surveyed the from_computed sites between the third reconciliation round and the next; both designers were handed its report.
Deviation: from the round after Ev's second comment the designers wrote their reports to files beside returning them; both the same way.
Three further returns after Ev's row-by-row answer (2026-11-20; the fork log's rounds 8 to 10). Tokens, cumulative: A 287997 / 317654 / 325346; B 304039 / 336767 / 345132. Briefs were files, identical for both (dl5-r9-brief.md, dl5-r10-brief.md), the last a short message differing only in which form each was asked to weigh against its own.
