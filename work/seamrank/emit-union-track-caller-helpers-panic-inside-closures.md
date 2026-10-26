---
id: emit-union-track-caller-helpers-panic-inside-closures
kind: issue
title: emit_union.rs's #[track_caller] lookups member_at, member_by_index and Parents::face panic inside unwrap_or_else closures, so the panic reports the helper's line
status: open
opened: 2026-10-21
priority: P4
cost: E
---


Found by `wire-emithelpers`'s sweep (branch `wire/emit-helpers-row-four`), 2026-10-21. Three `#[track_caller]` lookups in `crates/editor-core/src/names/emit_union.rs` raise their `unreachable!` inside an `unwrap_or_else` closure:
- `member_at` (`emit_union.rs:515`);
- `Flush::member_by_index` (`emit_union.rs:739`);
- `Parents::face` (`emit_union.rs:2446`).

A closure is not `#[track_caller]`, so the panic reports the helper's own line, not the caller's, and the attribute buys nothing. PR 5503's review named the shape. The repair is a `let … else` or a `match`, the same one `wire/emit-helpers-row-four` applied to `names/plane_pieces.rs`'s `piece_face` and `loop_half_edges`.

The sweep: a script over every `#[track_caller]` fn body in `crates/editor-core/src` for `unwrap_or_else|map_or_else|ok_or_else|and_then|map(|…| … unreachable!|panic!`. That pattern cannot match a panic in a named helper fn the closure calls, or `.expect(…)` (which is `#[track_caller]` itself, and so is fine).

## Re-checked (2026-10-26, tracker sweep)

Still real on main: `member_at` (`emit_union.rs:518`),
`Flush::member_by_index` (`:742`) and `Parents::face` (`:2513`) still
panic inside `unwrap_or_else`. Priced P4 E: the repair is written above
(a `let … else`), hygiene that moves no name.
