---
id: nightly-rustdoc-gate-red-on-main
kind: issue
title: the nightly's rustdoc (gate, every root) job is red on main, and no row names its cause
status: open
opened: 2026-10-28
priority: P1
cost: E
---


## What

Nightly 74 (run 40637814390, main `a1cfd1cfdb`, 2026-10-28) fails
`rustdoc (gate, every root)` (job 122649031004) with `Process completed
with exit code 1` after the `demos/tour` root documents; the log's last
60 lines carry no diagnostic, so the failing root and warning are
further up the log. No row on any slate names this job (grep over
`work/` for `rustdoc (gate`, 2026-10-28).

## Owed

Read the job's log for the first `warning:`/`error:`, find the merge
that introduced it, and fix it at its site. If the root cause is a gap
in the per-PR doc gate (a root or a cfg the PR row does not document),
say so in the PR and file it on CIW.
