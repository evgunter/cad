---
id: the-legal-operand-check-reads-only-that-a-far-union-builds
kind: issue
title: The legal-operand check reads only that a union with a far brick builds; hold it to its solid count and the brick's presence, not to a second quadrature
status: open
opened: 2026-10-11
priority: P3
cost: E
---

## What

Two helpers ask whether a boolean result is a legal operand, and both
read only that `topo::union(result, brick (50..51)³)` returns `Ok`:

- `crates/sweep/tests/common/differential.rs:417` (`outcome`, the
  `operand=` flag), read by 44 files under `crates/sweep/tests/`;
- `crates/sweep/src/test_support.rs:224-235` (`assert_legal_operand`).

A union that builds the wrong body passes both: a result read
inside-out so the union answers for its complement, or a far brick
dropped. PR 4725 re-baselined a row's `operand=false` to `true` on
this reading.

## The shape

Read the union's topology and keep the reading cheap. Every `outcome`
call is on the per-PR test shards, so the check must not add a
quadrature. Holding the union to its certified volume would run a
second `mass_properties` per row across 44 files. Instead:

- the union has exactly the result's solids plus one;
- exactly one of its solids has a bounding box equal to the far brick's
  `(50..51)³`, within the band;
- every other solid's bounding box is the result's box.

An inside-out read or a dropped brick fails one of these. Neither needs
a volume. A row that wants the volume check as well keeps doing it by
hand, as `a_plane_across_a_one_face_wall.rs` `every_op` (:104) does.

What would close it: `outcome`'s `operand` and `assert_legal_operand`
both check solid count and brick box as above, with no new
`mass_properties` call.

- 2026-11-22 — **Reformulated by CIW triage** (the earlier body is in git history): The asked-for shape adds a certified-volume read of the union to `outcome` (`crates/sweep/tests/common/differential.rs:417`), which 44 sweep test files call on the per-PR shards. That is a second `mass_properties` quadrature per row. Only one incident is named (PR 4725's re-baseline, 2026-10-11). A solid-count plus brick-box check catches both named failure modes (inside-out read, dropped brick) without a quadrature.
