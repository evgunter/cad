---
id: pcurve-chart-box-is-looser-than-harmonic-extent
kind: issue
title: Pcurve::chart_box is p0 +- |pl|*max|t| - twice the true span of a Harmonic image and what the mint's check 5 reads
status: open
opened: 2026-09-06
refs: [torus-operand-boxes-span-whole-ring, 1907]
---


## What

Found by CURVED-TORUS PR-1's dual (both arms). `geom_brep::Pcurve::chart_box`
(`pcurve_cache.rs`) boxes a `Harmonic` image as `p0 ± |pl|·max(|t₀|,|t₁|)`
— twice the true span for an edge starting at `t₀ = 0`, and possibly on
the wrong side of `p0`. The torus box unit had to write its own
`harmonic_extent` (`hull(P(t₀),P(t₁)) ± hypot(pa,pb)·(t₁−t₀)²/8`) to get a
usable window, and pinned a row that the chart box is NOT what it reads.
`chart_box` is what the mint's own check 5 (`trim_containment`) consumes,
so the mint's containment check is looser than it needs to be.

## Fix

`chart_box` takes the span form (the torus unit's `harmonic_extent`
moves down into `pcurve_cache.rs` as the one home), and check 5's
row family re-measures. E on the arithmetic; the consequence for
check 5's thresholds is the measurement.

## Home

TRIM — `pcurve_cache.rs` is this program's.
