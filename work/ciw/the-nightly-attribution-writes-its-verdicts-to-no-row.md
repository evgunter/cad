---
id: the-nightly-attribution-writes-its-verdicts-to-no-row
kind: issue
title: The nightly's attribution reports its verdicts in a job summary and writes no main-red row
status: open
opened: 2026-11-22
priority: P3
cost: M
refs: [dependents-slow-sets-skip-a-kernel-pr]
---


Ev's PR 6731 policy has the nightly write each attributed red up as a
`main-red` row. `nightly-attribution.yml` (from
`dependents-slow-sets-skip-a-kernel-pr`) stops at its job summary and
the `nightly-attribution` artifact (`scripts/nightly-attribution.py`,
`report`): an orchestrator reads the verdict and files the row.

Writing the row from the job means a push to main from CI (or a bot PR)
and choosing the owning program for a culprit merge, which is
`scripts/work.py territory` over the merge's diff. Two further steps, each
cheap once the first exists: comment on the culprit PR, and append to an
open row that already holds the test by name instead of opening a second.

Also open on the same job: its probes fit one binary group a night on a
hosted runner (a topo-lib probe built in about 9 to 10 minutes locally,
so 70 minutes holds about seven probes). Further groups come back as
ranges. If that proves too little, parallel probe jobs (a matrix over
binary groups) are the lever.
