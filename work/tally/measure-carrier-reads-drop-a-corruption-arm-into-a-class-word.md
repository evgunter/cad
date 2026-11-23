---
id: measure-carrier-reads-drop-a-corruption-arm-into-a-class-word
kind: issue
title: eval/measure.rs's carrier reads drop a ReadbackError and a LoopEdgesError into a Carrier::Other class word
status: open
opened: 2026-10-29
priority: P4
cost: E
---


Found by CLEAR's CLEARTEXT sweep for destroyed causes, which covered
`crates/editor-core/src/eval/measure.rs` (TALLY's territory by
`scripts/work.py territory`) and left it alone.

`eval/measure.rs`'s carrier read (`carrier_of`, the arm building a
`Carrier` from an `EntityKey`) destroys two typed causes:

- `EntityKey::Vertex`: `topo::readback::vertex_point(body, v)` answers
  `Err(_) => Carrier::Other("an unreadable vertex")`, dropping the
  `ReadbackError` (which names the dangling entity).
- `EntityKey::Face`: `reach_of` (`eval/measure.rs`, also read by
  `checks/at_rest.rs` and `mate/reach.rs`) reads each loop with
  `topo::props::loop_edges(body, lk).ok()?`, dropping a
  `LoopEdgesError` (`Corrupt { what }` or `NullScaffoldEdge { edge }`)
  into `Carrier::Other("a face with no readable boundary")`.

Both class words are honest kinds, so a reader is not sent at the
wrong argument; what is lost is the detail of a corruption arm, which
names a kernel defect when it fires. Low priority for that reason.
The two `map_err(|_| at_the_axis(..))` sites beside them destroy
nothing: `topo::AtTheAxis` is a unit struct.
