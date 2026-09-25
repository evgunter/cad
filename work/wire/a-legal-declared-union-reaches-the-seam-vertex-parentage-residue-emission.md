---
id: a-legal-declared-union-reaches-the-seam-vertex-parentage-residue-emission
kind: issue
title: A legal declared union reaches the seam-vertex-parentage residue Emission once a covered contact is declared
status: open
opened: 2026-09-25
priority: P1
cost: D
---


## What

`name_boolean_edges`'s seam-vertex pass (`crates/editor-core/src/names/emit_topo.rs`,
the catch-all arm after `SeamVertexParentage`) refuses
`Emission("seam vertex parentage underdetermined from incident edges")`,
the class reserved for a kernel bug, on a legal declared union.
`two-emitter-refusals-a-legal-declared-union-reaches` (closed, PR 2688)
gave one arm of that pass a typed refusal (`NamingError::SeamVertexParentage`)
and left this catch-all as "the unenumerated residue". No open row owns
the residue, and a legal document reaches it.

## Measured (EMIT, 2026-09-25, on `emit/pairwise-contact`)

The review corpus's `row`: `a` = [0,1]³, `b` = x 0.5..1.5 over the
same y and z, `g` = x 0.3..0.4, y -1..2, z 0.5..3.5, `h` = x 1.0..1.1,
y -1..2, z 0.5..3.5; `(a, b)` declared flush on all four families and
`(a, h)` declared on `a`'s x = 1 wall against `h`'s x = 1.0 wall. The
orders `[a, h, b, g]` and `[h, a, b, g]` refuse this `Emission`: `a`
and `h` fuse first, then `b` joins across the fused wall. `{a, b, h}`
does the same in `[a, h, b]` and `[h, a, b]`. Pinned by
`emit_union_rim_piece_ranks::an_undeclared_covered_contact_refuses_in_every_order_and_declared_fuses_where_b_covers_it`,
`a_contact_b_covers_refuses_undeclared_and_is_satisfied_declared_where_b_consumed_the_face`,
and `emit_seam_junction::a_junction_is_named_the_same_in_every_order_that_fuses`,
which admits it in those orders only.

Before DM4's pairwise contact rule these orders refused
`UndeclaredContact` undeclared, and nobody declared the covered
contact, so the residue was reached only by the orders GATHER's row
lists. The rule makes declaring `(a, h)` necessary, so every author of
such a document now meets this refusal in the orders where the two
declared members fold first.

## Why it matters

It is the two-emitter row's argument again: the document is legal, and
`Emission` tells its author the crate has a bug. A typed refusal naming
the construction, or a naming rule for the shape, is owed; which is the
two-emitter row's own question.
