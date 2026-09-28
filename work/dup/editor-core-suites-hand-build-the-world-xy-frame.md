---
id: editor-core-suites-hand-build-the-world-xy-frame
kind: issue
title: Thirty-seven hand-built world xy frames in editor-core's suites beside fixture::xy_frame
status: closed
opened: 2026-09-26
priority: P4
cost: D
closed: 2026-09-27
branch: dup/b7-b
pr: 3305
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
  left the frames themselves. That lane took them to be a different
  construction on 23 more files. In fact every one of those files was
  already in its diff.

## Why this sits on S-DUP's slate

One construction spelled 37 times beside its own door is S-DUP's
charter; `crates/editor-core/tests/` has several claimants and no single
ground-owner. Any claimant may take it by `git mv`.

## Closed 2026-09-27 — folded in the same PR that filed it

The premise that these frames were too many for that PR was false: all
23 files were already in its diff. The PR's review pass folded them.
`xy_frame` and `frame` moved into `editor_core::test_support` beside
the literals, so `src/mate/member.rs`'s unit tests reach them too;
`tests/fixture` and `viewer::test_support` re-export them.

- **37 of the 38 longhand world-xy frames** (36 in 22 suites and 1 in
  `member.rs`) are folded onto `xy_frame()`. `switch_slots` builds its
  frame as a bare `Datum` in a per-variant roster, not a node, and
  stays.
- **The second instrument** was aimed at what this row's could not
  see: a world frame spelled through the door's own general form. It
  found **31** `frame([0.0; 3], [1,0,0], [0,1,0])` calls (27 in
  editor-core, 4 in viewer's `datum_draw`), all folded onto
  `xy_frame()`.
- **Wrappers and adapters.**
  - Two wrappers that became the door under another name are deleted
    (`m10_2_measure::xy_frame`, `member.rs::frame_datum`).
  - Four `Recorder` adapters that reused the name `xy_frame` are
    renamed `insert_xy_frame`.
  - Four local copies of `frame`, and seven literal non-world frames,
    route through `frame(..)`.
- **Kept**: frames with a parameter or computed component, and the
  production frames in `viewer/src`.

The full census, its blind spots and the plants are in
`cross-crate-inline-expr-literal-sites-outside-the-viewer-suites`'s
Closed section.
