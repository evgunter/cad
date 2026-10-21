---
id: the-product-gathers-unreachable-carry-messages-say-a-name-carried-twice
kind: issue
title: The product gather's unreachable carry refusals say a name was carried twice, whatever SeatingRefusal arm they caught
status: open
opened: 2026-10-19
priority: P4
cost: E
---


Found by `wire-p4`'s sweep for the class of
`a-seated-entity-collision-refuses-as-a-name-minted-twice` (a seating
refusal reported as a different fact than the one it caught).

Two `unreachable!` sites in `crates/editor-core/src/product.rs`
receive a whole `SeatingRefusal` and word it as a name collision:

- `product_in`, after `tie_rows.finish(&mut names)`: "two copies
  carried one name, {e:?}: a copy's names are qualified by its
  placement";
- `carry_names`, after `rows.carry(..)`: "{node:?} carried a name
  twice, {e:?}: ...".

`TieRows::carry_as` now also refuses one entity carried as two
candidates (`SeatingRefusal::EntityNamed`), and the flush can refuse
`KindMismatch` or `ShortTie`. The argument "a copy's names are
qualified by its placement" rules out a NAME collision only. An entity
collision is ruled out for another reason (each graft mints distinct
aggregate keys), and neither message says so. The payload is printed,
so nothing is lost, but the sentence states the wrong invariant.


## Repair shape

Match the arm, or word each message as the invariant that rules out
every arm: a copy's rows are distinct names over distinct keys.

## Parked on the D10 hold (2026-10-19, WIRE)

`product.rs` is the product gather, held placement ground under D10,
so `wire-p4` left it alone and filed this rather than editing it.
Parked on `transform-retires-into-a-placement`, the row whose landing
releases that ground. Re-read both sites then: the gather may have
moved.
