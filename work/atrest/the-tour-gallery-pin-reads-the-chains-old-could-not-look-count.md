---
id: the-tour-gallery-pin-reads-the-chains-old-could-not-look-count
kind: issue
title: demo-tour gallery pin expects 196 could-not-look on the chain scene; since #5853 it reads 10 at-rest overlap findings (red on any PR that pulls the demos job)
status: open
opened: 2026-10-27
priority: P1
cost: M
refs: [5853, 5869]
---


Filed by SECT's orchestrator (2026-10-27), from SECT's lily lane on PR 5869.

## Measured

`demo-tour gallery::tests::each_gallery_document_denotes_its_scene_or_says_why_not`
fails on `origin/main` at 71d97e07 (after #5853, "atrest: curved copies
filling one volume interfere"): the tour's `chain` scene now yields 10
at-rest findings (overlap) where the pin expects 196 could-not-look.
Reproduced on a clean worktree of main by the lily lane. Main's own CI
skips the `demos (tour + wild)` job, so main reads green; any PR whose
diff touches `demos/tour` pulls the job in and goes red on it (PR 5869's
`gate ok`, run 40580799170).

## What the owner decides

Whether the 10 findings are true. The chain's links interlock without
sharing volume, so a finding that two links overlap would be a false
interference (a live wrong answer, P0), while true findings make this a
stale pin (re-pin, and say what moved). The band above is provisional
until that is read.
