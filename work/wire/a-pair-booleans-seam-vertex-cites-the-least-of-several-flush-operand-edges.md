---
id: a-pair-booleans-seam-vertex-cites-the-least-of-several-flush-operand-edges
kind: issue
title: A pair boolean's seam vertex cites the least of several flush edges of one operand, which at a union operand is the member minted first
status: open
opened: 2026-10-20
priority: P3
cost: E
---

Filed by WIRE's `wire-mintfirst` lane (branch
`wire/union-reads-not-mint-order`), from its sweep for mint-order reads.

## What

`flush_lines` in `crates/editor-core/src/names/emit_topo.rs` cites the
lines of one operand's edges at a seam vertex through `least_per_line`:
edges that run flush on one line count once, as the least in name
order. The union stopped choosing there (it cites the `Merged` set of
the flush member edges, `emit_union::Flush::sets_per_line`); the pair
boolean still chooses. When the operand is a union whose two members'
edges lie flush end to end at the vertex, their names are
`From { read, .. }` of two reads, and the least is the read minted
first, so a subtract from or by such a union names the vertex for
whichever member was authored first. The names README says so
("The pair boolean's seam vertex still cites the least of several flush
edges").

Unmeasured: `name_invariance.rs`'s history set holds no subtract whose
seam vertex lies where two of one operand's edges abut on one line (its
"the plate less a block and its copy" case is not one: the copy's edges
are one `Merged` edge each).

## What it needs

The union's rule at the pair: the set of the flush edges, oriented as
`a-unions-line-of-several-edges-is-oriented-by-its-least-edge` settles,
with a history fixture that reaches it. `emit_topo.rs` was outside
`wire-mintfirst`'s fence (another lane held it).

## Both seats (traced for PR 5488's review)

The names README said a subtract's seam vertex cites "the least in name
order, its two seats among them, so no seat is preferred". Since every
name compares `From { read }`, a least taken across the two seats would
prefer the seat minted first. Traced: `flush_lines` is called once per
seat (`emit_topo.rs`, the pinch arm of the seam-vertex naming,
`flush_lines(aes, a, ..)` and `flush_lines(bes, b, ..)`), so the least
is taken within one seat's edges and never sets one seat against the
other; the lines of both seats enter the vertex's run side by side. The
README now says that. What does compare across seats is the rank of
several vertices sharing one run, read along the run's least line,
which is `a-unions-line-of-several-edges-is-oriented-by-its-least-edge`'s
orientation question at the pair.

