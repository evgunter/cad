---
id: editor-core-suites-hand-build-the-world-xy-frame
kind: issue
title: Thirty-seven hand-built world xy frames in editor-core's suites beside fixture::xy_frame
status: open
opened: 2026-09-26
priority: P4
cost: D
---



## Finding

- **Where**: `crates/editor-core/tests/`, **37 world-xy frames in 23
  suites**, each a `Datum::Frame { origin: [0,0,0], u: [1,0,0], v:
  [0,1,0] }` written out field by field, beside
  `fixture::xy_frame()` (and `fixture::frame(origin, u, v)` for the
  general case) in the same binary. Largest: `seat7_sweep_lowering` 5,
  `m10_3_r1_probes_interval` 4, `m10_3_driver_interval` 3,
  `seat4_verb_lowering` 3.
- **Instrument, and its blind spot**: every `Datum::Frame {` in every
  tracked `.rs`, balanced to its closing brace, whitespace-stripped,
  with the literal doors (`len`/`scl` and the old private names)
  erased, then matched on the three world-xy component lists in either
  the per-component or the `[..].map(door)` spelling. It misses a frame
  at any other origin or axes (those are `fixture::frame` candidates,
  not `xy_frame`), a frame whose components are consts or params, and
  one built through a local helper that takes arrays. A wider textual
  count, `git grep -c 'Datum::Frame {'` over the suites, is **57 in 34
  files** and includes match patterns, so it is an upper bound, not a
  census.
- **Also outside the suites** (same instrument): `src/mate/member.rs`'s
  unit-test module 1 (no `xy_frame` it can reach — `test_support`
  would be the home), `crates/pncad/tests/all.rs` 1 and
  `crates/pncad-py/src/tests.rs` 3 (their own crates; see
  `facade-box-document-fixture-spelled-in-pncad-and-pncad-py`), and
  five in `demos/tour`, which are never converted.
- **Importance**: low. No oracle: the fixture and the longhand build
  one node.
- **Raised by**: the S-DUP lane closing
  `cross-crate-inline-expr-literal-sites-outside-the-viewer-suites`,
  2026-09-26, which folded these frames' literals onto `len`/`scl` and
  left the frames themselves: a different construction, and 23 more
  files than that PR could carry.

## Why this sits on S-DUP's slate

One construction spelled 37 times beside its own door is S-DUP's
charter; `crates/editor-core/tests/` has several claimants and no single
ground-owner. Any claimant may take it by `git mv`.
