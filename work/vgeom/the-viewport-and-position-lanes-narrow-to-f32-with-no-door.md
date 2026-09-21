---
id: the-viewport-and-position-lanes-narrow-to-f32-with-no-door
kind: issue
title: The remaining Rust/GPU float lanes are doored by their producers or not at all, and the f32 narrowing is doored nowhere
status: open
opened: 2026-09-17
refs: [2808]
---


Filed by the sweep the shader-mark-strength unit owed
(`the-shader-encodes-a-mark-strength-nothing-bounds`), which closed the
two members of its class that a type could close and is naming the
rest rather than leaving them in a PR body.

## The class

**A value crossing the Rust/GPU boundary with no door.** `crate::gpu`
is the crate's only `wgpu` module, so its two `Uniforms` constructions
and its six buffer fills are every write this crate makes. Two of the
float lanes were the unit's: `Mark::strength` and `Theme::ambient` are
`MixFraction`s now, bounded by their type. These are the others.

## The lanes, and what doors each

- **`ViewportSize::width_px` / `height_px`** — `pub f64` fields on a
  `pub` struct with no constructor (`crate::input`). They reach the
  GPU twice: as `edge.xy` in the shaded block, and as `viewport_px`
  inside `cursor_projection` for the id pass. Both `as f32`.
  Doored *incidentally and elsewhere*: `ViewportSize::aspect` answers
  `None` unless both are `> 0.0`, which a `NaN` fails, and
  `Camera::projection_matrix` refuses an aspect that is not finite —
  and `pane::viewport::viewport_ui` returns before building the
  callback in either case. So the production path admits neither
  today, and it admits them by two refusals sited for other reasons.
- **`pixels_per_point`** — `edge.z` is
  `EDGE_MARK_HALF_WIDTH_POINTS * pixels_per_point`, the toolkit's own
  `Context::pixels_per_point` as `f32`. Doored nowhere in this crate.
- **The `f64 → f32` narrowing itself**, which is the lane no guard
  above reaches. The threshold is `f32::MAX`, and the witness has to
  be a value the lane can hold: `viewport_ui` forms
  `f64::from(rect.width()) * pixels_per_point` from two `f32`-derived
  numbers, so the lane tops out near `f32::MAX * f32::MAX`
  ≈ `1.158e77` and a witness above that names a value the producer
  cannot make. **`f64::from(f32::MAX) * 2.0` is
  `6.805646932770577e38`** — finite, inside the lane, gives
  `aspect == 1.0` through both doors above, and narrows to `inf`
  (each figure executed). `vs_edge` then forms
  `offset = normal * side * edge.z / half_viewport`, which is `0` for
  an infinite half viewport and a `NaN` if `edge.z` is infinite too.
  The same narrowing has no door in `app::to_f32` (the
  view-projection) or at `scene.rs`'s `p.x as f32` (every scene
  position), and `EdgeOverlay`'s `pub Vec<[f32; 3]>` positions arrive
  already narrowed.

  **This row first named `1.0e308` and the review of #2808 corrected
  it.** The arithmetic was right in isolation — `1.0e308_f64 as f32`
  is `inf` — and wrong about the witness, because that value cannot
  appear in the lane the row is about. A reader re-deriving it would
  have found a producer that cannot reach it and distrusted the
  finding. Recorded rather than silently repointed: the defect and the
  threshold are what survived.

## Reachability, stated as a negative result

**No in-tree producer of a non-finite value in any of these lanes was
found.** Viewport dimensions come from an `egui::Rect` a window sizes;
scene positions come from the kernel's tessellation of a document.
That is a negative result about a search, not a proof: the search was
each lane's immediate producer, one step back, and the register's own
rule is that a value which is a number at every guard and stops being
one downstream is invisible to a sweep over guards.

## What a fix would have to decide

The `MixFraction` answer does not transfer: a viewport dimension has
no bounded domain to name, and the narrowing is a cast rather than a
field. The question is whether the boundary takes a door of its own
(one `fn lane(f64) -> Option<f32>` refusing a value the narrowing
turns into an infinity) or whether each producer states the bound —
and the first is one place where the second is five.

## Fence

`crates/viewer/src/gpu.rs`, `src/input.rs`, `src/scene.rs` — VGEOM's,
under the standing double claims with VIEW and CHROME. `src/app.rs`'s
`to_f32` is VSEAM's: the narrowing there is named above and a fix that
reaches it is announced, not assumed.
