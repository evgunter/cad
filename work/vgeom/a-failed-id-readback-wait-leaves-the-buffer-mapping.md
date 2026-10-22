---
id: a-failed-id-readback-wait-leaves-the-buffer-mapping
kind: issue
title: A failed id-readback wait returns with the readback buffer's map still pending, and the next query maps it again
status: open
opened: 2026-10-22
priority: P3
cost: E
---

Found by `fit/id-answer-split`, which made a failed readback a typed
`idpass::ReadbackFault` rather than the clear value.

## The finding

`crates/viewer/src/gpu.rs`, `ViewportRenderer::read_id_at`, calls
`map_async` on `IdPass::readback` and then `device.poll(...)`. When the
poll fails (`ReadbackFault::Wait`), the function returns before
`unmap`, so the buffer's map request is still pending. The next id
query calls `map_async` on the same buffer again. wgpu refuses a map on
a buffer that is not idle, and it reports that as a validation error,
which the uncaptured-error handler may raise as a panic. The
`ReadbackFault::Map` arm may have the same hazard, depending on what
state the buffer is left in after a failed map.

**Not verified.** It needs a real adapter and a way to make the wait
fail, and this lane had neither (`gpu::tests::every_pass_builds_on_a_real_device`
is the crate's one device row). What has to be confirmed is wgpu's state machine
for a buffer whose wait failed: whether `unmap` on a pending map
aborts it cleanly (then the repair is an `unmap` on the fault path), or
whether the buffer has to be rebuilt.

## Stakes

The first fault is now said on the status line as the device's fault.
What this row asks is whether the query after it turns that sentence
into a crash.

## Fence

`crates/viewer/src/gpu.rs` (VGEOM, OFFER).
