---
id: viewer-tests-each-spell-their-own-downward-pick-ray
kind: issue
title: Eight private spellings of the axis-aligned pick ray in crates/viewer/tests
status: open
opened: 2026-09-20
priority: P4
cost: E
---


## Finding

- **Where**: `crates/viewer/tests/`, eight private helpers in six
  files. `down_at(x, y)` at origin z = 1.0, byte-identical in
  `common/asm.rs`, `frame_policy.rs` and `select_pick.rs`; the same
  function at z = 0.5 as `down` in `review_gui2_r1.rs` and at z = 5.0
  as `down_at` in `review_gui2_r2.rs`; `up_at(x, y)` at z = −1.0,
  byte-identical in `common/asm.rs` and `review_gui4_r1.rs`; plus
  `select_pick::across_the_rim`, a fixed oblique ray and not a member.
- **What makes it a judgement**: the origin's height. Each spelling
  says "above anything THESE fixtures build", which is a claim about
  that suite's geometry, so the three z values are not a drift to
  reconcile — they are three fixtures. A shared door takes the height,
  or derives it from the document's bounds; deciding which is the work.
  The `up_at` pair is the easy half: two byte-identical copies of one
  ray, one of them already in `tests/common`.
- **Importance**: low-medium. No oracle: a wrong ray misses the body
  and reds the row that aimed it.
- **Instrument, and its blind spot**: a whole-function scan over every
  tracked `crates/viewer/tests/*.rs` for a body containing `Ray {`.
  It misses a ray built by a method or a `From` impl, one whose
  literal is written inline at a row's own site rather than in a
  helper, and any ray in another crate's suites.
- **Raised by**: the S-DUP lane closing
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `b29fe8bd1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner,
and one construction spelled eight times is S-DUP's charter. Any of the
five may claim it by `git mv`.
