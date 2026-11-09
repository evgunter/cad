---
id: a-covers-nearest-distance-from-a-line-is-a-squared-residual
kind: issue
title: geom::cover's nearest distance from a line site keeps the squared-distance form, ill-posed at f64 near the locus; the point site is re-posed as a distance, the line site has no closed form
status: closed
closed: 2026-11-09
pr: 6588
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

## Closed (2026-11-09, PR 6588)

Re-posed. `Arc::range` reads a line site through the arc's shadow
circle off the line: `p·cos t + q·sin t` splits exactly into a part
turning with `t` and one against it, each of constant length, and the
near and far are the point site's distance forms over the longer part
with the shorter as its departure. The squared form is deleted. Exact
for a circle about the line, and for a full turn of one in a plane
through it; a partial window of a circle in such a plane, and any
other pose, reads the annulus of the shadow's axes over the window,
sound and looser (a circle through the line read over `[1, 1.2]`
reads near `0` against a true `1.683`). Pinned by
`the_nearest_distance_from_a_line_just_off_the_locus_is_the_offset`
and `a_circle_about_the_line_reads_its_radius_in_either_sense`.
