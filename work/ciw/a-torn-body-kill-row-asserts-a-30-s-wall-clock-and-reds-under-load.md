---
id: a-torn-body-kill-row-asserts-a-30-s-wall-clock-and-reds-under-load
kind: issue
title: review_m1_pr4's torn-body kill row asserts a 30 s wall clock, which a loaded box fails
status: closed
opened: 2026-10-20
priority: P3
cost: E
closed: 2026-11-22
---

Found by review 2 of `restread/ef-pierce-charge` (PR 5421), which ran
it beside other lanes' builds.

## What

`crates/topo/src/review_m1_pr4.rs`
`kill_ops_on_torn_bodies_panic_only_naming_a_premise` ends with
`assert!(started.elapsed() < Duration::from_secs(30), "bounded walks
must keep this fast")`. Under load on a shared box it went red on that
assertion with every kill outcome as expected. The CI fast-set timing
(`work/ciw/fast-set-tests-over-one-second-are-not-enforced.md`) has the
row at 7–8 s alone, so the 30 s wall is a factor of four on a quiet
machine and none on a busy one.

The property it means to hold is that the kills' walks are bounded,
which a wall clock measures only through the machine's load. No
program's territory holds the file (`work.py territory --files`).

## Direction

State the bound as a count the walks report (steps, or the arena size
they visit), or drop the assertion and let the slow-set timing carry
the cost; a wall clock is not a gate.

Moved from `work/issues/` to `work/tint/` on 2026-11-03, when the `work/issues/` bucket was retired (Ev's direction, in chat); id and body unchanged.

Moved from TINT to CIW on 2026-11-20, by `git mv`, when TINT merged into CIW: TINT held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one CIW carries.

## Closed

Deleted the 30 s `elapsed` assertion (and its `Instant`) from `kill_ops_on_torn_bodies_panic_only_naming_a_premise`; the slow-set timing carries the cost.
