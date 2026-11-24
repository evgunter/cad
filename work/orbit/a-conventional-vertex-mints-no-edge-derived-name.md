---
id: a-conventional-vertex-mints-no-edge-derived-name
kind: issue
title: Naming a conventional vertex refuses typed: its edge-derived name, the ranking skip and the modulo-position order rows are unbuilt
status: closed
priority: P0
cost: H
opened: 2026-10-07
closed: 2026-11-24
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

## 2026-10-13: a document reaches it (EMIT, branch `emit/naming-refusals`)

`work/emit/a-sphere-cut-in-cap-fails-its-names-missing-upstream` is
closed. The lens ∩ tilted brick cap, in either slot, and the brick
minus the lens now name every face and edge, and refuse only
`ConventionalVertex` at the circle's one vertex
(`crates/editor-core/tests/emit_sphere_cut_in_cap.rs`,
`a_sphere_cut_in_refuses_only_where_its_rows_are_filed`, now
`every_op_over_a_sphere_cut_in_names_the_circles_vertex_for_the_circle`). The union and the
lens minus the brick keep the cut unglued
(`work/hone/a-sphere-cut-in-survives-unglued-in-the-union-of-a-lens-and-a-tilted-brick`).
When this lands, those three rows publish a table, and the pin gains
what it cannot assert today: the lens entities the cap's names cite,
the faces the cut divided among them.

Moved from SEAMRANK to BORDERS on 2026-11-20, by `git mv`, when SEAMRANK merged into BORDERS: SEAMRANK held no P0 or P1 row, so no sitting would dispatch it on its own (`work/README.md`, Track size), and its charter is one BORDERS carries.

## 2026-11-23 — the die and every cut-in op now refuse here (ORBIT)

Branch `orbit/transient-chart-cut` closes the sphere cut-ins' chart cuts
in the re-entered op's output stage (`topo::boolean::slit::close_cuts`),
so no result holds a chart cut. The section circle each cut crossed
then keeps one conventional vertex. Every op over the lens and the
tilted brick (`editor-core` `emit_sphere_cut_in_cap`) and over the ball
and the brick (`emit_ball_cut_by_brick`), in both orders, now refuses
`ConventionalVertex` here. Before, the unions and the sphere-first
differences refused at the surviving cut. So do the tour's die with
two pips dragged together, all eight edits
(`the_tours_die_with_two_pips_run_together_refuses_at_the_circles_vertex`,
now `the_tours_die_with_two_pips_run_together_builds`):
the kernel body under that union is sound and at its closed form.
This row now stands between a normal verb on the die and its names.

## 2026-11-24 — claimed by ORBIT, re-banded P0 (the die)

With ORBIT's PR 7602 (one sphere escape) and PR 7664 (the transient chart cut), the tour's die with two pips dragged together now BUILDS at the kernel: each ball is one face, at 2V − lens. All eight edits of `die_composed_tour` refuse at the union's naming with `NamingError::ConventionalVertex`, the section circle's conventional vertex. That is this row. The lens ∪ brick document (`emit_sphere_cut_in_cap`, `emit_ball_cut_by_brick`) refuses the same way under every op. This row now stands between a normal verb on normal geometry and its names, so it takes P0 (`work/README.md`, Priority: a blocker inherits the band of what waits on it). Moved from `work/borders/` (BORDERS is `ready`); id unchanged. The design is Ev's ruling on PR 4198, cited above. (ORBIT orchestrator)

## Closed (PR 7694, `orbit/conventional-vertex-names`)

Built as ruled. `NamingError::ConventionalVertex` and its pncad-py tag
`conventional_vertex` are retired. `emit_union::Flush::least_vertex`,
cited above, is no symbol on the branch or on main: the citation skips
landed in `emit_union::Flush::member_vertices` and
`emit_topo::name_edge_pieces`.

- **The name.** A conventional vertex is `ConventionalVertex(edge)`
  under its closed edge's node. Every emitter and every evaluation door
  that publishes a table finishes it through one door,
  `names::emit::finish`: the pass, then `check_total`. `check_total`
  refuses any vertex that carries a conventional vertex's name and is
  not one. Ties yield to the edges' names.
- **Nothing cites it.** A piece's `Ends`, the union's member-vertex
  reading, and the pair boolean's operand-vertex carry
  (`emit_topo`'s `named_in`) all skip it. So a later cut through its
  point is named as a crossing of the edge's interior, as a cut a hair
  away is (`a_cut_through_a_conventional_vertex_reads_its_edge`). A
  split cannot cut a sphere face yet, so its carry is unreached.
- **Resolution.** A reference to its name and a pick at it resolve to
  the edge (`names::resolves_as`). Listings and selectors skip it
  (`names::binds`). One predicate reads a conventional vertex's name
  through every wrapper (`emit::conventional_vertex_edge`).
- **Order rows.** These compare a conventional vertex modulo position
  through `fixture::point_modulo_position`. The witness is
  `emit_union_conventional_vertex_orders`.
- **Documents.** The die's union and its cube-less-union build and name
  in all eight edits, and so does every op of the lens ∪ brick and ball
  ∪ brick documents. Past the union, the die's fillets refuse at their
  own doors: in six edits a `Vanished` selection, in two the clearance
  refusal (`the_tours_die_with_two_pips_run_together_builds`).
