---
id: cone-apex-cap-refuses-degenerateface
kind: issue
title: props: a cone face bounded by one rim with the apex interior refuses DegenerateFace; its missing extreme is the apex, and the guard against its unbounded complement needs a sense bit fn cone does not take
status: open
opened: 2026-09-15
priority: P0
cost: H
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

## The shape the fold would take, and the one thing it does not carry

**The apex needs no σ.** The sphere's missing extreme is one of TWO
poles and σ picks between them, which is the whole content of
`sphere_rim_only_pole_level`. The cone's is the apex, level `0`, and
there is no second candidate — so the fold itself is the three lines
the spec asked about: push `T::zero()` when a generator-free cone
boundary's levels collapse, and the closed form
`sin α·Δu·|v_hi² − v_lo²|/2` measures the apex cap. (Corrected here on
the R2 review lane's reading, PR 2741: the first version of this filing
said σ was needed for the fold. It is not.)

**What is not free is the guard against the UNBOUNDED complement.**
The same rim traversed the other way bounds the rest of the nappe,
which runs to infinity and is no finite face of any solid — and with
the apex pushed unconditionally it would measure the apex cap's area
through the public `curved_face` door. That is the exact defect the
sphere arm's first landing shipped and the dual review caught
(`props_rim_only_closed`, PR 2741): an admitted input answered wrongly,
which the D2 addendum has no row for
(`d2-addendum-has-no-row-for-an-admitted-input-answered-wrongly`).

Deciding it needs the material side, i.e. σ, i.e. `Face::sense` — and
`fn cone` takes no sense bit, deliberately: a cone's flux needs no
material side at all, since generators run through the apex, so
`(p − apex)·n_chart = 0` and the anchored term vanishes. Inside a BODY
the inverted traversal is still caught, at tier 3's check 6, through
`boundary_material_sign`'s cone arm and `linear_rim_side`; at the
public door it is not caught at all. So the serving decision is
exactly: is the closed form allowed to answer a face whose orientation
only a body-level gate checks? The sphere arm answered "no" and paid
one extra decide for it, and this row should be taken the same way —
with `props_rim_only_closed`'s sibling on the cone (`Δu = τ` for an
apex cap) as part of the same change.

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
refusal is typed. **There is no recourse lane**, and a first version of
this filing said there was: `topo::props`' per-face dispatch routes
structurally on the carrier KIND, so only an `Ellipse`/`Nurbs`-trimmed
boundary or a spline chart enters `quad(…)`, and a circle-bounded cone
face refused by `curved_face` is refused, final. What row 2 costs here
is the whole answer, not a `pad > 0` enclosure. (R2 review lane, PR
2741; the same false sentence stood in issue 1250's imported text and
in `require_iso_rectangle`'s docs, and is corrected in both.)
