---
id: budget
kind: program
title: BUDGET — the tessellation budget instrument: what tess-lint reads, what it compares, and what its figures are worth
status: ready
opened: 2026-09-20
area: infra
prefix: budget/
tag: (BUDGET orchestrator)
ab_band: 9700-9799
paths: [tools/tess-lint/src/*, tools/tess-meter/src/*, docs/TESS-BUDGET.md]
keep_out: [opened by INSTR's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - INSTR measured 47.5 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are INSTR BUDGET and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, GUARD owns scripts/gates whole, TINT and VACUITY own the suite-side instruments, CHORD owns the tessellator arithmetic the budget measures]
priority: P3
---
The tessellation-budget instrument, split from the predicate
instrument INSTR keeps — they share a discipline and nothing else.

METER's standing rule is that **an instrument's claim and its own
execution must not part**, and these rows are where they have.
`tess-lint` admits `worst_cert = 0` as a READING where the kernel means
it as an absence, so two different facts arrive as one number. A re-cut
folds every column the gate reads but compares only some, so a column
can move without anything saying so. The growth margin is unprotected
from the split schedule's ceil quantisation. `tess-meter`'s 21-sample
retune figure names a draw nothing can reproduce.

And three rows are the figures' own bookkeeping: a cut line that stamps
the sweeping tree's HEAD without naming a baseline change, a pin table
with no whitespace-tail row, and a budget doc where nothing reds when an
unlabelled current figure arrives.

Charter and order: `work/budget/plan.md`; narrative in `work/budget/log.md`.
