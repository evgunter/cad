---
id: viewer-tests-rebuild-the-pick-index-in-every-suite
kind: issue
title: Fourteen private spellings of one PickIndex build in crates/viewer/tests
status: open
opened: 2026-09-20
---


## Finding

- **Where**: `crates/viewer/tests/`, fourteen private helpers in
  thirteen files, under six names — `index_of` (`blend_authoring`,
  `common/asm`, `edge_pick`, `focus_highlight`, `frame_policy`,
  `select_pick`), `fresh_index` (`index_memo`, `pick3_acceptance`,
  `review_pick2_r1`, `review_pick_r2`), `index_at` (`review_gui2_r1`,
  `review_gui2_r2`), `indexed` (`pick_windows`, `select_pick`).
- **The construction, which is one**: `session.landed_pair()`, then
  `session.landed_generation()`, then
  `PickIndex::build(doc, eval, PictureKey::of(generation, δ), session.tol())`.
  `crates/viewer/src/pane/viewport.rs` writes the same four arguments,
  so the shipped caller is a fifteenth site and the natural home is a
  `tests/common` door over it rather than a copy of it.
- **What makes the fold a judgement and not a substitution**: the δ.
  Each suite passes its own display tolerance and several take it from
  a private `delta()` chosen for that suite's geometry, so a shared
  door takes δ as a parameter and the per-suite value stays at the call
  site — the same split this row's parent settled for the fixtures.
  Three of the fourteen also open the session themselves
  (`pick_windows::indexed`, `select_pick::indexed`,
  `review_gui2_r1::landed`), which is a second door, not this one.
- **Importance**: medium. No oracle rides on it — a bug in the shared
  build would red every pick row in the crate, which is exactly the
  measurement the fold would need.
- **Instrument, and its blind spot**: `git grep -n 'PickIndex::build'`
  over every tracked file with no path argument, then a per-function
  read of each hit. It cannot see a site that reaches the index through
  `PickIndex::build_with` (`crates/viewer/src/evalseam.rs` does), nor
  one assembled by a macro.
- **Raised by**: the S-DUP lane closing
  `viewer-review-suite-fixtures-have-no-oracle-role`, 2026-09-20,
  measured at `b29fe8bd1`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled fourteen times
— is S-DUP's charter. Any of the five may claim it by `git mv`.
