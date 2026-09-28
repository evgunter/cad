---
id: product-doc-states-a-tier-3-prime-cost-sixty-times-the-measured-one
kind: issue
title: product.rs's module doc prices tier 3′ on the heat sink at ~11.4 s; the hosted series has measured ~180 ms since
status: closed
opened: 2026-09-28
priority: P3
cost: E
branch: gather/product-doc-cost-figure
closed: 2026-09-28
pr: 3378
---


Found while measuring `gather/assemble-single-local-battery`.

## The finding

`crates/editor-core/src/product.rs`'s module doc (the "division of
labour, not a gap" paragraph) justifies keeping the gather on tier 3's
local battery by what tier 3′ costs: "the heat sink at 160 fins (161
solids / 991 faces) costs ~11.4 s there — refusing, with 125 findings
— where THIS gather costs ~250 ms". The paragraph names its figures of
record as the hosted `registry split` row of
`crates/editor-core/tests/m4_pr8_latency.rs`, appended to
`docs/perf-data/rebuild-latency/`. The two newest entries there
(`1790505455-d0e3980.json`, `1790596034-793b5c4.json`) put that term
(`census_ms`, 125 findings, 991 faces) at 194 ms and 176 ms. A local
dev-profile run on the assemble branch measured the whole tier-3′ door
at 185–246 ms and the gather at 117–171 ms.

So the census is about as expensive as the gather now, where the
paragraph says it is about 45 times more. The division of labour may
still be right, since it also rests on tier 3′ being quadratic in the
aggregate. But its stated reason is sixty times off, and a reader
weighing "gate the product at 3′ once" against the current split is
reading the wrong number.

Note for whoever re-takes it: from `gather/assemble-single-local-battery`
on, `census_ms` times `gate_at_rest_declared` over the product's kept
verdict, which is the census alone and not battery plus census. The
hosted series drops by the battery's share at that merge, which was
~60–100 ms locally.
