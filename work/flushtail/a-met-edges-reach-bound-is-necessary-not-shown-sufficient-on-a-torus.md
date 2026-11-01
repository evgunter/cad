---
id: a-met-edges-reach-bound-is-necessary-not-shown-sufficient-on-a-torus
kind: issue
title: a met edge's reach bound on a curved root is necessary, not shown sufficient, on a torus: the near-bitangent shape off a near-full turn's top and bottom circles is unexercised
status: open
opened: 2026-11-01
priority: P3
cost: M
refs: [main-red-a-ray-about-a-partial-torus-rim-counts-its-ends-with-no-material-beyond, 6168]
---


Filed by FLUSH's PR 6168 fix pass (2026-11-01), from its FULL review.

## What

A met edge's half stands for a curved face's root far from where the ray
passes the edge only where `reaches_root_in_band`
(`crates/topo/src/boolean/solid_contain.rs`) says the ray reaches it in
band of the carrier. It reaches it when the rise off the carrier over the
distance, at the root's incidence, is not past the passage's own
clearance. On a convex carrier (a cylinder, a cone's nappe, a sphere) the
ray's distance is convex. A legitimate root reached from outside then
satisfies the bound (`|f'|·off ≤ f(passage)`), and a graze outside is
followed by no far root at all. So there the bound is both necessary and
sufficient for roots reached from outside.

On a torus the distance is not convex. The bound is necessary, but it is
**not shown sufficient**: a root might pass it and still be the ray's
crossing of the tube elsewhere. The argument offered is only that such a
root must be near-tangent, where `root_outward`'s incidence reads Zero
and the edge is graze-marked. That argument is not proved.

## Evidence so far

PR 6168's reviewer built two families to falsify it. Both read 0 wrong,
and both were set aside whole by `line_torus_roots`' Touch band, not by
the new bound. So neither exercises the bound:

- **(a)** 2,592 rays just inside the tube at a rim's top or bottom point,
  along the parallel's tangent, where the tube is parabolic
  (`f ≈ −δ + s⁴/16`), on 1.25π, 1.5π and 1.75π turns. Every ray was
  set aside at ε 1e-9, 1e-6 and 1e-12.
- **(b)** 4,800 rays on 1.9–1.99π turns, grazing one rim just outside
  the tube's bottom or top, crossing the missing wedge, and dipping
  near-tangent into the tube just past the other cap. Every ray was set
  aside at every ε.

Both are on `flush/torus-rim-review` @ `2a622422d`
(`review_probe_rays_inside_a_rims_parabolic_point`,
`review_probe_near_full_turn_tangent_entry_past_the_other_cap`).

## The unexercised shape

A ray near-bitangent to the tube's top or bottom circle on a near-full
turn, where the root past the other cap is near-tangent enough to pass
the bound but not so near that the Touch band takes it. Closing this row
takes one of two things:

- a proof that no torus root passes the bound and is another passage's
  crossing;
- or a family that reaches the shape, with the bound reading it right,
  or a narrowing of the bound where it does not.
