---
id: trimmed-slow-canary-for-a-seeded-crate
kind: issue
title: a trimmed, fast per-PR slow set that catches most of the reds a seeded crate's whole slow set would
status: open
opened: 2026-11-12
priority: P2
cost: M
---


## What

Ev, on PR 6731 (2026-11-12), ruled out running a seeded crate's whole slow set on its PR: about 5.5 min (topo) to 26 min (sweep) of gate wall time per PR (`dependents-slow-sets-skip-a-kernel-pr.md`, `log.md` 2026-11-10). Ev is "quite open to a trimmed-down, much faster set of tests which would still catch most of these issues". Today a PR runs only the slow tests its added lines spell (`scripts/slow_set.py`'s `is_new`), so a moved slow test in its own crate, or in a dependent, waits for the nightly.

## What it has to beat

- **Cost:** a few minutes of gate wall time at most, against the gate's kernel test job of about 11 min.
- **Catch:** the reds the counts in `dependents-slow-sets-skip-a-kernel-pr.md` record. Most are pinned ledgers and counts in a few tests: editor-core's `m10_sym_profile_interval` plate ledger (eight PRs), `m10_9_pins_interval`, step-export's `kernel_sidecar_fields_match_live_kernel`, sweep's `a_lofted_arc_wall_is_a_function_of_the_sections_bulge`. The slow set's time sits in a tail: 20 tests are 33% of it, and 1486 tests under 10 s each are 69 cpu-min in all.

## Candidate shapes, unmeasured

- **The reds' own history:** run the slow tests a nightly attribution (or a `work/` row's count) has ever traced to a PR that did not spell them. That list is short and is exactly the shape that recurs.
- **The cheap body of the slow set:** the slow tests under a few seconds each, sharded, where most ledger and count pins live; the tail stays nightly.
- **A `gated_to!` path marker** on the tests whose inputs are known by path, run when the diff touches one (the census row's option A).

The next step is measurement: for each shape, its gate cost and how many of the recorded reds it would have caught on their own PR.
