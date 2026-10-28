---
id: a-cut-through-an-instance-drops-its-parts-findings
kind: issue
title: A cut through an instance drops the part's carried findings, and the cut's door decides the part's overlap again as the host's
status: open
opened: 2026-10-28
priority: P2
cost: M
refs: [a-placement-carries-its-sources-at-rest-result, each-space-runs-its-census-between-arrivals]
---

Found by the carry (`a-placement-carries-its-sources-at-rest-result`),
sweeping the readers of a carried value.

A part `a ⊔ a'` whose blocks overlap, instantiated in a host, and the
host's `instance − tool` (a tool clear of both blocks). The cut
distributes over the instance's pieces
(`crates/editor-core/src/eval/wire/distribute.rs`, `distribute`): each
piece's result is a `Piece` whose carry is the piece boolean's own
output, so the instance's carried findings (`Carry::findings`, the
part's rows routed through the instance) are not on any piece, and the
cut's value carries none. The cut's door then censuses the cut pieces
and finds the part's overlap again, reported as the host's, under the
cut's read: the at-rest check over that host says
`read <cut>: read e8640e6f2244 and read c220056abee7 overlap, …`
where the instance alone says `document … through instance … reports
an overlap between its own arrivals`.

Per D10 `[assertions]` the overlap is the part's decision and is
reported once there, and per `[operations.operators]` a meeting inside
an operand is never changed by an operator above it. Whether a cut
door's census over pieces it cut decides a new meeting, or confirms
the operand's carried one, is the census-per-`⊔` unit's question
(each `⊔` decides its operands' meetings; the transitional check
matches carried findings): `each-space-runs-its-census-between-arrivals`.


