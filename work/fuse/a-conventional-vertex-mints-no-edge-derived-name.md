---
id: a-conventional-vertex-mints-no-edge-derived-name
kind: issue
title: Naming a conventional vertex refuses typed: its edge-derived name, the ranking skip and the modulo-position order rows are unbuilt
status: parked
opened: 2026-10-07
blocked_on: [a-sphere-cut-in-cap-fails-its-names-missing-upstream]
---


## The finding

The naming half of `curved-joinable-vertices-are-left-unjoined`'s
"Ruled (Ev, PR 4198)" section is guarded, not built. A table that
names a conventional vertex refuses typed: `emit::check_total` raises
`NamingError::ConventionalVertex` (pncad-py tag
`conventional_vertex`), naming this row, for any name but a
construction's own `MeridianVertex(Seam)`
(`emit::conventional_vertex_tests`). What is unbuilt:

- the conventional vertex (`topo::is_conventional_vertex`) mints no
  member- or position-citing name; where `emit::check_total` needs one,
  it is derived from its edge alone and resolves to the edge;
- ranking and citation skip it (`emit_union::Flush::least_vertex`, the
  crossings);
- EMIT's order rows (`emit_union_flush_names::the_zip_document_builds_one_vertex_set_in_every_order`
  and its siblings) compare conventional vertices modulo position
  through one helper;
- a pick at the vertex resolves to the edge;
- a union's curved edge is read by the flush rule
  (`emit_union::Flush` reads straight edges only; a pair boolean's
  curved joined edge is read along its carrier, `emit_topo::Track`).

The guard leaves a construction's own rim self-loop vertex
(`MeridianVertex(Seam)`) its name.

The edge-derived name is a **vocabulary addition**: a new `RoleSeg`,
with its content tag, its words, its nested references and a names
README clause. The ranking skip and the order-row helper compare a
vertex by that name, so they land with it.

## Why it waits

No document reaches it. A probe over the whole editor-core `ci` suite
on the curved join's branch found no closed join at all; the one
construction known to leave a conventional vertex — a sphere cut-in
cap, the snowman two-cut-ins shape as a document — fails EMIT upstream
of the join (`work/emit/a-sphere-cut-in-cap-fails-its-names-missing-upstream`),
measured with the joins disabled.
A cylinder's or a dome's cap keeps its seam strut's vertex on the rim,
so it never goes conventional. The vocabulary is not worth adding
unreached while the guard refuses loudly. The witness the ruling's
build listed — a cap on two flush blocks in all six member orders,
names equal modulo the conventional vertex — is built with it.
