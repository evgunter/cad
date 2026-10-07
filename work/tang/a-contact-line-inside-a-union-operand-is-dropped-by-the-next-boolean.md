---
id: a-contact-line-inside-a-union-operand-is-dropped-by-the-next-boolean
kind: issue
title: A union operand whose two pieces touch along a line loses that contact in the next boolean: the result is right and tier 3′ refuses its contacts
status: open
opened: 2026-10-07
priority: P1
cost: M
---


## What

Found by PR 4129's review 4 (N3), reproduced on main 1f27ea08.

U is the union of two upright triangular prisms, footprints about
(1.5, 1) of radius 0.4 over the sectors 0–50° and 180–230° (or 0–50°
and 120–170°), z 0.5–2.0 and 0.4–1.9. They touch only along the
vertical line through (1.5, 1). U is tier-3′ valid with its contacts.
Against the plate `[0, 3] × [0, 2] × [0, 1]`:

| op | counts | tier 3 | volume | tier 3′ |
|---|---|---|---|---|
| P − U | [14, 30, 19] | ok | 5.932588 (closed form) | `UndeclaredContact` (VertexOnEdge at (1.5, 1, 0.5), EdgeEdge) |
| U − P | [10, 18, 12] | ok | 0.116439 (closed form) | `UndeclaredContact` (VertexVertex at (1.5, 1, 1), VertexOnEdge) |
| P ∩ U | [10, 18, 12] | ok | 0.067412 (closed form) | `UndeclaredContact` (VertexVertex at (1.5, 1, 1), VertexOnEdge) |

The bodies' geometry is right: the volumes are the closed form, the
corners at every vertex are disjoint, and tier 3 passes. What is wrong
is the result's contact record. U's line contact, cut by the plate's
top, survives into the result as a touching of two pieces, and
`validate_pseudomanifold` finds it undeclared. The carry is
`boolean::ops`'s `carry(…)` (`crates/topo/src/boolean/ops.rs:694`),
which maps the operands' contacts through the descendants. A body whose
contacts lie is a silent wrong answer to any reader that trusts them,
so this is P1: the body is right and only its contact record is not.

## Done when

A row over these poses asserts tier 3′ with the result's contacts, in
every op and both orders. It is red on main today.
