---
id: pcurve-chart-box-is-looser-than-harmonic-extent
kind: issue
title: Pcurve::chart_box is p0 +- |pl|*max|t| - twice the true span of a Harmonic image and what the mint's check 5 reads
status: closed
opened: 2026-09-06
refs: [torus-operand-boxes-span-whole-ring, 1907]
priority: P1
cost: H
branch: pcert/chart-box-harmonic-extent
closed: 2026-10-01
pr: 3610
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

## Closed (PR 3610, 2026-10-01)

`Pcurve::harmonic_span_box` is the one home: the linear part's exact
hull plus the trig part's own meet of a chord bound and the `[−M, M]`
ball, monotone under span restriction (fuzz-checked in
`crates/geom-brep/tests/chart_box_span.rs`, which stays in the suite).
`chart_box`'s Harmonic and IsoLine arms, the torus window and
`chord_join`'s azimuth range read it; the torus unit's
`harmonic_extent` is gone. The row's premise about check 5 was wrong:
the mint's window is the hull of the same rows' `chart_box`, so check 5
is tautological at mint and at rest and no verdict moved (measured over
~32k rows); `split_cache` is its one non-tautological caller, which the
restriction monotonicity serves.
