---
id: the-front-probe-for-review-is-dead-without-the-app-feature
kind: issue
title: PickIndex::front_at_for_review is dead code in a test build without the app feature
status: review
pr: 6938
branch: placing/p3p4-cleanup
opened: 2026-11-14
priority: P4
cost: E
---


Found by `placing/editor-preview` while building the viewer's tests at
default features (`cargo test -p viewer --lib`).

`PickIndex::front_at_for_review` (`crates/viewer/src/pickindex.rs`) is
`#[cfg(test)]`, and its one caller is an app-feature test in
`crates/viewer/src/app.rs` (the `front_at_for_review` call in the
`properties_pane_tests`), so a test build without `--features app`
warns `method front_at_for_review is never used` (dead_code). No gate
builds that configuration with `-D warnings`, so it is a warning, not a
red.

**Owed.** Gate the method `#[cfg(all(test, feature = "app"))]`, as its
caller is. **Done when** `cargo test -p viewer --lib --no-run` builds
with no warning.

**Carried by `placing/p3p4-cleanup`** (PLACING's
`reviewer-probes-sit-in-the-shipped-properties-pane-tests`): the probe
that called the method only printed what it read, so it went with the
reviewer probes, and the method went with its last caller.
