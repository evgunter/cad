---
id: a-crossing-cites-its-edge-by-a-name-that-holds-that-edges-ends-so-names-grow-exponentially
kind: issue
title: A crossing cites its crossed edge by the edge's full name, which holds that edge's Ends, so a name grows exponentially with how deep its piece sits in a chain of cuts
status: open
opened: 2026-10-06
priority: P1
design: true
refs: [a-second-crossing-by-one-face-renames-the-first-and-its-pieces, edge-pieces-are-named-by-their-ends]
---


## What

A crossing vertex cites the edge it crosses by that edge's full
published name:
- `Crossing { edge, .. }` and `EdgeCrossing { a, b, .. }` in a boolean or union;
- `CrossingVertex { edge, .. }` in a Split.

When that edge is itself a piece of an earlier cut, its name carries
`Fragment(Ends)`, the names of its two end vertices. Those are crossings,
and each of them cites the edge again. So a piece's name at depth k holds
its parent's name inside each of its two ends, and the parent's ends hold
the grandparent's:

S_k = 2·S_{k−1} + S_{k−2}, about 2.41× (1 + √2) per level.

The sites, at PR 4203's reviewed head `17bd923af2`:
- `crates/editor-core/src/names/emit_topo.rs:1922-1943`: the Split's
  `CrossingVertex { edge: parent.name, .. }`, citing the crossed piece by
  its full name.
- `emit_topo.rs:470-478`: the boolean's `Crossing { edge: pa, .. }` and
  `EdgeCrossing { a: pa, b: pb, .. }`.
- `emit_topo.rs:2480-2514` (`name_edge_pieces`): `Fragment(Ends)` over the
  end vertices' published names, which with `Ends` on every piece of a
  divided edge (`Lone::Piece`) reaches a lone piece too.

## Measured

From the review of PR 4203: a 20×2×1 block, trimmed k times at alternate
ends.

| trims | longest name before (merge-base) | longest name with `Ends` on every piece |
|---|---|---|
| 4 | 31 words | 1,216 words |
| 8 | 31 words | 42,084 words / 495 KB JSON |
| 14 | — | 8.3M words / 98 MB per name; saying the table takes 53 s |

The corpus (`name_words_corpus::NAME_WORDS`, full form) with `Ends` on
every piece:

| | p99 | max |
|---|---|---|
| before | 69 | 111 |
| after | 166 | 718 |

## Already on main

The mechanism needs no lone piece, only a piece named by its ends whose
ends cite the edge it divides. Main has that wherever an edge is cut into
several pieces. The review measured eight slots on main: 16k words /
210 KB.

`Ends` on every piece of a divided edge, a lone one included, carries the
same growth into the commonest shape, a chain of trims, where main's
lone piece stays bare.

## Blocks

`Ends` on every piece of a divided edge, ruled in
`a-second-crossing-by-one-face-renames-the-first-and-its-pieces` (Ev,
PR 4134). It was ruled without this number. PR 4203 lands the crossing's
sense and holds `Ends`-always back until this is settled.

When it lands, it also owes a union seam's lone piece. A seam a later
member cuts to one piece is published as the seam's one edge, bare, and
gains `Ends` when a second piece appears.
