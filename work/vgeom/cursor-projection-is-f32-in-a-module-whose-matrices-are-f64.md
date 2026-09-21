---
id: cursor-projection-is-f32-in-a-module-whose-matrices-are-f64
kind: issue
title: cursor_projection's home argument names f64 doors for an f32 function, and the f64-to-f32 matrix cast it needs has four spellings and no home
status: dispatched
opened: 2026-09-06
refs: [2089]
priority: P1
cost: D
branch: vgeom/f32-seam
---



Found by the style review of #2089.

## The home argument is stated in types it does not share

`crates/viewer/src/camera.rs:878-881` is the whole of the new home
argument:

> the matrix it transforms is the one [`Camera::view_projection`]
> produces, and the cursor it takes is in the frame [`Camera::project`]
> answers in

Neither is true at the type level:

- `Camera::view_projection` (`camera.rs:652`) returns `[[f64; 4]; 4]`;
  `cursor_projection` (`camera.rs:908-912`) takes `&[[f32; 4]; 4]`.
- `Camera::project` (`camera.rs:666`) answers `Option<[f64; 3]>`;
  `cursor_projection` takes `cursor_ndc: [f32; 2]`.
- The PR body's third check — *"`ray_through` takes a cursor and a
  viewport the same way"* — is the loosest of the three.
  `Camera::ray_through` (`camera.rs:705`) takes `cursor_px: [f64; 2]`
  in **pixels**, `+y` down, plus a typed `crate::input::ViewportSize`;
  `cursor_projection` takes NDC, `+y` up, `f32`, and a bare
  `[f32; 2]`. Same nouns, three different spellings of each.

The claims are true up to a conversion — but the conversion happens two
modules away, in a driver: `crates/viewer/src/pane/viewport.rs:446`
calls `to_f32(&matrix)` and `:423-424` calls `viewport.ndc_of(cursor)`
then casts to `f32`. So `camera` is now the home of a function it
cannot feed from its own doors.

## The cast has four spellings and no home

The `f64` → `f32` 4×4 matrix cast is written out four times:

- `crates/viewer/src/app.rs:1839` — `to_f32`, `pub(crate)`, so no test
  can reach it;
- `crates/viewer/tests/review_gui2_r1.rs:251`;
- `crates/viewer/tests/review_gui2_r2.rs:484`;
- `crates/viewer/tests/select_pick.rs:403`.

None of the four names the others. The matrix-times-point multiply is
in the same state: `camera.rs:675` inside `Camera::project`,
`select_pick.rs:452`'s `mul_point`, and an inline `apply` closure at
`review_gui2_r1.rs:267`, against `camera.rs:1005`'s `mul` for the `f64`
matrix-matrix case.

`the-point3-to-gpu-corner-cast-is-at-three-sites` is the open row for
this class one dimension down (`Point3<f64>` corners), and it does not
count matrix casts. This is the same seam.

## Why the move sharpens it rather than causing it

Nothing here is new code and nothing is wrong at runtime. What the move
does is put the `f32` transform in the file that owns the `f64`
vocabulary, so the missing conversion door is now a gap **inside** one
module's public surface rather than a boundary between two. If
`camera` is the home of "the projection algebra", the cast from the
algebra's own output type to the GPU's is part of that algebra.

## Confidence

`sure` on every type and every site above. `likely` that a `to_f32`
with a home in `camera` is the shape that follows from the move's own
argument; `unsure` whether the doc sentences should be reworded or the
types should be made to agree.

## Closed

Closed by `vgeom/f32-seam`, which took this row with
`the-point3-to-gpu-corner-cast-is-at-three-sites`,
`the-viewport-and-position-lanes-narrow-to-f32-with-no-door` and
`the-one-free-transform-is-the-only-total-door-in-camera` as **one
question**: where the `f64` → `f32` conversion lives in this crate,
and whether it refuses.

**The answer: one home, `crate::narrowing::Narrow`, and it refuses.**
A single trait with a single method, implemented for `f64`, for
`[T; N]` where `T` narrows (which covers a pair, a triple and a
column-major 4x4 matrix in one impl) and for `Point3<f64>`. It
answers `None` when the RESULT is not a finite `f32` — a test on the
narrowed value rather than on the input, because `f32::MAX` is about
`3.40e38` and a finite `f64` is what turns into an infinity. The
module holds the crate's one `as f32`.

No second door was minted. `Camera::view_projection_f32` and
`SceneMesh::build` do not re-decide the conversion; they call it and
say what a refusal means where they stand.

**The four spellings are zero.** The matrix cast is now
`Camera::view_projection_f32` — `Camera::view_projection` narrowed
through the one door, refusing `CameraError::UndrawableProjection`.
`app::to_f32` had that one caller and is deleted (announced to VSEAM,
whose row is
`work/vseam/app-rs-lost-its-matrix-narrowing-when-the-seam-got-a-home`),
and all three test spellings are gone because the tests now call the
camera door. `rg -n 'as f32' crates/viewer/tests` returns nothing.

**The populations this row stated had both moved.** Its four matrix
spellings were `app.rs:1839`, `review_gui2_r1.rs:251`,
`review_gui2_r2.rs:484` and `select_pick.rs:403`. On `origin/main` at
`5cc1db9d` they were `app.rs:2231`, `review_gui2_r1.rs:209`,
`review_gui2_r2.rs:463` and `select_pick.rs:412` — and
`review_gui2_r2.rs:484` was a DIFFERENT subject by then
(`let v = [p.x as f32, …]`, a point cast). The count of four was still
right; three of the four numbers were not.

**The home argument now states its types, and does not claim they
agree.** `cursor_projection` still takes `f32` and the header says
why, which is the half the row left `unsure`: the matrix it transforms
has to be the one the GPU is actually rasterizing with, so handing it
the `f64` original would make the id pass compute with a matrix the
shaded pass does not have — the divergence `idpass::disagreement`
exists to report. What made the argument false was the module having
no door that PRODUCED that value; it has one now.

**The matrix-times-point multiply** the row names as the same class
one step on is untouched and is not this unit's: `camera.rs`'s `mul`,
`select_pick.rs`'s `mul_point` and `review_gui2_r1.rs`'s `apply`
closure all remain. Their inputs no longer carry a hand-rolled cast,
which was the seam half; sharing the multiply is a separate question
and no row was filed for it, because the two test copies are eight
lines of loop in two review suites and `work/README.md`'s *the
tracker is not comprehensive* covers exactly that.
