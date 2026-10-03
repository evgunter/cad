---
id: edge-edge-membership-reads-a-reflex-dihedral-wedge-as-convex
kind: issue
title: Edge-edge membership decides a germ by a convex-wedge test, which holds only within a half-turn: a reflex dihedral wedge along a coincident edge is read wrong
status: open
opened: 2026-10-03
priority: P1
cost: M
refs: [reflex-corner-vertex-vertex-sites-refuse-under-a-tilted-cap]
---


(Found by the review of PR 3900 (`join/reflex-corner-review`, Style
S8), filed by the reflex-corner lane. Not measured.)

## What

`boolean/recl.rs` `resolve_edge_edge` decides an edge-edge germ by
membership: a germ exists iff exactly one of A's two flanking
representatives lies inside B's dihedral wedge about the common line.
"Inside" is the convex-wedge test, In against both of B's flanking
planes. That holds only for a wedge within a half-turn. Its own doc
says so: "reflex dihedral wedges along a coincident edge are not yet
discriminated — the A/B symmetry check refuses loudly if it bites".

It is the same flaw the strut order had before PR 3900: a
half-turn-local reading applied to a sector that can reach past a
half-turn. The 315° reflex corner's vertical edge is such a wedge
(315° of material about it), and the reflex probe puts the other
operand's edges on it.

## Why P1

Nothing measures whether the A/B symmetry check catches every misread.
A reflex wedge read as convex calls a representative outside that is
inside. If both solids misread consistently, the check passes and the
germ's existence is wrong, which can build a wrong body. Unmeasured
either way.

## What the taker owes

- A pose that puts an edge of one operand along a reflex dihedral
  edge of the other: the reflex prism's vertical corner edge with a
  prism sharing it, at several wedge angles on both sides of a
  half-turn.
- Its measurement against the closed form.
- The membership test read by the wedge's actual extent (the
  representative's side of each plane, and which side the wedge
  spans), or a typed refusal where the wedge is reflex.
