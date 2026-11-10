---
id: the-certified-endpoint-census-counts-lines-not-reads
kind: issue
title: The certified-endpoint census counts lines carrying .lo()/.hi(), not reads, so a rustfmt reflow moves its pins
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by ONECOPY's endpoint-census-reads unit, fixing `main-red-the-certified-endpoint-census-counts-one-arcs-split-midpoint-lines`.

`crates/geom-core/tests/certified_endpoint_census.rs::every_production_endpoint_read_is_counted_and_dispositioned` counts production LINES carrying `.lo()` or `.hi()` (`:946`), as its own docs say ("a LINE count, not a site count"; blind spot 4). So the census answers a formatting question: rustfmt splitting one line that spells two reads moves a pin without moving a read. That is how 4566f51520 reddened it, and that unit re-pinned it.

**Measured, 2026-11-10:** counting occurrences instead (one per `.lo()`/`.hi()` on a line) moves **42 of the 51** pinned files. Examples: `topo/src/injective.rs` (70, 27) → (121, 40); `ssi/one_arc.rs` (20, 13) → (47, 34); `clearance.rs` (30, 7) → (53, 16). Every one of those pins carries a prose note that splits its count into per-function categories ("the Krawczyk test's 5 …, `farthest`'s 2 …"). The new numbers are mechanical, but recounting 42 notes by category is the work, and it is why this is its own row.

**The shape that ends it:** count parsed read sites (`.lo()`/`.hi()` method calls in the CODE view), each attributed to its enclosing function by its item extent rather than by the nearest `fn` line above. Re-pin all 51 with their notes recounted, and retire blind spot 4. A reflow then moves nothing.

**Done** when the census counts reads, its pins and notes match, and a plant splitting a two-read line in a pinned file leaves it green.
