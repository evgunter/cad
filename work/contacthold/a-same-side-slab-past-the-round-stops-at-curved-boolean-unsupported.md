---
id: a-same-side-slab-past-the-round-stops-at-curved-boolean-unsupported
kind: issue
title: A same-side slab running past a quarter round's ends (sameside-flush, beyond) stops at CurvedBooleanUnsupported, a separate door from the short-circuit
status: open
opened: 2026-10-11
priority: P3
---


Seen by both reviews of PR 4704, and recorded in
`a-same-side-tangent-touch-refuses-at-every-op`'s Finding. The
`sameside-flush` and `beyond` postures (the slab's `y` running past the
round's ends) refuse `CurvedBooleanUnsupported` at every op, on head and
on base. That is a different door from the short-circuit; one designer
suspects `recl::require_same`'s declared-Tangent arm, or the cap arc's
vertex on the slab top's interior. Measure which door refuses before
acting on either theory. The reviewers' print rows in
`crates/sweep/tests/review_4704_r1_probes.rs` and
`review_4704_flush_round_oracle.rs` reproduce it.

## Staggered edges on the line, measured (2026-10-12, PR 4781's fix pass)

The class is wider than the slab past the round: wherever the two
edges on the touch line are staggered (a vertex of one lies inside the
other's edge), every op refuses `CurvedBooleanUnsupported`, the same as
on main, for plane × cylinder and cylinder × cylinder alike. Where the
two edges coincide end to end, every op answers
(`common/touch_union.rs::assert_touch_at_every_op`).

Fixtures, all against the unit cylinder about the origin, z ∈ [0, 2]
(`crates/sweep/tests/review_4781_r1_probes.rs::review_4781_touch_pairs_every_op`,
which pins the refusal at every op, both orders, three scales):

- the box `x ∈ [1, 3], y ∈ [0, 2]` on the cylinder's seam ruling
  `x = 1, y = 0`, shorter (`z ∈ [0.5, 1.5]`) or taller (`z ∈ [-1, 3]`);
- the cylinder of radius 0.5 about `(1.5, 0)`, `z ∈ [0.5, 3]`, kissing
  at the same ruling at a staggered height (and radius 1 about
  `(2, 0)`, `z ∈ [1, 3]`, which reads the same).

**The door is `recl::require_same`'s `declares_tangent` arm**, reached
from `recl::resolve_edge_sector`: a backtrace taken at each
`CurvedBooleanUnsupported` raise site in `boolean/` showed that arm and
no other for all four fixtures at ∪, ∖ and ∩. The staggered vertex
puts an edge–sector event on the line, and that lane hands the
verified `Tangent` flank pair to the carrier-identity check, which
refuses a tangent pair, instead of to `sectors::tangent_lump` as
`resolve_edge_edge`'s membership does.
