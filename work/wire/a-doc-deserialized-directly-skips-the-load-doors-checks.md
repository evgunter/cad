---
id: a-doc-deserialized-directly-skips-the-load-doors-checks
kind: issue
title: Doc derives a public serde::Deserialize, so a Doc built with serde_json::from_str skips the load door's checks and can reach panics the doors hold
status: closed
opened: 2026-10-20
priority: P2
cost: M
closed: 2026-10-21
---


Found by PR 5457's FULL review (`wire-evalarms-rev`, 2026-10-20), by
inspection, not by execution.
- `crates/editor-core/src/doc.rs` ~796 derives `serde::Deserialize` on
  `Doc`, publicly. A `Doc` built by `serde_json::from_str` skips
  `persist/check.rs`'s checks (`check_id_shape` ~2005, and the rest). Its
  `evaluate` can then reach the row-4 panics that hold invariants the
  insert, edit and load doors keep:
  - `ProfilePieces::publish` (`eval/anchor.rs` ~162–188);
  - `wire.rs` ~1788, ~2906 and ~4812.
- D9 as rewritten on PR 5411: no input should lead to a known panic. A
  public deserializer is an input door.
- Repair: make the derive crate-private (route deserialization through
  the load door only), or have `Deserialize` validate as the load door
  does. Check pncad-py and the viewer for direct `serde` use of `Doc`
  first.

Closed by PR 5514 (2026-10-21). `Doc` derives `Serialize` only, and the load door parses through a private `SnapshotWire`. A `compile_fail` doctest witnesses that `serde_json::from_str` into a `Doc` no longer compiles. The review showed that load is byte-identical across every saved file in the tree and 66 mutations of them.
