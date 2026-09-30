---
id: a-slot-across-a-sunk-cylinder-boss-refuses-duplicate-merged-face-name
kind: issue
title: Subtracting a slot across a cylinder boss sunk into a plate refuses Naming(Duplicate) on a Merged face name, an emission bug on a legal recipe
status: closed
opened: 2026-09-30
priority: P0
cost: M
closed: 2026-09-30
branch: emit/sunk-boss-slot-duplicate
refs: [a-merged-face-with-several-same-side-constituents-has-no-chord-rule, curved-seam-pieces-have-no-ranking-direction]
---


## Closed — the faces name; the recipe now refuses the chord rule

`emit_topo::name_boolean` minted one bare `Merged(set)` per merge
group, so two merge groups listing one constituent set collided at
insert. The set is the merged face's parent (N3), and a parent held as
several faces qualifies each with `Fragment(Borders)` over the divider
walls it borders (N2). The merge pass now collects the merged faces by
set: a set held by one face keeps the bare name, and a set held by
several goes through `name_parent_faces`, with the parent's other
faces (its operand faces' unmerged pieces and other merges listing
them) passed as `merged`. Here the two wall faces are
`Merged([.. Piece(0), .. Piece(1)])` + `Borders` of the slab's x = 1.4
wall and of its x = 1.6 wall.

The recipe still refuses, one pass later:
`NamingError::MergedChordConstituents`. The slab wall's seam chord
borders a merged face that holds two faces of the boss on the side the
chord reads through to. That is the missing rule
`work/wire/a-merged-face-with-several-same-side-constituents-has-no-chord-rule.md`
owns, and this recipe is filed there as a new witness.
`emit_union_borders::a_slot_across_a_sunk_boss_divides_its_merged_wall_by_the_slot_walls`
pins the refusal for the pair boolean and the union in both member
orders; it is red on `91c2e13013`, where they refuse `Duplicate`.

The slab turned 90° refuses `SplitReference { curved: true }`, which
`curved-seam-pieces-have-no-ranking-direction` already records.

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
