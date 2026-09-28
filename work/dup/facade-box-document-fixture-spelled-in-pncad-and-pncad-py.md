---
id: facade-box-document-fixture-spelled-in-pncad-and-pncad-py
kind: issue
title: The one-box document fixture is spelled twice, in pncad's facade suite and in pncad-py's unit tests
status: closed
opened: 2026-09-26
priority: P4
cost: D
closed: 2026-09-28
---



## Finding

- **Where**: `crates/pncad/tests/all.rs`'s `doors_xy_frame`,
  `doors_square`, `doors_insert` and `doors_box_doc`, and
  `crates/pncad-py/src/tests.rs`'s `product_memo_rows::{xy_frame,
  square, insert, box_doc}`: the same four fixtures — the world xy
  frame, a `[0,s]²` chain on it, one `InsertNode` through `apply` with
  the refusing reach, and square(2) extruded 1.5 — written twice, line
  for line, through `pncad::document`.
- **And the literal pair under them**: `crates/pncad/tests/all.rs`'s
  file-level `len` / `scl` and `crates/pncad-py/src/tests.rs`'s — the
  same `Expr::literal(v, Dimension::{Length, Scalar}).expect(..)` pair,
  spelled through `pncad::document`, one per crate. They are members of
  this row: whatever home the fixtures get, the pair goes with them (and
  if that home is `editor_core::test_support` re-exported, as `viewer`
  does, the pair is already there).
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

## Closed — copies kept, pinned by `box_document_fixture_twins_agree`

Ev's ruling (2026-09-28): keep the two copies and pin them against
drift. `crates/pncad/tests/all.rs` and `crates/pncad-py/src/tests.rs`
each carry the four fixtures and the `len`/`scl` pair as ONE text, at
file level, between `// BEGIN box-document fixture twin` and its `END`
marker: `xy_frame`, `square`, `insert` and `box_doc(label) -> (doc,
profile id, body id)`, every path spelled from `pncad::` with no import
from the file around it, so nothing needs normalising.
`box_document_fixture_twins_agree`, in pncad's suite, reads its own
file and the sibling's through `test_utils::source::{crate_dir,
sentinel_region}` and fails on any byte of difference, naming both
files, the first differing line in each, and saying to edit both
together. `scripts/ci-filter.py` reads the relative path as a read edge,
so a diff touching only the pncad-py file puts `pncad` in the run.

Folded onto the pinned fixture in the same pass: pncad's
`unit_vector_witness_through_the_facade` and
`the_hollowed_box_through_the_facade` module-local `insert`s (and the
latter's hand-spelled xy frame), and pncad-py's
`resolution_status_tags_are_stable` insert closure and xy frame and
`the_load_door_reaches_dimension_mismatch_arms_as_a_typed_dimension_refusal`'s
xy frame. `square_at` (pncad, the offset square) stays a separate
fixture: it is the chain generalised by an origin, not a copy.
