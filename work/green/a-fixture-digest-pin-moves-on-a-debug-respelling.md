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
