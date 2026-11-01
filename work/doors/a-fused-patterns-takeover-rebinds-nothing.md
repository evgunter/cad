---
id: a-fused-patterns-takeover-rebinds-nothing
kind: issue
title: A fused pattern takes the prototype's copy over but rebinds nothing: the union carries each face once per member, so no face has a successor
status: open
opened: 2026-11-01
priority: P3
cost: M
design: true
---


Found by the PR 6264 re-verify (probe `rv_a_fused_pattern_reseats_the_mate_on_the_copys_top`, landed `#[ignore]`d on this row in `crates/viewer/tests/takeover_reseats.rs`). The fused pattern gesture (`DocSession::add_pattern`'s `PatternOutputChoice::Fused` arm, `crates/viewer/src/session.rs`) takes the prototype's world copies over with `Union { members: Bodies::Family(row) }`, where `row` is a placement of the prototype read at the pattern's index. The takeover door (`crates/editor-core/src/take_over.rs`) reads where the copy's names went off the union's table, by the prototype's read (`names::successors`).

The union reads the family, not the prototype, and carries each face of the prototype once per member: the block's top is

`From { read: <the union's family read>, of: <row's name> [Member { at: [(k, 0)], of: <block's top> }] }`

and the same with `(k, 1)`. Neither row is a carry of the prototype's read, and the top is held by two rows, so the top has no successor and nothing is rebound: a mate on the copy's top goes stale, and the mated block's placement fails (`SideUnevaluated`).

**Decide when dispatched:** whether a takeover by a fused pattern re-seats a copy's names on the member at the prototype's own index (the first member, which stands where the prototype stood) — a successor read through a family's `Member` at a stated index — or leaves them stale with their diagnosis. The first names one member as "the" body the copy stood for, which is a ruling about what a fused pattern is.
