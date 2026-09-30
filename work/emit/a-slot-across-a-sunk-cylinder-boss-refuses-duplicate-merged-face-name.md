---
id: a-slot-across-a-sunk-cylinder-boss-refuses-duplicate-merged-face-name
kind: issue
title: Subtracting a slot across a cylinder boss sunk into a plate refuses Naming(Duplicate) on a Merged face name, an emission bug on a legal recipe
status: open
opened: 2026-09-30
priority: P0
cost: M
---


## What

A slot cut across a cylinder boss that is sunk into a plate refuses
`Naming(Duplicate)`. That is the emission-bug category, and the recipe
is legal. The duplicated name is a merge of the boss wall's two
lateral pieces:

    Merged([FromA(FromB(<cyl> Lateral Piece(0))), FromA(FromB(<cyl> Lateral Piece(1)))])

The slot leaves two wall faces, one on each side, and each merges the
same two cosurface pieces. Both come out under that one name.

## Evidence

Found by a scratch probe (not committed) while looking for a fixture
for `curved-seam-pieces-have-no-ranking-direction`. It uses the
helpers in `crates/editor-core/tests/emit_union_borders.rs`:
- plate `block((0,3), (0,3), 0.0, 1.0)`;
- boss `cylinder([1.5, 1.5, 0.5], [1,0,0], [0,1,0], 1.0)`, radius 0.3
  via `circle_split(.., 2, ..)`, z from 0.5 to 1.5;
- `u = Union(plate, boss)`, which evaluates;
- slab `block((1.4, 1.6), (-1, 4), 0.8, 1.2)`;
- `Subtract(u, slab)` refuses the `Duplicate` above.

The same slab turned 90° (x in [-1, 4], y in [1.4, 1.6]) instead
refuses `SplitReference` at the seam chain.

## Fix direction

Not investigated. N3's merge policy gives a merged face its
constituents' name. Two disjoint merged faces with the same
constituents need a discriminator, such as the `Borders` rule a split
face's pieces get, or the merge should not pair pieces across the slot.
