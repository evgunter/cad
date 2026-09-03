---
id: viewer-chrome-not-in-nextest-archive
kind: issue
title: Viewer chrome has no CI-gated coverage - nextest archive builds without --features app
status: open
opened: 2026-08-31
github: 1385
refs: [1375]
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
