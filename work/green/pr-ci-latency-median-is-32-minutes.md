---
id: pr-ci-latency-median-is-32-minutes
kind: issue
title: PR CI's median wall-clock is 27-33 minutes against the 15-minute bar and the alarm's 20, and no live row holds it
status: dispatched
opened: 2026-11-21
priority: P2
cost: M
---


The nightly's `PR CI latency (alarm)` job: "median PR CI wall-clock 27.2 min exceeds 20 min" on the scheduled 11-21 run (44870651337), and 32.5 min on the dispatched 17:00Z run (44927630239). The bar is 15 minutes, typically under 10 (`memories/orchestration-model.md`, Friction). `ciw/latency-cut` and GREEN's earlier wall-clock rows are closed, so nothing holds it. 11-15 measured 20.8 minutes; it has grown since.

Owed: measure where the median's minutes go now (queue vs run, which job is the long pole, which PRs), and cut it.
