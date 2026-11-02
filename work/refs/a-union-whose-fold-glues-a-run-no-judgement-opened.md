---
id: a-union-whose-fold-glues-a-run-no-judgement-opened
kind: issue
title: A union whose members glue a face run only together refuses: N2's third clause has no exact descent to read
status: open
opened: 2026-11-02
priority: P1
cost: H
---


## What

N2's link rule (`crates/editor-core/src/names/README.md`, "A union's face
is named for its PARENT") has three clauses. The first two, a pairwise
judgement's merge (faces of one member included) and a judgement's cover,
are read by `emit_union`'s `Links::judged`. The third, "the finished body
holds area of both in one face, read exactly", is not built: a union
covers material, so its members together can expose and glue a run of
faces no two of them glue (a closed curved run, such as a sunk boss's
two-piece wall, that only several members together open). The union reads
a finished face's descent off the fold's lineage, which over-approximates,
so `Parents::of` refuses such a face as
`NamingError::UnsupportedUnjudgedGlue`, before the edge and vertex passes.

Building clause 3 means reading, per finished face, which member faces
it holds AREA of, exactly. Two candidate sources:

- **The kernel records it.** `topo::MemberLineage`
  (`crates/topo/src/boolean/nary.rs`) gives a piece of a face everything
  the face descends from, and a merged face the union of its constituents'
  sets, so it lists member faces a piece holds no area of, and it only
  grows along the fold, so a lineage link would make names depend on
  member order. The merge would record, per piece of a merged face, which
  constituents' area it holds.
- **A curved face-holds-point door**: read each member face's area at a
  point of the finished face (cf.
  `work/issues/a-face-holds-a-point-has-no-public-door.md`).

Then the edge and vertex passes need an audit once faces linked by clause
3 reach them: they assume every finished edge is a member edge's piece or
a seam between two parents. One shape that broke this is closed: a seam a
later fold step merged is spelled `Merged` of seams, and `name_by_parents`
now reads a `Merged` edge off every member edge as a seam. Its rows,
`emit_union_borders::a_cap_and_a_bar_over_a_sunk_boss_name_alike_in_every_order`
and `emit_union_borders::a_three_member_intersect_of_a_sunk_boss_names_alike_in_every_order`,
reached `Flush::meeting`'s `unreachable!` ("neither a member edge's piece
nor a seam") through clause 1 in 2 of 6 member orders before that. Still
open: a tied row whose candidates lie partly along member edges and
partly off them falls through `group_member_edges` and would reach that
`unreachable!` (no recipe witnesses it yet). Re-run the audit against the
witnesses below with their faces linked, each in every member order.

## Witness

`crates/editor-core/tests/wire_legal_union_refusals.rs`, each in every
member order, each refusing `UnsupportedUnjudgedGlue` and listing both
boss wall pieces:

- `a_flat_union_of_plate_boss_and_bar_refuses_as_unjudged_glue`: flat
  `Union[plate, boss, bar]`, the bar x 1.4 to 1.6, y 0.5 to 2.5, z 0.8 to
  2.0;
- `two_blocks_that_open_a_sunk_boss_wall_only_together_refuse_as_unjudged_glue`:
  `[Union[plate, boss], lo, hi]`;
- `two_bars_that_sever_a_sunk_boss_wall_only_together_refuse_as_unjudged_glue`:
  two bars over the north wall piece, z 0.8 to 1.25 and z 1.2 to 2.0.
- `two_half_bosses_and_a_bar_refuse_as_unjudged_glue`: the boss
  intersected with a south and a north block (y 1 to 1.5, 1.5 to 2) as two
  members and a bar z 0.4 to 1.6: a glue across members, not of one
  member's faces.

Each flips to naming when clause 3 lands, with one table in every member
order.

**One more thing for the audit (PR 6342 delta review, unsure).** `off_members` reads as a seam any `Merged` edge row that lies in neither `flush.edges` nor `flush.spans`. That would include a `Merged` of a member-edge piece and a seam lying partly along a member edge. For planar members this is argued impossible: a face ends at its own member's edge or under another member's material. Curved members are unchecked. No test reads such a mixed edge.
