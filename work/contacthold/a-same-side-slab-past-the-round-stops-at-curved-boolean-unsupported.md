---
id: a-same-side-slab-past-the-round-stops-at-curved-boolean-unsupported
kind: issue
title: A same-side slab running past a quarter round's ends (sameside-flush, beyond) stops at CurvedBooleanUnsupported, a separate door from the short-circuit
status: closed
opened: 2026-10-11
priority: P3
branch: contacthold/claim-free-tangent-glue
pr: 4831
closed: 2026-10-12
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

## More reach, since PR 4831

With the claim-free door, more pairs are glued `Tangent` and meet this arm:
the stadium (`pi_seam_and_kiss_through_the_boolean::the_stadium_refuses_at_the_edge_sector_tangent_door`,
a slab whose top and bottom end on a rod's rulings, normals aligned) now
refuses `CurvedBooleanUnsupported` here in both orders, traced with a probe
in the arm. A thin disc internally tangent in a fat one but run long
(z ∈ [−1, 2] against [0, 1]) moved from `CurvedPierceUnsupported` to
`CurvedBooleanUnsupported` (site not traced); at equal heights it answers
every op.

## Answered (PR 4831's fix pass, 2026-10-12)

`recl::resolve_edge_sector` now lumps a glued tangency's on-flanker by
the side its carrier curves to (`sectors::tangent_lump`), as
`resolve_edge_edge` already did, and resolves Table II's `Rule` cells on
that lump; `recl::require_same`'s tangent arm is unreachable and refuses
`ClassificationInvariant`. Measured, each against its closed form, tier 3
and point membership where the row checks it:

- `review_4781_touch_pairs_every_op`: all 72 staggered lines answer, both
  orders, three scales (the row now demands `OK` on every line);
- `review_4704`'s `sameside-flush` and `r1_same_side_beyond_the_rim`'s
  `beyond`, the row's original fixtures, answer every op
  (`the_rims_half_turn_postures_answer_against_the_oracle` now includes
  the `sameside-*` poses);
- the stadium (`the_stadium_answers_every_op`), the kiss with overlap,
  `review_4831_probes::p4_a_plane_tangent_to_part_of_a_hole`, the thin
  disc run long (`p8`), a rod in a plate's bore (`p9`), one turn of the
  rod in a revolved tube's bore, and the in-band parallel cylinders of
  `tang_circle_cylinder` all answer.
