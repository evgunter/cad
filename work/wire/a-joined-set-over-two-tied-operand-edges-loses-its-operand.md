---
id: a-joined-set-over-two-tied-operand-edges-loses-its-operand
kind: issue
title: A pair boolean's joined edge over two operand edges one tie names takes a bare From name with no operand, and its vertex refuses
status: open
opened: 2026-10-20
priority: P3
cost: M
---



Filed by `wire-emis-topo` (the `emit_topo.rs` third of
`names-emission-raises-panic-or-take-their-true-ending`).

## What

`name_boolean_edges` names a joined edge that lies along several operand
edges by `set_name`, which hands each member's `From`-wrapped name to
`join_names::joined_name`. That dedups the names, and one name left is
returned bare. Where an operand's table ties two of the members (two
placements of one prototype), the set collapses to one `From` name, but
its `EdgeGroup` keeps `root: None, side: None`, as a set's does.
`name_boolean_vertices` then reads a `From`-headed edge as a carried
one, finds no operand, and refuses `Emission("a carried edge's group
names no operand")` (`crates/editor-core/src/names/emit_topo.rs`). The
body is sound and the input legal, so the refusal is a defect.
Unmeasured: no test builds the shape.

## Next

Build the witness (a joined edge over two tied edges of one operand),
then decide whether a set whose names tie is a set (`Merged` of one
name twice) or the one tied edge it spells, and give the group the
fields that reading needs.

## Findings (2026-10-28, `wire-joinedsets`): no witness without held ground

Not built. The refusal on the current tree is unchanged.

**What the shape needs.** The set has to collapse to one name. That
takes members that are all one operand's (`OpSide::wrap` spells each
side's read into the name) and all named alike. Members cover the
joined edge together, so two of them have to abut, collinear, at a
vertex that the boolean's join removes. So one operand has to hold two
tied edges end to end.

**Why no placement-free tie gives that.** The ties this tree builds
without `Place` or `Pattern` are N2 ties: pieces of one parent with
equal discriminators (U-cutter caps by `Borders`), and the seams or
pieces descended from them (`split_joined_keys_tie`). Every fixture of
them uses `fixture::slid`, but a full-turn revolve could host the
U-cutter clear of coplanarity.

- Two candidates are disjoint pieces of one parent. Two coplanar
  pieces meeting at a vertex or an edge is a non-manifold body, or a
  pair the output stage merges. So their seams cannot abut.
- Two pieces of one parent edge abutting at `v` with equal `Ends` need
  equally named far ends, and both pieces must survive past `v`. A
  transversal crossing at `v` flips in and out, so one of the two is
  dropped.

The row's own premise, "two placements of one prototype", is the
natural shape: two `Pattern` or placed copies of a bar end to end, and
their union or subtract. That is D10-held ground.

**The reading, once it can be built.** N3 says a `Merged` of one
element is that element, so a set whose names tie is the one tied edge
it spells, not `Merged` of one name twice. Its `EdgeGroup` then needs:

- `side: Some(op)`;
- `set` kept, because no single `root` holds it.

The vertex pass should read a vertex on it through `set_holding`, by
dispatching on a non-empty `set` before the name's head.

One question stays open. The set's base `From(x)` is also `edge_line`
of the tied roots' other pieces, so its `rec.record` group lands
beside their line groups. Whether it belongs in each candidate's group
instead has to be settled against the witness.

## Review note (PR 5916, 2026-10-28)

- **A gap in the no-witness argument.** The rule that a transversal crossing flips in and out ignores a non-transversal split. A tool edge touching E at v from outside leaves both pieces surviving, with tied far ends (crossings of two tied faces). Not built: it needs degenerate contact.
- **A decline guard is gone once this row is fixed.** A tied set that collapses to one `From` name would record under the same base as its roots' `record_on_line` line group. `line_counts` then gives `[line, set]`, which reads as a two-parent tie or a size mismatch where it used to decline. Decide this with the fix.
