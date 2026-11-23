---
id: continues-reads-its-directions-product-at-its-lower-end-without-asking
kind: issue
title: continues decides that an arc leaves its vertex the way the shared arc arrives by the lower end of an enclosure no ask covers
status: open
opened: 2026-11-17
priority: P2
cost: C
refs: [injective-reads-past-the-trv-census-decide-on-clamped-enclosures]
---

## Finding

`topo::injective::continues` (PR 7089) decides its direction check as
`(de.dot(d_o) * at(-1.0)).lo() > 0.0`. That is a one-sided read of an
enclosure of the two arcs' derivatives. It neither asks
`is_certified()` nor goes through a `decide` site. A bracket that may
not certify carries ordinary endpoints, so a lower end read this way
can take the certifying branch (the endpoint census's rule).

The check is load-bearing. The continued-conic reading's argument rests
on `continues`: `o` does not lie back over `e`.

## Evidence

The endpoint census (`crates/geom-core/tests/certified_endpoint_census.rs`)
counts it as one of `injective.rs`'s unasked reads. PR 7093 re-blessed
the file at 160 reads, 64 asking, and names this read there. Main alone
read 153 against its pin of 154 after PR 7089. That is PR 7089's net
change of −1, and that PR's diff did not re-bless the census.

## What a fix needs

Ask `is_certified()` on the product before reading it, or decide it at
a named site with a margin, as the other corner readings decide theirs.
