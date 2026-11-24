---
id: the-viewer-jobs-slow-step-reruns-the-default-feature-viewer-slow-tests
kind: issue
title: ci.yml's viewer job's slow-set step re-runs the default-feature viewer::all slow tests the test job already ran, about 70 s a run
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by PR 7599's review (MINOR 1), which added the step.

**What happens.** `.github/workflows/ci.yml`'s viewer job, step "viewer app-feature slow-set tests (the ones this diff adds or edits)", runs `-E "package(viewer) & $slow"`. That picks every selected viewer slow entry, the default-feature `viewer::all` ones as well as the `app::` ones. The viewer job runs only when viewer is seeded, and then the test job's slow-set step has already run those `viewer::all` entries. On PR 7599's probe (dispatch run 45237388246, a comment edit to `crates/viewer/src/app.rs`):
- `test (app)`'s slow step ran 8 tests in 23 s.
- The viewer step ran the same 8 plus the 2 `app::` tests in 71.5 s, about 69 s of it the duplicates (`display_budget::*`, `review_pick_r2::*`).

**Why it is not just `binary_id(viewer)`.** Some `crates/viewer/tests/*.rs` modules are `#[cfg(feature = "app")]` (`tests/drive.rs`, `tests/chrome_labels.rs`). A future app-gated `viewer::all` slow entry would be missed by a filter on the lib binary alone.

**Owed.** Run in the viewer step only the selected entries that the default-feature listing does not hold. One way: diff the two listings the jobs already make. Or say in the step's comment that the duplication is accepted. Cost only, not correctness: P3.
