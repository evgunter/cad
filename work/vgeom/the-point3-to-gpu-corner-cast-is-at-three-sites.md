---
id: the-point3-to-gpu-corner-cast-is-at-three-sites
kind: issue
title: three sites cast a Point3 to a GPU corner and the prose reconciling them names two
status: closed
opened: 2026-09-06
refs: [2083]
priority: P1
cost: E
branch: vgeom/f32-seam
pr: 3030
closed: 2026-09-21
---


Found by the style review of #2083 (pre-existing; the move carried the
reconciling sentence into a new file, where it now understates the
count).

`crates/viewer/src/marks.rs:131-133` — `EdgeOverlay`'s doc — says *"The
buffers are `f32` because that is what a GPU consumes and this is the
display seam — the same cast, at the same boundary, that
[`crate::scene::SceneMesh`] makes."* That names two sites. There are
three:

- `crates/viewer/src/marks.rs:300` —
  `|point: &Point3<f64>| [point.x as f32, point.y as f32, point.z as f32]`
- `crates/viewer/src/scene.rs:526` —
  `positions.push([p.x as f32, p.y as f32, p.z as f32])`
- `crates/viewer/src/pane/viewport.rs:311` —
  `.push([world.x as f32, world.y as f32, world.z as f32])`

The third is not mentioned anywhere. This is
`docs/prompts/reviewer-style-lane.md` Q2's *"comment that exists to
reconcile two spellings of one rule"* shape: the sentence is the only
thing holding the three in correspondence, the code compiles either
way, and the sentence is already wrong about how many there are.

Whether the cast wants a named door (`Point3<f64> -> [f32; 3]` at the
display seam, once) is the question; there is no constant or tolerance
involved, only a repeated three-field cast, so the cost of a door is
small and the cost of the current arrangement is a sentence that has
to be maintained by hand.

## Where else to look

`grep -rn 'as f32' crates/viewer/src` — the same seam is crossed by the
normal and by the `[f32; 4]` colour paths, and neither is covered by
the sentence.

## Confidence

`sure` on the three sites and on the prose naming two. `unsure` whether
a shared door is worth it.

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

**The population was three and is five, in two shapes.** Re-derived on
`origin/main` at `5cc1db9d` with `rg -n 'as f32' crates/viewer/src`:

| site | shape | disposition |
|---|---|---|
| `marks.rs`'s `segments_of` corner closure | `Point3<f64>` | `Narrow` |
| `scene.rs`'s `positions.push` | `Point3<f64>` | `Narrow`, refusing the scene |
| `pane/viewport.rs`'s `push_segment` | `Point3<f64>` | `Narrow`, dropping the leg |
| `pane/viewport.rs`'s datum `.push([point[0] as f32, …])` | `[f64; 3]` | `Narrow` — a FIFTH member, not a fourth: the source is an array, so a `Point3`-shaped sweep cannot see it |
| `scene.rs`'s `triangle_normal` | `[f64; 3]`, a DIRECTION | `Narrow`, keeping the existing degenerate fallback |

The last is the one the pattern matches and the property does not: a
unit normal's components are within `±1` by construction (`len` is at
least the largest `|n[i]|`), so the narrowing there cannot refuse. It
goes through the door anyway, so the crate has one spelling, and
`triangle_normal`'s doc now says why its fallback is right where a
position's refusal is.

**Two sites the row's `## Where else to look` named are NOT members**,
and saying so is the receipt: `viewport_px` and `pixels_per_point`
(`pane/viewport.rs`, two and one sites) are the narrowing row's scalar
lanes, not corners, and they cross the same door; the `[f32; 4]`
colour path the row also names carries `MixFraction`s, bounded by
their type since #2808, and never touches an `f64`.

**The sentence that held them in correspondence is gone**, which was
the row's Q2 half. `EdgeOverlay`'s doc no longer names
`crate::scene::SceneMesh` as *"the same cast, at the same boundary"*;
it names the door. A sentence that has to be maintained by hand was
wrong about the count on the day it was written and would have been
wrong again at five.

**What the sweep could not match.** `rg 'as f32'` is a text pattern
over a cast between two float types, and it cannot tell
`pane/features.rs:27`'s `usize as f32` indent step from a narrowing.
That site is the one remaining `as f32` under `crates/viewer/src`
besides the door's own, and it is not this seam. No mechanical guard
holds the one-home claim for that reason, and `narrowing.rs`'s header
says so at the claim site.
