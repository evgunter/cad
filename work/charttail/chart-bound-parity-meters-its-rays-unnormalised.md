---
id: chart-bound-parity-meters-its-rays-unnormalised
kind: issue
title: chart_bound::parity casts SCHEDULE_2D's members unnormalised, so its side and advance margins read up to 1.25x the metres they claim
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Unmeasured: no fixture is known to reach a verdict it moves. Found by
the sweep of CLEAVE's `cleave/ray-walk` branch, which put
`chart_bound::parity` on the shared walk driver (`ray_walk::walk`)
and left its frame alone, since a reader keeps its own frame.

`chart_bound::parity` (`crates/topo/src/chart_bound.rs`) casts each
`SCHEDULE_2D` member as it is in the table, `d = m`, with
`side = (−d.y, d.x)`. The members are not unit vectors — `(1.0, 0.75)`
has length 1.25, `(0.5, 1.0)` 1.118 — and `ray_walk::ray_crossings`
reads a vertex's side as `w·side` and a crossing's advance as
`(xᵢyⱼ − xⱼyᵢ)/(yⱼ − yᵢ)` over `xs = w·d`: each is the metres it claims
times `|d|`. So `chart_bound_side` and `chart_bound_advance` decide a
margin up to 1.25× the true one, and a vertex truly inside the band of
a ray line can read definitely off it.
`docs/predicate-dimension-audit.md`'s rows for the two names say metres.

`chart_region::point_in_polygon`, which shares the table, normalises
each member first (`r.map(T::from_f64).normalize()`); the fix is the
same line here. The cell test drops a cell only on a definite verdict,
so the overstatement can drop one a sound margin would have kept.
