---
id: loop-boundary-discards-selftest-plant-misses-census-face-cycles
kind: issue
title: loop-boundary-discards.sh --selftest fails on main: its plant in census.rs face_cycles no longer yields the 'matched 2' diagnosis it expects
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by BOXES' `cover-levers` lane, and reproduced by the BOXES orchestrator on a clean `origin/main` worktree (2026-11-09):

```
SELFTEST FAILED: loop-boundary-discards fired on (plant_extra_site crates/topo/src/census.rs face_cycles wrapped-return) with an unexpected message — wanted a diagnosis carrying `matched 2`:
UNREG|crates/topo/src/census.rs|4|face_cycles|let LoopBoundary::Cycle { first } = body .get_loop(lk) .ok_or_else(corrupt)? .boundary else { return
```

The selftest plants a second discard beside a registered `face_cycles` site in `census.rs` and expects a MISCOUNT ("matched 2"). It now gets UNREG, which suggests the registered entry no longer names the site the plant sits beside. Likely cause: recent changes to `census.rs` (the cover build and others) moved or re-spelled that site, and the register entry was updated without the selftest's plant target.

The gate itself still passes on main and in PR CI, so this is the selftest, the guard's guard, going stale. Fix: point the plant at a registered site that exists, or derive the plant target from the register rather than naming it. P3: a guard that cannot show it goes red. (BOXES orchestrator)
