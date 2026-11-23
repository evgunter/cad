---
id: main-red-geom-core-test-support-fails-items-after-test-module-under-all-targets
kind: issue
title: Main red under clippy --all-targets: geom-core's src/test_support.rs fails items_after_test_module
status: open
opened: 2026-11-23
priority: P2
cost: E
---


Found by DOCTAIL's 7434 lane (2026-11-23) running `cargo clippy -p geom-core --all-targets -D warnings` on origin/main. Filed by the DOCTAIL orchestrator; not yet bisected.

## What

`crates/geom-core/src/test_support.rs` trips clippy's `items_after_test_module` under `--all-targets`. `-p geom-core --lib` is clean. If CI's lint job runs geom-core with `--all-targets`, main's lint is red.

## Done when

The module order in `test_support.rs` is fixed, so clippy `--all-targets -D warnings` is clean on geom-core, and the change that introduced it is named.
