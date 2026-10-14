---
id: a-clamped-carrier-foot-is-the-stored-knot
kind: issue
title: A carrier foot the domain clamp stopped at an end enters as an unknown, though it is the stored knot
status: closed
opened: 2026-10-10
priority: P3
cost: E
refs: [nurbs-weights-and-knots-are-the-scalar, topo-keeps-computed-values-in-t]
pr: 4708
closed: 2026-10-11
---


`replace_face.rs`'s `plan_reanchors` enters a spline carrier foot
through `carrier_foot` (`T::from_computed`) also when the domain clamp
stopped it at an end (`at == lo || at == hi`). Such a foot is exactly
the carrier's stored knot-domain end, the value `corner_of_root`'s
`SplineRoot::Short { end }` arm enters through `from_f64` as stored
carrier data. At `Sym` the same end is two different symbols, so a
`Zero` between a clamped corner and that end is decided numerically
where it could be proved. This is the safe direction: it loses a
`Zero` and changes no geometry.

The fix: enter a clamped foot as the domain end, in the same way and
in the same change that `nurbs-weights-and-knots-are-the-scalar`
settles how a stored knot enters `T`.

Closed by #4708 (`nurbs-weights-and-knots-are-the-scalar` u4):
`carrier_foot` enters a foot the clamp stopped at an end as the
carrier's held end knot, and `corner_of_root`'s short root reads the
same held end (`a_short_roots_end_is_the_held_end_knot`).
