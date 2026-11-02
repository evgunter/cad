---
id: a-clearance-sweep-is-not-cancellable-inside-its-node
kind: issue
title: A clearance node's certified sweep reads no cancel token, so a long sweep holds the certify worker after an edit
status: open
opened: 2026-11-02
priority: P3
cost: M
---

Found by the correctness review of PR 6357 (m4).

The viewer certifies a landing's clearance requirements at rest
(`viewer::evalseam::ThreadCertifier`, running
`pncad::analysis::clearances_at_nominal`), and the next edit cancels the
job. The evaluation reads its `CancelToken` between nodes only, so one
clearance node whose certified sweep runs long (a fine budget, curved
faces close together) holds the worker until it finishes, and the next
landing's certify waits behind it. A latency cost, not a wrong answer:
the late answer is dropped by generation.

## Repair shape

The sweep (`topo::clearance::Sweep`, through `eval::wire`'s
`wire_clearance`) reads the evaluation's token between subdivision
rounds and answers `Canceled`, as the coincidence door's replay reads it
between margins.

- 2026-11-02 — moved from CLEARTAIL to CLEAR when CLEARTAIL merged back into CLEAR (the budget rule now counts P0/P1 only); id and body unchanged.
