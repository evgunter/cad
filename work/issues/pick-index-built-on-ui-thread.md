---
id: pick-index-built-on-ui-thread
kind: issue
title: The pick index is built on the UI thread, so a landing that costs seconds is a frame that costs seconds
status: open
opened: 2026-08-29
github: 1259
refs: [1217, 1247]
---

## From GitHub issue 1259

opened 2026-08-29, 0 comments.

## What happens

`ViewerApp::sync_scene` runs at the top of `eframe::App::ui`, and inside it `PickCache::sync` calls `PickIndex::build`, which per product root runs `NodePick::build_all` → `mesh::tessellate` + `MeshPick::build` (the triangle BVH). All of it on the UI thread. While it runs the window does not repaint and still shows the previous document.

GUI-3 moved the **evaluation** off-thread — `EvalService`, `Inline` + `Thread`, cancel-and-restart, per-job `CancelToken`, the busy/canceled chrome. The tessellation and index that follow it did not come with it, so the seam that exists to keep the frame loop responsive stops one step short of the expensive step.

## What it costs

Measured on a 2.8 GHz Xeon, release, after the median-partition BVH build ([#1217](https://github.com/evgunter/cad/pull/1217)) and with the display budget ([#1247](https://github.com/evgunter/cad/pull/1247)) choosing δ:

| | triangles | tessellate | index total |
|---|---|---|---|
| `hollowring` at δ = 0.1 mm (what it asked for before the budget) | 3 984 276 | 6.5 s | 13.4 s |
| `hollowring` at δ = 0.400 mm (what the budget opens it at) | 998 576 | — | ~2.3 s |

End to end, Open… → the new document on screen went 25 s → 16 s (the BVH partition) → 8 s (the budget). **The remaining 8 s is mostly this**, and the budget cannot take it further without giving up picture quality it should not have to trade.

The budget also only binds when a document ARRIVES, deliberately (#1247's design: it sets a default, it is not a cap), so an edit that makes a document much denser at the δ in force reaches the UI thread with nothing in front of it.

## Shape of the fix

The worker returns the index with the run; a δ change re-submits. `PickCache`'s retry policy ("at most one attempt per (generation, δ), a refusal is kept and readable rather than retried into a stall") moves with it, and a δ change while a build is in flight wants the same cancel-and-restart the evaluation already has.

Two things already exist that make this less of a leap than it sounds: an `Evaluation` already crosses the seam, so the payloads are `Send`; and the chrome already knows how to draw a picture older than the document — that is the `canceled — showing an older result` state, spinner, Cancel and Re-evaluate included.

## Why this is filed and not done

It changes the seam GUI-3 §5 ratified, and the §5 re-take ("GO ON EGUI, AUTHORITATIVE") rests on a complete frame-state inventory that this would extend — an index that lands asynchronously is new frame state with a new staleness rule. That wants a ruling, not a commit.

## Home

`work/issues/` — the GUI-3 §5 seam is GUI-era ground and that program is closed; PERF's keep_out cedes per-frame rendering and hover-picking to the viewer.
