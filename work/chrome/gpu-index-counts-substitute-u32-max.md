---
id: gpu-index-counts-substitute-u32-max
kind: issue
title: gpu.rs substitutes u32::MAX for a draw count it could not convert
status: open
opened: 2026-09-12
priority: P3
cost: E
---

## Finding

`crates/viewer/src/gpu.rs`, two sites — the scene geometry's
`index_count` (~`:558`) and the edge geometry's `vertices` (~`:956`):

```rust
index_count: u32::try_from(scene.indices().len()).unwrap_or(u32::MAX),
```

`u32::MAX` is a count the function did not compute, handed to a draw
call in the shape of one it did. It is the same class as
`viewer-grid-pitch-nonfinite-fallback` (closed): a refusal spelled as
a plausible reading. The consequence is a draw over a range that does
not exist rather than a refusal a caller could act on.

**Unreachable in practice and filed anyway.** Four billion indices is
past what the pipeline could hold, so this is not a live defect; it is
here because the closing PR of the grid-pitch row disclosed it in its
census and `work/README.md` wants a disclosed residue to have a file
rather than a line in a merged PR body. The honest fix is a typed
refusal in a path that today has none, which is why it was not carried
in that PR.

## Fence

`crates/viewer/src/gpu.rs` — CHROME's and VIEW's by the territories
table.

## Parked on VIEW's index-buffer row, 2026-09-15

`gpu.rs` is ceded to VIEW under the carve-out, and this row's two sites
are not equally affected. VIEW's `scene-mesh-carries-an-identity-index-
buffer` proposes deleting `SceneMesh::indices` and swapping
`draw_indexed` for the `draw` the edge overlay already uses — **which
deletes the `index_count` site this row is about**. The `vertices` site
survives, and VIEW's replacement draw count needs the same conversion,
so that change MOVES this row rather than closing it.

Parked rather than left open because a row nobody intends to work
should not read as available on the board. The trigger is a real item
lint can see close. Whoever takes either should take both — neither
program can see that from its own slate alone.

## The trigger fired, and it minted the defect again (2026-09-15)

VIEW closed `scene-mesh-carries-an-identity-index-buffer` hours after
this row was parked on it, so `parked` became false and lint said so.
Re-opened. **What the trigger did is the point.**

The park note predicted: *"that change DELETES the `index_count` site
this row is about. The `vertices` site survives, and VIEW's replacement
draw count needs the same conversion, so that change MOVES this row
rather than closing it."*

That is exactly what happened. `SceneMesh::indices`, `draw_indexed` and
`set_index_buffer` are gone — `gpu.rs` now carries a test asserting
their absence — and the site this row cited went with them. In its
place:

```rust
fn corner_count(scene: &SceneMesh) -> u32 {
    u32::try_from(scene.positions().len()).unwrap_or(u32::MAX)
}
```

**The same substitution, one function further out, and now in the only
place the draw range is derived** — `corner_count`'s own doc says *"This
is the only place that number is derived, so the two passes over one
scene cannot draw different ranges of it."* Centralising the number
made this the single point where a failed conversion becomes a draw
count of `u32::MAX`, which is a larger exposure than the two sites it
replaced, not a smaller one.

So the population is unchanged at two, by subject rather than by line:
`gpu::corner_count`, and the `vertices` binding in the buffer-build
path. Both are `u32::try_from(...).unwrap_or(u32::MAX)`.

**This is the case for the note that rode the park.** A unit closing a
substitution minted one — `docs/REVIEW-STYLE-DISPATCH.md` §2's first
shape — in a diff whose author had no reason to know this row existed.
Neither program could see it from its own slate: VIEW was removing an
index buffer, CHROME was tracking a conversion.

Ground is ceded to VIEW under the 2026-09-15 carve-out, so CHROME does
not work it. Re-homing to VIEW is the obvious next step and is not taken
unilaterally here.

## The cession that held this row is spent (2026-09-21, orchestrator)

The paragraph above ends *"ground is ceded to VIEW under the carve-out,
so CHROME does not work it"*. That carve-out is retired
(`work/chrome/plan.md`, *Territory — the carve-out is spent*): VIEW
re-scoped on 2026-09-17 and dispatches no new units, and
`work/README.md`'s 2026-09-20 ruling makes shared ground legitimate
with awareness — not cession — as what is owed.

So nothing but scheduling holds this row now. **`crates/viewer/src/
scene.rs` and `gpu.rs` are VGEOM's as well as CHROME's since the
re-scope**, so the open question is no longer *may CHROME work this*
but *should this row sit on VGEOM's slate instead* — a `git mv` if so,
per `work/README.md`. That call is taken by the wave that takes the
row, not here.
