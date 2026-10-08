---
id: a-union-seam-cut-to-one-piece-keeps-its-bare-name
kind: issue
title: A union seam a later member cuts to one piece keeps its bare name, so it is renamed when a second piece appears
status: open
opened: 2026-10-06
priority: P3
cost: M
refs: [a-second-crossing-by-one-face-renames-the-first-and-its-pieces]
---


## What

N2 names every piece of a divided edge by its ends, a lone one included
(`a-second-crossing-by-one-face-renames-the-first-and-its-pieces`). The
build reads "divided" off what the op divided: the reduction's split
record for a pair boolean's operand edges (`BooleanNaming::divided_edges`),
the finished body for a union's member edges (`emit_union::Flush::spans`),
and every fragment of a Split. A seam's one edge, and a Split's one section
chord across a face, are taken as their whole parent and keep the bare
name (`emit_topo::name_edge_pieces`'s `whole`).

That is right in a pair boolean, where nothing cuts a seam the op mints. A
union's seam, though, can be cut by a later member, and where the later
member swallows all but one piece, that piece is published as the seam's
one edge, bare: it gains `Ends` when a second piece appears, the group-size
dependence N2 removed for operand edges. Nothing in the suite was found
pinning it.

## Next

Read whether a union seam's one edge spans the whole line its two parents
meet along in the finished body (the `Flush::spans` read, over the seam's
parent faces' common boundary), and qualify it when it does not.
