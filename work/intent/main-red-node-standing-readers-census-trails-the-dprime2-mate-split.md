---
id: main-red-node-standing-readers-census-trails-the-dprime2-mate-split
kind: issue
title: main red: node_standing's READERS census trails PR 5613 (D′2): mate/corner.rs and viewer/src/session.rs read node results unlisted, mate/pin.rs is listed and gone, wire.rs reads one fewer
status: parked
opened: 2026-10-25
priority: P1
cost: E
rides_with: assemble-is-an-operator-and-the-product-is-a-slot
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot]
---



Found by FLUX (unit F2) while separating main's reds from PR 5668's, and
filed at the orchestrator's request.

## Measured

`crates/editor-core/tests/node_standing.rs`'s
`every_node_result_reader_goes_through_usable_or_is_listed` is red on
origin/main at `a4f59ff66d` (`node_standing.rs:591`). The census of
node-result reads differs from `READERS` in four files:

| file | reads found | `READERS` lists |
|---|---|---|
| `crates/editor-core/src/mate/corner.rs` | 3 | absent |
| `crates/viewer/src/session.rs` | 3 | absent |
| `crates/editor-core/src/mate/pin.rs` | absent | 3 |
| `crates/editor-core/src/eval/wire.rs` | 9 | 10 |

Bisected on main's first-parent line: green at `5ae8cf0833^1` and red at
`5ae8cf0833` (PR 5613, `intent/dprime2-mates-read-geometry`). That PR
added `mate/corner.rs`.

## What closes it

Each new reader reads through `Evaluation::usable` or gets its `READERS`
line with a reason. The `pin.rs` line goes, and the `wire.rs` count moves
to the count the census finds.
