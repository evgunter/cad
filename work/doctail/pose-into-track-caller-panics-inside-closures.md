---
id: pose-into-track-caller-panics-inside-closures
kind: issue
title: edit.rs's #[track_caller] pose_into panics inside unwrap_or_else and map_or_else closures, so the panic reports pose_into's line
status: open
opened: 2026-10-21
---


Found by `wire-emithelpers`'s sweep (branch `wire/emit-helpers-row-four`), 2026-10-21. `pose_into` (`crates/editor-core/src/edit.rs:1760`) is `#[track_caller]`, but it raises its `unreachable!` inside a `def.kept().unwrap_or_else(|| …)` closure, and its `.expect` inside a `map_or_else` closure. The closure is not `#[track_caller]`, so the panic reports `pose_into`'s own line, not its caller's. PR 5503's review named the shape. The repair is a `let … else` or a `match`.

This is pose ground, which is placement-adjacent, so WIRE left it alone under the D10 hold. Check that hold before taking this row.
