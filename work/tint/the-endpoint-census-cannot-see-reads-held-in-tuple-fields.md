---
id: the-endpoint-census-cannot-see-reads-held-in-tuple-fields
kind: issue
title: certified_endpoint_census counts .lo()/.hi() calls, so an endpoint read copied into a tuple field and compared there is invisible to it
status: open
opened: 2026-10-14
---


(Found at the FULL review of PR 5094, `nurbs/pair-walk-domain-side-end`.)

## What

`crates/geom-core/tests/certified_endpoint_census.rs` (`every_production_endpoint_read_is_counted_and_dispositioned`) counts endpoint reads by their call sites. In PR 5094, `ssi/four.rs`'s Krawczyk test changed shape:
- before, it compared `v.lo()` and `v.hi()` against the piece directly;
- after, `image` copies them into `k_box`'s `(f64, f64)` fields once, after `is_certified()`, and `krawczyk_with` compares those fields.

The file's count fell from 5/4 to 3/2. The containment reads did not go away: they moved to a place the census cannot see.

That case is safe, because the copy happens past the certification check. But the same shape could carry an uncertified bracket's endpoints out of view: an `Interval` read into a tuple or array, then compared, with no `is_certified()` asked.

## What is open

Either the census follows endpoint values into the fields they are stored in, or its docs name this as a blind spot with the pattern a reviewer greps for: a `(x.lo(), x.hi())` stored and later compared.
