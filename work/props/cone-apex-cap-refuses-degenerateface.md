---
id: cone-apex-cap-refuses-degenerateface
kind: issue
title: props: a cone face bounded by one rim with the apex interior refuses DegenerateFace; its missing extreme is the apex, and the guard against its unbounded complement needs a sense bit fn cone does not take
status: open
opened: 2026-09-15
---


Filed by the PROPS sphere-pole-side unit (`docs/PROPS-SPHERE-POLE-SIDE-SPEC.md`,
the cone-apex deliverable), which served the sphere's rim-only polar cap
(issue 1250) and measured this sibling rather than serving it.

## The input, and the measurement

A cone face bounded by **one rim circle and nothing else**, the apex
interior to it — issue 1250's shape on the other singular chart. The
parse pushes one level (the rim's signed slant `v₀`), `min_max` gives
`lo == hi`, and `require_extent` refuses `DegenerateFace`. Executed on
the 45° cone about `+Z` with the rim at signed slant `1`, both
traversals:

```
apex cap, rim u 0→2π    REFUSE DegenerateFace
apex cap, rim u 2π→0    REFUSE DegenerateFace
```

(`crates/geom-brep/tests/props_sphere_pole_side.rs`, the row named
`the_cone_apex_cap_still_refuses_degenerate_face` — kept as this issue's
executed record.) `props_cone_nappe` does not catch it first: the extent
refusal comes before the nappe classify.

## Why the sphere's mechanism does not carry over

The sphere's missing extreme is one of **two** poles, and σ — the rim's
own `u`-traversal direction under the face's sense bit — picks between
them; that is the whole content of `sphere_rim_only_pole_level`. The
cone's missing extreme is the apex, level `0`, and there is no second
candidate: what σ would decide on a cone is not WHICH extreme but
WHETHER the face is the apex cap at all, its complement being the rest
of the nappe, unbounded. So the three-line version — push `T::zero()`
whenever a generator-free cone boundary's levels collapse — would
answer the apex cap correctly and answer the unbounded complement with
the cap's number, which is a wrong answer where there is a refusal
today.

The guard is the same predicate the sphere uses (`rim_interior_side`:
σ must point toward `0` from `v₀`), and it needs `Face::sense`.
`fn cone` does not take it, deliberately: a cone's flux needs no
material side at all — generators run through the apex, so
`(p − apex)·n_chart = 0` and the anchored term vanishes — and
`boundary_material_sign`'s cone arm reaches its side through
`linear_rim_side` instead. Serving this face therefore means giving
`fn cone` the sense bit and stating what that bit now underwrites,
which is a design change on the cone arm rather than a fold on its
levels.

## Not the cylinder's shape

A cylinder's rim-only face is **genuinely extent-less** and needs no
lane: the surface is unbounded along its axis in both directions, so
one rim circle bounds no finite face whichever way it is traversed and
there is no missing extreme for a traversal to name. Pinned as the
negative result in `a_cylinder_rim_only_face_is_extent_less`
(same suite). The cone is bounded on the apex side only, which is what
makes it this row rather than that one.

## Classification

D2 addendum **row 2**: reachable by input, valid, lane unbuilt — the
refusal is typed. The recourse today is the certified-quadrature lane
at the cost of a `pad > 0` enclosure, exactly as issue 1250's was.
