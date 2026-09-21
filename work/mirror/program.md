---
id: mirror
kind: program
title: MIRROR — the checkers and local scripts beside CI: the parity reader, the render lanes, the calibrators and their pins
status: open
opened: 2026-09-20
area: infra
prefix: mirror/
tag: (MIRROR orchestrator)
ab_band: 9900-9999
paths: [scripts/check-ci-mirror-parity.py, local-scripts/*]
keep_out: [opened by CIW's 2026-09-20 priority-seam cut (Ev, in chat) per work/README.md Track size - CIW measured 67 budget points and was cut into tracks meant to run in PARALLEL (Ev, in chat: for these high priority tracks it is ideal to have several components that can be worked on in parallel), the siblings are CIW BLIND MIRROR and they share their parent's territory by design - shared ground is legitimate by the README's 2026-09-20 rule and what is owed is awareness while a lane is LIVE, so run scripts/work.py territory on your branch and announce the seam in the PR rather than drawing a fence, GUARD owns scripts/gates whole, TCOST owns what the suite costs, BUDGET owns the tessellation instrument, INSTR owns k-lint and the K-report]
priority: P4
---
The `scripts/` and `local-scripts/` half of the S-QA ground, split from
the workflow half CIW keeps: the CI-mirror parity checker, the render
and provenance lanes, the opt-level calibrator, the criterion emitter,
the k-probe sweep and the pin readers.

The parity checker is itself the largest item here — it has grown 64%
in three days, is ~90% of the wall on a docs or tracker PR, and its
claim-10 readers are blind through `bash -c`, open with the same
five-statement preamble three times over, and still discard
step-level keys. A checker that expensive and that near-sighted is
worth a sitting of its own.

Beside it, the calibrator records without its selftest, the cpuinfo
selftest asserts nothing a broken parse would fail, two anchored pin
readers live in two homes, and three hand-written shell command
splitters exist that nothing compares.

Charter and order: `work/mirror/plan.md`; narrative in `work/mirror/log.md`.
