---
id: union-in-every-order-helpers-are-copied-six-times
kind: issue
title: The 'union in every member order' test helper is written six times across editor-core's union suites
status: open
opened: 2026-10-14
priority: P3
cost: E
---


## What

Six editor-core suites each write their own "build the union in every
member order" helper, each with its own permutation generator:

- `tests/wire_legal_union_refusals.rs`: `orders`, `every_order`;
- `tests/union_pinch_member_order.rs`: `orders`, `every_order`;
- `tests/emit_union_member_order.rs`: `orders`, `union_of`;
- `tests/emit_shared_rim_several.rs`: `permutations` (shared with
  `emit_union_rim_piece_ranks`'s `runs`);
- `tests/emit_union_flush_names.rs`, through `emit_union_rim_piece_ranks`'s
  `runs`;
- `tests/emit_union_curved_flush.rs`: `every_order`, over
  `emit_shared_rim_several::permutations`.

## Next

One helper in `tests/fixture/mod.rs` (the union over `members` in each
order through `fixture::union_over`, evaluated, with its order label),
and the six suites read it. `tests/fixture/` is compiled by
`crates/viewer` by path, so the viewer must still build.
