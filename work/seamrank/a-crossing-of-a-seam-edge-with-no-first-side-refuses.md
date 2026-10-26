---
id: a-crossing-of-a-seam-edge-with-no-first-side-refuses
kind: issue
title: A crossing of a seam edge whose two sides carry one name has no orientation to read its sense along, and refuses where a lone one used to be named
status: parked
opened: 2026-10-06
priority: P3
cost: M
design: true
blocked_on: [a-unions-line-of-several-edges-is-oriented-by-its-least-edge]
---


## What

A crossing's sense is read along the crossed edge as the body stores it, and
along a seam edge as the loop of its pair's first side runs
(`emit_topo::crossed_edge_orientation`). A seam whose two sides carry one
name (two placements of one prototype, N1), or whose faces the names do not
tell apart, has no first side, so no sense of a crossing of it is a fact of
the names. Before the sense, a lone crossing of such an edge was named with
no qualifier and several tied; now the pair emitter and the Split refuse it
as `NamingError::Emission` ("a crossed seam edge has no first side to orient
its crossings by", `emit_topo::oriented`, `emit_topo::sense_of`). No test or
corpus document reaches it (a full `editor-core` run, instrumented, found no
unoriented crossed edge).

## The question

What such a crossing is named: a tie with no sense, a sense read along some
other covariant direction, or the refusal. A design question, open until a
model reaches it.

## Parked (2026-10-26, tracker sweep)

The refusal ("a crossed seam edge has no first side to orient its
crossings by", `emit_topo.rs`) sits in the orientation code that
FORK-ORIENT (fork-log row 197; Ev, 2026-10-26; PR 5705) rewrites: N2
now reads a seam along each of its sides, a transversal crossing being
`Through { entering }` over the seam's sides in name order, so no first
side is asked. That build is WIRE's
`a-unions-line-of-several-edges-is-oriented-by-its-least-edge`. Re-read
this row when it lands: the refusal goes with the first-side read, but
where both sides carry ONE name an `entering` subset may still not say
which side the crossing enters along, and that residue is this row's
question.
