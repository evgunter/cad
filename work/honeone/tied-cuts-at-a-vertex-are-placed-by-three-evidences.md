---
id: tied-cuts-at-a-vertex-are-placed-by-three-evidences
kind: issue
title: Two germs tied at first order at a vertex are placed by three rules from three evidences (beside_order, tied_held, tie_breaks_after)
status: open
opened: 2026-11-12
priority: P3
cost: M
---

Re-homed from `germ-orders-about-a-normal-have-two-homes`'s last two
sections when `topo::star` stage 3 closed that row (`onehome/star-cut`):
the one order door (`star::germ_order`, `star::before`, `star::turn`)
now serves both verbs with one set of K names and hands every tie to
its caller, so what is left is which evidence a caller's tie reads.

## What

Two cuts tied at first order at a vertex meet three rules, each
answering which side of the shared line each copy lies on, from
different evidence:

- `boolean/insert.rs` `beside_order`: the side of the other solid's
  face flush with the sector beside each cut's edge (its flankers),
  through `cut_order`, the orbit's one position order.
- `boolean/insert.rs` `tied_held`: the side each pair's run leaves the
  line to, read off its own piece's codes, reached where `beside_order`
  reads nothing.
- `boolean/vtxfac.rs` `tie_breaks_after`: the side each pierce germ's
  partner lies on, by the chords' nesting (the ring's `Ties::along`).

Nothing makes them agree. PR 6184 measured `beside_order` against
`tied_held` over the topo integration suite: 1015 of 1015 agree where
both read; `tie_breaks_after` was not compared, its germs carry no
flankers. The split's tie (`section_chords`'s `curving`, one order
down) and the ring's one-pierce tie read second order by
`geom_brep::tied_germ_turn`.

Unwitnessed (PR 6184's review): `beside_order` also reads two cuts
whose flush flankers lie on one side of the line, at different offsets
from it, apart. That reading comes from the far bounds' angles from the
line, not from a side, and no test or pose reaches it.

## Owed

Name which evidence decides a tie of cuts at a vertex and check the
others against it, or say why each reader's evidence is the only one it
has.

