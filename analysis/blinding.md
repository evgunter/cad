# Design fork: patherror-display-renders-float-noise

Dispatched 2026-11-19 by the SCALAR orchestrator. Arm byte 26 (< 86): three designers. Blinding byte 88 (mod 3 = 1): Fable is B; the two Opus designers are A and C (A is the first dispatched).
Protocol at dispatch: ddd201f411
Deviations: none at dispatch.

[ev] PR 7154; docs/DESIGN-FORK-LOG.md row 291. Models as the harness dispatched them: A = Opus 5.5 (Agent model 'opus'), B = Fable 5.2 (Agent model 'fable'), C = Opus 5.5 (Agent model 'opus').
Rounds: first reports, then two reconciliation rounds. No further designers.
Tokens (harness, cumulative per agent at each return): A 131661 / 147210 / 160397; B 118788 / 138605 / 151115; C 108497 / 124380 / 137881.
Deviations: the orchestrator merged origin/main into the checkout the designers were reading during round 1 of reconciliation (a tracker-only merge).
