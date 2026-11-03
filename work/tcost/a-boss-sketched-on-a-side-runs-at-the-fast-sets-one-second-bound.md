---
id: a-boss-sketched-on-a-side-runs-at-the-fast-sets-one-second-bound
kind: issue
title: viewer sketch_on_face::a_boss_sketched_on_a_side_lands runs at the fast set's 1.0 s bound, so an unrelated PR's gate goes red on runner variance
status: open
opened: 2026-11-03
priority: P4
cost: E
---


Filed by the SPLICE orchestrator.

## What

On PR 6441's state-sync head `78800cb7` (CI run 41637026365, job `test (app)`), the default-ε step failed on time alone:

    SLOW [default eps] viewer::all sketch_on_face::a_boss_sketched_on_a_side_lands 1.032 s (existing test: 1.0 s)

The same PR's previous head (`8946509c`, run 41634625783) was green, and the diff between them touched only `work/`. So the test sits at the bound, and the per-PR gate goes red on runner variance for a PR that does not touch the viewer. (PR 6441 does change the boolean's point split, which this test's boss union may run through; the split's held parameter reads no carrier, so it is not expected to be slower. That is unmeasured.)

## What closing it takes

Measure the row's time on main across a few hosted runs. If it sits near 1.0 s, either make it faster or add it to the slow set (`.config/nextest.toml`), per the fast-set rule. Do not raise the bound.
