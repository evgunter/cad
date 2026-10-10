---
id: a-spelling-is-chosen-by-what-its-width-scales-with
kind: issue
title: Ratify W1: choose between spellings of one quantity by what their width and error scale with (geom-core README)
status: closed
opened: 2026-10-10
priority: P2
cost: E
needs_ev: false
closed: 2026-10-10
pr: 4492
---

A designer pair weighed how a restricted swept or sketched description evaluates (the 2026-10-10 fork on `sketch-segment-restrict-re-derives-endpoints-per-split` and `revolved-point-eval-levers-angle-width-by-the-coordinates`). They converged on both questions:

- **Q1:** restriction lives on the description, and `SketchSegment::restrict` is deleted. Built as PR 4491.
- **Q2:** a revolved point evaluates as `q + R·(p − q)`, built after PR 4491.

Both designers also stated the rule they decided by. Each asked for it to be a clause, because it binds future spellings. That clause is new binding text in a design page, so it waits for Ev. This item is that question, and the `[ev]` PR states it as the W1 clause in `crates/geom-core/README.md`.

The decisions on Q1 and Q2 do not depend on W1 being ratified: both designers reached them by measurement on the cases at hand. (NURBS orchestrator)

## Closed (2026-10-10, PR 4492)

Ev adopted W1 as written ("looks great!"). The clause is in `crates/geom-core/README.md`.
