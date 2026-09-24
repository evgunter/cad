---
id: check-9-nesting-is-line-bounded-only
kind: issue
title: check 9's nesting half is silent on every ARC-BEARING outer loop: an annular rim between two circles (every shelled vessel of revolution) still accepts a ring outside its outer loop
status: dispatched
opened: 2026-09-13
priority: P0
cost: H
parent: ATREST-5
---



Raised by `tier3-accepts-a-ring-outside-its-outer-loop`, which gave
check 9 a nesting half. That half asks the crate's one trilean
containment walk — `splitting::containment::point_in_loop` — whether
each ring vertex lies inside the region the face's outer loop bounds.
Which walk can express a loop's region is not a question this program
answers: `boolean::contain`'s `loop_shape` classifies it, and check 9's
gate (`validate::nesting_normal`) is that classifier plus "the surface
is a `Plane`". The arm runs on the `Polygon` class alone — no arc
anywhere, so the polygon the walk reads IS the loop's region.

**What is left, and it is exactly the three arc-bearing classes.**

1. **`ArcParity`** — arcs over at least three vertices. The polygon
   through them is a proper REGION and `contfp` walks it for one
   point's verdict, but it is not the LOOP's region: an arc bowing
   outward leaves region between the polygon and the boundary, and a
   point there reads `Out` when it is in. The unit tried to answer
   this class and a valid body refused —
   `review_fillet_h7_r1_probes::a_cap_carrying_a_ring_keeps_it_through_the_cut_off`'s
   bored D-rod, whose transverse cap's major arc dips past the chord
   its vertices span and whose bore sits in the lune between the two,
   giving two `RingOutsideOuter`s on a body every other check blesses.
   The fallback to the line-only class is that measurement. What
   closes this third is the general arc-aware parity walk (#1076), and
   the variant split that states the gap
   (`LoopShape::Polygon`/`ArcParity`) is what makes it addressable.
2. **`Disc`** — every edge an arc of ONE circle. The polygon through
   such a loop's vertices has zero area, so the parity walk answers
   `Out` for every interior point and answering from it would REFUSE
   valid bodies. The region is that circle's disc and
   `boolean::contain`'s `disc_side` decides it EXACTLY (one radial
   margin, one decide) — the widening this row holds. `disc_side` is
   private to `boolean::contain` and the decide is S-BOOL's, so the
   widening is theirs to make or to open. This third does not wait on
   #1076.
3. **`NoWalk`** — arc-bearing over fewer than three vertices where the
   arcs are not one circle: a half-disc cap, a lens of two different
   circles. No available walk expresses the region; #1076 subsumes it.

Silent in all three, and silence is the correct direction there: this
arm REFUSES a body on an `Out`, which is what makes it stricter about
its walk's domain than `contfp`, whose `Out` classifies a point. A loop
`loop_shape` cannot CLASSIFY (a carrier-agreement escalation, or a loop
it cannot read) is silent for the same reason: the gate failed to open,
and answering anyway is the false-refusal direction.

What the silence costs, concretely: the shape the parent item came
from. `shell::shell_open`'s rim glue picks host and guest by the sealed
arm's shell list, and on a planar rim of line carriers an inverted
pick now reds at the verb's closing `validate_geometric`. On a vessel
of revolution the rim is an annulus between two CIRCLES — the disc
class on both loops — so the arm is silent and the inverted pick still
validates at rest. (Through `shell_open` itself such a pick dies
earlier, in the naming record's `ring_rows` walk; what the silence
costs is the class being loud wherever ELSE it is minted, which is the
whole point of stating an invariant at rest.) The comment at
`shell.rs`'s `(host, guest)` assignment says exactly this and names
this row's subject as what is left.

The seam is already open on one side: `loop_shape` and its `LoopShape`
are `pub(crate)` as of `tier3-accepts-a-ring-outside-its-outer-loop`,
which also split the old `Parity` variant into `Polygon` and
`ArcParity` so the two soundness cases its own doc distinguished in
prose are distinguishable in code (announced on `work/bool/log.md`;
`contfp`'s behaviour is unchanged, it walks both). `disc_side` is not
open, and it is the decide this row's second third needs;
`crates/topo/src/boolean/` is S-BOOL's ground and `validate.rs` is this
program's, so that visibility change is announced on S-BOOL's board
before it lands.
