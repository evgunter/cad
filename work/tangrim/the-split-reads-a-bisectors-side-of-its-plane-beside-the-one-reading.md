---
id: the-split-reads-a-bisectors-side-of-its-plane-beside-the-one-reading
kind: issue
title: splitting::wide_entries reads a bisector's side of the split plane itself (split_bisector_side), beside bound_side::read
status: open
opened: 2026-11-01
priority: P2
cost: M
parent: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
---


Items 5 and the residue of 9 of
`the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading`,
not built in stage 2 (`tangrim/sector-passes-descend`).

`splitting::neighborhood::wide_entries` decides a subdivision
direction's side of the split plane itself:
`decide("split_bisector_side", Margin::levered(d·n̂, arm))`. That is the
sign `bound_side::read` gives with a bisector reach against a plane
datum, and its tie, since stage 2, descends by the bisector's sector
face's bend (`Reach::Bisector`'s bend, `bound_side::Reach::bisector`),
where `wide_entries` reads `On`. It is also the fourth spelling of
`d̂·n̂·arm`: stage 2 routed the other three (`bound_side::rise`,
`geom_brep::sense_margin`, `conic_bend_side`'s residue) through
`geom_brep::rise`. It was left because `split_bisector_side` is a
K-REPORT / k-lint row (`docs/K-REPORT.md`, `tools/k-lint`): moving it
renames a tracked predicate, and the baseline is re-derived with it.
