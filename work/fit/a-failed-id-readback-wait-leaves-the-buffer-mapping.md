---
id: a-failed-id-readback-wait-leaves-the-buffer-mapping
kind: issue
title: A failed id readback leaves the readback buffer mapped or pending, and the next query's submit panics the UI thread
status: closed
closed: 2026-10-22
branch: fit/id-answer-split
pr: 5601
opened: 2026-10-22
priority: P1
cost: E
---

Found by `fit/id-answer-split`, which made a failed readback a typed
`idpass::ReadbackFault` rather than the clear value. Its mechanism was
corrected by that PR's review, which read the wgpu sources in the
lockfile (wgpu 30.0.0, wgpu-core 30.0.1, egui-wgpu 0.36.1).

## The finding

`crates/viewer/src/gpu.rs`, `ViewportRenderer::read_id_at`, called
`map_async` on `IdPass::readback`, then `device.poll(...)`, then
`get_mapped_range`, and unmapped only after a successful read. Both
fault arms returned through a `?` before the `unmap`:

- **`ReadbackFault::Wait`** left the buffer with its map pending: the
  client's `map_context.mapped_range` is `Some`, and the core
  `map_state` is `Waiting`.
- **`ReadbackFault::Map`** left the client's `mapped_range` `Some` too,
  so it was just as sticky. Every later query faulted again, for the
  rest of the session.

What follows is **not** a refused second map:

1. The next query's `map_async` is refused silently. The client sees
   `mapped_range.is_some()` and hands `Err(BufferAsyncError)` to the
   callback, and the callback (`|_| {}`) discarded it
   (`wgpu src/api/buffer.rs`, `BufferSlice::map_async`).
2. The fault comes earlier, at that query's `queue.submit`. The
   `copy_texture_to_buffer` into a buffer whose `map_state` is not
   `Idle` is `QueueSubmitError::BufferStillMapped`, which is a
   validation error (`wgpu-core src/device/queue.rs`).
3. No uncaptured-error handler is installed by the viewer, egui-wgpu
   or eframe, so wgpu's `default_error_handler` panics
   (`wgpu src/backend/wgpu_core.rs`).

So the first fault was said on the status line, and the next cursor
move crashed the app.

## Stakes

One transient device fault, such as a timed-out wait, killed the
session on the next hover.

## Closed — every readback exit leaves the buffer clean (fit/id-answer-split)

`read_id_at` now reads through `map_and_read` and then releases the
buffer by `release_after(&read)`:

- A read that succeeded unmaps (`Release::Unmap`). Its state is
  `Active`, so the unmap is valid.
- Every fault rebuilds the buffer (`Release::Rebuild`,
  `IdPass::readback`). Unmapping on a fault is wrong in one of the
  states a fault can leave: after a refused mapping the core state is
  `Idle`, and `buffer_unmap` on an `Idle` buffer is `NotMapped`, a
  validation error that panics the same way. Dropping a buffer is
  allowed in any state: `buffer_drop` ignores its unmap's result.

The map callback's outcome is no longer discarded. It goes through a
channel to `map_outcome`, and an `Err(BufferAsyncError)` becomes
`ReadbackFault::Map` in the device's words. A callback with no outcome
after the wait is also a `Map` fault. A mapping shorter than one id is
its own arm, `ReadbackFault::Short`.

**Headless rows** (`gpu::tests`):

- `every_readback_fault_rebuilds_the_buffer_and_only_a_read_unmaps_it`
  pins the release rule.
- `a_map_callback_that_did_not_succeed_is_the_readbacks_fault` pins
  that the callback's error is carried.
- `a_query_posts_its_fault_as_the_fault_under_its_serial` pins that
  `prepare`'s one posting site, `IdQuery::reply`, posts the fault as
  the fault.

**Needs a device.** That a real wait or map fault is followed by a
clean next query (no `BufferStillMapped` panic) needs an adapter and a
way to make the device fail. `every_pass_builds_on_a_real_device`
covers only the success path.
