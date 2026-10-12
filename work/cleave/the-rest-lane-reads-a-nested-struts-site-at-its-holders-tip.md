---
id: the-rest-lane-reads-a-nested-struts-site-at-its-holders-tip
kind: issue
title: The REST lane reads a nested dangling null edge's site at its holder's tip, where no real edge leaves
status: closed
opened: 2026-10-03
priority: P2
cost: E
closed: 2026-10-08
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

## Re-pointed from the D10 hold (2026-10-08)

Waits on `intent-stage4-is-built`, not on the whole program: rest::enumerate_segments is the declared-REST zip, which stage 4 deletes (the-declared-rest-zip-retires-at-stage-4-and-the-join-needs-three-arms); a witness needs a declared Rest contact. (INTENT's re-homing of the parked rows, `work/intent/log.md`.)

## Closed (2026-10-08, INTENT stage 4 A (`intent/s4-a-join`))

`boolean/rest.rs` is deleted, and the join builds every union it built (125 of the door's 185 openings across the topo, sweep and editor-core suites; the other 60 were its declines and refusals, which the join now builds sound or refuses with its own answer). The nested-strut scene builds there: `rest_nested_strut.rs`'s `a_pinch_apex_meeting_one_vertex_builds_in_either_order` (both orders, additive volume, tiers 2, 3 and 3′).
