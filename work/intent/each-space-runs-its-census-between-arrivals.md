---
id: each-space-runs-its-census-between-arrivals
kind: unit
title: Each space runs its at-rest census between its arrivals; the graft supplies the arrivals and record_at_rest takes them
status: open
opened: 2026-10-19
priority: P1
cost: H
refs: [a-census-contact-is-a-row-and-a-record, a-placement-carries-its-sources-at-rest-result]
---

D10 `[assertions]` (PR 4666, Ev 2026-10-19: "the second seems solidly better, yes"): each space's census examines its carried bodies; between two arrivals it decides and records; inside one it decides nothing and reads nothing. This unit lands a transitional check inside each arrival (confirm carried records, match carried findings, report anything else as that arrival's door's defect, typed); `the-in-arrival-census-check-retires` removes it. A finding is reported once, as the deciding space's. A space outside the product is checked too.

Today the census runs once per document, at the product gather: `product_recorded` calls `topo::record_at_rest(body, records, groups)` with each world copy as its own group (`crates/editor-core/src/product.rs`). What changes:
- the graft that assembles a space's carried bodies returns the aggregate with its arrivals (one placement's copies at one index value, or one op's output), and `record_at_rest` takes that one value. The `groups` closure and the "in no group" arm retire;
- a census per space with two or more arrivals, memoised with the space's inputs. A space placed n times is decided once;
- a finding names its cells on the space's bodies (PR 5315's picks, or the pick's definition where nobody picked the copy);
- the unproven-coincidence lint reports a space's findings once, in that space.

Open measure: `assemble-aggregate-census-is-quadratic-in-solids` for spaces with many arrivals. Also check that `Measure`'s door refuses a `Gap` whose two refs are in different spaces (`crates/editor-core/src/measure.rs`); "an assertion quiets only findings of its own space" rests on it.
