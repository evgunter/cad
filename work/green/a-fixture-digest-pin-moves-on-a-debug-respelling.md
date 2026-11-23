---
id: a-fixture-digest-pin-moves-on-a-debug-respelling
kind: issue
title: A fixture::digest pin goes red when a geometry type's Debug spelling changes, with no value moved
status: open
opened: 2026-11-22
priority: P3
cost: M
---


`fixture::digest` (`crates/editor-core/tests/fixture/digest.rs`) feeds
geometry in through `Debug`. A change that only respells a type,
such as PR 7131's `Plane { origin, normal, u_ref }` to
`Plane { frame: Frame { origin, axis, u_ref } }`, moves every digest
that walks a plane. Those reds fall outside each pin's stated *Breaks if*.
The kite moat's pin (`split_pinch_keeps.rs:1606-1616`) was red for
that reason alone. PR 7400's STYLE review established this by dumping
the digest's chunks on both sides of 7131: the dumps differ in 227
lines, all of them `Plane` respellings, and each side is byte-identical
to the other after rewriting into the old spelling. The doc at
`fixture/digest.rs` does not state this cost, and no one owns a
respelling map. Every future `Debug` change on a geometry type
therefore costs a hand bisect.

The fix could digest point bits and curve parameters directly instead of
their `Debug` text, or keep `Debug` and state the cost where the doc
says geometry enters. Either way, the choice belongs to whoever owns the
fixture.

The same pin caught a real value move later. PR 7411 (`3547fe5764`)
changed `plane_offset` to a `Dot2` read. That moved 102 of the 2061
chunks: 96 certificate `max_residual` readings, 2 vertex bits and 12
plane–plane line chunks. Every move was toward the exact value. PR
7597 re-pinned it to `0xfe07_0805_06b3_9e32`, with the evidence on
`main-red-the-kite-moats-digest-moved-in-value-at-pr-7411`. Telling
the two kinds of move apart took a chunk dump and a Python replay
against exact fractions. Any fix should keep the value channel and
shed only the spelling channel.
