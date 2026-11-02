---
id: a-covers-nearest-distance-from-a-line-is-a-squared-residual
kind: issue
title: geom::cover's nearest distance from a line site keeps the squared-distance form, ill-posed at f64 near the locus; the point site is re-posed as a distance, the line site has no closed form
status: open
opened: 2026-11-02
priority: P3
cost: M
refs: [cover-geom-readings]
---


Found by PR 6383's delta review and its second fix pass (2026-11-02).

`geom::cover`'s `distance_range(Site::Line)` nearest bound, for arcs
and the tube, still reads squared-distance harmonics. At f64 that
cancels near the locus, with error growing like `ε·scale²/near`: the
shape `crates/geom-core/README.md` "Float rounding" says is re-posed as
a distance, never charged. The point site was re-posed: `Arc::point_near`
and the tube's spine-circle form. The line site was not, because the
distance from a line to a circle is a quartic's root and has no closed
form.

No production reader takes a line site's nearest bound today. The
blend's `sheet_clip` is the likely first, in `cover-app-readers`.
Either re-pose it (a certified quartic root, or a bound posed as a
distance), or make the line-site near unavailable until a reader needs
it, so nothing reads an ill-posed value.
