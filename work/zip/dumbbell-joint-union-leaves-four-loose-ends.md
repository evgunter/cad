---
id: dumbbell-joint-union-leaves-four-loose-ends
kind: issue
title: A dumbbell's two halves unioned across a declared joint disc refuse Join(UnpairedLooseEnds { count: 4 }), torus or cylinder handle alike
status: open
opened: 2026-09-25
refs: [torus-operand-gate-admission]
priority: P0
cost: H
---

## What

Two halves of a dumbbell, each a full revolve about `y` (a bell of
radius 1.5 over `|y| ∈ [0.5, 1.5]` on a handle reaching the joint plane
`y = 0` at radius 0.3), pre-merged with `Body::merge_coplanar_faces`,
are unioned with the joint discs and the handle×handle pairs declared
`Rest`. It refuses `Join(UnpairedLooseEnds { count: 4 })`, and the
refusal is the same for both handles:

- a concave TORUS waist (`R = 0.8`, `r = 0.5`), whose two waists are
  one carrier meeting tangentially along the joint circle;
- the CONTROL, a straight cylinder of radius 0.3.

Pinned by `crates/sweep/tests/germ_torus_doors.rs`,
`the_torus_waisted_union_stops_at_the_join_like_the_cylinder_control`.

## Where it is raised (measured, instrumented)

`bool_connect` → `crates/topo/src/boolean/join.rs` (the leftovers
check at the end of the chord join) refuses `UnpairedLooseEnds`. The op
carries declarations and is a union, so `ops.rs` hands the reduction to
the declared-REST zip (`rest::try_rest_union`). For both handles the
zip declines at its segment enumeration (`enumerate_segments` answers
`None`: "matching did not complete — not this lane's frontier"), so the
join's refusal surfaces verbatim.

The joint is a pure REST contact (two coplanar opposite-oriented discs,
bounded by the joint circle) with the handle walls meeting along that
same circle. This is Ev's user-facing case (the dumbbell unioned in the
viewer, `work/germ/torus-operand-gate-admission.md`), and after the
GERM torus doors it is the next refusal whatever the handle.

## Home

ZIP (`join.rs`; `rest.rs` is shared with TANG).
