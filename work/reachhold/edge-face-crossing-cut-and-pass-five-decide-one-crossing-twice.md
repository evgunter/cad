---
id: edge-face-crossing-cut-and-pass-five-decide-one-crossing-twice
kind: issue
title: A straight crossing is decided twice, by the edge-on-face cut's side rows and by pass 5's gap and span rows, and nothing ties the two
status: open
opened: 2026-10-08
priority: P3
cost: M
---


Found by CONTACT-12's review (branch `contact/12-ef-crossing-cuts`).

Two lanes decided whether a straight boundary edge of a face crosses a
census edge lying in the face's plane:
- `census.rs` `chord_crossing`, the edge-on-face lane's cuts, read
  `pm_census_ef_cross_side` (the ends' signed in-plane distances from
  the edge's line) and then `pm_census_ef_cross_span`. Both are retired
  (below).
- Pass 5 (`pair_edge_edge` / `crossing_in_both_interiors`) reads
  `pm_census_ee_parallel` (levered), `pm_census_ee_gap` and
  `pm_census_ee_span`.

The cut's bound at that crossing is backed through pass 5's own backing
(`ee_recorded`, `ee_cross_backed`). Nothing ties the two readings
together, so near the band one lane can call a crossing that the other
does not. A tie would read one decision in both places, for example the
edge-on-face lane asking pass 5's `crossing_in_both_interiors`, or both
reading the same metric rows. No configuration that separates them has
been built.

## Where it stands

The edge-on-face cut no longer reads its own side rows: `census::edge_crossings` reads `geom_brep::curve_pair::meet`, the one reading of two spans' meetings (stage (c) of `the-curve-pair-reader` (closed, PRs 5154, 5162 and 5227)). Pass 5 (`crossing_in_both_interiors`) and the line lane's confirm (`interiors_meet`) still read their own rows for two lines. The tie this row asks for is pass 5 and the line confirm reading `meet` too.
