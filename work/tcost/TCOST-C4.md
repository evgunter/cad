---
id: TCOST-C4
kind: unit
title: "sccache trial re-read: per-crate hit stats on warm runs, verdict under F4"
status: closed
pr: 1648
branch: tcost/c4-sccache-reread
opened: 2026-09-03
closed: 2026-09-03
refs: [852, 853, 1648]
---

CI-posture unit (Ev's ask). The kernel is compiled ~9–10 times per
code-tier run in ~8 unifications that share no artifacts; the one lever
across RUNS is content-keyed sccache, wired since #852 (F4) and off
today (`vars.SCCACHE = 0`) with its reading still owed. Make the rig
unconditional for a trial window, read `sccache --show-stats` on warm
runs per lane separating dependency hits (prove nothing) from workspace
crates and test binaries (the hypothesis — 82 % of the build job),
compare archive-step durations only at the same tier and package set,
and write the verdict under F4 and beside the local revert either way.

## Read (PR 1648)

**Negative, and structurally so.** sccache 0.16.0 refuses
`--crate-type bin`, which is every test binary in the nextest archive —
so the 82 % was never available to it at any hit rate. Seven runs, all
at `tier=all` with the same package set and the lane asked for by
trailer: the two where both caches restored show 18 cacheable units
(the workspace libs; 18 hits on the default lane, 5 on interval) and 47
`crate-type` refusals. The ~205 MB per-lane object cache also failed to
survive 38-, 60- and 88-minute gaps between runs.

The rig stays, off by default (`vars.SCCACHE == '1'`), so the repo
variable can be deleted. Verdict in `docs/CI-MINUTES-2026-08.md` F4,
raw readings under `docs/perf-data/sccache-trial/`, local half in
`docs/LOCAL-BUILD-PERF.md`. Closes `sccache-trial-verdict-to-read` when
1648 merges.

The larger finding the trial turned up — `Swatinem/rust-cache` restores
nothing on most build jobs, so they compile ~300 units from scratch —
is F4's closing paragraph and is not this unit's to fix.
