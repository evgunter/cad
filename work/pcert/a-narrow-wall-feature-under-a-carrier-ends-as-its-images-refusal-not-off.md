---
id: a-narrow-wall-feature-under-a-carrier-ends-as-its-images-refusal-not-off
kind: issue
title: A carrier off a spline wall at a feature of the wall narrower than the read's samples ends as its image's refusal, not read off
status: open
opened: 2026-11-11
priority: P3
cost: M
---

Filed by `pcert/stated-image-bounded` (PR 6815, review fix LL2), from the
review's probe on `review/6815-full` at `1827b743`
(`crates/geom-brep/src/pcurve_cache/review_6815_probes.rs`,
`wall_bump_under_a_line_f64` / `_interval`).

## What

A bilinear spline wall whose one interior `u` row (at `u = 1/128`, rows at
`0, 1/256, 1/128, 3/256, 1`) is pushed 1 mm off the plane, rational and
not, under a straight `Line` carrier on the plane, stated by its chart
image. Since PR 6815 the edge certificate does not certify it: the pcurve
certificate's envelope refuses. It ends `ChartImageUnavailable`, the
image's own refusal, not `Incidence(Off)` (`CarrierOffChart`).

That is sound but not the definite ending D4 ¶1 (iv) asks for. The
spline chart's read samples the even schedule and a spline carrier's own
breakpoints and Greville abscissae (`spline_incidence::own_fractions`).
A line has none, and the wall's feature lies between the even samples.

## Fix

Give the read the wall's own features too: the chart's knot lines and its
control rows' Greville lines, mapped to carrier parameters through the
stated image (or the foot), added to the samples the same way. Row: the
review's probe, ending `Incidence(Off)` at `f64` and `Interval`.
