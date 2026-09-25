---
id: union-refuses-in-some-member-orders-and-publishes-in-others
kind: issue
title: A union refuses in some member orders and publishes in others, over PR 3112's review corpus and the #3168 review fixtures
status: open
opened: 2026-09-24
priority: P1
cost: H
---


## What

The same union, with the same members and declarations, refuses in some
member orders and publishes in others. A document that fuses as written
can therefore refuse after the author reorders its members. The refusal
is loud, so there is no wrong answer, but the order still decides
whether there is an answer.

## Measured (EMIT, 2026-09-24, on `emit/rim-piece-ranks`)

Source: `crates/editor-core/tests/emit_union_rim_piece_ranks.rs`, whose
rebind row now checks this. It runs PR 3112's review corpus (as
`probe_corpus`) and the #3168 review fixtures, every order.

25 cases refuse in some orders and publish in others. The row pins
them in `KNOWN_MIXED`, so a new one, or a change in any of these,
turns it red.

| refusal | cases | owner |
|---|---|---|
| `DeclareResolve` (Vanished): a declared face the fold has consumed | `abg`, `abgids`, `abglow`, `abgg2`, `fam0{00,01,02,12,22}`, `fam1{00,01,02,12,22}`, `fam2{00,01,02,12,22}`, `r1flush`, `r2endsg`, part of `row`/`rowids`/`r4trig` | `work/gather/member-space-look-through-stops-at-splits-containment-and-fragmented-merges.md` (P0) |
| `UndeclaredContact`: `h` (x 1.0..1.1) meets `a`'s x = 1 wall flush when it is folded before `b` covers that wall | part of `row`, `rowids` | none; this row |
| `Boolean(Containment(RayExhausted))`: `c` (x 0.8..2.0), flush with both `a` and `b`, folded between them | `r4tri` (`[0,2,1]`, `[2,0,1]`), part of `r4trig` | none; this row. The same on main before #3167 (the #3168 review's probe) |

The last two are unowned. `UndeclaredContact` is a fold-order
dependence of the contact check itself: whether a contact is flush
depends on whether another member has covered it yet. `RayExhausted`
is a kernel containment refusal on a body that other orders build
without trouble. Compare
`work/zip/two-parts-of-one-body-at-one-boolean-refuse-as-ray-exhausted.md`.

