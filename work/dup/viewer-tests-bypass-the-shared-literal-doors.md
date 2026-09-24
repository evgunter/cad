---
id: viewer-tests-bypass-the-shared-literal-doors
kind: issue
title: Thirty inline Expr::literal spellings in crates/viewer/tests beside the common::len/scl/ang doors
status: open
opened: 2026-09-20
priority: P4
cost: E
---


## Finding

- **Where**: `crates/viewer/tests/`, thirty inline
  `Expr::literal(v, Dimension::{Length, Scalar, Angle})` spellings in
  fifteen files — `combine_ops` 6, `creation_ops` 3, `index_memo` 3,
  `blend_authoring`, `docm9_range_vs_probe`, `gesture_table`,
  `pick3_acceptance`, `review_pick2_r1`, `review_pick_r2` 2 each, and
  `cascade_delete`, `doc_io`, `docm1_face_frame`, `landing_gathers`,
  `path_authoring`, `pick_windows` 1 each. Split by dimension:
  Angle 15, Length 12, Scalar 3.
- **The home already exists**: `common::{len, scl, ang}`, which these
  suites' own binary mounts. The private WRAPPERS around the same call
  were folded on 2026-09-20 by the unit that measured this; what is
  left is the arm that writes the call out at the use site instead.
- **What makes the fold a judgement**: `docm9_range_vs_probe.rs` is
  `#![cfg(feature = "interval")]` and reaches `Expr` through
  `editor_core` rather than through the `pncad::document` façade, so
  its two sites are only type-checked in one lane and were left out of
  that unit deliberately. The rest are plain substitutions.
- **Importance**: low. No oracle: `common::len` and a hand-written
  `Expr::literal(_, Dimension::Length)` are the same call, and a bug in
  the shared door reds ninety-two rows (measured, 2026-09-20).
- **Instrument, and its blind spot**: a regex for
  `Expr::literal(<no comma>, Dimension::X)` over every tracked
  `crates/viewer/tests/*.rs`, `common/mod.rs` excluded as the home.
  It misses a literal whose value expression contains a comma
  (`Expr::literal(f(a, b), …)`), `Expr::literal_with_unit`, and every
  other crate's suites — the same class certainly runs wider, and
  `git grep -l 'Dimension::Length).expect'` alone names forty-five
  files outside this crate.
- **Raised by**: the S-DUP lane closing
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `b29fe8bd1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner,
and one call spelled thirty times beside its own door is S-DUP's
charter. Any of the five may claim it by `git mv`.
