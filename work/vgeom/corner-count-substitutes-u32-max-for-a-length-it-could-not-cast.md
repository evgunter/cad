---
id: corner-count-substitutes-u32-max-for-a-length-it-could-not-cast
kind: issue
title: A corner count that does not fit u32 is drawn as u32::MAX rather than refused
status: closed
opened: 2026-09-16
priority: P1
cost: E
closed: 2026-09-21
branch: vgeom/refusal-floor
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

## Closed — one cast, refusing, at both sites

The cast is now `draw_range(len: usize) -> Option<u32>`, spelled once:
`corner_count` is `draw_range(scene.positions().len())` and the edge
geometry's `vertices` is `draw_range(positions.len())`. Its doc states
what it can answer and why `u32::MAX` is not it.

Both callers take the refusal the way each already has a nothing to
do: `ViewportRenderer::ensure_geometry` drops the held buffers and
uploads none, which is the state the passes already read as *there is
nothing to draw* (`read_id_at`'s `geometry.as_ref()?` and its
`corners == 0`), and `EdgePass::ensure_geometry` clears `held`, which
is the state an empty overlay already produces. **Nothing is uploaded
before the refusal**, so a scene that cannot be drawn does not pay for
its buffers either.

**This is a door hardening and not a fixed defect.** The row's own
reachability section is unchanged and is not upgraded by this unit:
`u32::MAX + 1` corners is `2^32 × 12 B = 51.5 GB` of positions before
normals, ids or flags, so the allocation bounds the count and not the
code, and nothing in the crate bounds it. No user-visible behaviour
changes here. What changes is that the door states the answer it can
give rather than substituting one.

**Row**: `crates/viewer/src/gpu.rs`,
`a_vertex_table_longer_than_a_draw_range_has_no_draw_range`. The
refused length is `u32::MAX + 1`, reached through `usize::try_from` so
a 32-bit target — which cannot express such a length at all — skips it
rather than failing; the pair is the lengths the two call sites really
produce, each of which has to come back as itself. `u32::MAX` is named
in the message because it is the value the row exists to exclude and
an `is_some()` assertion would have passed the unfixed door.

**Mutation**: restoring `unwrap_or(u32::MAX)` inside `draw_range` reds
that row and nothing else in the 798-row app-feature suite.

PR: `vgeom/refusal-floor`.
