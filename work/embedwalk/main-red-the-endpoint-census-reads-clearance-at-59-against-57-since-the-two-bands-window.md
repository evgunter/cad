---
id: main-red-the-endpoint-census-reads-clearance-at-59-against-57-since-the-two-bands-window
kind: issue
title: main red: the endpoint census reads clearance.rs at 59 endpoint reads against its pin of 57 since the two bands' window (PR 7786) landed beside the trimmed free-form window (PR 7787)
status: dispatched
opened: 2026-11-26
priority: P1
cost: E
branch: embedwalk/census-two-bands
---


## Finding

Nightly run 45761344079 (main `cb4ff30253`, 2026-11-26), job
`full suite (eps default, slow set included)`:
`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
reads `crates/topo/src/clearance.rs` at (59, 16) against its roster's
(57, 16). PR 7787 set the roster to 57 on its head, before PR 7786
landed; 7786's window code (the sheet arm's latitude met with
`sheet_latitude`, and `lifted_ends`) adds the two reads, and 7786's own
gate never selected the census row (a slow test in a file it did not
edit). The census's other diffs in the same run (`ssi/certify.rs`
27/23, `spline/compose.rs` 1/0) are GREEN's
`main-red-the-endpoint-census-misses-ssi-certify-and-spline-compose-reads`.

Fix: disposition the two new reads in the census roster and its prose
(both are on lifted f64 ends of a certified enclosure, compared against
±π to filter to the whole sheet), and move clearance.rs's row to 59.
