---
id: the-nightly-build-job-holds-over-five-scheduled-main-nights
kind: issue
title: The nightly's full-suite build job has to stay green on five scheduled main nights after PR 7365 before its shutdown cause counts as gone
status: open
opened: 2026-11-22
priority: P2
cost: E
---


Filed by GREEN's `green/nightly-build` lane, from PR 7365's review.

## What

`the-nightlys-shared-build-job-dies-to-a-runner-shutdown-most-nights`
closed on PR 7365's change: swap, the leaf crates at opt 1, and the
scalpel in its own job. The evidence is one dispatched night (44989234943),
which `record` skips by design. Before the change the build job was
killed on four nights of seven (11-16, 11-18, 11-19, 11-22). At that
rate one clean night is luck about four times in ten.

No other signal will catch a returning kill. Every nightly since 11-08
concludes `failure` on main's own reds, so a killed build job does not
change the run's colour. And `full suite coverage` is skipped along with
the legs, so nothing distinct goes red.

## Done when

Five scheduled nightlies on main after PR 7365 merges each have
`full suite, build once` green at the step level (jobs API). If one is
killed instead (exit 143, or "lost communication"), reopen the shutdown
row with that job's id and the sampler's last lines, if its log survives.
