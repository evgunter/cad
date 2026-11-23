---
id: an-in-band-pair-every-touch-of-which-a-seat-holds-is-silent
kind: issue
title: an in-band face pair whose every touch a confirmed pair holds reports nothing, since the flush door leaves Door 1's in-band pairs to the census
status: open
opened: 2026-10-26
priority: P3
cost: E
refs: [an-in-band-face-pair-at-rest-is-not-one-finding]
---


The census reports an in-band face pair (`topo::Undecidable::ThinInBand`)
only where a sweep found something between its cells
(`crates/topo/src/census.rs` `Declared::absorb`: "A lane that finds
nothing reports nothing"). A lane that a confirmed pair answers
(`Declared::confine`) finds nothing. And `flush_at_rest` no longer
reports Door 1's in-band pairs (`FLUSH_UNDECIDED` on Door 1's `Undecided`
or `Escalated`). So a near-flush pair is silent, neither a flush row nor
a finding, if every touch between its cells is held by a confirmed pair.

It is not reached by two walls standing off by less than the band beside
a seat (`census_records_what_it_decides::a_near_flush_side_is_its_pairs_finding_and_the_seat_is_one_pair`):
an edge of one wall lying along the other wall's face is held only by a
confirmed pair naming that face, and a seat names neither wall. Reaching
it needs the in-band face itself in a confirmed area pair with a face of
the other solid. No fixture builds that yet.

The fix, if a fixture reaches it: report an in-band pair whose faces'
boundaries meet in band (the flush door's `parallel_overlap` test), as
well as one whose cells' touches were absorbed.
