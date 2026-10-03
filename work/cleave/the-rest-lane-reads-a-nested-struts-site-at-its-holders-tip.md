---
id: the-rest-lane-reads-a-nested-struts-site-at-its-holders-tip
kind: issue
title: The REST lane reads a nested dangling null edge's site at its holder's tip, where no real edge leaves
status: open
opened: 2026-10-03
priority: P2
cost: E
---


## What

At a vertex that several crossing pairs cut, a dangling null edge
whose segment another's holds whole is minted at that one's tip
(`insert::mint_directed`, `crates/topo/src/boolean/insert.rs`), so
its `BoolNullEdgeRecord::at_vertex` is the tip, a vertex that only
null edges leave. `rest::enumerate_segments`
(`crates/topo/src/boolean/rest.rs`) takes each pair's site from
`at_vertex`, and `arc_along` there finds no arc (`Ok(None)`), so the
germ falls to the straight test. `undo_struts` needs the tip: it is
the end the edge was hung at.

No witness puts a nested strut in the REST lane. The witnesses for
nested struts (`pit_holding_spikes` and `lens_in_a_lens` in
`crates/topo/tests/union_flush_onto_edge_contact.rs`) have no
opposite-oriented flush faces.

## Owed

Find a witness with a rest contact beside a nested strut. If the
straight test mispairs it, read the arcs from the vertex the chain of
null edges hangs from.
