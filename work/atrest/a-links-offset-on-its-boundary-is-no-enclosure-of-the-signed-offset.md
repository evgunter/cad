---
id: a-links-offset-on-its-boundary-is-no-enclosure-of-the-signed-offset
kind: issue
title: Link::offset reads the unsigned distance for a point decided on a sector, which does not enclose the signed offset
status: open
opened: 2026-11-01
---


`Link::reading` (`crates/topo/src/link.rs`) maps `Material::On(_)`, a point decided on a sector's plane strictly inside it, to `(d, Some(false))`, so `Link::offset` reads `+d`, the distance itself. A point decided on the plane is within the band's zero of it on either side: one 1e-9 inside a ridge's face reads `+1e-9`. Under an enclosure scalar, `+d` is then no enclosure of the true signed offset, which spans `[-d, d]`. A point on no sector, in band (`touches`), reads `either(d, -d)`, the hull, and so does a point on a bound (`Material::Boundary`). `Link::lowest_on` also reads the `Some(false)` as membership, and uses it to settle the refused candidates near it.

The inside case predates ONEREAD's Cone/Link split (PR #6228). The split moved the bound case to `Material::Boundary`, which reads the hull as it did before. The fix is probably to read `On` as `touches` reads (hull, membership `None`). Check `Gap`'s `gap_reads_vertex_and_edge_cells` rows at interval first.

Filed by ONEREAD lane 1, from the dual review of PR #6228 (W4).
