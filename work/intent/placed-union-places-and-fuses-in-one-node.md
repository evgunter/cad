---
id: placed-union-places-and-fuses-in-one-node
kind: issue
title: PlacedUnion both places copies and fuses them; it retires into a placement of the copies and a union reading them
status: open
opened: 2026-10-08
priority: P0
refs: [transform-retires-into-a-placement, declared-pairs-retire]
---

Ev, 2026-10-08: "PlacedUnion sounds like it needs to be fixed". Under
D10 a union reads shapes already in one space and places nothing, and a
placement is the operation that defines a copy. `Node::PlacedUnion`
(`crates/editor-core/src/node.rs`; `wire_placed_union` in
`src/eval/wire.rs`; `name_placed_union` in `src/names/emit.rs`) does
both in one node: it places copies of one prototype by a pattern rule
and fuses them. REFERENCES DM4 now says it retires into a placement of
the copies (a `Pattern`) and an n-ary `Node::Union` reading them.

Still saying otherwise: `crates/editor-core/README.md`, "The group
boolean: `PlacedUnion`" (and its row in the code map), the
`DESIGN.md` companion-table row for that page, the "Also kept from
elsewhere" sentence of `crates/editor-core/REFERENCES.md`, and the
tour's `heatsink` and `impeller` scenes (`demos/README.md`). That
section's certified disjointness (`topo::Separation`) is the at-rest
census's job once the copies are a placement's (stage 5 C retires
`Separation` into the resident).

Lands after stage 3 C/D (a pattern is a placement of several copies,
FORK-S3M) so the copies it would read exist; the union of a pattern's
members is DM4's node, whose names are keyed by member.
