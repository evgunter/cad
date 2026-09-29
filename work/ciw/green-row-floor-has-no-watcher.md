---
id: green-row-floor-has-no-watcher
kind: issue
title: DESIGN.md's green-row floor (engineering convention 3) has no watcher in the repo that asserts it
status: open
opened: 2026-09-04
priority: P3
cost: E
design: true
---

Re-homed from BLIND (closed 2026-09-28) and rewritten against the
latency-cut CI (`work/ciw/latency-cut.md`).

## Finding

`docs/DESIGN.md` §D9's engineering convention 3 says *any merge-gating
checks watcher asserts a minimum green-row count equal to the current
full CI matrix, bumped in the same PR that grows the matrix*. Nothing in
the repository implements it, and the thing it counts is gone: `ci.yml`
has no test matrix, and its one required check, `gate ok`, lists every
job in its `needs:` and fails on any `failure` or `cancelled` result.
The convention was earned by a stale-matrix trap (#113: a branch
predating new rows showed green on the old matrix).

## What is owed

A DESIGN.md revision, so Ev's (an `[ev]` PR): retire the convention, or
restate it as what `ci.yml` actually enforces — `gate ok` names every
job, and a job that did not run reads `skipped`, not green. Do not build
a row-count watcher: there is no matrix for it to count.
