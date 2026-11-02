---
id: the-snap-respells-the-charts-value-read-back
kind: issue
title: snap::snapped re-derives the value chart's spin and slide read-back in the viewer because the kernel's Chart has no public door
status: open
opened: 2026-11-02
priority: P3
cost: M
rides_with: mate-tool-commits-one-mate
---


Found by PLACING's second INTENT sweep (2026-11-02; Ev on PR 6281), read at `72c637cf98`.

`editor_core::mate::value::Chart` is `pub(crate)`, and no public door reads a bundle's values at a pose. So `snap::snapped` (`crates/viewer/src/session/snap.rs`) re-derives the spin (`atan2(to×copy·about, to·copy)`, with `about = ±d` by `flipped`) and the slides (`carried[a]·t`).

These match `Chart::spin` and `Chart::slide` term for term today, so there is no behaviour break; this was compared by reading, not run. `[placement.values]` is one chart, and `mate/turn.rs` declares itself the one home of these readers. A second spelling is a drift risk.

The bundle editor rewrites `snapped` (the "generalised snap", `mate-tool-commits-one-mate`'s build notes). When it does, it reads values back through a kernel door rather than re-spelling them. Opening that door is MSOLVE's ground (`mate/value.rs`), so the lane says so on MSOLVE's log.
