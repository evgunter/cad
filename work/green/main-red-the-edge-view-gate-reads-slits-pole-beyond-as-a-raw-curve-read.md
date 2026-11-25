---
id: main-red-the-edge-view-gate-reads-slits-pole-beyond-as-a-raw-curve-read
kind: issue
title: topo's live::tests::production_reads_an_edges_curve_only_through_the_view is red on main: slit.rs's pole_beyond reads the curve arena raw
status: open
opened: 2026-11-25
---


Found by `boxfinish/wall-roots`'s differential, red on main at
`f21ca495ca` and still red at `561b1af820` (main merged in), with the
same payload on both trees:

> an edge's curve is read through `EdgeView` (`Body::edge`,
> `proven_edge`, `linked_edge`, `edge_views`); a raw read is listed here
> only for a reader that reports its own miss:
> topo/src/boolean/slit.rs:1317 (`pole_beyond`) reads the curve arena raw
> (`.curves.get(`)

The gate is `topo::live::tests::production_reads_an_edges_curve_only_through_the_view`
and the reader is `topo::boolean::slit::pole_beyond`. That branch's diff
does not touch `topo`. Main at `1e5b1ce503` was green on this row, so a
merge between `1e5b1ce503` and `f21ca495ca` added the read; the merge
was not bisected.
