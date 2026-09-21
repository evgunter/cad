---
id: blind
kind: program
title: BLIND — CI instruments that cannot see what they name: the denials, the rosters and the runs that read as something else
status: open
opened: 2026-09-20
area: infra
prefix: blind/
tag: (BLIND orchestrator)
ab_band: 9800-9899
paths: []
keep_out: [opened by CIW's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CIW measured 67 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CIW BLIND MIRROR and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, GUARD owns scripts/gates whole, TCOST owns what the suite costs, BUDGET owns the tessellation instrument, INSTR owns k-lint and the K-report]
priority: P3
---
Ev's medium band on CI ground: **tooling that prevents a SILENT bug**,
as distinct from tooling whose payoff is main being red less often —
which is what CIW keeps and what Ev put at low.

Every row here is an instrument that passes while blind. A source-level
`allow` can return a denying CI row to what it was and nothing reads
for it. `check-run-jobs.py` holds no expected-job roster, so a job that
never ran reads the same as a job that passed. `doc-gate.sh` cannot see
a broken intra-doc link inside a `cfg(test)` module. No browser rustdoc
pass runs, so a doc comment on a wasm-only item is checked by nothing.
`ci-local.sh`'s `topo_release` guard greps for a row that cannot match.
The session-start hook is exercised by nothing.

**The row that just proved itself** is
`red-run-whose-jobs-never-started-reads-as-a-broken-tree`: a run whose
jobs were never acquired is indistinguishable from a broken tree. On
2026-09-20 a billing lapse stopped jobs being acquired on PR #2939, and
the three unguarded jobs read as failures with no logs — exactly the
confusion this row describes, met in the wild the day it was scored.

Claims no paths: the instruments are spread across `.github/workflows`,
`scripts/` and `local-scripts/`, and each unit announces its own
fence.

Charter and order: `work/blind/plan.md`; narrative in `work/blind/log.md`.
