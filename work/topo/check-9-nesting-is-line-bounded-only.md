---
id: check-9-nesting-is-line-bounded-only
kind: issue
title: check 9's nesting half is silent on the DISC class and the no-walk class: an annular rim between two circles (every shelled vessel of revolution) still accepts a ring outside its outer loop
status: open
opened: 2026-09-13
---



Raised by `tier3-accepts-a-ring-outside-its-outer-loop`, which gave
check 9 a nesting half. That half asks the crate's one trilean
containment walk — `splitting::containment::point_in_loop` — whether
each ring vertex lies inside the region the face's outer loop bounds.
Which walk can express a loop's region is not a question this program
answers: `boolean::contain`'s `loop_shape` classifies it, and check 9's
gate (`validate::nesting_normal`) is that classifier plus "the surface
is a `Plane`". The arm runs on the `Parity` class — no arc at all, or
arcs over at least three vertices, where the polygon through the
vertices is a proper region and the walk is measured correct.

**What is left, and it is exactly two loop classes.**

1. **`Disc`** — every edge an arc of ONE circle. The polygon through
   such a loop's vertices has zero area, so the parity walk answers
   `Out` for every interior point and answering from it would REFUSE
   valid bodies. The region is that circle's disc and
   `boolean::contain`'s `disc_side` decides it EXACTLY (one radial
   margin, one decide) — the widening this row holds. `disc_side` is
   private to `boolean::contain` and the decide is S-BOOL's, so the
   widening is theirs to make or to open.
2. **`NoWalk`** — arc-bearing over fewer than three vertices where the
   arcs are not one circle: a half-disc cap, a lens of two different
   circles. No available walk expresses the region; the general
   arc-aware parity walk (#1076) subsumes this third, and the disc
   third does not wait on it.

Silent in both, and silence is the correct direction there. A loop
`loop_shape` cannot CLASSIFY (a carrier-agreement escalation, or a loop
it cannot read) is silent for the same reason: the gate failed to open,
and answering anyway is the false-refusal direction.

What the silence costs, concretely: the shape the parent item came
from. `shell::shell_open`'s rim glue picks host and guest by the sealed
arm's shell list, and on a planar rim in the parity class an inverted
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
are `pub(crate)` as of `tier3-accepts-a-ring-outside-its-outer-loop`
(announced on `work/bool/log.md`). `disc_side` is not, and it is the
decide this row needs; `crates/topo/src/boolean/` is S-BOOL's ground
and `validate.rs` is this program's, so that visibility change is
announced on S-BOOL's board before it lands.
