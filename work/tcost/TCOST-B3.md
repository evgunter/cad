---
id: TCOST-B3
kind: unit
title: rust-cache misses on the build job: 'No cache found' on five of seven build jobs and the control
status: closed
opened: 2026-09-03
refs: [TCOST-C4]
branch: tcost/b3-rust-cache-misses
pr: 1684
closed: 2026-09-03
---


Cut at TCOST-C4's report (PR 1648): across the sccache trial's seven
runs and its rig-inert control, `Swatinem/rust-cache` reported
`No cache found` on five of seven `build + archive` jobs, so on those runs
the job compiled ~300 units from scratch — the build profile's
critical path. What the seven runs establish (the reviewer's
narrowing): all were one PR branch and the two restores were that
branch's own saves; GitHub scopes caches to the branch plus the
default branch, and F3 means main never runs this job — so "a
branch's first build job restores nothing", and F4's premise
("rust-cache already caches the deps") is false for exactly that
first job. C4 wrote the finding into CI-MINUTES F4 as an
open lever and did not fix it. This unit: read the cache key the job
computes on those runs against the keys saved by green main runs
(lane, profile env, lockfile hash, `workspaces`/`shared-key`), find
why the restore misses (key rotation per lane/ε? the eviction of
workspace members? a save that only happens on main?), and land the
fix with a before/after `build test binaries + archive` reading at a
matched tier. Build-side track: Opus implementer, batched style
review, no A/B row.
