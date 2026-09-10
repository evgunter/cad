---
id: cursor-projection-is-f32-in-a-module-whose-matrices-are-f64
kind: issue
title: cursor_projection's home argument names f64 doors for an f32 function, and the f64-to-f32 matrix cast it needs has four spellings and no home
status: open
opened: 2026-09-06
refs: [2089]
---



Found by the style review of #2089.

## The home argument is stated in types it does not share

`crates/viewer/src/camera.rs:878-881` is the whole of the new home
argument:

> the matrix it transforms is the one [`Camera::view_projection`]
> produces, and the cursor it takes is in the frame [`Camera::project`]
> answers in

Neither is true at the type level:

- `Camera::view_projection` (`camera.rs:658`) returns `[[f64; 4]; 4]`;
  `cursor_projection` (`camera.rs:914-918`) takes `&[[f32; 4]; 4]`.
- `Camera::project` (`camera.rs:672`) answers `Option<[f64; 3]>`;
  `cursor_projection` takes `cursor_ndc: [f32; 2]`.
- The PR body's third check — *"`ray_through` takes a cursor and a
  viewport the same way"* — is the loosest of the three.
  `Camera::ray_through` (`camera.rs:711`) takes `cursor_px: [f64; 2]`
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

- `crates/viewer/src/app.rs:1777` — `to_f32`, `pub(crate)`, so no test
  can reach it;
- `crates/viewer/tests/review_gui2_r1.rs:251`;
- `crates/viewer/tests/review_gui2_r2.rs:484`;
- `crates/viewer/tests/select_pick.rs:403`.

None of the four names the others. The matrix-times-point multiply is
in the same state: `camera.rs:681` inside `Camera::project`,
`select_pick.rs:452`'s `mul_point`, and an inline `apply` closure at
`review_gui2_r1.rs:267`, against `camera.rs:1011`'s `mul` for the `f64`
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
