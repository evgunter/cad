---
id: an-aligned-in-band-pair-is-never-read-as-interference
kind: issue
title: an aligned in-band face pair is never read as interference: the region door's carrier gate escalates on every pair standing off in band
status: open
opened: 2026-10-26
priority: P2
cost: M
refs: [an-in-band-face-pair-at-rest-is-not-one-finding]
---


The corrected ruling on PR 5782 reads an aligned planar pair whose Door 1
is `Undecided` by the area the two faces share
(`crates/topo/src/census.rs` `in_band_reading`):
- the region door reads a positive area: `SameSideInterference`;
- the area is empty or thinner than the band: `ThinInBand`;
- otherwise: `MayOverlap`.

For every pure offset, the first branch is unreachable today; for a tilted pair it is unproven either way. `same_side_overlap` calls
`RegionLane::declared_overlap` with a `Bridged` verdict. The planar
door's carrier gate escalates (`chart_region_carrier_tilt`) on every pair
whose carriers stand off in band, so it never returns a positive area.
`thin_overlap` catches the empty case, so every aligned in-band overlap
whose two faces stand off by a pure offset reads `MayOverlap`.

A tilted pair that Door 1 reads in band is unproven. Its carrier gate
might read Zero and reach the region walk (PR 5782's second review).
The row stands as "every pure offset; tilts unproven". That is safe, because it blocks the
records, but it is never decided as the interference it is, so no
assertion can quiet it as one.

Reached by `census_records_what_it_decides::an_overlap_whose_aligned_faces_are_in_band_is_never_cleared`:
two blocks sharing a unit cube, lifted by an in-band offset so their
caps are aligned in band.

The fix: a region reading across an in-band offset. For example, project
one face onto the other's plane and read the overlap in that one chart,
the offset priced as Door 1's in-band margin. Or a positive-area witness:
a point decidedly inside both faces' projections.
