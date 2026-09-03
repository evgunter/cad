---
id: TCOST-C4
kind: unit
title: sccache trial re-read: per-crate hit stats on warm runs, verdict under F4
status: dispatched
opened: 2026-09-03
branch: tcost/c4-sccache-reread
---

CI-posture unit (Ev's ask). The kernel is compiled ~9–10 times per
code-tier run in ~8 unifications that share no artifacts; the one lever
across RUNS is content-keyed sccache, wired since #852 (F4) and off
today (`vars.SCCACHE = 0`) with its reading still owed. Make the rig
unconditional for a trial window, read `sccache --show-stats` on ≥3 warm
runs per lane separating dependency hits (prove nothing) from workspace
crates and test binaries (the hypothesis — 82 % of the build job),
compare archive-step durations only at the same tier and package set,
and write the verdict under F4 and beside the local revert either way.
