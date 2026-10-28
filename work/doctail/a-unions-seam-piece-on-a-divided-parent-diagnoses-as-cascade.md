---
id: a-unions-seam-piece-on-a-divided-parent-diagnoses-as-cascade
kind: issue
title: A vanished union seam piece whose side parent is held as several faces diagnoses as a Cascade through that parent
status: open
opened: 2026-10-28
priority: P3
cost: M
---

Filed by WIRE's `wire-unionrecord` lane, building a group-size witness
for a union's tied seam. Measured on a document; not yet pinned by a
test.

## What

A union names a seam edge `Seam` of the two PARENTS on its sides, each
spelled under the union's own id (`emit_union.rs` `name_by_parents`).
A parent the finished body holds as several faces is never a row: its
faces are published as `Fragment(Borders)` pieces of it. So when a
piece of such a seam vanishes, `resolve/mod.rs`'s cascade rung
(`vanished_diagnosis`'s `first_vanished` walk, `lookup(new.eval, inner)
.is_none()`) finds the side's parent name in no table of the current
run and answers `Cascade { through: <that parent> }`, though the parent
did not vanish (it is held as pieces in both runs, and was never a row
in the last-good run either). The group-size rung below it is never
asked.

A pair boolean's seam does not meet this: its sides are operand names,
which are rows of the operand's table.

## Reproduction

`fixture::u_cutter_tie`'s subtract `sub`, a block D at x ∈ [3, 3.5],
y ∈ [0.5, 3.5], z ∈ [0.5, 1.5] standing on both prong floors (as in
`lib_g14_split_walls::a_unions_seams_of_a_tied_face_tie_under_the_bare_name`),
and a post x ∈ [2.8, 3.2], z ∈ [0.5, 1.4] on each floor across D's
wall x = 3 (y ∈ [1.2, 1.3] and [2.7, 2.8]), all in one `Union`. Lifting
the posts 0.6 off the floors makes each floor's seam with D's wall whole
again, and each former piece resolves `Vanished` with
`Cascade { through: <the floor, as the union's parent> }`: D divides
each floor in two. With D moved to the prongs' mouth (x ∈ [3.5, 4.5]),
the floors stay whole and the cascade runs through D's wall instead,
which shows in each prong. With D in one prong's mouth only (the
`resolve_group_membership::a_unions_seam_of_a_tied_parent_counts_its_pieces`
document), neither side is divided and the same edit reaches
`GroupResized { was: 2, now: 1 }`.

## What it needs

The cascade rung's walk should not blame a name the minting node cites
as a parent it holds in pieces, the way it already exempts a line
(`cited_lines`) and a name retired into a live merge
(`retired_into_a_live_merge`). Then a row pinning the reproduction
above at `GroupResized`.
