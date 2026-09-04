---
id: viewer-chrome-not-in-nextest-archive
kind: issue
title: Viewer chrome has no CI-gated coverage - nextest archive builds without --features app
status: review
opened: 2026-08-31
github: 1385
refs: [1375]
branch: chrome/viewer-app-feature-ci-coverage
pr: 1755
---

## From GitHub issue 1385

Opened 2026-08-31; 0 comments.

Recorded deviation from GAUTH-1 (PR #1375), filed as its schedule per protocol v5.

The hosted CI nextest archives are built without the viewer's `app` feature (`cargo nextest archive ${cargo_scope} ...` at ci.yml ~1762 and ~2300), so nothing behind `#[cfg(feature = "app")]` is compiled into the test archive and no chrome row can gate today — `tests/chrome_labels.rs` (`#[cfg(feature = "app")]`) is silently feature-skipped in the archive lane.

What this leaves untested at the gate, concretely, as of GAUTH-1:
- the selection-stream → tool.pick feed in `ViewerApp::ui` (mate tool face feed, revolve tool node feed, and the new one-active-tool exclusivity);
- the creation forms in `app.rs` (New… control, add-datum/add-profile/extrude forms, revolve tool panel) — including form-level affordances like the bore < radius guard, which exist ONLY in chrome;
- everything `chrome_labels.rs` already pins (document_name, initial_layout).

The headless op-replay suites (`creation_ops.rs`, `panel_edits.rs`, ...) gate the vocabulary the forms emit, per G1 — this issue is about the widget-to-op wiring above that seam.

Possible shapes (not adjudicated): compile the archive with `--features app` (costs the eframe/wgpu dependency tree in the test build — measure before adopting); a separate small `cargo test -p viewer --features app` job for the cfg'd headless-testable chrome rows; or extracting the feed/exclusivity logic below the `app` cfg so ordinary suites reach it. Whichever is taken should make `chrome_labels.rs`'s silent feature-skip loud.

## Home

GAUTH's closing entry names this issue as its residue and the fix touches `.github/workflows/ci.yml`, but GAUTH, GUI and S-QA are all closed programs, so it lands in `work/issues/`.

## Fixed (CHROME, 2026-09-04)

Measured, then sited. The archive is NOT given `--features app`: it is
built once and downloaded by every leg of the `test` matrix
(`ci.yml:2333`, `shard: [1, 2]`), so the size delta is paid per leg and
buys nothing for rows that already gate. The row instead sits beside
the app-feature clippy step under the same `run_viewer_toolkit` axis
Ev's viewer-CI-posture ruling put there. Numbers and their blind spots
are in PR 1755.

**The item's premise was incomplete: there were THREE silent skips in
`viewer`, not one.** `chrome_labels.rs` was the one this item names;
`error_display.rs`'s `StartupError` row and every unit test inside
`src/app.rs` and `src/gpu.rs` were equally invisible. All three now
carry markers that print in the PASS list by NAME
(`app_lane_skipped_no_chrome_coverage_here` and siblings), following
the existing house convention rather than a second one.

**And the suspected cause was not the cause.** `tests/all.rs` and
`autotests = false` were checked and cleared — the aggregator lists
`chrome_labels` and a guard already enforces that. The invisibility
was purely the `#![cfg(feature = "app")]` inner attribute.

**Sweep**: 78 file-level feature-gated suites across the workspace, of
which 5 carried a loud-skip marker. The 3 viewer hits are fixed; the
other 75 are not this unit's — their features (`interval`, `probe`,
`budget`) each have a CI lane that builds them, so those suites gate
somewhere and their default-lane absence is a designed asymmetry.
`app` was the only feature that gated nowhere. **What the pattern
could not match**: content made absent by a manifest
`required-features` rather than a source `cfg` (this crate's
`[[bin]] viewer` is exactly that), `cfg(all(..))`/`cfg(any(..))`
spellings, and a gated `mod` inside an ungated file.
