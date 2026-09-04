---
id: viewer-expresses-no-gpu-adapter-preference
kind: issue
title: The viewer expresses no backend or adapter preference, and a faulting ICD cannot be refused typed
status: open
opened: 2026-09-04
---


## Finding

`app::run` states its window intent explicitly and its GPU intent not
at all: `NativeOptions`'s remaining fields come from
`..Default::default()`, so `wgpu_options` is
`egui_wgpu::WgpuConfiguration::default()` and the app takes whatever
`RequestAdapterOptions::default()` returns. No backend is preferred,
no adapter is preferred, and there is no second attempt if the first
choice fails.

**The ingredient for a startup crash is demonstrated in the wild.**
Issue #1097's hardware run (Ev, 2026-09-04) reports that on that
machine wgpu finds and names an Intel Vulkan adapter whose path
**access-violates during device creation** — so the backend had to be
pinned to D3D12 to take a Vulkan reading at all. An access violation
inside a driver is not a catchable Rust error, so no typed refusal is
possible after the fact; the only lever is which adapter is asked
first.

**Not an observed crash, and the item should not be read as one.** On
that same machine the default selection picked D3D12 and the app ran
cleanly — every reading in #1097 was taken that way. What is
established is that a faulting ICD exists on a machine wgpu enumerates
it on, and that nothing in this crate expresses a preference that
would keep the app off it.

## Why this shape is already precedent here

`app::run`'s own comment argues exactly this case for the *window*:
a bare `NativeOptions::default()` "leaves resizability and the window's
size to whatever the winit backend negotiates", which on one real WM
produced an unusable window — so the intent is stated rather than
negotiated. The WSLg arm goes further and prefers X11 by hand when it
detects WSL. Both were added by this same first-light item. The GPU
half of the same argument was never made.

## What a taker decides

Whether to express a preference (a backend order, or
`PowerPreference`), whether to fall back to a second adapter when
device creation fails *recoverably* — noting that an AV is not
recoverable, so a fallback only helps for errors wgpu actually returns
— and whether a preference belongs in the app or in a documented
`WGPU_BACKEND` override for the operator.

## Home

`work/chrome/` — `crates/viewer/src/app.rs`'s `run` is this program's
ground.

Opened from issue #1097's hardware run (Ev, 2026-09-04).
