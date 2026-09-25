---
id: group-resized-does-not-name-the-cutter-that-stopped-cutting
kind: issue
title: GroupResized states that a fragment group changed size but not which cutter stopped (or started) cutting it, though the prior table's Seam rows name every cutter
status: closed
opened: 2026-09-23
priority: P2
cost: D
closed: 2026-09-25
pr: 3205
---


`Diagnosis::GroupResized { node, was, now }` (`resolve::group_resized`,
`crates/editor-core/src/resolve/mod.rs`) says a vanished fragment
name's group changed size between the last-good and current name
tables. It does not say WHY, and the tables it already reads can say
more: the entities that divided the parent are named in the prior table
at the minting node, as `Seam { a, b }` rows at the fragment
boundaries. Measured on `tests/fixture/pr4.rs`'s sliding union (the
corpus `flip-vanish` row), the prior table at the union carries
`Seam { a: RimEdge(End, seg 0) @ A's extrude, b: CapVertex(End, v 0) @
B's extrude }` and three siblings — the cutter of the vanished ranked
rim edge is B's cap VERTEX, a point on the edge, not a face.

What this row asks for: a field on the arm, say `cutters_gone:
Vec<StableName>`, listing the cutters whose `Seam` row with the parent
is present in the prior table at the minting node and absent in the
current one (and the symmetric `cutters_new` for a group that grew).
On the corpus row it would name B's cap vertex; on the bool7 slot's
y 2.5 / 3.5 collapse (`bool7_shadow_exec::a_collapsed_sideof_group_is_diagnosed_group_resized_and_offers_the_survivor`)
it would correctly name NONE, because the bar's walls still cut the
cap there — a different and equally useful statement.

What makes it more than a scan:

- A seam row can itself be ranked (`Seam{a,b} + Fragment(OrderAlong)`)
  or tied, so matching "the same cutter" is on the `Seam { a, b }` base,
  not the row.
- A cutter can be RENAMED rather than withdrawn (an upstream edit that
  re-qualifies the partner); that reads as one gone and one new, and
  the field's docs have to say so rather than call it a withdrawal.
- Which seams bound WHICH group: a node's table carries seams for every
  group, so the match is on seams whose `a` (or `b`) is the group's
  parent name — for a face group, the seam EDGES; for an edge group,
  the seam VERTICES. Each emitter's seam spelling has to be read, not
  assumed (`names/emit_topo.rs`: `name_boolean_edges`,
  `name_boolean_vertices`, `name_split_edges_vertices`).

A new field on a `Diagnosis` arm is an N5 change, so the PR is `[ev]`.
Filed as C′ of the EMIT group-resized proposal (the unit that landed
`GroupResized`), deferred there by the orchestrator's ruling.
