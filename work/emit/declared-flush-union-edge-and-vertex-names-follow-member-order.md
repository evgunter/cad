---
id: declared-flush-union-edge-and-vertex-names-follow-member-order
kind: issue
title: A declared flush union's edge, vertex and face names follow member order
status: open
opened: 2026-09-23
priority: P1
cost: H
---


## What

`emit_union`'s module header says a union's names record WHICH MEMBER
an entity came from, and not which fold step reached it
(`crates/editor-core/src/names/emit_union.rs`, module doc). What is
pinned today is narrower:
- a disjoint union's face names (`docm3_union::a_members_face_names_are_the_same_first_or_last`);
- the merged rows of a declared chain (`docm8_flat_merged`'s chain rows).

A declared flush union breaks the claim for edges, for vertices, and
for some faces.

## Evidence

Measured 2026-09-23 on `origin/main` at `d2578ac26`, plus PR 3112,
which does not touch these paths. The base document, built from
`docm7_union_declare::block` / `declared_union`:
- `a` = x∈(0,1), `b` = x∈(0.5,1.5), both y,z∈(0,1);
- declared with `flush_pairs((a, a), (b, b))`, which covers both caps
  and both y-walls.

Every order below fuses. The orders give different published tables.

**Edges and vertices: which member was the A operand.** `[a, b]` and
`[b, a]` differ in 40 edge and vertex rows.
- The member folded in first keeps its collinear rim edges, as
  `[FromMember(m, RimEdge(..)), Fragment(OrderAlong { rank, of: 2 })]`.
- The other member's matching rim edges stay whole.
- The corner vertex of one member that lies on the other member's rim
  edge is `[FromMember(m, CapVertex(..))]` in one order, and
  `[Seam { a: <other member's rim edge>, b: <m's cap vertex> }]` in the
  other.

The union's collapse orders a `Seam`'s two sides by name, but it cannot
undo which operand was A.

**Faces: two causes.** PR 3112's review measured differing face rows
across the fused orders of a third-member family over the same pair:

| document | differing face rows |
|---|---|
| `abys` | 4 |
| `fam012` | 16 |
| `fam212` | 16 |
| `fam022` | 4 |

- Most of those rows are one face whose `Fragment(SideOf(..))` lists the
  same partners in a different order. That is the missing `SideOf`
  re-sort in `collapse`, owned by
  `work/emit/name-ordered-positions-in-a-path-have-no-single-home.md`.
- In `fam012` and `fam212` (third member x∈(0.3,0.4) or x∈(1.2,1.3),
  y∈(0.5,2), z∈(-0.5,2.5)), a y = 0 wall is a `Merged` row in some
  orders and a lone `FromMember` face in others. Whether the declared
  merge happens depends on the order.

Remeasured 2026-09-24 once `names::canonical` sorts `SideOf` in the
collapse, over the same probe: `abys` is now the same in both fused
orders. The rest is unchanged. In `fam012` (10 rows), `fam212` (8) and
`fam022` (4), the differing faces list different PARTNERS: a member's
cap, or the merged cap, depending on which step cut the face. None of
them lists the same partners in a different order.

**The same happens without a declaration, once there are three
members.** Measured 2026-09-23 while fixing
`seam-chain-ranks-are-oriented-a-first-so-an-operand-swap-may-reverse-them`.

The document is a plain `Node::Union` of three members:
- `slab` = [0,3]×[0,2]×[0,1];
- `rib` = an inverted U with arms at x∈[0.5,1] and x∈[2,2.5], y∈[0.5,1.5], z 0.5..2;
- a second slab, [0,3]×[0.2,1.8]×[1.7,1.8].

All six member orders fuse. Against `[slab, rib, s2]`:
- `[rib, slab, s2]` publishes the same table;
- `[slab, s2, rib]` and `[s2, slab, rib]` differ in 16 rows;
- `[rib, s2, slab]` and `[s2, rib, slab]` differ in 20 rows.

In THIS document the differences are fold history, not side
orientation. When `s2` reaches the rib first, the rib's caps and its
seam-partner faces carry a `Fragment(SideOf(..))` naming `s2`'s caps.
The later seam with the slab then embeds that fragmented face name. When
the slab reaches the rib first, the same faces are named without it.

That attribution is for this document only; it is not a claim about the
class. A cutter that crosses the seam edges makes the same name bind
different pieces across member orders, because the seam's pieces are
ranked by two different rankers. That case is its own row:
`work/emit/union-seam-edge-ranks-follow-which-step-split-the-seam.md` (P0).

The union of three disjoint-ish overlapping blocks
(`[0,1]³`, `[0.5,1.5]×[0.2,0.8]×[0.2,0.8]`, `[1.2,2.2]×[0.1,0.9]×[0.1,0.9]`)
is identical in all six orders.

## Why it matters, and why P1 rather than P0

A downstream reference to one of these entities breaks when the author
reorders the members. That dependence is exactly what the node exists
to remove. The review measured **zero** cases of a name rebinding to a
different point across orders. A reference therefore vanishes, typed,
and never silently lands on the wrong entity.

The fix lies somewhere in three places:
- the pair emitter's fused-edge rule;
- a canonical choice on the union's side;
- for the `SideOf` rows, the class row's single canonicalizer.

Nobody has measured which fixes what.

## Found by

The order sweep for `seam-junction-vertex-name-cannot-be-collapsed-by-the-union-fold`,
widened by PR 3112's review.
`crates/editor-core/tests/emit_seam_junction.rs` asserts that only the
junction rows are equal across orders, for this reason.

## Correction (EMIT, 2026-09-23)

The "zero cases of a name rebinding" measurement above compared vertex
points only. With edges compared by their end points, 18 of the 162
pairs of fused orders in PR 3112's review corpus rebind a name on main
(24 names, all `FromMember` rim-edge pieces). That is its own P0 row:
`work/emit/union-rim-piece-ranks-follow-fold-order-so-rim-names-rebind.md`.

## After `emit/rim-piece-ranks` (EMIT, 2026-09-24)

The published table now numbers each member edge's pieces by the cells
the finished body's vertices cut that edge into, counting cells another
member holds (`emit_union::rank_member_edges`), and a seam vertex cites
the member edge it lies on whole (`emit_union::cite_member_edges`).

Measured with #3168's review probe: PR 3112's review corpus plus the
review's fixtures (`r1two`, `r1three`, `r1flush`, `r2ends`, `r2endsg`,
`r3nest`, `r3nest2`), every member order, 226 fused cells, comparing
vertices, edges and faces. No pair of fused orders rebinds a name, and
no cell that fuses on main refuses.

An earlier draft of that branch ranked only the pieces a member keeps,
and this note then said a name "no longer lands on the wrong piece".
That was wrong: which member keeps a flush stretch depends on order, and
`r2ends` (a member flush with two partners) still rebound 8 names in a
pair of orders that both fuse on main.

What this row describes is still there. 295 of the corpus's 336 pairs
of fused orders publish different name SETS: a flush stretch is named
for whichever member was folded first, and so are the `Seam` vertices
and edges, member vertices and `Merged` faces around it. A reference
vanishes, typed, on a reorder.
