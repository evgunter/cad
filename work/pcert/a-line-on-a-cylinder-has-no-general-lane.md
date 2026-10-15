---
id: a-line-on-a-cylinder-has-no-general-lane
kind: issue
title: A line on a cylinder has no general (projected) lane, so an in-band ruling reading has no lane that does not rest on the answer
status: open
opened: 2026-10-15
priority: P3
cost: M
refs: [band-scale-torus-geometry-ends-as-a-kernel-defect]
---


## What

`projected::project` refuses `Curve3::Line` ("a line … has a
closed-form image, never a projected one"), so a line on a cylinder
has no general lane. Under the D3 lane-choice rule (`[ev]` PR 5113), an
in-band ruling reading (`pcurve_chart_radial_moving`) needs a lane
whose certificate does not rest on the class. The designers kept the
meridian form there, because its check-4 envelope is built from the
true difference of forms (`pcurve_cache.rs`, `derive_class`'s comment).
That made whether a closed form qualifies depend on how each form's
envelope is built: a per-class soundness argument.

## The question for Ev

Ev asked (5113, 2026-10-15): "for the cylinder, is that because there
is no more general form? … i don't see how this is consistent with the
other behavior." Recommended: in band always takes the general lane,
and every class a lane choice reads gets one. A line on a cylinder gets
a projected image (the chart inverse of a line is affine on each
piece); the D3 clause "or a closed form whose own envelope carries the
miss" goes; the meridian form is used only where the ruling is shown.
The D3 edit is in the `[ev]` PR that carries this question. Building the
lane joins unit E of the 5142/5113 implementation plan.

## The answer

Ev on `[ev]` PR 5186 (2026-10-15, "sounds good!"): in band always takes the general lane, and every class a lane choice reads has one. A line on a cylinder gets a projected image; the meridian form is used only where the ruling is shown. Built in unit E of the 5142/5113 implementation plan.
