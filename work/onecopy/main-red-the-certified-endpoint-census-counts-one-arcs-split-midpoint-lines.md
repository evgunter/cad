---
id: main-red-the-certified-endpoint-census-counts-one-arcs-split-midpoint-lines
kind: issue
title: main red: certified_endpoint_census reads ssi/one_arc.rs at 20/13 against its pinned 18/12, two midpoint lines split by 4566f51520
status: open
opened: 2026-11-10
priority: P2
cost: C
---


Found by ONECOPY's roster-reach unit, running the slow set on main's tree, and bisected there.

`crates/geom-core/tests/certified_endpoint_census.rs::every_production_endpoint_read_is_counted_and_dispositioned` is red at every ε. It reads `crates/geom-brep/src/ssi/one_arc.rs` at 20 read lines, 13 of which ask, against its pinned `(18, 12)` at `certified_endpoint_census.rs:520-522`. The census counts LINES carrying `.lo()` or `.hi()` (`:946`), not reads.

**Bisected to 4566f51520** ("onecopy: route hand-spelled bracket middles to their homes, gate them", row `a-bracket-middle-is-also-spelled-half-lo-plus-half-hi-and-f64-midpoint`). That commit routed the one-line midpoints `(0.5 * (p.x.lo() + p.x.hi()), 0.5 * (p.y.lo() + p.y.hi()))` and `(0.5 * (xi.lo() + xi.hi()), 0.5 * (xj.lo() + xj.hi()))` in `one_arc.rs` through `geom::mid_param`. rustfmt then split each across two lines: same reads, two more read lines. The other one_arc.rs commits in range, 9bd2ded37d and de3229faf1, move no `.lo()`/`.hi()` line count. No read was added or removed; this is a reformatting the line-keyed census sees as a move.

It slipped through because the census is in the slow set, and 4566f51520 did not spell it.

**Done** when the pin reads `(20, 13)` with its note's count of asking reads updated, after checking which of the two new lines the census classes as asking. Alternatively, the census could count reads rather than lines, which would make a reformat move nothing.
