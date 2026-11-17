---
id: inline-of-a-part-whose-product-is-a-pick-reaches-an-unreachable
kind: issue
title: inline of an instance whose part's product slot reads a pick panics at the spliced product's unreachable!
status: open
opened: 2026-11-17
priority: P2
cost: M
---


## What

`inline` (`crates/editor-core/src/refactor.rs`, `inline`) takes the
part's product as `part.product().filter(|&read|
part.operation_of(read).is_some())`, which a pick passes
(`Doc::operation_of` follows a pick to its placement), and then reads the
spliced product as `read_map.get(&product)`, with
`unreachable!("the part's live product {product:?} is spliced with its
operation")` where it is absent. `read_map` is `carry`'s output map: the
carried operations' outputs and the world, never a pick. So a part
whose product slot reads a pick (`DocEdit::SetProduct` with
`Operand::Pick`, which the slot admits as a node's body seat does, or
`Operand::Var` of a pick variable, which it admitted before) reaches the
`unreachable!` when an instance of it is inlined.

Found reading `inline` for the product-seat unit
(`split-widens-a-product-reading-one-of-several-cut-copies`); not
reproduced by a row.

## Owed

Inline places the spliced product as the instance copied it: the new
`Place`'s source a source over a pick of the spliced copy by the spliced
path (`host_path`), and the instance's bundle sides authored over that
read; a pick of a pick walked the same way. Then `flatten`'s
`in_source`/`unpicked`, which assume the placement's source is a copy or
a `⊔`, owe the same case. A row: a part whose product picks one piece of
a copy of a `⊔`, instanced, inlined and flattened, delivering that piece.
