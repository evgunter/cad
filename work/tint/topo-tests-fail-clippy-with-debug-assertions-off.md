---
id: topo-tests-fail-clippy-with-debug-assertions-off
kind: issue
title: crates/topo/tests/loop_reparenting_pcurve_rows.rs fails clippy -D warnings with debug assertions off (8 dead-code errors); CI lints only the debug-assertions-on config
status: open
opened: 2026-09-30
priority: P3
cost: E
---


## What

Two TOPO lanes, the revert-anchors unit (PR 3546) and its follow-up
(PR 3562), each linted with debug assertions off: they built with
`--config 'profile.dev.package.topo.debug-assertions=false'` and ran
`cargo clippy --all-targets -- -D warnings`.

Both met eight dead-code errors in
`crates/topo/tests/loop_reparenting_pcurve_rows.rs`, from helpers used
only under `#[cfg(debug_assertions)]`. They are not from either diff.

CI's lint job runs clippy in the default (debug-assertions-on) config
only. So the file is clean where CI looks and red in the config the
release-only rows (`review_d18`'s torn-body rows, the `corrupt input
(release profile)` job) build.

## Shape to consider

Either gate the helpers with the same `cfg` as their callers, or have
the lint job also run clippy with debug assertions off, for the crates
the release job tests.
