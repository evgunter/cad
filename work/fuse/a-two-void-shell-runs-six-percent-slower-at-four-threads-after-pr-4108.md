---
id: a-two-void-shell-runs-six-percent-slower-at-four-threads-after-pr-4108
kind: issue
title: The two-void shell row runs about 6% slower at 4 threads after PR 4108; the census gate's role read is the unconfirmed suspect
status: open
opened: 2026-10-06
priority: P3
cost: E
refs: [one-home-for-where-a-shell-stands]
---


## The finding

The 4108 lane's release spot-check on e9a1aeec ran 2 rounds, with
medians in ms, against origin/main at merge time:

| row | @4 main | @4 head |
|---|---|---|
| `shell`, two-void box | 2.18–2.24 | 2.35–2.38 |

That is about 6% slower, outside the round range. Every other row
(classify at 41 and 161 bricks, hollow box, vessel; `shell` at 1
thread) is within noise.

The lane's unconfirmed suspect: the census gate's role read now goes
through the `stands` ladder, which re-derives an interval per shell
of a multi-shell solid. Measure it with more rounds. If the gap is
real, profile it; the remedy is the role read's cost, not the
ladder's single home.
