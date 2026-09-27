---
id: facade-box-document-fixture-spelled-in-pncad-and-pncad-py
kind: issue
title: The one-box document fixture is spelled twice, in pncad's facade suite and in pncad-py's unit tests
status: open
opened: 2026-09-26
priority: P4
cost: D
---



## Finding

- **Where**: `crates/pncad/tests/all.rs`'s `doors_xy_frame`,
  `doors_square`, `doors_insert` and `doors_box_doc`, and
  `crates/pncad-py/src/tests.rs`'s `product_memo_rows::{xy_frame,
  square, insert, box_doc}`: the same four fixtures — the world xy
  frame, a `[0,s]²` chain on it, one `InsertNode` through `apply` with
  the refusing reach, and square(2) extruded 1.5 — written twice, line
  for line, through `pncad::document`.
- **Why it is a design question and not a fold**: the two are
  different crates' tests and neither can reach the other's. A shared
  home would be a `pncad` test-support surface (a feature-gated
  `doc(hidden)` module, as `editor-core` and `viewer` have), which the
  façade does not carry today and whose closure-property suite
  (`this_file_reaches_the_kernel_only_through_pncad`) is written from
  a consumer's seat. Whether the façade should grow one is the
  question; until it does, each crate keeping its own copy is the
  honest state.
- **Instrument**: found by reading, while folding the literal doors
  both copies use. Not swept for further copies of the same four
  fixtures in other crates.
- **Importance**: low. No oracle.
- **Raised by**: the S-DUP lane closing
  `cross-crate-inline-expr-literal-sites-outside-the-viewer-suites`,
  2026-09-26.

## Why this sits on S-DUP's slate

One construction spelled twice is S-DUP's charter, and the two sites sit
in two crates with no common owner.
