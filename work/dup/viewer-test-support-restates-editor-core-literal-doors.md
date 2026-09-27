---
id: viewer-test-support-restates-editor-core-literal-doors
kind: issue
title: viewer::test_support defines len, scl and ang a second time beside editor_core::test_support's
status: open
opened: 2026-09-26
priority: P4
cost: E
---



## Finding

- **Where**: `crates/viewer/src/test_support.rs` defines `len`, `scl`
  and `ang` — `Expr::literal(v, Dimension::X).expect(..)` over
  `pncad::document`'s re-export of the kernel's `Expr` — and
  `crates/editor-core/src/test_support.rs` now defines the same three
  over the same type. Two crates, one construction, two definitions.
- **Why it was not folded where it was found**: the lane that gave
  `editor_core::test_support` its literals was fenced out of
  `crates/viewer/` for the batch.
- **The fold, as far as it has been read**: viewer already reaches
  `editor_core::test_support` from its suites (a dev-dependency with
  `test-support` on, for the pick doors). For viewer's OWN
  `test_support` — which its `src` unit tests read too — to re-export
  editor-core's, viewer's `test-support` feature would forward
  `editor-core/test-support`, the shape `editor-core`'s manifest
  already uses for `profile` and `bvh`, and which
  `scripts/gates/test-features-dev-only.sh` permits (a test feature
  forwarding a test feature). `len_mm`, `len3`, `scl3`, `len2` and
  `scl2` stay viewer's: editor-core has no notation-keeping or
  array literal door. Not built or run; the manifest edge is the part
  to check first.
- **Importance**: low. No oracle; the two definitions are one call.
- **Raised by**: the S-DUP lane closing
  `cross-crate-inline-expr-literal-sites-outside-the-viewer-suites`,
  2026-09-26.

## Why this sits on S-DUP's slate

One construction defined twice is S-DUP's charter; `crates/viewer/`
has several claimants. Any may take it by `git mv`.
