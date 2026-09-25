---
id: a-contained-flush-operand-with-every-vertex-on-the-boundary-refuses-as-ray-exhausted
kind: issue
title: A shell with every vertex on the other operand's boundary exhausts the uncut-shell vertex probe and refuses as Containment(RayExhausted), though no ray was cast to exhaustion
status: open
opened: 2026-09-25
priority: P0
cost: D
refs: [two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted, union-refuses-in-some-member-orders-and-publishes-in-others]
---


## What

`b` = x 0.5..1.5 × y 0..1 × z 0..1 unioned into `a ∪ c` = x 0..2 over
the same y and z, with every coplanar pair of y-walls and caps declared
flush, refuses `Boolean(Containment(RayExhausted))`. The right answer
is the accumulation unchanged: `b` lies inside it, flush on four
sides. The same three blocks in any order that does not fold `b` last
fuse.

The refusal comes from the containment fallback's vertex probe,
`classify_shells` (`crates/topo/src/boolean/ops.rs:2114`). It runs when
the operands have no crossings, and it classifies an uncut shell by
walking the shell's vertices and asking `point_in_solid` about each
one, skipping `OnBoundary` answers. All eight of `b`'s vertices sit on
edges of the accumulation's y-walls and caps, so every probe answers
`OnBoundary`, the walk ends with no verdict, and
`verdict.ok_or(BooleanError::Containment(PointInSolidError::RayExhausted))`
(`ops.rs:2159`) raises a refusal whose text says every schedule ray
grazed. No ray was cast to exhaustion. The shell is decidable, since
the interior of `b`'s x = 0.5 face is strictly inside the other
operand, but the probe never looks at anything except vertices.
`finish.rs`'s `classify_shell` (the uncut-component probe,
`crates/topo/src/boolean/finish.rs:182`) ends the same way and has the
same blind spot.

## Measured (EMIT, 2026-09-25, on origin/main `2139eefa8e`)

`crates/editor-core/tests/emit_union_rim_piece_ranks.rs`, the `r4tri`
case (blocks `A`, `B`, `C8`, every pair declared flush on the four
families): orders `[a, c, b]` and `[c, a, b]` refuse. The site was
confirmed by tagging the three `RayExhausted` raise sites in
`crates/topo/src/boolean` (`finish.rs` `classify_shell`, `ops.rs`
`classify_shells`, and the schedule walk at the end of
`solid_contain.rs`'s `point_in_faces`). Only `ops.rs`
`classify_shells` fired, once per refusing order. The other four
orders fuse. In each of them the last member has a vertex strictly
outside the accumulation (x = 0 or x = 2), so the probe finds `Out`.
The union fold runs the pair verb (`eval/wire.rs`, `wire_union`), so
the pairwise chain `(a ∪ c) ∪ b` hits the same fallback.

## Not the same defect as the zip row

`work/zip/two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted.md`
hands the boolean one body twice, which is a state no document should
reach, and the fix it proposes is an operand-identity refusal before
the kernel runs. That fix would not reach this case, because `b` and
`a ∪ c` are distinct and legal operands with a definite answer. The
two share the exhaustion shape: every probed vertex is `OnBoundary`,
and the refusal says "rays" when the witness set ran out. Which of the
two probes the zip row's case reaches has not been re-measured.

## What a fix needs

This is the kernel owner's decision. Possible directions:

- When every vertex answers `OnBoundary`, fall back to a witness in
  the relative interior of a face that is not coplanar with the other
  operand.
- Refuse with a variant that names what actually ran out, for example
  "every vertex witness lies on the other boundary", in place of
  `RayExhausted`.

The first gives the answer. The second only makes the refusal honest.

## A second route in (GERM torus doors, 2026-09-25)

Two IDENTICAL full tori with every wall pair declared `Rest`
(`crates/sweep/tests/mate7a_torus_rest.rs`,
`the_admitted_torus_lane_stops_at_the_uncut_shell_probe`) now pass the
crossing layer, through the circle rung's carrier-identity rung, and
land here. Tagging both sites showed only `ops.rs` `classify_shells`
fires. Every vertex of each torus lies on the other's boundary, so the
vertex probe ends with no verdict and refuses `RayExhausted`. The right
answer is either torus. Same blind spot, curved operand.
