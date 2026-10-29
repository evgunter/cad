---
id: a-held-germ-can-land-on-a-record-the-lump-cancelled
kind: issue
title: A held germ can land on a record 15.10 cancelled, whose uniform side mark_germ cannot clean
status: open
opened: 2026-10-29
---


Found by the sweep of the fix for
`a-boss-inside-a-hole-touching-along-short-arcs-breaks-a-germ-invariant`
(branch `splitdoor/boss-in-hole-germ`). Unmeasured: no body is known to
reach it.

`crates/topo/src/boolean/recl.rs` `find_on_record` picks the germ host
for an edge-sector event (`resolve_edge_sector`'s `EdgeGerm::Held`) and
for a bisector graze (`resolve_bisector_graze`): any record of the pair
that still carries an `On` code, whether or not it still intersects. Its
doc says "cancelled/rewritten records are never resurrected as germs",
on the premise that a cancelled record carries no `On`. That premise
does not hold: `cancel_uniform` cancels a record when EITHER side reads
uniform, and the two neighbour rewrites in `recl_sectors` (the
declared-`Tangent` per-bound propagation and the Eq. 15.3 lump's)
rewrite one bound of one side, so a neighbour can leave 15.10 with one
side uniform (`(Out, Out)`) and an `On` still on the other. Such a
record is never a mention (`recl_edges` collects mentions from
`intersect` records only), but an event from another record can still
hold its germ there, and `mark_germ` then refuses
`ClassificationInvariant { what: "germ record without a clean In/Out
transition after rewrite" }`: it rewrites only `On` codes, and the
uniform side has none.

The boss-in-a-hole pose reached the same refusal by a different road
(`sectors::pair_search` recorded a pair whose one side read uniform);
that is fixed at `pair_search`, and does not cover this one.

To do: build a witness (a coplanar lump beside an edge-sector or
bisector-graze event at one vertex pair, the neighbour's other side
`On` along the event's ray) before changing `find_on_record`; requiring
`r.intersect` there is the obvious repair, and what it changes for a
germ that today lands on an event-cancelled record is the question.
