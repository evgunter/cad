---
id: chart-region-short-loop-mixes-a-periodic-wrap-with-a-collapsed-loop
kind: issue
title: ChartRegionError::ShortLoop mixes a loop a periodic chart wraps in one or two iso-lines with a loop collapsed in its structure, so it aborts as mixed
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found splitting `ChartRegionError::DegenerateLoop` (`onecopy/section-error-standing`). Its orientation raising, the band deciding a loop's signed area `Zero`, stays `DegenerateLoop` and stands in band. Its structural raising, a loop imaged to fewer than three straight chart segments, is now `ShortLoop` (`crates/topo/src/chart_region.rs:321`, raised at `:2721`), and that aborts as mixed.

## What

`loop_uv_polygon` (`chart_region.rs:2658`) pushes one entry vertex per half-edge, so `ShortLoop` is any loop of one or two edges. That covers two cases:

- **A loop on a periodic chart closed by one or two iso-lines.** An example is a cylinder band's end circle, whose pcurve is one `IsoLine` across the full period. The body is valid, and the region representation is not built for it (row 2).
- **A loop of one or two straight segments on a plane.** Such a loop encloses nothing: a collapsed loop (1(a), or row 4 at a build, since tier 1 admits the loop).

## What closing it takes

Tell the two apart at the raise, by whether the chart is periodic and the loop's segments span its period. The wrap becomes a limit variant beside `SeamBranch`, and the collapse an abort. Then measure whether any census or ladder reach moves. The census today reads neither (`onecopy/section-error-standing`'s probe).
