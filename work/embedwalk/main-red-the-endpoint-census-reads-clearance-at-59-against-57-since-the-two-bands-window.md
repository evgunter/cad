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

## 2026-11-26: the two reads, read off the census

The guess above was half right. A per-function count of
`clearance.rs`'s production `.lo()`/`.hi()` sites at the roster's
commit (`8fc1fdf768`, 57) against main (59) moves one function alone:
both new reads are `lifted_ends`'s (`Interval::from_certified(lo).lo()`
and `Interval::from_certified(hi).hi()`), which PR 7786 added. `window_of`'s
own count did not move, and the sheet arm is one of `lifted_ends`' TWO
callers: the torus arm's `within_turn` is the other, where the ends are
compared against a turn's width, not against `±π`.

Neither read is sound as written, so the fix is not the disposition
this row proposed. `from_certified` keeps a refused bracket's ordinary
endpoints under `Trv`, and the reads took them without asking: nothing
upstream refuses an uncertified end (`cover::widened` asks only
`is_bounded`), so a window end that does not certify narrowed the
torus or sheet channel, the certifying direction of a superset claim.
`lifted_ends` now reads both ends through `certified_bracket`, the
refusal door, and an end that does not certify leaves the channel its
canonical range. The census reads `clearance.rs` at (57, 16) again,
and its roster row stands.
