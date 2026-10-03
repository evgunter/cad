---
id: rigid-map-row-asks-a-sampled-residual-for-sub-ulp-agreement-at-eps-1e-12
kind: issue
title: rigid_map_near_eps_plane_nurbs asks the sampled on-locus residual to re-derive to 1e-6 relative, which at eps 1e-12 is 1e-18 m, under one ulp of a unit-scale point; red since #3524
status: dispatched
opened: 2026-10-01
priority: P2
cost: E
refs: [3524, 3737]
branch: reach/rigid-map-1e12
---


## What

`crates/topo/tests/rigid_map_near_eps_plane_nurbs.rs`,
`the_certificate_re_derives_within_rounding_under_the_map`, fails at
`CAD_TOLERANCE_EPS=1e-12`:

    (0, 0, 1) by 0.135: limb 1 and the tube are frame-invariant here

**Bisect** (the reviewer of #3737, first-parent on main): green on
`fe2447a440`, red on `ba06ed4bf3` (#3524's merge), red on main
(`9fb3de684`) and on #3737. It is a nightly row; no PR's gate ran it.

**Which conjunct fails.** Instrumented on #3737 at 1e-12, the four
claims of the combined assertion (`rel(image.on_locus_max,
seated.on_locus_max) < 1e-6`, the rung, the transversality, the box
count) read:

- `on_locus_max`: 9.666309e-13 against 9.666839e-13 seated, **relative
  5.48e-5** — the one that fails;
- rung 0.125 = 0.125; transversality relative 1.3e-16; boxes 32 = 32.

So the failure is the sampled residual, not limb 1's certificate. The
drift is 5.3e-17 m absolute, a quarter of one ulp at unit scale
(2.2e-16 m), and the fixture's points sit at unit scale. A rotation
re-rounds every sampled point, so the residual cannot re-derive to
better than about an ulp of the coordinates. The row's 1e-6 relative
ceiling is 1e-18 m at this ε: under the rounding of the quantity it
measures. #3524 changed the fit's arithmetic enough to move the sample
by that quarter ulp; at `fe2447a440` the row was green by its last
bits.

## The shape of a fix

State the on-locus claim against the sampler's rounding, absolute in
metres at the fixture's coordinate scale (a few ulps of the box), the
way the limb-2 drift is already stated against `DRIFT`. The default
and 1e-6 rows pass with large margin either way; only 1e-12 sees this.
