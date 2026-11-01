---
id: a-pair-booleans-seam-vertex-cites-the-least-of-several-flush-operand-edges
kind: issue
title: A pair boolean's seam vertex cites the least of several flush edges of one operand, which at a union operand is the member minted first
status: closed
opened: 2026-10-20
priority: P2
cost: E
closed: 2026-10-26
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


## Moved to WIREREST (2026-10-20)

WIRE measured 69 budget points against 30. This row is P3 or P4, and WIRETAIL already stood at 25 of 30, so it moved by `git mv` (id and body unchanged) to WIRE's second tail. Signed (WIRE orchestrator).

## The orientation is settled (2026-10-26, tracker sweep)

What this row waited for is ruled: FORK-ORIENT (fork-log row 197; Ev,
2026-10-26; PR 5705) reads a line of several things along each of them
and ranks nothing, so the pair's set rule needs no orientation choice.
The names README still says "The pair boolean's seam vertex still cites
the least of several flush edges" (`names/README.md:423`), and
`emit_topo::flush_lines` still reads `least_per_line`. WIRE's
`a-unions-line-of-several-edges-is-oriented-by-its-least-edge` builds
the ruling on `emit_topo.rs`; this row can ride with it or follow it.


## Evidence from PR 5758 (2026-10-26)

PR 5758 settles the orientation half: a union reads a line of several
flush edges `Through { entering }` the edges whose direction enters, and
a seam edge by the side it enters along. The pair boolean's citation is
unchanged: `flush_lines` still cites the least of several flush edges of
one operand and reads the crossing's sense along that least edge as its
operand stores it, so where those edges run opposite ways the sense
(`Enters` or `Leaves`) reads which was minted first. The rank across
seats this row mentions is gone (nothing is ranked). The fix is the
union's rule at the pair: cite the set, read it `Through`.

## Back to WIRE at P2 (WIRE orchestrator, 2026-10-26)

PR 5758 built Ev's ruling on PR 5705, "an orientation is read, never chosen", at the union and at seams. The pair boolean still reads a line of several flush operand edges along the LEAST of them: `emit_topo::flush_lines` / `least_per_line`, and N2's "at a pair boolean as the least of them" (README ~391). That is the one place the approved rule is not yet built. Under it, the line's sense names each flush edge it enters along (`Through { entering }`), as the union's `Merged` line does now. PR 5758's review (finding 1) points here.

Closed by PR 5778 (2026-10-26). A pair boolean's line of several flush edges is now `Merged` of them and reads `Through { entering }`, through the union's shared `line_name` / `sense_along_line`. `least_per_line` is deleted. With this, Ev's 5705 rule is built at every door. No published table reaches the arm; fold steps only. The witness is emitter-level and goes red on base.
