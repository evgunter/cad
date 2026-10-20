---
id: union-naming-refusals-a-correct-kernel-can-reach-have-no-unruled-word
kind: issue
title: Six union-naming refusals a correct kernel can reach still speak NamingError::Emission's kernel-defect sentence, for want of an Unruled word
status: open
opened: 2026-10-20
priority: P2
cost: M
---

Filed by WIRE's `wire-emis-union` lane (branch `wire/emission-union`),
from its share of `names-emission-raises-panic-or-take-their-true-ending`,
its pass over the `NamingError::Emission` raises of
`names/emit_union.rs`, `join_names.rs`, `borders.rs`, `canonical.rs`
and `defer.rs` under D9 as Ev rewrote it on PR 5411.

## What

Each raise below is one a correct kernel can reach: a legal recipe
arrives at a shape no naming rule covers. D9 gives it row 2: a refusal
in the `Unruled` category (`names::emit`'s `Category`), ending in
`NOT_YET_ENDING`. Each still raises `NamingError::Emission`, whose
sentence says the kernel is defective. None is reached by a test or a
filed document today, so none is known reach. The fix is a variant
(or several) in `names/emit.rs`, which belongs to another of the three
lanes on the parent row, so this lane did not add one.

- `join_names::name_joins`, `Member::Outright`: a door's join takes an
  edge it minted outright (an end arc, a mitre, a slit, a remnant).
  `names/README.md` states the refusal as the rule ("a join that takes
  one is refused"). Nothing in `sweep::BlendNaming::edge_joins` or
  `topo`'s shell record rules out such a join.
  `emit_blend::tests::a_join_over_an_edge_the_blend_minted_outright_refuses`
  plants the record.
- `emit_union::Flush::meeting`, "one member edge meets several faces of
  other members and no seam at a union vertex". The argument in the code
  that no manifold union reaches this covers planar faces only.
- `emit_union::cite_member_edges`, `CITED_GROUP_NO_MEMBER_EDGE`. This is
  the sibling of `CITED_GROUP_NOT_ONE_SEAM`, which a legal document
  reaches
  (`work/wire/a-bridge-met-with-the-wedges-and-the-other-bridge-names-vertices-on-no-single-seam-alike.md`):
  the vertices sharing a seam name have no member edge to be ranked
  along.
- `emit_union::Parents::cited`, "a name a union publishes cites faces of
  several parents". A tied member face is keyed in `of_member` to the
  parent of each of its candidates. Where those candidates go to
  different parents, a name that embeds the face cites several.
- `emit_union::collapse_one`, `Unrankable::RunLeastLineMoved`. The
  collapse maps fold-space names to member-space names, and that map
  does not preserve name order. So a ranked run's least line, or the
  order of a seam-only run, can change with the member order
  (`canonical::RankRule::along_run`).
- `emit_union::collapse_one`, `JUNCTION_LINES_COLLIDE`. Two fold-space
  seam lines between the same two member faces, met at different steps,
  collapse to one line (`collapse_table`'s doc says this happens for
  seam EDGES). A junction whose run holds both of them collides.

## Done when

Each of the six is either measured to be unreachable, and then panics
under row 4's message, or refuses under an `Unruled` variant that names
the shape it stopped at.
