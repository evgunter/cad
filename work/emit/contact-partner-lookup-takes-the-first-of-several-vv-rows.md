---
id: contact-partner-lookup-takes-the-first-of-several-vv-rows
kind: issue
title: The seam-vertex pass takes the first contact-record vv row for a vertex, by row order, and silently ignores any other
status: open
opened: 2026-09-23
priority: P3
cost: E
---


## What

`crates/editor-core/src/names/emit_topo.rs`, `name_boolean_vertices`,
the contact-record partner read:

```rust
OpSide::A(ka) => match rc.vv.iter().find(|r| r.a == ka) { … }
OpSide::B(kb) => match rc.vv.iter().find(|r| r.b == kb) { … }
```

`reduction_contacts.vv` can hold several rows for one vertex (one
vertex coincident with more than one vertex of the other operand, e.g.
where the other operand has two coincident vertices at a pinch, or a
row per sweep event). `find` takes whichever is first in reduction
order and ignores the rest. The name the vertex gets then depends on
row order, not on the geometry, and nothing says a choice was made.

Found by PR #3103's review (NOTE-3), while `the-b-side-contact-record-rescue-arm-never-fires`
made the read symmetric.

## Band

P3: this is latent. No fixture in the tree is known to hold two vv rows
for one seam vertex, so no wrong name has been observed. The row is a
silent choice that would become one: "tooling that prevents SILENT
bugs". It would be P0 once a witness exists.

## Fix shape

Collect every matching row. For one row, behave as today. For several
rows that resolve to one upstream name, use it. For several distinct
names, refuse with a typed error that names the vertex and the
candidates, instead of choosing. First measure whether any suite
reaches more than one row: count matches per seam vertex over
`cargo test -p editor-core --test all`. If none does, the refusal is the
whole fix.
