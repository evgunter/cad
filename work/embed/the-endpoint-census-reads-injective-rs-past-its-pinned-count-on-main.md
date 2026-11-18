---
id: the-endpoint-census-reads-injective-rs-past-its-pinned-count-on-main
kind: issue
title: The endpoint census reads injective.rs at 173 reads, 75 guarded, against its pinned 160 and 64, so certified_endpoint_census is red on main
status: open
opened: 2026-11-18
priority: P1
cost: E
---

Found by lane `embed/conic-vertex` (2026-11-18), running the census the
brief asks of every `injective.rs` lane. The census is in the slow set
and the PR gate does not select it.

`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
reads `crates/topo/src/injective.rs` at `(173, 75)` against the
pinned `(160, 64)`. Main's own `injective.rs` (at `763d6fe0`, swapped
into the lane's checkout for one run) reads the same `(173, 75)`, so the
drift is main's: 13 reads, 11 of them guarded, added since the last
bless, most likely by PRs 7089 (`Reading::Conic`, `continues`) and 7093
(`present_vertex_corner`). The conic-vertex change moves neither count.

**What closes it:** each new site found, argued safe at its site or
asking `is_certified()` by name, the file's remainder note extended, and
the census re-blessed at what main reads.
