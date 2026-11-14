---
id: the-projected-speed-levers-restate-the-projected-frame-box
kind: issue
title: speed.rs's projected lever arms restate projected.rs's frame box, rho range and tube box, and its chart table restates the channel table
status: open
opened: 2026-11-14
---

Filed by TESSLANE's `one-chord-rule-over-each-boundary-images-speed-bound`,
whose fix pass found it. Not scheduled.

## What

`pcurve_cache/speed.rs` bounds a projected image's speed by the
carrier's `sup|C′|` over lower bounds on its lever arms (`Levers`:
`conic_levers`, `line_levers`, `net_levers`, `Levers::tube`).
`pcurve_cache/projected.rs` already encloses the same quantities per
piece for the image's box and certificate: `ProjectedImage::frame_box`,
`rho_range` and `tube_box`, and `SpanHull` carries `rho_lo`, `rho_hi`
and `z`. `Levers::tube`'s gap is the same expression as `tube_box`'s.
`projected_speeds`' chart table (`|∇u|`, `|∇v|` per chart) restates the
channel table `geom::ChartChannels` reads.

They differ in reach: the levers read a conic's closed form over the
span (exact on a circle of a sphere) and a net's control box over the
whole domain; the frame box reads per piece. Routing the levers through
the frame box needs the box per overlapped piece and a fold, more than
the fix pass that found this could take.

## Fix

Read the lever arms off `frame_box`/`rho_range`/`tube_box` over the
pieces the span overlaps, and the gradient norms beside the channel
table; or state why the speed bound needs its own enclosures.
