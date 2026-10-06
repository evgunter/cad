---
id: stacked-plates-with-mismatched-fillet-radii-refuse-in-both-orders
kind: issue
title: Two stacked plates whose corner fillets differ in radius refuse their union in both operand orders
status: closed
opened: 2026-10-01
priority: P3
cost: M
branch: reach/mid-edge-tangency
pr: 3846
closed: 2026-10-02
---

Found by the review of PR 3657, measured on `d2d5b09076`.

## Repro

Two 6 × 4 × 1 plates stacked on one footprint, corner fillets of
r = 0.5 on one and r = 0.3 on the other, every finding declared: the
union refuses in both operand orders. The fillets of the two plates
are different cylinders (different axes and radii), so they are no
continuation, and the larger fillet's tangent points fall in the middle
of the smaller-radius plate's flat wall edges.

Probably the same family as
`a-stack-across-a-mid-edge-tangency-builds-in-one-operand-order-only`,
except that here neither order splits the edge first. The refusal's
kind was not recorded by the review; record it when the row is picked
up.

## Measured on `cd49025f`

The "both orders" above does not hold. Every finding declared, the
union and the difference build with the LARGER-fillet plate as A and
refuse `CurvedPierceUnsupported` with the smaller-fillet plate as A,
whichever plate is on top: the same one-order-only class as
`a-stack-across-a-mid-edge-tangency-builds-in-one-operand-order-only`,
and closed by its fix. The 0.3-over-0.5 and 0.5-over-0.3 stacks are
rows of `a_tangency_in_the_middle_of_an_edge_builds_in_either_operand_order`.

## Closed (2026-10-02, PR 3846)

The mismatched-radius stacks (0.3 over 0.5, 0.5 over 0.3, 0.25 over 1,
1 over 0.25, 1.9 over 0.2) build in both orders, through the
deferral that closes
`a-stack-across-a-mid-edge-tangency-builds-in-one-operand-order-only`.
Near-equal radii (0.5 against 0.500001) still refuse in both orders, on
main and head alike: the band cannot tell them apart.
