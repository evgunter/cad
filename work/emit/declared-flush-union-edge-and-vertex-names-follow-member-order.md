---
id: declared-flush-union-edge-and-vertex-names-follow-member-order
kind: issue
title: A two-member declared flush union names 40 edge and vertex rows differently in its two member orders
status: open
opened: 2026-09-23
priority: P1
cost: H
---


## What

`emit_union`'s module header states that a union's names record WHICH
MEMBER an entity came from and not which fold step reached it
(`crates/editor-core/src/names/emit_union.rs`, module doc). For FACES
that holds and is pinned (`docm3_union::a_members_face_names_are_the_same_first_or_last`,
the `docm8_flat_merged` chain rows). For EDGES and VERTICES of a
declared flush union it does not.

## Evidence

Measured 2026-09-23 on `origin/main` at `d2578ac26` plus the seam
junction fix (`emit/seam-junction`), which does not touch this path.
The document: `a` = x∈(0,1), `b` = x∈(0.5,1.5), both y,z∈(0,1),
declared with `flush_pairs((a, a), (b, b))` (both caps and both
y-walls) — the `docm7_union_declare::block` / `declared_union`
fixture. `[a, b]` and `[b, a]` both fuse, and their published tables
differ in 40 rows. Shape of the difference, member ids `a` = 2,
`b` = 5:

- in one order, `a`'s four rim edges along the y-walls are
  `[FromMember(a, RimEdge(..)), Fragment(OrderAlong { rank, of: 2 })]`
  and `b`'s are whole `[FromMember(b, RimEdge(..))]`; in the other,
  the roles swap;
- the corner vertex of one member lying on the other's rim edge is
  `[FromMember(m, CapVertex(..))]` in one order and
  `[Seam { a: <other member's rim edge>, b: <m's cap vertex> }]` in
  the other.

The same split shows in any document built on that pair: with a third
member `y` = y∈(0.3,0.4) slab (x∈(-1,2), z∈(0.5,3.5)), the two orders
that fuse (`[a, b, y]` and `[b, a, y]`) give two distinct tables.

## Why it matters

A downstream reference to an edge or vertex of a declared union moves
(or vanishes) when the author reorders the members, which is the
dependence the node exists to remove. The pair emitter keeps the A
operand's collinear edge and fragments it, and names the other
operand's corner as an edge × vertex seam; the union's collapse
canonicalizes a `Seam`'s two sides but cannot undo which operand was
A. The fix is probably at the pair emitter's fused-edge rule or a
union-side canonical choice; it is not measured which.

## Found by

The order sweep for `seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold`
(`crates/editor-core/tests/emit_seam_junction.rs` asserts only the
junction rows equal across orders, for this reason).
