---
id: check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk
kind: issue
title: check 9's nesting arm is silent on the ArcParity and NoWalk outer-loop classes until an arc-aware walk exists: a ring outside an arc-bearing outer loop that is not one circle still certifies
status: dispatched
opened: 2026-09-24
priority: P0
cost: H
parent: ATREST-12
---


Split from `check-9-nesting-is-line-bounded-only` when ATREST-5 closed
its `Disc` third. Check 9's nesting arm (`validate::nesting_region`,
`validate::ring_nesting`) now decides an outer loop in
`boolean::contain::loop_shape`'s `Polygon` class through
`splitting::point_in_loop` and one in its `Disc` class through
`boolean::contain::disc_side`. What is left is the two classes no
available instrument expresses, both waiting on the general arc-aware
parity walk (`work/tang/arc-aware-point-in-loop`, #1076):

1. **`ArcParity`** — arcs over at least three vertices. The polygon
   through them is a proper REGION and `contfp` walks it for one
   point's verdict, but it is not the LOOP's region: an arc bowing
   outward leaves region between the polygon and the boundary, and a
   point there reads `Out` when it is in. The unit that gave check 9
   its nesting half tried to answer this class and a valid body
   refused —
   `review_fillet_h7_r1_probes::a_cap_carrying_a_ring_keeps_it_through_the_cut_off`'s
   bored D-rod, whose transverse cap's major arc dips past the chord
   its vertices span and whose bore sits in the lune between the two,
   giving two `RingOutsideOuter`s on a body every other check blesses.
   What closes this third is the general arc-aware parity walk, and
   the variant split that states the gap
   (`LoopShape::Polygon`/`ArcParity`) is what makes it addressable.
2. **`NoWalk`** — arc-bearing over fewer than three vertices where the
   arcs are not one circle: a half-disc cap, a lens of two different
   circles. No available walk expresses the region; the arc-aware
   walk subsumes it.

Silent in both, and silence is the correct direction there: the arm
REFUSES a body on an `Out`, which is what makes it stricter about its
walk's domain than `contfp`, whose `Out` classifies a point. An
escalation stays an escalation, never a guess.

What the silence costs: a ring outside an outer loop that mixes arcs
with lines (a slot, a rounded rectangle, a D-shaped cap) or carries
arcs of two circles still certifies at rest. `shell::shell_open`'s rim
on such a loop is pinned only structurally (the `(host, guest)`
comment in `shell.rs`).

**When the trigger fires**: widen `validate::nesting_region` to admit
the two classes through the arc-aware walk, flip
`validate::tests::an_arc_bearing_outer_loop_is_the_gates_residue`
from asserting silence to asserting the inverted pick is refused by
name, and run the D-rod row above as the false-refusal guard.
