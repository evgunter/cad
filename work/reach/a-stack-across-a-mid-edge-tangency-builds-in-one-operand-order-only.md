---
id: a-stack-across-a-mid-edge-tangency-builds-in-one-operand-order-only
kind: issue
title: A sharp-over-rounded stack (and a concave L) builds only with the rounded operand as A: the mid-edge tangency is a frontier in the other order
status: review
opened: 2026-10-01
priority: P2
cost: M
branch: reach/mid-edge-tangency
pr: 3846
---

Found while building the continuation ruling (PR 3657) and widened by
its review, measured on `d2d5b09076`.

## Repro

A sharp 6 × 4 × 1 plate stacked on a rounded one (corner fillets
r = 0.25, 0.5 or 1), every finding declared:

- rounded plate as A: builds, exact and valid;
- sharp plate as A: refuses `CurvedPierceUnsupported` with the sharp
  operand named (`crates/topo/src/boolean/reduce.rs:1449`, the
  crossing layer's frontier).

A concave L with rounded corners stacked on its sharp twin behaves
the same. `crates/sweep/tests/reach_continuation.rs`
(`a_tangency_in_the_middle_of_an_edge_keeps_its_typed_refusal`) pins
both orders at r = 0.5.

## Why

The sharp plate's straight bottom edges pass the rounded plate's
fillet tangent points in the middle of the edge. The one-sided cover
records endpoints only, so a mid-edge tangency is a typed frontier.
When the rounded operand is A, its own wall edges are swept first and
split the sharp edges at the tangent points, so the touch lands on
endpoints and is covered. The outcome therefore depends on sweep
order rather than geometry. A fix either splits at structural tangent
points before the sweep, or gives the cover a mid-edge arm.

## Measured on `cd49025f`, and the fix

Still refusing on `origin/main` at `cd49025f`: with the sharp operand as
A, union, intersect and sharp − rounded refuse `CurvedPierceUnsupported`
(the sharp bottom edge × the fillet cylinder, `wall_crossing`'s line ×
wall roots answering `Tangent`), at every radius, plate and L alike.
Holding the pair for both directions' splits exposed a second
order-dependence behind it: the `Rest` zip minted a STRAIGHT seam chord
in the sharp operand's bottom face along each fillet's rim and kept
operand A's seam edge, so with the sharp plate as A the result carried a
chord where the arc belongs (`describe_minted_edges` refused it,
`JoinDesync`). PR 3814's twin chord (`rest::Twin`) closed the second on
main; the hold-then-settle on `reach/mid-edge-tangency` closes the first;
`a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order`
holds every pose in both orders through ∪, A ∖ B, B ∖ A and ∩.
