---
id: app-rs-lost-its-matrix-narrowing-when-the-seam-got-a-home
kind: issue
title: app::to_f32 was removed by vgeom/f32-seam when its last caller went, and app.rs now holds no display-seam helper
status: open
opened: 2026-09-21
priority: P4
cost: E
---



Filed by `vgeom/f32-seam` as the announced half of a crossing into
this program's ground, per `docs/prompts/implementer-discipline.md`
§6 and the Fence clause of
`work/vgeom/the-viewport-and-position-lanes-narrow-to-f32-with-no-door.md`
(*"`src/app.rs`'s `to_f32` is VSEAM's: the narrowing there is named
above and a fix that reaches it is announced, not assumed"*).

## What the crossing was

VGEOM's four `f32`-seam rows were taken as one question — where the
`f64` → `f32` conversion lives and whether it refuses — and the
answer is one home, `crate::narrowing::Narrow`, which refuses a value
whose narrowing is not a finite `f32`. `app::to_f32` was one of four
spellings of the matrix half of that conversion (the other three were
in `crates/viewer/tests/`), it was `pub(crate)` so no test could
reach it, and it did not refuse.

`crates/viewer/src/pane/viewport.rs` was its ONLY caller. That caller
now reads `Camera::view_projection_f32`, the camera's own door at the
seam, so `to_f32` had no caller left — and a `pub(crate)` function
with no caller is a `dead_code` warning under `-D warnings`. It was
therefore deleted in the same diff: nine lines, no behaviour of
`app.rs` changed, and the alternative was leaving the branch red.

## What is left for this program, and it may be nothing

The disposition this row wants is a yes or a no. Checked before
filing: after the change `rg -n 'as f32' crates/viewer/src` returns
`narrowing.rs` (the door's own single cast) and
`pane/features.rs:27`'s `usize as f32` indent step, and nothing under
`app.rs` or `session*.rs`. So this program's files hold no `f64` →
`f32` narrowing at all and there is nothing to re-home.

It is a row rather than a line in a PR body because a PR body is not
a slate: the crossing should be visible where this program reads, and
closing it is a read rather than a build.

## One citation this program owns, made stale by the same diff

`work/vdoc/every-crate-root-reexport-is-a-second-path-not-the-only-one.md`
(VDOC's, not this program's) lists `to_f32` among the imported names
in its six-`use`-statement receipt. The count is unchanged —
`pane/viewport.rs` still imports `ViewerBehavior` and `chrome` — and
only the name in the list is now wrong. Reported here and in the PR
body rather than edited across two fences.
