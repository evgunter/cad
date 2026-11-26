---
id: the-endpoint-census-blind-spot-seven-lists-too-few-path-spelled-reads
kind: issue
title: the endpoint census's blind spot 7 lists the path-spelled reads in its population as complete, and clearance's boundary_of reads one it does not name
status: open
opened: 2026-11-26
priority: P3
cost: E
---

## Finding

`crates/geom-core/tests/certified_endpoint_census.rs`'s module docs,
blind spot 7, say a read spelled as a path (`Bounds::lo(x)`) is not
counted, and then name "the production ones in the population":
`ssi/certify.rs`'s `certify_branch`, `edge_nurbs.rs`'s per-sample lever
and `interval.rs`'s own body. The list is read as complete and is not:
`crates/topo/src/clearance.rs`'s `boundary_of` (in the population)
writes `curve.carrier().map_scalar(|x| Bounds::lo(x))` into the rim's
`stated` carrier.

That read is sound: it runs only where `T::WITNESS ==
Witness::Inexact`, a point scalar whose low end is the value itself,
and `injective.rs`'s `one_conic` reads the result only as an identity
(`one_stated_conic`), so no certification bracket is read. What is
wrong is the census's sentence, which claims an enumeration it did not
take.

Found by the EMBEDWALK census-two-bands lane's second sweep pass (the
path-spelled shape its `.lo()`/`.hi()` sweep could not match).

Fix: add `clearance.rs`'s `boundary_of` stated read to blind spot 7's
list with its point-scalar argument, or word the list as examples
rather than the whole population. A grep for `Bounds::(lo|hi)` over the
population, taken at the fix's merge base, is the receipt.
