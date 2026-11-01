---
id: subdivision-width-row-has-no-row-that-fails-without-it
kind: issue
title: certified_subdivision's width row can be disabled and every topo and sweep row stays green
status: open
opened: 2026-10-11
priority: P3
cost: E
refs: [the-half-angle-ladder-certifies-in-band-configurations]
---



Found by the review of PR 4600 (GERM's half-angle meters) and
confirmed on its fix pass. In `crates/topo/src/boolean/circle_roots.rs`,
`certified_subdivision`, the width row (`rows.width`, e.g.
`bool_conic_quadric_sub_width`) answers `Uncertain` for a piece that is
neither clear nor monotone once it is narrower than the band's arc —
the guard the function's docs name for a double root. Mutated to
`if false && !definitely(rows.width, speed_hi * (r - l))`, the whole
topo suite (`cargo nextest run -p topo`, slow set included) and the
whole sweep suite (`-p sweep`, 2,597 rows) stay green at the default ε,
and so do the four `conic_quadric::in_band_sweep` rows (the review).

That is consistent with the PR 4600 measurement that a graze's in-band
stretch of arc (about `√(Kερ)`) is far wider than the band's arc: the
walk refuses first, because no split share reads a definite sign, or
runs into the walk's piece budget (`crate::bisect::PIECE_BUDGET`). So the row may be unreachable before the
other guards, or reachable only where nothing in the tree poses.

## What would close it

Either a pose (static, shape 2) whose answer moves when the width row
is disabled — a piece narrower than the band, neither clear nor
monotone, that the deeper walk would certify — pinned as a row; or an
argument that the split-share and budget guards subsume it, recorded
in the function's docs, and the row retired.
