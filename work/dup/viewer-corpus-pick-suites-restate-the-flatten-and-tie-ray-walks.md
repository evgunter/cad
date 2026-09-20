---
id: viewer-corpus-pick-suites-restate-the-flatten-and-tie-ray-walks
kind: issue
title: flatten, tie_rays_for and the answers walk are restated across four corpus pick suites
status: open
opened: 2026-09-20
---


## Finding

- **Where**: the four corpus pick suites in `crates/viewer/tests/` —
  `index_memo.rs`, `pick3_acceptance.rs`, `review_pick2_r1.rs`,
  `review_pick_r2.rs`.
  - `fn flatten(index: &PickIndex) -> Vec<FlatPart>` in
    `pick3_acceptance`, `review_pick2_r1` and `review_pick_r2`.
  - `fn tie_rays_for(index: &PickIndex)` in `index_memo` (answering
    `Vec<Ray>`), `pick3_acceptance` and `review_pick2_r1` (both
    answering `Vec<(Ray, f64)>`), plus `index_memo`'s `rays_for`,
    which is the same axis-and-diagonal enumeration without the tie.
  - the per-ray answer walk, `service_answers` in `index_memo` and
    `answers` in `pick3_acceptance`.
- **What makes the fold a judgement, not a substitution**: two of the
  suites are promoted review suites for the pick door, and the walks
  they restate are the SECOND implementation their rows compare the
  door against. `tie_rays_for`'s two return types are also a real
  difference (`Vec<Ray>` against `Vec<(Ray, f64)>`), so the reconcile
  comes before the move. The surviving per-row test applies: name the
  helper each row would otherwise read, and say whether a bug in it
  would be invisible.
- **Importance**: medium. Unlike the door classes closed beside it,
  this one carries an oracle — these ARE the hand-parallel walks — so a
  wrong fold here makes a suite compare the door against itself.
- **Instrument, and its blind spot**: `git grep -n 'fn flatten(\|fn
  tie_rays_for\|fn rays_for\|fn answers(\|fn service_answers('` over
  every tracked file, no path argument, then a read of each body. It is
  name-shaped and therefore structurally blind to a fifth copy under a
  new name; a whole-function scan over the four files is the second
  instrument this owes before a count is published.
- **Raised by**: the S-DUP lane closing the four viewer-suite door rows,
  2026-09-20, measured at `cd9fdfd6b`.

## Why this sits on S-DUP's slate

`crates/viewer/tests/` is claimed by `chrome`, `tcost`, `tint`, `vdoc`
and `view` (`work.py territory`), so there is no single ground-owner to
file it with, and the subject — one construction spelled more than once
— is S-DUP's charter. Any of the five may claim it by `git mv`.

