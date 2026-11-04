---
id: both-sphere-witnesses-under-every-boolean-run-at-the-fast-sets-one-second-bound
kind: issue
title: sweep spheres_crossing_off_every_edge::both_witnesses_build_under_every_boolean runs at the fast set's 1.0 s bound, so an unrelated PR's gate goes red on runner variance
status: open
opened: 2026-11-04
priority: P4
cost: E
---


Filed by the SPLICE orchestrator.

## What

On PR 6476's head `f8a4ecfd` (CI run 41743102954, job `test (kernel)`), the default-ε step failed on time alone:

    SLOW [default eps] sweep::all spheres_crossing_off_every_edge::both_witnesses_build_under_every_boolean 1.043 s (existing test: 1.0 s)

The same PR's previous head (`241c3f34`, run 41740089217) was green on the same tree up to a merge of main that touched neither this test nor the sphere arm. So the test sits at the bound, and the per-PR gate goes red on runner variance. It is the second such row this sitting, beside `a-boss-sketched-on-a-side-runs-at-the-fast-sets-one-second-bound`.

## What closing it takes

Measure the row's time on main across a few hosted runs. If it sits near 1.0 s, either make it faster or add it to the slow set (`.config/nextest.toml`), per the fast-set rule. Do not raise the bound.
