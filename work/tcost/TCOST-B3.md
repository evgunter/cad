---
id: TCOST-B3
kind: unit
title: rust-cache misses on the build job: 'No cache found' on five of seven build jobs and the control
status: open
opened: 2026-09-03
refs: [TCOST-C4]
---


Cut at TCOST-C4's report (PR 1648): across the sccache trial's seven
runs and its rig-inert control, `Swatinem/rust-cache` reported
`No cache found` on five of seven `build + archive` jobs, so that job
compiles ~300 units from scratch on most runs — the build profile's
critical path — and F4's premise ("rust-cache already caches the
deps") is false for it. C4 wrote the finding into CI-MINUTES F4 as an
open lever and did not fix it. This unit: read the cache key the job
computes on those runs against the keys saved by green main runs
(lane, profile env, lockfile hash, `workspaces`/`shared-key`), find
why the restore misses (key rotation per lane/ε? the eviction of
workspace members? a save that only happens on main?), and land the
fix with a before/after `build test binaries + archive` reading at a
matched tier. Build-side track: Opus implementer, batched style
review, no A/B row.
