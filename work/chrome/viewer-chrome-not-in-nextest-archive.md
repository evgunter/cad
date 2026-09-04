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

**Sweep**: `git grep -lE '^#!\[cfg\(' -- '*/tests/*.rs'` at merge base
`2f11071` finds 87 file-level gated suites — `interval` 66, `probe` 15,
`all(feature = "probe", feature = "interval")` 1
(`crates/editor-core/tests/m10_3_driver_k_probe_interval.rs:49`),
`budget` 2, `oracle-inari` 2, `app` 1. 85 of those are under
`crates/*/tests/`; the two `oracle-inari` suites are
`interval-transcendentals/tests/`. The 3 viewer hits are fixed. The
other 84 are not this unit's, and the reason has to be EXECUTION and
not compilation, because "it builds somewhere" is precisely what this
unit denies is coverage (`ci.yml` ~3532: `--all-targets` type-checks a
test target and runs none of it). Per feature, from `ci.yml`:

- `interval` (67, counting the `all(..)` suite) — RUN. `cargo nextest
  run --archive-file nextest-interval.tar.zst` (`ci.yml:2888`) executes
  the interval archive. Only on the `interval` draw of the LANE axis,
  1 of 2 per run; the default lane compiles them and stops (`cargo test
  --no-run --workspace --features interval`, `ci.yml:1940`).
- `probe` (16, counting the same `all(..)` suite) — RUN.
  `scripts/k_probe_sweep.sh` (`ci.yml:3972`) invokes each rostered
  suite under `--test all`, and `scripts/gates/probe-suite-census.sh
  --check-executed` floors what it ran; all 16 are rostered in that
  script's `RUN_FLOOR`. Only on the `dev-probe` draw of the
  `klint_row` axis, 1 of 5.
- `budget` (2, both `crates/mesh`) — RUN, `cargo test -p mesh
  --features budget` (`ci.yml:3769`). Only on the `dev-budget` draw,
  1 of 5.
- `oracle-inari` (2) — RUN, `cargo test --release --features
  oracle-inari` (`ci.yml:3135`), but the `oracle-certify` job fires
  only for a non-`push` event whose diff touches
  `interval-transcendentals/` (`RUN_INTERVAL_ORACLE`,
  `scripts/ci-filter.py:1725`). A diff elsewhere never executes them.

`app` was the only feature whose suites executed under no draw at all.

The 5 suites carrying a loud-skip marker before this unit are disjoint
from the 87, and necessarily: a file-level `#![cfg(feature = …)]`
compiles the marker out with everything else, so a marker can only sit
in a file gated per-item.

**What the pattern could not match**: content made absent by a manifest
`required-features` rather than a source `cfg` (this crate's `[[bin]]
viewer` is exactly that); an item-level `#[cfg]` inside an ungated file
(`crates/topo/tests/review_m3_pr2.rs` and
`crates/geom-core/tests/k_stats_doors.rs` gate that way, and are absent
from the 87); a feature-gated `#[cfg(test)] mod` inside `src/` (the
shape of two of the three viewer hits); and an inner attribute not at
column 0.
