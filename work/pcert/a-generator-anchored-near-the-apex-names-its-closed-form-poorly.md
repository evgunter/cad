---
id: a-generator-anchored-near-the-apex-names-its-closed-form-poorly
kind: issue
title: A cone generator whose line origin sits near the apex reads its ruling gate in band, so a true generator loses its closed form
status: open
opened: 2026-11-04
priority: P3
cost: M
---



Found in the review of PR 6504 (`pcert/one-ruling-reading`, fix list
AA7 s2).

## What

The cone's line arm (`derive_class`, `Curve3::Line` on
`Surface::Cone`) names its generator from the line's anchor: the
azimuth of `radial(origin − apex)` when the origin's height is
definite. Its ruling gate (`pcurve_cone_chart_ruling`, `ruling_miss`)
reads the line's distance from that generator over the span.

With the origin near the apex, `radial(origin − apex)` is short. Its
azimuth carries the origin's rounding over its length, and over a span
far from the origin the generator it names leaves the true one by that
angle times the span's reach. A true generator anchored a few ε above
the apex therefore reads its gate in band and takes the general lane,
not its closed form. That is a lost closed form, not a wrong answer:
the general lane certifies it or ends "not yet".

## Why not in PR 6504

The anchor is also check 4's. `incidence`'s `ConeRuling` terms
rebuild the generator from the same anchor (`anchored`, the origin's or
the direction's datum), and its `Line` term is levered by the span's
reach from the origin. Naming the generator from the span's farther
end instead changes check 4's lemma, and with it what a stored row
means, so it is its own unit.

## Fix

Name the generator from the span's farther end from the apex, in the
arm and in check 4's terms together. Re-derive the `ConeRuling` lemma
on `EnvelopeStatement::MapResidualClosedForm` for that anchor.

Row: a generator anchored at `2·zero`, `10·zero` and `1e-7` above the
apex, read over `[0.5, 2]`, keeps the closed form at both scalars.
