---
id: a-swaying-loft-corner-refuses-as-a-vanishing-span
kind: issue
title: A loft whose corner path sways sideways refuses at certification as a vanishingly short edge: the span meter's chord bound collapses on a sound curve
status: closed
opened: 2026-10-07
priority: P3
cost: M
branch: nurbs/swaying-loft-span-meter
closed: 2026-10-10
refs: [nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one]
---


## What

A `Node::Loft` through four 2 × 2 squares at z = 0, 1, 2, 3 whose x
offsets alternate `−a, a, −a, a`, skinned at v-degree 2, builds at
`a = 0.25` and `a = 0.5` and refuses from `a = 0.75` up:
`Loft(Euler(Certification { Escalated { check: ParamSpan, cause:
Indeterminate { margin: −0.67…, predicate: "nurbs_span_meter" } } }))`,
with margins −3.05, −5.70, −8.49 and −14.2 at `a` = 1, 1.25, 1.5, 2.
At v-degree 3 the same four sections refuse already at `a = 0.5`
(margin −2.53). Each corner edge is a smooth curve some 3 m long that
never stalls; the refusal says "its length is too close to zero to
decide at this tolerance".

`geom_brep::certify`'s NURBS span arm (`run_checks`, the
`Curve3::Nurbs` arm) gates `Margin::metered(d1 − d0,
speed_lower_bound())` definitely positive. `NurbsCurve3::speed_lower_bound`
(`crates/geom/src/curves/nurbs.rs`) is a control-polygon bound
projected on chord directions; a corner that sways across its chord
has derivative coefficients pointing back along every chord it tries,
so the bound goes negative though the curve's speed does not. The
refusal is the gate's sound answer to the bound it was handed, not a
wrong certificate; what is wrong is that a plain wavy loft cannot be
built, and that the sentence blames the edge's length.

The misrouted sentence is the class
`work/encl/certify-collapsed-arm-gates-route-as-the-decision-they-guard.md`
names; the reversed-domain half is
`nurbs-span-meter-cannot-tell-a-reversed-domain-from-a-collapsed-one`.
What this row adds is the reach: a sound curve the meter cannot bound.
The meter has a global and a per-span assembly; a finer one (chords
over refined sub-spans where both collapse) is one way to let it
answer.

Found building the stand-in body for
`work/emit/a-crossing-of-a-nurbs-edge-ties-for-want-of-its-parameter.md`
(`emit_topo`'s `nurbs_crossings_rank_by_parameter` rows use `a = 0.25`).

## Closed

`NurbsCurve3::speed_lower_bound`'s integral arm gained a third
assembly, `piece_assembly`: every nonempty span cut into
`INTEGRAL_METER_SPLITS` (16) pieces, each projecting the derivative's
Bernstein coefficients on that piece (blossoms of the derivative
spline, formed at `T` from the stored net, so the interval lane
encloses them) onto their sum's direction. It joins the global and
per-span chord assemblies by `max`, so no carrier's bound went down.
The loft family `a ∈ {0.75, 1, 1.5, 2}` at v-degree 2 and `a = 0.5` at
v-degree 3 now builds closed
(`sweep/tests/a_swaying_loft_corner_meters.rs`); on the interpolated
swaying corner the meter reads 0.57–0.99 of the true minimum speed,
never above it, and a randomized set of nets that stall exactly still
refuses (`geom/tests/curves/swaying_corner_meter.rs`). The refusal's
routing is no longer this row's: ENCL's `SpanMeterCollapsed` already
names the meter, not the edge's length.
