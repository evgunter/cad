---
id: loft-builds-at-interval-and-refuses-a-rational-column-at-sym
kind: issue
title: A three-section loft builds at Interval and refuses its line seam on a rational chart column at Sym<Interval>, over the same box
status: open
opened: 2026-10-28
priority: P2
cost: M
---


Found by SYMCLEAR (CLEAR, PR 5928), measured on 2026-10-28.

The fixture is `m10_5_clearance_interval.rs`'s
`a_lofts_walls_have_windows_and_its_clearance_holds`: three rectangle
sections at z = 0, 1, 2, scaled 1.0 / 1.6 / 1.0, lofted with
`v_degree: 2`, over the ε-scaled box of a `place` parameter no node reads.

- At `Interval` the loft builds.
- At `Sym<Interval>` (shipped budget, `with_session`) it fails after 21 s
  (debug build) with `Loft(Pcurve(Certify { error: IsoUnsupported { what:
  "a LINE seam on a RATIONAL chart column: …" } }))`, from
  `PcurveCache::certify`'s `'seam` block (`crates/geom-brep/src/pcurve_cache.rs`).
  The session's `SymCounts`: `symbolic: 405, numeric: 314,
  checks_expanded: 195, checks_frozen: 1, frozen: 3642`.

So the two lanes disagree on which shape the loft's chart column is.
The tier changes how a margin decides, and here a decision the tier
answers (`symbolic: 405`) sends the seam to the rational arm the
numeric lane does not reach. Either the tier proved a weight
relation that makes the column read as rational, or the limb's
rational test reads the column differently at `Sym`. Not run further
than this.

It matters because `DriveConfig::default()` drives at the tier, so a
leaf of a drive over a loft would refuse where the numeric replay
builds. The clearance row asks on the numeric lane by name
(`ClearanceQuery::certified_at(SymbolicDials::off())`) and cites this
file. Close this by making the two lanes agree, then dropping the dial
from that row.

- 2026-11-02 — moved from PCTAIL to PCERT by `git mv` when PCTAIL merged into PCERT, id and body unchanged (the track budget now counts only P0, P1 and unbanded rows, and PCTAIL had fallen far under it).

## A second witness, newly reachable (PR 6528's review, 2026-11-04)

Since PR 6528 a refined interior arc rim is read (`Pcurve::IsoArc`'s
`ends`), so degree-1 lofts that refused at their section rim now build,
and one of them meets this row's split. A square whose +x side has
bulges `[1, 0.2, 0, 0.7, 1]` at z = `0..4`, `loft_body` at v-degree 1,
builds at `f64` and at `Interval`, and refuses at `Sym<f64>`
(`SymBudget { max_terms: 4096, max_degree: 128 }`) with

    the pcurve of half-edge HalfEdgeKey(36v1) failed certification:
    pcurve certification: the iso-line lane refuses this class — a LINE
    seam on a RATIONAL chart column: the Greville hull is a
    linear-precision fact and the rational basis has none — …

`[1, 0.2, 1]` builds at all three scalars. Measured with the reviewer's
probe (`review/6528-full` at `5a1b5e4b88`,
`crates/sweep/tests/rev6528_probes.rs::probe_refined_loft_at_each_scalar`).
It is not pinned as a row: the `Sym` loft costs about five minutes.
