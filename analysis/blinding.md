# Design fork: dl5-has-no-arm-for-a-selection-whose-result-is-the-payload

Dispatched 2026-11-19 by the SCALAR orchestrator. Arm byte 196 (>= 86): two designers. Blinding byte 250 (parity 0): Opus is A, Fable is B.
Protocol at dispatch: ddd201f411
Deviations: none at dispatch.

[ev] PR 7155; docs/DESIGN-FORK-LOG.md row 292. Models as the harness dispatched them: A = Opus 5.5 (Agent model 'opus'), B = Fable 5.2 (Agent model 'fable').
Rounds: first reports, then three reconciliation rounds (the third on the home alone). No further designers.
Tokens (harness, cumulative per agent at each return): A 90971 / 103379 / 112387 / 118149; B 101773 / 113090 / 124197 / 136633.
Deviations: the orchestrator merged origin/main into the checkout the designers were reading during round 1 of reconciliation (a tracker-only merge).
