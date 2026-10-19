---
id: two-first-harmonic-root-readers
kind: issue
title: Two first-harmonic root readers, geom-brep's pose::harmonic_roots and topo's circle_roots::first_harmonic_roots_in_band, decide an in-band extreme differently
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [5227]
---

From the stage (b) review residue (`curve-pair-reader-stage-b-review-residue` (closed, PR 5154), item 8), carried out of stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227).

Two readers solve the same question, the roots of `c₀ + A·cos(θ − φ)` along a conic:
- `geom_brep::curve_pair::pose::harmonic_roots`: extremes read through `roots::extreme_sign`. An extreme in the zero band is a touch, unless the root gap, read at the extreme less its noise, is definite. A definite straddle is two roots, read from the extreme that has crossed less.
- `topo::boolean::circle_roots::first_harmonic_roots_in_band`: the circle × surface doors' reader. It meters each root's slack against the band, reads the roots against the span, and charges phase noise. An extreme in band refuses `Unsettled::Tangency`.

**They decide an in-band extreme differently:** one reads a touch, the other refuses. The straddle policies differ too: `pose::constant` escalates a straddle, while `circle_roots::constant_residual_roots` refuses `Tangency`. Both now read `roots::spread_readings`.

**Why not in stage (c).** The topo reader carries span and slack meters the pose has no use for, and moving either onto the other moves every circle-root door row (387 topo conic and torus rows), so it needs its own unit. A route: one `geom_brep::roots` first-harmonic reader that answers the roots and a typed touch, with the span reading left to each caller (the pose's `Span::contact`, the doors' span rows).
