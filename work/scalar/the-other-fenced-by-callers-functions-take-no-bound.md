---
id: the-other-fenced-by-callers-functions-take-no-bound
kind: issue
title: The functions fenced from a dual only by their callers, other than carrier_span, take no bound; and carrier_span's bound has two readings
status: open
opened: 2026-11-20
priority: P2
cost: M
parent: from-computed-claims-a-zero-tangent-at-a-dual
---


From `certification-interiors-are-closed-to-a-dual-by-a-marker-bound` and its review (PR 7190), 2026-11-20.

## What

The unit said "`carrier_span` and the other 'fenced by its callers' functions take the bound here". The PR bounded `carrier_span` and no other, because nothing lists the others. They are the certification routines that name no lane and whose bound admits a dual, kept from one only by who calls them. The first-wave list measured by type-check is in `the-certification-files-hold-body-data-beside-certification` and `spline-incidence-is-reached-from-incidence-readers-open-to-a-dual`; that list covers five files of geom-brep and nothing in topo, sweep or mesh.

## `carrier_span`'s bound: two readings

The PR wrote `carrier_span<T: CertifiedBounds>` (`crates/geom-brep/src/carrier_net.rs`), a sole bracket door.

- For: it keeps a dual out with no entry on the bounds allowlist, where `T: Certifies + Bounds` is the compound form and would need a pin. All seven callers already carried `CertifiedBounds`.
- Against: the body reads `lo`/`hi` and calls `from_computed`; it never calls `certified_bracket`. `CertifiedBounds` grants a certified-enclosure right the body does not use, so the exclusion was bought with a wider capability, where the scope rule says to write the bound that states which scalars the code means something at.

The function also left `geom-core/tests/bounds_census.rs`'s roster of sole `T: Bounds` doors and is on no roster now.

## What it asks

The list of fenced-by-callers functions outside the five files, each bounded or filed; and a ruling between the two readings above, which decides how every such bracket-reading function is spelled.
