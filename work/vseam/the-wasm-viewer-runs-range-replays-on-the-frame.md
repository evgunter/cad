---
id: the-wasm-viewer-runs-range-replays-on-the-frame
kind: issue
title: On wasm the viewer's inline range seam runs a range? replay inside the frame's pump
status: open
opened: 2026-11-23
priority: P2
cost: M
---


Found in review of PR 7548 (OFFER's `certify-affordance-on-the-bounds-panel`).
The twin of `work/coincide/the-wasm-viewer-runs-the-door-inside-the-landing-pump`.

`range?` runs the certified search on its own seam
(`crates/viewer/src/evalseam.rs`, `RangeService`). The native build
spawns `ThreadRanger`, a worker of its own. The wasm build
(`crates/viewer/src/app.rs` `ranger`) and every session built by
`DocSession::new` / `DocSession::inline` use `InlineRanger`, whose `poll`
builds the witness and then takes one step per call, and
`DocSession::pump` polls it once per frame. So on wasm every frame
while a run proves waits for one replay: a second or more while the
interval is wide, 20 s to 2 min near a width that certifies
(`crates/editor-core/src/range.rs`, What it costs). A run stops only
when the user stops it or it can no longer move, and a curved field
never finishes, so the frame stalls for as long as the run lasts.

That contradicts the unit's "off the frame". The fix is the door's: a
Worker-backed seam, or an inline slice that bounds a frame's share of
a replay. A replay takes a `Stop` and reads it between nodes, which a
slice could use.

Done when a wasm frame during a `range?` run draws without waiting for
a replay.
