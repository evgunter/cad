---
id: corner-count-substitutes-u32-max-for-a-length-it-could-not-cast
kind: issue
title: A corner count that does not fit u32 is drawn as u32::MAX rather than refused
status: open
opened: 2026-09-16
priority: P1
cost: E
---



## Finding

From the same sweep as
`id-readback-failure-reads-as-nothing-under-the-cursor`, for the same
class: **a door that hands back a value it did not compute, in a field
shaped like one it did.**

Two sites in `crates/viewer/src/gpu.rs` turn a length into a draw
range by a cast whose failure arm is a number:

- `corner_count`, `u32::try_from(scene.positions().len())
  .unwrap_or(u32::MAX)` — the function's own doc calls it *"the only
  place that number is derived, so the two passes over one scene
  cannot draw different ranges of it"*, and a length that does not fit
  makes both passes draw a range that is not the scene's.
- the edge geometry's `let vertices = u32::try_from(positions.len())
  .unwrap_or(u32::MAX);`, stored beside the buffer it describes.

`u32::MAX` is a corner count neither call computed, in the field that
carries the one they did.

## The reachability, executed rather than asserted

**Not reachable on any machine this runs on, and that is a bound on
memory rather than on the code.** `u32::MAX + 1` corners is 2^32
positions; `SceneMesh`'s positions are three `f32` each, so the
position table alone is `2^32 * 12 B = 51.5 GB`, before normals, ids
or flags. `scene::TRIANGLE_BUDGET` is **not** a cap on this — its own
refusal sentence says *"A finer δ typed in the View pane is still
honoured — this is a starting point, not a cap"* — so nothing in the
crate bounds the count; the allocation does.

So this is a door hardening and not a defect, filed because the class
it belongs to is one this program is sweeping and an unreachable
member is still a member. The honest repair is that the cast refuses
loudly rather than that the number changes: a scene whose corner count
does not fit the draw call is a scene that cannot be drawn, and
`frame::scene_badge` is where that already gets said.

## Fence

`crates/viewer/src/gpu.rs` — VIEW's.
