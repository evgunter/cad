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

Also open on the same job: its capacity. On the three real nights read
(11-17, 11-20, 11-21) 25 to 37 tests went newly red, across 6 to 10 test
binaries, over 150 to 172 first-parent commits. A full bisect of one
binary is about 8 builds plus up to 2 at the ends. A topo-lib probe
build took 5.5 to 8 minutes on a 4-core box, so the 60-minute budget
holds about one binary's bisect a night, plus a second when it shares
the first's culprit (2 builds). The rest come back as ranges. Parallel
probe jobs (a matrix over binaries) are the lever if that is too little.
