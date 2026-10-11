---
id: a-placement-copy-is-its-own-table-in-the-product
kind: unit
title: A placement copy is its own table in the product: Placed retires and the product reads one table per copy
status: open
opened: 2026-10-11
priority: P2
refs: [a-name-is-scoped-by-the-variable-holding-its-body, the-at-rest-census-is-a-check]
---

Filed by FORK-DM4 unit 2 (`a-name-is-scoped-by-the-variable-holding-its-body`),
which built one name table per output variable and kept `RoleSeg::Placed`
(orchestrator ruling Q2: the product change is its own unit).

NAMES N1, the scope: a name tells apart only what its variable holds. A world
placement's copy is its own output (D10), so its own table already tells its
names apart; `RoleSeg::Placed { of }` restates the output, as the split's half
did before unit 2. It stays only because the product merges every copy into
one table. The names/README tie paragraph ("A tie's candidates keep their
identity", `crates/editor-core/src/names/README.md`) says so: "its names are
qualified by the copy (`RoleSeg::Placed` at the placement) as a pattern copy's
are, so a name reaches the product once per copy by construction and the
gather has no once-per-product rule to keep."

The change: the product carries one table per copy, read by the copy's
placement variable, and a placement adds no name segment (N1's pass-through
set); `Placed` retires. Every reader of the product's one table moves to the
per-copy tables:

- product and export (`crates/editor-core/src/product.rs`, the gather's
  pass 3 carries each source's rows into one aggregate table; STEP export
  and the instance's `PartValue::names` read that table);
- the at-rest census (`the-at-rest-census-is-a-check` and the other at-rest
  items: it attributes a finding to names read off the aggregate);
- the viewer's pick (`crates/viewer`, which inverts an entity of the product
  to a name through the one table).

It meets stage 4 F's ground (declared pairs retire, which removes the other
reader that names an entity across copies) and the at-rest census items, so
it is sliced with them rather than before.
