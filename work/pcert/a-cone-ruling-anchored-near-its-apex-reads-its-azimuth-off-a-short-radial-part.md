---
id: a-cone-ruling-anchored-near-its-apex-reads-its-azimuth-off-a-short-radial-part
kind: issue
title: A cone ruling anchored near its apex reads its azimuth off a short radial part, a miss of about 1/d at ε 1e-9
status: open
opened: 2026-11-12
priority: P2
cost: M
---

Filed by `pcert/derived-sphere-image-accuracy` (PR 6870), from its FULL
review's M2. The PR first took the review's fix, then reverted it,
because it changes the class reading.

## What

`pcurve_cache.rs` `chart_image`, cone arm, ruling class: an anchored
line reads its azimuth (and its generator `ĝ`) off `r_ref = radial(w)`,
`w` the line's origin less the apex. Near the apex that part is
`d·sin α` long, but its error is an ulp of the coordinates (about 10 m
here). The azimuth shows at the span's far radius, so the image misses
by about `1/d`. In the review's probe at ε 1e-9, the miss is 1.9e-10 at
`d = 1e-5` and 8.2e-10 at `d = 1e-7`. At ε 1e-12 the line takes the
general lane or refuses. Measured on axis-aligned fixtures (an apex
about 3 m out, `α = 0.1`, `d = 0.25`), main's arithmetic leaves the
ruling's azimuth about 1e-14 off.

## Why the plane projection is not the fix

The review proposed reading `r_ref` in the plane of the axis and the
line (normal `axis × dir`). On a true ruling that is exact: every `d`
from 1e-3 to 1e-9 certifies at 1e-12 with miss ≤ 1e-14.

But the anchored reading names the generator **through the anchor**.
The projection replaces it with the generator along the line's own
horizontal direction, which for a line off its ruling is another
generator. `ruling_reading_rows`
`an_in_band_tilt_takes_the_general_lane_on_each_chart` turns a line
in-band off its generator in the tangent plane. Under the projection
its class reading leaves the band (`ImageMismatch` at both scalars, not
`Escalated` `ChartClass`). Two more rows go red at ε 1e-12 at
`Interval`:
- `a_true_ruling_keeps_the_closed_form_on_both_nappes` escalates at
  1.14e-12;
- `flat_cone_harmonic_images`
  `a_ruling_on_a_narrow_cone_encloses_its_envelope_tightly` reads a
  width of 3.33e-13, over its 2.66e-13 bar.

These were on PR 6870's CI run at `5425867d`.

## Direction

Keep the generator through a point of the line, but read it off a point
whose radial part is long: the span's far end, or the farther of the
anchor and the far end. For a ruling every point names the same
generator, so the C4 argument the derivation's comment makes ("where
the parameter starts moves nothing") holds. The nappe is decided at the
anchor, so a reference elsewhere must stay on that nappe over the span.
That is a class-reading design change; it needs its own unit, with
`ruling_reading_rows` as its gate.

## One reader, when fixed (PR 6870 delta review)

The derivation and check 4's incidence lemma read this arm's direction separately: the torus `side` against `w_r`, and the cone's `ĝ`. They agree today only because both are on main's reading. A fix must give them one shared reader, as PR 6870 did for the sphere with `sphere_meridian_direction`. Otherwise the mint door's false definite (PR 6870's M1) comes back.
